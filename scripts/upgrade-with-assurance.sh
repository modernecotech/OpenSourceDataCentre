#!/usr/bin/env bash
set -euo pipefail

ring="RING_STAGING"
service="osdc-platform"
change_ref=""
strict_security=0
evidence_root=""

usage() {
  cat <<'EOF'
usage: scripts/upgrade-with-assurance.sh [--ring RING_ID] [--service NAME] [--change-ref REF] [--strict-security] [--evidence-dir DIR]

Creates a local upgrade evidence bundle and runs the OSDC assurance gate set.
This script is intentionally GitOps-friendly: it does not mutate infrastructure.

Options:
  --ring RING_ID        Upgrade ring to validate. Default: RING_STAGING.
  --service NAME        Service or platform slice being upgraded.
  --change-ref REF      Change, PR, issue, or Renovate branch reference.
  --strict-security     Fail if an installed optional security scanner fails.
  --evidence-dir DIR    Write evidence under DIR. Default: target/assurance/upgrades/<timestamp>.
EOF
}

while (($#)); do
  case "$1" in
    --ring)
      ring="${2:-}"
      shift 2
      ;;
    --service)
      service="${2:-}"
      shift 2
      ;;
    --change-ref)
      change_ref="${2:-}"
      shift 2
      ;;
    --strict-security)
      strict_security=1
      shift
      ;;
    --evidence-dir)
      evidence_root="${2:-}"
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

if [[ -z "$ring" || -z "$service" ]]; then
  echo "--ring and --service require non-empty values" >&2
  exit 2
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required to read upgrade ring metadata" >&2
  exit 1
fi

timestamp="$(date -u +"%Y%m%dT%H%M%SZ")"
if [[ -z "$evidence_root" ]]; then
  evidence_root="target/assurance/upgrades/$timestamp"
fi
mkdir -p "$evidence_root"

ring_metadata="$evidence_root/ring-metadata.tsv"
if ! python3 - "$ring" > "$ring_metadata" <<'PY'
import csv
import sys
from pathlib import Path

ring_id = sys.argv[1]
path = Path("data/software/upgrade-rings.csv")
with path.open(newline="", encoding="utf-8") as handle:
    for row in csv.DictReader(handle):
        if row["ring_id"] == ring_id:
            for key in [
                "ring_id",
                "scope",
                "cadence",
                "entry_criteria",
                "automated_tests",
                "promotion_gate",
                "rollback_strategy",
                "owner",
                "status",
            ]:
                print(f"{key}\t{row[key]}")
            raise SystemExit(0)
print(f"unknown ring: {ring_id}", file=sys.stderr)
raise SystemExit(2)
PY
then
  cat "$ring_metadata" >&2 || true
  exit 2
fi

commit="$(git rev-parse --short HEAD 2>/dev/null || printf unknown)"
branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || printf unknown)"
status_file="$evidence_root/preflight-git-status.txt"
plan="$evidence_root/upgrade-plan.md"
decision="$evidence_root/promotion-decision.md"

git status --short > "$status_file"

cat > "$plan" <<EOF
# OSDC Upgrade Plan

- Service: \`$service\`
- Ring: \`$ring\`
- Change reference: \`${change_ref:-not-provided}\`
- Generated UTC: \`$timestamp\`
- Git branch: \`$branch\`
- Git commit: \`$commit\`

## Ring Metadata

\`\`\`text
$(cat "$ring_metadata")
\`\`\`

## Required Decision Rule

The change may be promoted only if the assurance run passes and any scanner
warnings have an owner, expiry, and evidence link before production.
EOF

assurance_args=(--ring "$ring" --evidence-dir "$evidence_root/assurance-run")
if ((strict_security)); then
  assurance_args+=(--strict-security)
fi

if scripts/assurance-run.sh "${assurance_args[@]}"; then
  result="pass"
else
  result="fail"
fi

cat > "$decision" <<EOF
# OSDC Promotion Decision

- Service: \`$service\`
- Ring: \`$ring\`
- Result: \`$result\`
- Assurance evidence: \`assurance-run/README.md\`
- Git status: \`preflight-git-status.txt\`

## Operator Action

If result is \`pass\`, open or approve the GitOps promotion PR for the next ring.
If result is \`fail\`, fix failed gates or create an accepted-risk record with owner,
expiry, compensating control, and approval evidence.
EOF

echo "upgrade evidence: $evidence_root"
echo "result: $result"

if [[ "$result" != "pass" ]]; then
  exit 1
fi
