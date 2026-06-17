# Portal API Prototype

Last reviewed: 2026-06-17.

`osdc-portal` is the first Rust-served prototype for the OSDC GUI surfaces:

- Tenant portal: `/user`
- Operator console: `/operator`
- Edge Shield console: `/edge`
- Cost planner: `/planner`
- Lifecycle console: `/lifecycle`
- Commercial console: `/commercial`
- Developer console: `/developer`
- Data platform console: `/data-platform`

It deliberately starts as a small standard-library HTTP server. That keeps the first interface easy to inspect, portable, and dependency-light. The production version can later move to `axum`, OpenAPI, auth middleware, PostgreSQL, and adapters for OpenStack, NetBox, Ceph, Kubernetes, Kueue, OpenBao, and the facility gateways.

Current maturity: prototype. Most routes below return hardcoded or embedded sample data so UI, API, and data-model contracts can stabilize before real infrastructure adapters are attached. `/api/catalog/sovereign-services` is loaded from `data/software/service-catalogue-v1.csv`; commercial-readiness, delivery, commissioning, engineering, operations, site-selection, physical-security, sustainability, AI-ready, developer-platform, and data-platform planning routes are loaded from CSVs under `data/`. `/api/lifecycle/overview` composes those catalogues into one design-to-run operator view.

## Current Routes

