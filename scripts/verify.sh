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
    Path("data/bom/bom-250kw-open-regional.csv"): "line_id",
    Path("data/costing/marketplace-price-basis-2026.csv"): "item_family",
    Path("data/costing/scenario-costs-2026.csv"): "scenario_id",
    Path("data/hardware/compute-baseline-2026.csv"): "profile_id",
    Path("data/software/config-script-catalogue.csv"): "script_id",
    Path("data/software/core-cloud-services.csv"): "service_id",
    Path("data/software/edge-shield-service-map.csv"): "service_id",
    Path("data/software/edge-shield-services.csv"): "service_id",
    Path("data/software/proprietary-open-source-equivalents.csv"): "proprietary_service",
    Path("data/software/proprietary-to-open-source-map.csv"): "proprietary_system",
    Path("data/software/security-control-map.csv"): "control_id",
    Path("data/software/security-controls.csv"): "control_id",
    Path("data/software/service-catalogue-v1.csv"): "service_id",
    Path("data/software/upgrade-policy.csv"): "update_class",
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
        or route in {"/user", "/operator", "/edge", "/planner", "/health"}
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
