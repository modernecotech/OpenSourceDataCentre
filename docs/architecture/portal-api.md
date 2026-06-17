# Portal API Prototype

Last reviewed: 2026-06-17.

`osdc-portal` is the first Rust-served prototype for the OSDC GUI surfaces:

- Infrastructure workbench: `/infrastructure`
- Tenant portal: `/user`
- Operator console: `/operator`
- Edge Shield console: `/edge`
- Cost planner: `/planner`
- Lifecycle console: `/lifecycle`
- Hardware provisioning console: `/hardware`
- Commercial console: `/commercial`
- Assurance console: `/assurance`
- Developer console: `/developer`
- Data platform console: `/data-platform`

It deliberately starts as a small standard-library HTTP server. That keeps the first interface easy to inspect, portable, and dependency-light. The production version can later move to `axum`, OpenAPI, auth middleware, PostgreSQL, and adapters for OpenStack, CloudStack, Proxmox, NetBox, Ceph, Kubernetes, Kueue, OpenBao, PowerDNS, Keycloak, GitOps, and the facility gateways.

Current maturity: prototype. Many catalogue routes are now loaded from CSVs under `data/`, while status summaries and action previews still use in-process sample state so UI, API, and data-model contracts can stabilize before real infrastructure adapters are attached. OSDC is not a replacement for mature systems such as OpenStack, CloudStack, Proxmox, Kubernetes, Ceph, NetBox, MAAS, Foreman, SONiC, or OpenBMC; the portal is the workflow, policy, evidence, cost, and GitOps layer above them. `/api/infrastructure/workbench` composes workflow, connector, deployment, test, gate, and automation catalogues into the main user/operator front door. `/api/lifecycle/overview` composes project catalogues into one design-to-run operator view.

## Current Routes

