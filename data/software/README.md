# Software Service Data

This directory stores CSV catalogues for the open-source cloud and edge stack.

- `open-cloud-service-map.csv` maps broad datacentre service domains to open-source systems.
- `core-cloud-services.csv` captures the AWS/Azure-like services selected for first implementation in the portal.
- `edge-shield-services.csv` captures the Cloudflare-equivalent Edge Shield functions for Radxa nodes.

Keep these files as simple rectangular CSVs with a single header row. When adding fields, update the Rust sample data and portal tests at the same time so the GUI and documentation remain aligned.

Validation:

```bash
find data/software -name '*.csv' -print0 | xargs -0 -n1 sh -c 'awk -F, "NR == 1 { cols = NF } NR > 1 && NF != cols { exit 1 }" "$0"'
```
