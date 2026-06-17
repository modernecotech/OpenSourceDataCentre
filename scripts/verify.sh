#!/usr/bin/env bash
set -euo pipefail

run_live=0
run_security=0

for arg in "$@"; do
  case "$arg" in
    --live)
      run_live=1
      ;;
    --security)
      run_security=1
      ;;
    *)
      echo "unknown option: $arg" >&2
      echo "usage: scripts/verify.sh [--live] [--security]" >&2
      exit 2
      ;;
  esac
done

cargo fmt --check
cargo test

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required for repository metadata checks" >&2
  exit 1
fi

python3 - <<'PY'
from pathlib import Path
import csv
import json
import re
import sys

ROOT = Path.cwd()
ROOT_RESOLVED = ROOT.resolve()
SKIP_PARTS = {".git", "target"}
failures = []


def fail(message):
    failures.append(message)


def repo_files(pattern):
    for path in sorted(ROOT.rglob(pattern)):
        try:
            relative = path.relative_to(ROOT)
        except ValueError:
            continue
        if any(part in SKIP_PARTS for part in relative.parts):
            continue
        if path.is_file():
            yield path


workspace = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
if 'license = "UNLICENSED"' in workspace:
    fail("Cargo.toml still declares the workspace UNLICENSED")
if 'license = "Apache-2.0"' not in workspace:
    fail("Cargo.toml must declare the Rust workspace as Apache-2.0")
if not (ROOT / "LICENSE.md").exists():
    fail("LICENSE.md is required")

for path in repo_files("*.json"):
    try:
        json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"{path}: invalid JSON: {exc}")

unique_columns = {
    Path("data/ai-ready/high-density-rack-classes.csv"): "rack_class_id",
    Path("data/bom/bom-250kw-open-regional.csv"): "line_id",
    Path("data/commissioning/commissioning-evidence-register.csv"): "evidence_id",
    Path("data/commercial/audit-evidence.csv"): "evidence_id",
    Path("data/commercial/access-roles.csv"): "access_role_id",
    Path("data/commercial/colocation-products.csv"): "product_id",
    Path("data/commercial/commercial-gap-register.csv"): "gap_id",
    Path("data/commercial/cross-connect-products.csv"): "product_id",
    Path("data/commercial/remote-hands-pricebook.csv"): "pricebook_id",
    Path("data/commercial/remote-hands-products.csv"): "product_id",
    Path("data/commercial/sla-classes.csv"): "sla_class_id",
    Path("data/commercial/standards-control-matrix.csv"): "requirement_id",
    Path("data/costing/marketplace-price-basis-2026.csv"): "item_family",
    Path("data/costing/scenario-costs-2026.csv"): "scenario_id",
    Path("data/delivery/action-tracker.csv"): "action_id",
    Path("data/delivery/authority-permits.csv"): "permit_id",
    Path("data/delivery/project-gates.csv"): "gate_id",
    Path("data/delivery/risk-register.csv"): "risk_id",
    Path("data/engineering/engineering-evidence-register.csv"): "evidence_id",
    Path("data/hardware/compute-baseline-2026.csv"): "profile_id",
    Path("data/hardware/provisioning-pipeline.csv"): "stage_id",
    Path("data/hardware/provisioning-profiles.csv"): "profile_id",
    Path("data/hardware/provisioning-requests.csv"): "request_id",
    Path("data/operations/procedure-catalogue.csv"): "procedure_id",
    Path("data/security/physical-security-controls.csv"): "control_id",
    Path("data/security/scanner-coverage.csv"): "scanner_id",
    Path("data/security/threat-management-stack.csv"): "component_id",
    Path("data/site-selection/site-selection-scorecard.csv"): "criterion_id",
    Path("data/software/assurance-automation-jobs.csv"): "job_id",
    Path("data/software/config-script-catalogue.csv"): "script_id",
    Path("data/software/core-cloud-services.csv"): "service_id",
    Path("data/software/data-access-policies.csv"): "policy_id",
    Path("data/software/data-ontology-objects.csv"): "object_id",
    Path("data/software/data-pipelines.csv"): "pipeline_id",
    Path("data/software/data-platform-services.csv"): "service_id",
    Path("data/software/data-platform-templates.csv"): "template_id",
    Path("data/software/data-products.csv"): "product_id",
    Path("data/software/developer-platform-services.csv"): "service_id",
    Path("data/software/developer-promotion-gates.csv"): "gate_id",
    Path("data/software/developer-templates.csv"): "template_id",
    Path("data/software/deployment-stack-profiles.csv"): "profile_id",
    Path("data/software/deployment-environments.csv"): "environment_id",
    Path("data/software/edge-shield-service-map.csv"): "service_id",
    Path("data/software/edge-shield-services.csv"): "service_id",
    Path("data/software/infrastructure-workflows.csv"): "workflow_id",
    Path("data/software/live-adapter-roadmap.csv"): "milestone_id",
    Path("data/software/open-cloud-service-map.csv"): "cloud_domain",
    Path("data/software/proprietary-open-source-equivalents.csv"): "proprietary_service",
    Path("data/software/proprietary-to-open-source-map.csv"): "proprietary_system",
    Path("data/software/security-control-map.csv"): "control_id",
    Path("data/software/security-controls.csv"): "control_id",
    Path("data/software/service-catalogue-v1.csv"): "service_id",
    Path("data/software/system-ui-connectors.csv"): "connector_id",
    Path("data/software/test-harness-catalogue.csv"): "test_id",
    Path("data/software/upgrade-rings.csv"): "ring_id",
    Path("data/software/upgrade-test-gates.csv"): "gate_id",
    Path("data/software/upgrade-policy.csv"): "update_class",
    Path("data/software/vscode-workflows.csv"): "workflow_id",
    Path("data/sustainability/sustainability-metrics.csv"): "metric_id",
}