| Route | Type | Purpose |
| --- | --- | --- |
| `/` | HTML redirect | Redirects to `/infrastructure` as the front-door workbench. |
| `/infrastructure` | HTML | Unified infrastructure workbench for guided create/manage/upgrade/scan workflows, deployment profile selection, connector paths, required tests, blocking gates, automation jobs, and evidence targets. |
| `/user` | HTML | Tenant provisioning and resource-management GUI. |
| `/operator` | HTML | Datacentre operator GUI for power, cooling, hardware, cloud stack health, and operations. |
| `/edge` | HTML | Edge Shield GUI for DNS, proxy, cache, WAF, tunnels, config scripts, and rollout previews. |
| `/planner` | HTML | Cost and scale planning GUI for 50 kW to 5 MW scenarios. |
| `/lifecycle` | HTML | Unified design-to-commission-to-operations console across gates, permits, evidence, work items, services, config scripts, and documents. |
| `/hardware` | HTML | Hardware provisioning console for source-of-truth reservation, BMC validation, commissioning, imaging, security enrolment, platform handoff, and evidence closeout. |
| `/commercial` | HTML | Commercial readiness console for gaps, standards, SLA classes, colocation products, cross-connects, remote hands, access roles, and audit evidence. |
| `/assurance` | HTML | Assurance console for broad test harnesses, automated upgrade rings, blocking gates, threat-management components, and scanner coverage. |
| `/developer` | HTML | Developer platform console for Forgejo-style repos, CI, Harbor, GitOps, OpenTofu, templates, environments, promotion gates, and VS Code workflows. |
| `/data-platform` | HTML | Optional open-source data platform console for governed data products, lakehouse, catalog, lineage, ontology, pipelines, dashboards, apps, and AI context. |
| `/api/catalog/hardware` | JSON | Chosen SBC/GPU hardware baseline. |
| `/api/connectors/systems` | JSON | Portal connector matrix for backend systems, adapter modes, endpoint patterns, auth models, write modes, owners, and evidence. |
| `/api/hardware/provisioning` | JSON | Combined hardware provisioning view with metrics, hardware connectors, pipeline stages, provisioning profiles, and request queue. |
| `/api/hardware/provisioning-pipeline` | JSON | Rack-to-running hardware provisioning stages from request intake through assurance closeout. |
| `/api/hardware/provisioning-profiles` | JSON | User-facing hardware provisioning profiles mapped to MAAS, Ironic, Metal3, Tinkerbell, target pools, images, and post-install hooks. |
| `/api/hardware/provisioning-requests` | JSON | Sample hardware request queue with profiles, sites, rack policy, network zone, approval gates, current stage, target environment, and status. |
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
| `/api/deployment/stack-profiles` | JSON | Recommended Proxmox, CloudStack, OpenStack, Ceph, Kubernetes, NetBox, bare-metal, Edge Shield, and GitOps pairings by deployment size. |
| `/api/infrastructure/workbench` | JSON | Composed workbench view joining infrastructure workflows, deployment stack profiles, system connectors, required test harnesses, upgrade gates, automation jobs, and metrics. |
| `/api/commercial/gaps` | JSON | Commercial-readiness gap register for certification, MEP, operations, compliance, and interconnection gaps. |
| `/api/commercial/standards` | JSON | Standards/control matrix mapping candidate standards to evidence files and owners. |
| `/api/commercial/sla-classes` | JSON | Power, cooling, network, remote-hands, and cloud-platform SLA class templates. |
| `/api/commercial/colocation-products` | JSON | Rack, cage, suite, power, access, and remote-hands product templates. |
| `/api/commercial/cross-connect-products` | JSON | Meet-me-room, IP transit, IXP, and cloud-on-ramp-equivalent product templates. |
| `/api/commercial/remote-hands-products` | JSON | Remote/smart-hands task classes and evidence requirements. |
| `/api/commercial/remote-hands-pricebook` | JSON | Remote/smart-hands billing units, response targets, approvals, and evidence references. |
| `/api/commercial/access-roles` | JSON | Customer, carrier, staff, security, and break-glass access role templates. |
| `/api/commercial/audit-evidence` | JSON | Audit-evidence register with owners, cadence, and evidence paths. |
| `/api/assurance/overview` | JSON | Combined assurance view for test harnesses, upgrade rings, gates, threat stack, scanner coverage, and metrics. |
| `/api/assurance/automation-jobs` | JSON | Runnable assurance automation jobs for repository checks, upgrade dry runs, scanner bundles, SBOM export, live smoke tests, GitOps staging, and finding sync. |
| `/api/assurance/test-harnesses` | JSON | Broad test-harness catalogue across software, data, config, GitOps, Kubernetes, endpoints, network, runtime, and facility commissioning. |
| `/api/assurance/upgrade-rings` | JSON | Automated upgrade promotion rings for development, staging, canary, production, facility controls, Edge Shield, and data-platform releases. |
| `/api/assurance/upgrade-gates` | JSON | Blocking upgrade gates for discovery, build, scan, posture, staging, stateful restore, rollback, approval, and audit. |
| `/api/assurance/threat-stack` | JSON | Open-source Wiz-like threat-management component catalogue. |
| `/api/assurance/scanner-coverage` | JSON | Scanner coverage catalogue for source, dependency, image, IaC, Kubernetes, host, network, runtime, compliance, and OT scans. |
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

CSV-backed catalogue sources currently wired into the portal include:

