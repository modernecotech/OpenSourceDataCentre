#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo test

find data -name '*.csv' -print0 |
  xargs -0 -n1 awk -F, '
    NR == 1 { cols = NF }
    NR > 1 && NF != cols {
      printf "%s:%d has %d fields, expected %d\n", FILENAME, NR, NF, cols
      bad = 1
    }
    END { exit bad }
  '

if [[ "${1:-}" == "--live" ]]; then
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