for path in repo_files("*.csv"):
    if "data" not in path.relative_to(ROOT).parts:
        continue
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.reader(handle))
    if not rows:
        fail(f"{path}: empty CSV")
        continue
    width = len(rows[0])
    for number, row in enumerate(rows[1:], start=2):
        if len(row) != width:
            fail(f"{path}:{number} has {len(row)} fields, expected {width}")

    relative = path.relative_to(ROOT)
    unique_column = unique_columns.get(relative)
    if unique_column:
        header = rows[0]
        if unique_column not in header:
            fail(f"{path}: missing primary id column {unique_column}")
            continue
        index = header.index(unique_column)
        seen = {}
        for number, row in enumerate(rows[1:], start=2):
            value = row[index]
            if not value:
                fail(f"{path}:{number} has blank {unique_column}")
            elif value in seen:
                fail(f"{path}:{number} duplicates {unique_column} {value!r} from line {seen[value]}")
            else:
                seen[value] = number

path_like_columns = {
    "doc_path",
    "evidence_file",
    "evidence_path",
    "next_artifact",
    "next_evidence",
    "required_evidence",
}
for path in repo_files("*.csv"):
    if "data" not in path.relative_to(ROOT).parts:
        continue
    with path.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        if not reader.fieldnames:
            continue
        columns = path_like_columns.intersection(reader.fieldnames)
        if not columns:
            continue
        for number, row in enumerate(reader, start=2):
            for column in sorted(columns):
                value = row.get(column, "")
                if not value or value.startswith(("http://", "https://")):
                    continue
                if "/" not in value:
                    continue
                if not (ROOT / value).exists():
                    fail(f"{path}:{number} column {column} points at missing path {value}")

catalog_path = ROOT / "data/software/service-catalogue-v1.csv"
allowed_maturity = {"experimental", "pilot", "production-baseline", "optional", "deprecated"}
catalog_ids = set()
if catalog_path.exists():
    with catalog_path.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        required = {
            "service_id",
            "proprietary_service",
            "open_equivalent",
            "bundle",
            "priority",
            "workflow",
            "maturity",
        }
        missing = required.difference(reader.fieldnames or [])
        if missing:
            fail(f"{catalog_path}: missing required columns {sorted(missing)}")
        for number, row in enumerate(reader, start=2):
            service_id = row.get("service_id", "")
            if service_id:
                catalog_ids.add(service_id)
            maturity = row.get("maturity", "")
            if maturity not in allowed_maturity:
                fail(f"{catalog_path}:{number} has unsupported maturity {maturity!r}")

for path in sorted((ROOT / "examples/service-catalogue").glob("*.json")):
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        continue
    for service_id in data.get("services", []):
        if service_id not in catalog_ids:
            fail(f"{path}: references unknown service_id {service_id!r}")