- Hardware catalog from `data/hardware/compute-baseline-2026.csv`.
- Hardware provisioning catalogues from `data/hardware/provisioning-pipeline.csv`, `data/hardware/provisioning-profiles.csv`, and `data/hardware/provisioning-requests.csv`.
- System UI connector catalogue from `data/software/system-ui-connectors.csv`.
- Open cloud service catalog from `data/software/open-cloud-service-map.csv`.
- Core AWS/Azure-like service catalog from `data/software/core-cloud-services.csv`.
- Edge Shield service catalog from `data/software/edge-shield-services.csv`.
- Edge Shield deployment catalog from `data/software/edge-shield-service-map.csv`.
- Security control catalog from `data/software/security-control-map.csv`.
- Proprietary-to-open-source replacement catalog from `data/software/proprietary-to-open-source-map.csv`.
- Sovereign cloud service catalogue from `data/software/service-catalogue-v1.csv`.
- Deployment substrate pairings from `data/software/deployment-stack-profiles.csv`.
- Infrastructure workflow mappings from `data/software/infrastructure-workflows.csv`.
- Upgrade policy from `data/software/upgrade-policy.csv`.
- Software security controls from `data/software/security-controls.csv`.
- Config script catalogue from `data/software/config-script-catalogue.csv` and source examples from `examples/config-scripts/`.
- Developer platform catalogues from `data/software/developer-*.csv`, `data/software/deployment-environments.csv`, `data/software/vscode-workflows.csv`, and `examples/developer-platform/`.
- Data platform catalogues from `data/software/data-*.csv` and `examples/data-platform/`.
- Assurance test, upgrade, automation, and scanner catalogues from `data/software/test-harness-catalogue.csv`, `data/software/assurance-automation-jobs.csv`, `data/software/upgrade-rings.csv`, `data/software/upgrade-test-gates.csv`, `data/security/threat-management-stack.csv`, and `data/security/scanner-coverage.csv`.
- Cost and carbon estimates from the Rust calculator crate.
- Commercial readiness data from `data/commercial/` is already wired into `/api/commercial/*`.
- Site-selection, physical-security, sustainability, and AI-ready planning data is already wired into the corresponding `/api/*` catalogue routes.
- Engineering evidence and operations procedure catalogues are already wired into `/api/engineering/evidence` and `/api/operations/procedures`.
- Delivery and commissioning catalogues from `data/delivery/` and `data/commissioning/` are already wired into `/api/delivery/*` and `/api/commissioning/evidence`.

The next production step is live adapters and persistence:

- Inventory and rack truth from NetBox/openDCIM.
- DNS zone and record workflows from PowerDNS.
- Identity tenants, groups, and roles from Keycloak.
- Secret namespaces and policies from OpenBao.
- GitOps pull requests or Argo CD/Flux changes for declarative configuration.
- VM state from Proxmox, CloudStack, or OpenStack depending on deployment profile.
- Bare-metal state from MAAS, Foreman, OpenStack Ironic, Metal3, or Tinkerbell.
- Storage state from Ceph.
- Kubernetes and queue state from Kubernetes, Kueue, and Slurm.
- Facility status from Modbus/BACnet/OPC UA gateways.
- PostgreSQL persistence for lifecycle state, approval history, evidence records, and audit events.

## Run

```bash
cargo run -p osdc-portal -- 127.0.0.1:8787
```

Open:

- `http://127.0.0.1:8787/infrastructure`
- `http://127.0.0.1:8787/user`
- `http://127.0.0.1:8787/operator`
- `http://127.0.0.1:8787/edge`
- `http://127.0.0.1:8787/planner`
- `http://127.0.0.1:8787/lifecycle`
- `http://127.0.0.1:8787/hardware`
- `http://127.0.0.1:8787/commercial`
- `http://127.0.0.1:8787/assurance`
- `http://127.0.0.1:8787/developer`
- `http://127.0.0.1:8787/data-platform`

The GUI includes table filtering, infrastructure workflow preview recalculation, tenant provisioning preview recalculation, hardware request preview, CSV export for visible infrastructure/tenant/planner/lifecycle/hardware/commercial/assurance/developer/data-platform tables, repo document links, VS Code clone/action links, and visible action feedback for staged infrastructure, operator, lifecycle, hardware, commercial, assurance, developer, data, and edge workflows.

The Radxa-local edge service can be run separately:

```bash
cargo run -p osdc-edge -- 127.0.0.1:8790
```

Open:

- `http://127.0.0.1:8790/`
- `http://127.0.0.1:8790/api/status`
- `http://127.0.0.1:8790/api/config-preview`
