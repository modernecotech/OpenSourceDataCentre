#!/usr/bin/env bash
set -euo pipefail

ring="RING_DEV"
strict_security=0
evidence_dir=""

usage() {
  cat <<'EOF'
usage: scripts/assurance-run.sh [--ring RING_ID] [--strict-security] [--evidence-dir DIR]

Runs the local OSDC assurance gate set and writes an evidence bundle.

Options:
  --ring RING_ID        Upgrade ring being tested. Default: RING_DEV.
  --strict-security    Fail if an installed optional security scanner fails.
  --evidence-dir DIR   Write evidence to DIR. Default: target/assurance/<timestamp>.
EOF
}

while (($#)); do
  case "$1" in
    --ring)
      ring="${2:-}"
      shift 2
      ;;
    --strict-security)
      strict_security=1
      shift
      ;;
    --evidence-dir)
      evidence_dir="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$ring" ]]; then
  echo "--ring requires a non-empty value" >&2
  exit 2
fi

if ! awk -F, -v ring="$ring" 'NR > 1 && $1 == ring { found = 1 } END { exit found ? 0 : 1 }' data/software/upgrade-rings.csv; then
  echo "unknown ring: $ring" >&2
  echo "known rings:" >&2
  awk -F, 'NR > 1 { print " - " $1 }' data/software/upgrade-rings.csv >&2
  exit 2
fi

timestamp="$(date -u +"%Y%m%dT%H%M%SZ")"
if [[ -z "$evidence_dir" ]]; then
  evidence_dir="target/assurance/$timestamp"
fi

mkdir -p "$evidence_dir"
manifest="$evidence_dir/evidence-manifest.tsv"
command_log="$evidence_dir/commands.log"
report="$evidence_dir/README.md"
git_status_file="$evidence_dir/git-status.txt"

printf 'phase\tstep\trequired\toutcome\tartifact\tcommand\n' > "$manifest"
: > "$command_log"
git status --short > "$git_status_file"

overall_status="pass"
required_failures=0
security_warnings=0

record_step() {
  local phase="$1"
  local step="$2"
  local required="$3"
  local outcome="$4"
  local artifact="$5"
  local command="$6"
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$phase" "$step" "$required" "$outcome" "$artifact" "$command" >> "$manifest"
}

run_required() {
  local phase="$1"
  local step="$2"
  local artifact="$3"
  local command="$4"
  local log_file="$artifact.log"

  echo "[$phase] required: $step" | tee -a "$command_log"
  echo "+ $command" >> "$command_log"
  if bash -lc "$command" > "$artifact" 2> "$log_file"; then
    record_step "$phase" "$step" "yes" "pass" "$artifact" "$command"
  else
    local rc=$?
    overall_status="fail"
    required_failures=$((required_failures + 1))
    record_step "$phase" "$step" "yes" "fail:$rc" "$artifact" "$command"
  fi
}

run_optional() {
  local phase="$1"
  local step="$2"
  local availability="$3"
  local artifact="$4"
  local command="$5"
  local log_file="$artifact.log"

  if ! bash -lc "$availability" >/dev/null 2>&1; then
    record_step "$phase" "$step" "optional" "skipped:not-installed" "$artifact" "$availability"
    return 0
  fi

  echo "[$phase] optional: $step" | tee -a "$command_log"
  echo "+ $command" >> "$command_log"
  if bash -lc "$command" > "$log_file" 2>&1; then
    if [[ ! -f "$artifact" ]]; then
      cp "$log_file" "$artifact"
    fi
    record_step "$phase" "$step" "optional" "pass" "$artifact" "$command"
  else
    local rc=$?
    security_warnings=$((security_warnings + 1))
    if ((strict_security)); then
      overall_status="fail"
      required_failures=$((required_failures + 1))
      record_step "$phase" "$step" "strict" "fail:$rc" "$artifact" "$command"
    else
      record_step "$phase" "$step" "optional" "warning:$rc" "$artifact" "$command"
    fi
  fi
}

run_required "build" "cargo_fmt" "$evidence_dir/cargo-fmt.txt" "cargo fmt --check"
run_required "build" "cargo_test" "$evidence_dir/cargo-test.txt" "cargo test"
run_required "metadata" "repository_verify" "$evidence_dir/repository-verify.txt" "scripts/verify.sh"

run_optional "supply-chain" "cargo_audit" "cargo audit --version" "$evidence_dir/cargo-audit.txt" "cargo audit"
run_optional "supply-chain" "syft_spdx_sbom" "command -v syft" "$evidence_dir/osdc-sbom.spdx.json" "syft dir:. -o spdx-json='$evidence_dir/osdc-sbom.spdx.json'"
run_optional "supply-chain" "syft_cyclonedx_sbom" "command -v syft" "$evidence_dir/osdc-sbom.cdx.json" "syft dir:. -o cyclonedx-json='$evidence_dir/osdc-sbom.cdx.json'"
run_optional "scan" "trivy_filesystem" "command -v trivy" "$evidence_dir/trivy-fs.sarif" "trivy fs --format sarif --output '$evidence_dir/trivy-fs.sarif' ."
run_optional "scan" "grype_directory" "command -v grype" "$evidence_dir/grype.json" "grype dir:. -o json > '$evidence_dir/grype.json'"
run_optional "scan" "osv_scanner_source" "command -v osv-scanner" "$evidence_dir/osv-scanner.json" "osv-scanner scan source . --format json > '$evidence_dir/osv-scanner.json'"
run_optional "scan" "gitleaks_source" "command -v gitleaks" "$evidence_dir/gitleaks.json" "gitleaks detect --source . --report-format json --report-path '$evidence_dir/gitleaks.json' --no-git"
run_optional "scan" "semgrep_source" "command -v semgrep" "$evidence_dir/semgrep.json" "semgrep scan --config auto --json --output '$evidence_dir/semgrep.json' ."
run_optional "iac-policy" "checkov_deploy" "command -v checkov" "$evidence_dir/checkov.json" "if [ -d deploy ]; then checkov -d deploy -o json --output-file-path '$evidence_dir/checkov.json'; else printf '{\"skipped\":\"deploy directory not present\"}\n' > '$evidence_dir/checkov.json'; fi"
run_optional "kubernetes" "kubescape_cluster" "command -v kubescape" "$evidence_dir/kubescape.json" "kubescape scan --format json --output '$evidence_dir/kubescape.json'"

commit="$(git rev-parse --short HEAD 2>/dev/null || printf unknown)"
branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || printf unknown)"

cat > "$report" <<EOF
# OSDC Assurance Evidence

- Ring: \`$ring\`
- Result: \`$overall_status\`
- Generated UTC: \`$timestamp\`
- Git branch: \`$branch\`
- Git commit: \`$commit\`
- Strict security: \`$strict_security\`
- Required failures: \`$required_failures\`
- Optional scanner warnings: \`$security_warnings\`

## Files

- Manifest: \`evidence-manifest.tsv\`
- Command log: \`commands.log\`
- Git status: \`git-status.txt\`

This bundle is designed for later ingestion into DefectDojo, Dependency-Track,
Wazuh/OpenSearch, Forgejo issues, or the OSDC portal evidence model.
EOF

echo "assurance evidence: $evidence_dir"
echo "result: $overall_status"

if [[ "$overall_status" != "pass" ]]; then
  exit 1
fi
