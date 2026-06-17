# Portal API Prototype

Last reviewed: 2026-06-14.

`osdc-portal` is the first Rust-served prototype for the three OSDC GUI surfaces:

- Tenant portal: `/user`
- Operator console: `/operator`
- Edge Shield console: `/edge`
- Cost planner: `/planner`

It deliberately starts as a small standard-library HTTP server. That keeps the first interface easy to inspect, portable, and dependency-light. The production version can later move to `axum`, OpenAPI, auth middleware, PostgreSQL, and adapters for OpenStack, NetBox, Ceph, Kubernetes, Kueue, OpenBao, and the facility gateways.

## Current Routes

| Route | Type | Purpose |
| --- | --- | --- |
| `/user` | HTML | Tenant provisioning and resource-management GUI. |
| `/operator` | HTML | Datacentre operator GUI for power, cooling, hardware, cloud stack health, and operations. |
| `/api/catalog/hardware` | JSON | Chosen SBC/GPU hardware baseline. |
| `/api/catalog/services` | JSON | Open cloud service map. |
| `/api/catalog/core-services` | JSON | Chosen AWS/Azure-like core services and their open-source OSDC implementation. |
| `/api/catalog/sovereign-services` | JSON | Broad sovereign cloud service catalogue across cloud core, edge, developer, security, data, AI, and operations bundles. |
| `/api/catalog/upgrade-policy` | JSON | Managed patch and upgrade classes with gates, owners, and rollback requirements. |
| `/api/catalog/blueprints` | JSON | First provisioning blueprints for VMs, GPU model endpoints, databases, Kubernetes, and buckets. |
| `/api/config/scripts` | JSON | Browser-editable config script catalogue with validation and rollout metadata. |
| `/api/edge/services` | JSON | Open Cloudflare-alternative service map for Radxa edge nodes. |
| `/api/edge/status` | JSON | Edge Shield metrics, Radxa nodes, service status, and rollout queue. |
| `/api/edge/config-preview` | JSON | Generated Radxa edge config files and rollout checks before deployment. |
| `/api/provisioning/options` | JSON | Tenant form options for services, shapes, Linux images, storage, and networks. |
| `/api/provisioning/preview` | JSON | Sample provisioning plan with backend stack, power estimate, monthly estimate, and operator checks. |
| `/api/tenant/summary` | JSON | Tenant metrics, service tiles, site-flow status, and resources. |
| `/api/operator/status` | JSON | DC power/cooling metrics, hardware pools, open cloud stack health, and operations queue. |
| `/api/cost/planning` | JSON | Combined cost metrics, scale scenarios, category costs, and marketplace price basis. |
| `/api/cost/scenarios` | JSON | Four datacentre build scenarios from 50 kW edge micro to 5 MW national AI-ready. |
| `/api/cost/categories` | JSON | Category-level cost ranges for each scenario. |
| `/api/cost/price-basis` | JSON | Marketplace and derived unit-cost planning basis. |
| `/health` | text | Readiness check. |

## Data Direction

The next production step is to replace the in-process sample data with adapters:

- Hardware catalog from `data/hardware/compute-baseline-2026.csv`.
- Open cloud service catalog from `data/software/open-cloud-service-map.csv`.
- Core AWS/Azure-like service catalog from `data/software/core-cloud-services.csv`.
- Edge Shield service catalog from `data/software/edge-shield-services.csv`.
- Edge Shield deployment catalog from `data/software/edge-shield-service-map.csv`.
- Security control catalog from `data/software/security-control-map.csv`.
- Proprietary-to-open-source replacement catalog from `data/software/proprietary-to-open-source-map.csv`.
- Sovereign cloud service catalogue from `data/software/service-catalogue-v1.csv`.
- Upgrade policy from `data/software/upgrade-policy.csv`.
- Software security controls from `data/software/security-controls.csv`.
- Config script catalogue from `data/software/config-script-catalogue.csv` and source examples from `examples/config-scripts/`.
- Inventory and rack truth from NetBox/openDCIM.
- VM and bare-metal state from OpenStack Nova/Ironic.
- Storage state from Ceph.
- Kubernetes and queue state from Kubernetes, Kueue, and Slurm.
- Facility status from Modbus/BACnet/OPC UA gateways.
- Cost and carbon estimates from the Rust calculator crate.

## Run

```bash
cargo run -p osdc-portal -- 127.0.0.1:8787
```

Open:

- `http://127.0.0.1:8787/user`
- `http://127.0.0.1:8787/operator`
- `http://127.0.0.1:8787/edge`
- `http://127.0.0.1:8787/planner`

The GUI includes table filtering, tenant provisioning preview recalculation, CSV export for visible tenant/planner tables, and visible action feedback for staged operator and edge workflows.

The Radxa-local edge service can be run separately:

```bash
cargo run -p osdc-edge -- 127.0.0.1:8790
```

Open:

- `http://127.0.0.1:8790/`
- `http://127.0.0.1:8790/api/status`
- `http://127.0.0.1:8790/api/config-preview`