expected_config_examples = {
    "edge_caddyfile": ROOT / "examples/config-scripts/edge/Caddyfile",
    "edge_powerdns": ROOT / "examples/config-scripts/edge/pdns-osdc.conf",
    "edge_coraza": ROOT / "examples/config-scripts/edge/coraza-crs.conf",
    "edge_crowdsec": ROOT / "examples/config-scripts/edge/crowdsec-acquis.yaml",
    "edge_wireguard": ROOT / "examples/config-scripts/edge/wireguard-osdc-edge.conf",
}
config_catalog = ROOT / "data/software/config-script-catalogue.csv"
if config_catalog.exists():
    with config_catalog.open(newline="", encoding="utf-8") as handle:
        for number, row in enumerate(csv.DictReader(handle), start=2):
            script_id = row.get("script_id", "")
            if not row.get("validation_command"):
                fail(f"{config_catalog}:{number} missing validation_command")
            if row.get("risk") not in {"low", "medium", "high", "critical"}:
                fail(f"{config_catalog}:{number} has unsupported risk {row.get('risk')!r}")
            expected = expected_config_examples.get(script_id)
            if expected and not expected.exists():
                fail(f"{config_catalog}:{number} points at missing example {expected}")

commercial_gap_register = ROOT / "data/commercial/commercial-gap-register.csv"
allowed_gap_priorities = {"critical", "high", "medium", "low"}
allowed_gap_status = {"open", "in-progress", "blocked", "review", "closed"}
if commercial_gap_register.exists():
    with commercial_gap_register.open(newline="", encoding="utf-8") as handle:
        for number, row in enumerate(csv.DictReader(handle), start=2):
            if row.get("priority") not in allowed_gap_priorities:
                fail(
                    f"{commercial_gap_register}:{number} has unsupported priority {row.get('priority')!r}"
                )
            if row.get("status") not in allowed_gap_status:
                fail(
                    f"{commercial_gap_register}:{number} has unsupported status {row.get('status')!r}"
                )
site_scorecard = ROOT / "data/site-selection/site-selection-scorecard.csv"
if site_scorecard.exists():
    with site_scorecard.open(newline="", encoding="utf-8") as handle:
        for number, row in enumerate(csv.DictReader(handle), start=2):
            try:
                weight = int(row.get("weight", ""))
            except ValueError:
                fail(f"{site_scorecard}:{number} has non-integer weight {row.get('weight')!r}")
                continue
            if not 1 <= weight <= 10:
                fail(f"{site_scorecard}:{number} weight must be between 1 and 10")

physical_controls = ROOT / "data/security/physical-security-controls.csv"
if physical_controls.exists():
    with physical_controls.open(newline="", encoding="utf-8") as handle:
        for number, row in enumerate(csv.DictReader(handle), start=2):
            if row.get("status") not in {"template", "implemented", "review", "retired"}:
                fail(
                    f"{physical_controls}:{number} has unsupported status {row.get('status')!r}"
                )

sustainability_metrics = ROOT / "data/sustainability/sustainability-metrics.csv"
allowed_metric_stages = {
    "design-estimate",
    "commissioning-measurement",
    "operating-measurement",
    "pilot-measurement",
    "audit-reporting",
}
if sustainability_metrics.exists():
    with sustainability_metrics.open(newline="", encoding="utf-8") as handle:
        for number, row in enumerate(csv.DictReader(handle), start=2):
            if row.get("stage") not in allowed_metric_stages:
                fail(
                    f"{sustainability_metrics}:{number} has unsupported stage {row.get('stage')!r}"
                )

engineering_evidence = ROOT / "data/engineering/engineering-evidence-register.csv"
allowed_engineering_status = {"template", "draft", "review", "approved", "retired"}
if engineering_evidence.exists():
    with engineering_evidence.open(newline="", encoding="utf-8") as handle:
        for number, row in enumerate(csv.DictReader(handle), start=2):
            if row.get("priority") not in allowed_gap_priorities:
                fail(
                    f"{engineering_evidence}:{number} has unsupported priority {row.get('priority')!r}"
                )
            if row.get("status") not in allowed_engineering_status:
                fail(
                    f"{engineering_evidence}:{number} has unsupported status {row.get('status')!r}"
                )

