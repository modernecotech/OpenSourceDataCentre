#!/usr/bin/env bash
set -euo pipefail

milestone=""
mode="plan"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --milestone)
      milestone="${2:-}"
      shift 2
      ;;
    --mode)
      mode="${2:-}"
      shift 2
      ;;
    --all)
      milestone=""
      shift
      ;;
    *)
      echo "unknown option: $1" >&2
      echo "usage: scripts/adapter-proof.sh [--all|--milestone ADAPT_001] [--mode plan]" >&2
      exit 2
      ;;
  esac
done

python3 - "$milestone" "$mode" <<'PY'
from pathlib import Path
import csv
import datetime as dt
import re
import sys

milestone = sys.argv[1]
mode = sys.argv[2]
root = Path.cwd()
proofs_path = root / "data/software/live-adapter-proof-catalogue.csv"
roadmap_path = root / "data/software/live-adapter-roadmap.csv"

proofs = list(csv.DictReader(proofs_path.open(newline="", encoding="utf-8")))
roadmap = {
    row["milestone_id"]: row
    for row in csv.DictReader(roadmap_path.open(newline="", encoding="utf-8"))
}
portal_migration = root / "crates/osdc-portal/migrations/0001_osdc_portal_state.sql"
selected = [row for row in proofs if not milestone or row["milestone_id"] == milestone]
if not selected:
    raise SystemExit(f"no proof rows matched milestone {milestone!r}")

stamp = dt.datetime.now(dt.UTC).strftime("%Y%m%dT%H%M%SZ")
out_dir = root / "target/assurance/adapter-proofs" / stamp
out_dir.mkdir(parents=True, exist_ok=True)
latest = root / "target/assurance/adapter-proofs/latest"
latest.mkdir(parents=True, exist_ok=True)

def portal_migration_lines():
    if not portal_migration.exists():
        return ["PostgreSQL migration: missing"]
    sql = portal_migration.read_text(encoding="utf-8")
    tables = re.findall(
        r"CREATE TABLE IF NOT EXISTS\s+([a-zA-Z0-9_.]+)",
        sql,
        flags=re.IGNORECASE,
    )
    indexes = re.findall(
        r"CREATE INDEX IF NOT EXISTS\s+([a-zA-Z0-9_]+)",
        sql,
        flags=re.IGNORECASE,
    )
    lines = [
        f"PostgreSQL migration: `{portal_migration.relative_to(root)}`",
        f"Portal-state tables found: {len(tables)}",
    ]
    lines.extend(f"- Table: `{table}`" for table in tables)
    lines.append(f"Indexes found: {len(indexes)}")
    return lines

manifest_rows = []
for row in selected:
    roadmap_row = roadmap.get(row["milestone_id"], {})
    slug = row["adapter_target"].lower().replace(" ", "-")
    body = [
        f"# Adapter Proof: {row['proof_id']}",
        "",
        f"- Milestone: {row['milestone_id']}",
        f"- Adapter target: {row['adapter_target']}",
        f"- Mode: {mode}",
        f"- Scope: {row['scope']}",
        f"- Required env: {row['required_env']}",
        f"- Required gate: {row['required_gate']}",
        f"- Owner: {row['owner']}",
        f"- Roadmap status: {roadmap_row.get('status', row['status'])}",
        f"- Production write path: {roadmap_row.get('production_write_path', 'unknown')}",
        "",
        "This is a local plan-mode proof. It records the intended command and evidence contract without contacting the target system.",
        "",
        f"Proof command: `{row['proof_command']}`",
        f"Next step: {roadmap_row.get('next_step', 'not recorded')}",
        "",
    ]
    if row["milestone_id"] == "ADAPT_009":
        body.extend(["## Migration Inspection", "", *portal_migration_lines(), ""])
    artifact = out_dir / f"{slug}.md"
    latest_artifact = latest / f"{slug}.md"
    artifact.write_text("\n".join(body), encoding="utf-8")
    latest_artifact.write_text("\n".join(body), encoding="utf-8")
    manifest_rows.append(
        [
            row["proof_id"],
            row["milestone_id"],
            row["adapter_target"],
            row["status"],
            str(artifact.relative_to(root)),
        ]
    )

manifest_header = "proof_id\tmilestone_id\tadapter_target\tstatus\tevidence\n"
manifest_body = "\n".join("\t".join(item) for item in manifest_rows) + "\n"
(out_dir / "manifest.tsv").write_text(manifest_header + manifest_body, encoding="utf-8")
(latest / "manifest.tsv").write_text(manifest_header + manifest_body, encoding="utf-8")
print(out_dir.relative_to(root))
PY