| Route | Type | Purpose |
| --- | --- | --- |
| `/user` | HTML | Tenant provisioning and resource-management GUI. |
| `/operator` | HTML | Datacentre operator GUI for power, cooling, hardware, cloud stack health, and operations. |
| `/edge` | HTML | Edge Shield GUI for DNS, proxy, cache, WAF, tunnels, config scripts, and rollout previews. |
| `/planner` | HTML | Cost and scale planning GUI for 50 kW to 5 MW scenarios. |
| `/lifecycle` | HTML | Unified design-to-commission-to-operations console across gates, permits, evidence, work items, services, config scripts, and documents. |
| `/commercial` | HTML | Commercial readiness console for gaps, standards, SLA classes, colocation products, cross-connects, remote hands, access roles, and audit evidence. |
| `/developer` | HTML | Developer platform console for Forgejo-style repos, CI, Harbor, GitOps, OpenTofu, templates, environments, promotion gates, and VS Code workflows. |
| `/data-platform` | HTML | Optional open-source data platform console for governed data products, lakehouse, catalog, lineage, ontology, pipelines, dashboards, apps, and AI context. |
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
| `/api/commercial/gaps` | JSON | Commercial-readiness gap register for certification, MEP, operations, compliance, and interconnection gaps. |
| `/api/commercial/standards` | JSON | Standards/control matrix mapping candidate standards to evidence files and owners. |
| `/api/commercial/sla-classes` | JSON | Power, cooling, network, remote-hands, and cloud-platform SLA class templates. |
| `/api/commercial/colocation-products` | JSON | Rack, cage, suite, power, access, and remote-hands product templates. |
| `/api/commercial/cross-connect-products` | JSON | Meet-me-room, IP transit, IXP, and cloud-on-ramp-equivalent product templates. |
| `/api/commercial/remote-hands-products` | JSON | Remote/smart-hands task classes and evidence requirements. |
| `/api/commercial/remote-hands-pricebook` | JSON | Remote/smart-hands billing units, response targets, approvals, and evidence references. |
| `/api/commercial/access-roles` | JSON | Customer, carrier, staff, security, and break-glass access role templates. |
| `/api/commercial/audit-evidence` | JSON | Audit-evidence register with owners, cadence, and evidence paths. |
| `/api/site-selection/scorecard` | JSON | Site-selection due-diligence criteria and evidence targets. |
| `/api/security/physical-controls` | JSON | Physical security control catalogue for zones, access, CCTV, loading dock, and break-glass procedures. |
| `/api/sustainability/metrics` | JSON | Sustainability measurement boundaries for PUE, WUE, CUE, renewable fraction, heat reuse, e-waste, and battery lifecycle. |
| `/api/ai-ready/rack-classes` | JSON | AI/high-density rack classes with cooling, network, storage, and evidence requirements. |
| `/api/engineering/evidence` | JSON | Engineering evidence register for MEP, thermal, life-safety, controls, validation, and commissioning artifacts. |
| `/api/operations/procedures` | JSON | Operations procedure catalogue for MOP, SOP, EOP, permits, lockout/tagout, incidents, drills, alarms, telemetry, and capacity. |
| `/api/delivery/gates` | JSON | Project lifecycle gates from concept through owner-engineer review, design freeze, construction readiness, operational readiness, and handover. |
| `/api/delivery/permits` | JSON | Authority permit register for planning, grid, fire, environmental, water, telecom, building-control, generator, security, and waste approvals. |
| `/api/delivery/risks` | JSON | Delivery risk register with owners, criticality, status, mitigations, and next evidence. |
| `/api/delivery/actions` | JSON | Action tracker for unresolved delivery work, owners, due dates, dependencies, and evidence artifacts. |
| `/api/commissioning/evidence` | JSON | Commissioning evidence register for L1-L5 and integrated systems tests, acceptance criteria, owners, and evidence paths. |
| `/api/lifecycle/overview` | JSON | Composed lifecycle view that joins project gates, delivery risks/actions, commercial gaps, permits, engineering evidence, commissioning, operations, audit evidence, standards, service catalogues, config scripts, and key documents. |
| `/api/developer/platform` | JSON | Developer platform overview with services, templates, deployment environments, promotion gates, and VS Code workflow links. |
| `/api/data-platform/overview` | JSON | Data platform overview with open-source service stack, data products, pipelines, ontology objects, access policies, and starter templates. |
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
- Sovereign cloud service catalogue from `data/software/service-catalogue-v1.csv` is already wired into `/api/catalog/sovereign-services`.
- Upgrade policy from `data/software/upgrade-policy.csv`.
- Software security controls from `data/software/security-controls.csv`.
- Config script catalogue from `data/software/config-script-catalogue.csv` and source examples from `examples/config-scripts/`.
- Developer platform catalogues from `data/software/developer-*.csv`, `data/software/deployment-environments.csv`, `data/software/vscode-workflows.csv`, and `examples/developer-platform/`.
- Data platform catalogues from `data/software/data-*.csv` and `examples/data-platform/`.
- Inventory and rack truth from NetBox/openDCIM.
- VM and bare-metal state from OpenStack Nova/Ironic.
- Storage state from Ceph.
- Kubernetes and queue state from Kubernetes, Kueue, and Slurm.
- Facility status from Modbus/BACnet/OPC UA gateways.
- Cost and carbon estimates from the Rust calculator crate.
- Commercial readiness data from `data/commercial/` is already wired into `/api/commercial/*`.
- Site-selection, physical-security, sustainability, and AI-ready planning data is already wired into the corresponding `/api/*` catalogue routes.
- Engineering evidence and operations procedure catalogues are already wired into `/api/engineering/evidence` and `/api/operations/procedures`.
- Delivery and commissioning catalogues from `data/delivery/` and `data/commissioning/` are already wired into `/api/delivery/*` and `/api/commissioning/evidence`.

## Run

```bash
cargo run -p osdc-portal -- 127.0.0.1:8787
```

Open:

- `http://127.0.0.1:8787/user`
- `http://127.0.0.1:8787/operator`
- `http://127.0.0.1:8787/edge`
- `http://127.0.0.1:8787/planner`
- `http://127.0.0.1:8787/lifecycle`
- `http://127.0.0.1:8787/commercial`
- `http://127.0.0.1:8787/developer`
- `http://127.0.0.1:8787/data-platform`

The GUI includes table filtering, tenant provisioning preview recalculation, CSV export for visible tenant/planner/lifecycle/commercial/developer/data-platform tables, repo document links, VS Code clone/action links, and visible action feedback for staged operator, lifecycle, commercial, developer, data, and edge workflows.

The Radxa-local edge service can be run separately:

```bash
cargo run -p osdc-edge -- 127.0.0.1:8790
```

Open:

- `http://127.0.0.1:8790/`
- `http://127.0.0.1:8790/api/status`
- `http://127.0.0.1:8790/api/config-preview`