operations_procedures = ROOT / "data/operations/procedure-catalogue.csv"
if operations_procedures.exists():
    with operations_procedures.open(newline="", encoding="utf-8") as handle:
        for number, row in enumerate(csv.DictReader(handle), start=2):
            if row.get("criticality") not in allowed_gap_priorities:
                fail(
                    f"{operations_procedures}:{number} has unsupported criticality {row.get('criticality')!r}"
                )
            if row.get("status") not in {"template", "draft", "implemented", "review", "retired"}:
                fail(
                    f"{operations_procedures}:{number} has unsupported status {row.get('status')!r}"
                )

delivery_status_values = {"template", "open", "in-progress", "blocked", "review", "closed", "approved"}
for relative in [
    Path("data/delivery/project-gates.csv"),
    Path("data/delivery/authority-permits.csv"),
    Path("data/delivery/risk-register.csv"),
    Path("data/delivery/action-tracker.csv"),
    Path("data/commissioning/commissioning-evidence-register.csv"),
]:
    path = ROOT / relative
    if not path.exists():
        continue
    status_column = "status"
    with path.open(newline="", encoding="utf-8") as handle:
        for number, row in enumerate(csv.DictReader(handle), start=2):
            if row.get(status_column) not in delivery_status_values:
                fail(f"{path}:{number} has unsupported status {row.get(status_column)!r}")
            if "criticality" in row and row.get("criticality") not in allowed_gap_priorities:
                fail(f"{path}:{number} has unsupported criticality {row.get('criticality')!r}")

link_pattern = re.compile(r"!?\[[^\]]*]\(([^)]+)\)")
for path in repo_files("*.md"):
    text = path.read_text(encoding="utf-8")
    for match in link_pattern.finditer(text):
        target = match.group(1).strip()
        if not target or target.startswith(("#", "http://", "https://", "mailto:")):
            continue
        if target.startswith("/"):
            continue
        if target.startswith("<") and ">" in target:
            target = target[1 : target.index(">")]
        else:
            target = target.split()[0]
        target = target.split("#", 1)[0].split("?", 1)[0]
        if not target:
            continue
        resolved = (path.parent / target).resolve()
        try:
            resolved.relative_to(ROOT_RESOLVED)
        except ValueError:
            fail(f"{path}: link escapes repository: {match.group(1)}")
            continue
        if not resolved.exists():
            fail(f"{path}: broken local link: {match.group(1)}")

portal_doc = ROOT / "docs/architecture/portal-api.md"
portal_source = ROOT / "crates/osdc-portal/src/main.rs"
if portal_doc.exists() and portal_source.exists():
    doc_routes = set(re.findall(r"\| `(/[^`]+)` \|", portal_doc.read_text(encoding="utf-8")))
    source_routes = set(
        re.findall(r'\("GET", "([^"]+)"\)', portal_source.read_text(encoding="utf-8"))
    )
    documentable_routes = {
        route
        for route in source_routes
        if route.startswith("/api/")
        or route in {"/user", "/operator", "/edge", "/planner", "/lifecycle", "/hardware", "/developer", "/data-platform", "/health"}
    }
    for route in sorted(doc_routes - source_routes):
        fail(f"{portal_doc}: documents missing portal route {route}")
    for route in sorted(documentable_routes - doc_routes):
        fail(f"{portal_doc}: missing route documentation for {route}")

if failures:
    print("repository verification failed:", file=sys.stderr)
    for failure in failures:
        print(f" - {failure}", file=sys.stderr)
    sys.exit(1)

print("repository metadata checks passed")
PY

if (( run_security )); then
  if ! cargo audit --version >/dev/null 2>&1; then
    echo "cargo-audit is required for --security checks" >&2
    echo "install with: cargo install cargo-audit --locked" >&2
    exit 1
  fi

  if ! command -v syft >/dev/null 2>&1; then
    echo "syft is required for --security SBOM generation" >&2
    echo "install from: https://github.com/anchore/syft" >&2
    exit 1
  fi

  cargo audit
  mkdir -p target
  syft dir:. -o spdx-json=target/osdc-sbom.spdx.json
fi

if (( run_live )); then
  if ! command -v curl >/dev/null 2>&1; then
    echo "curl is required for --live checks" >&2
    exit 1
  fi

  curl -fsS http://127.0.0.1:8787/health >/dev/null
  curl -fsS http://127.0.0.1:8787/api/cost/planning >/dev/null
  curl -fsS http://127.0.0.1:8787/api/edge/config-preview >/dev/null
  curl -fsS http://127.0.0.1:8790/health >/dev/null
  curl -fsS http://127.0.0.1:8790/api/config-preview >/dev/null
fi
