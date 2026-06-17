# Software Service Data

This directory stores CSV catalogues for the open-source cloud and edge stack.

- `open-cloud-service-map.csv` maps broad datacentre service domains to open-source systems.
- `core-cloud-services.csv` captures the AWS/Azure-like services selected for first implementation in the portal.
- `edge-shield-services.csv` captures the Cloudflare-equivalent Edge Shield functions for Radxa nodes.
- `edge-shield-service-map.csv` expands Edge Shield into default production and simple deployment stacks.
- `security-control-map.csv` maps security control areas to open tools and required evidence.
- `proprietary-to-open-source-map.csv` maps common proprietary infrastructure platforms to OSDC open-source replacements.
- `service-catalogue-v1.csv` is the broad sovereign cloud service catalogue across cloud core, edge, developer platform, security, data, AI, and operations.
- `proprietary-open-source-equivalents.csv` maps Google/Microsoft/AWS-style service classes to OSDC open-source equivalents.
- `upgrade-policy.csv` defines update classes, cadence, gates, owners, and rollback requirements.
- `security-controls.csv` maps managed security controls to tools, evidence, UI surfaces, and service bundles.
- `config-script-catalogue.csv` lists tool config scripts exposed through the browser editor workflow.

Keep these files as simple rectangular CSVs with a single header row. When adding fields, update the Rust sample data and portal tests at the same time so the GUI and documentation remain aligned.

Validation:

```bash
find data/software -name '*.csv' -print0 | xargs -0 -n1 sh -c 'awk -F, "NR == 1 { cols = NF } NR > 1 && NF != cols { exit 1 }" "$0"'
```
