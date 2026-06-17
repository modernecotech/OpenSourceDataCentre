# Open Source Data Centre

Open Source Data Centre is a reference architecture, software stack, mechanical design system, and procurement toolkit for countries that need sovereign, sustainable, affordable datacentres.

The project is designed for developing-world governments, universities, public-sector clouds, national AI programmes, hospitals, telecom operators, and regional infrastructure companies that need to host sensitive data locally without becoming permanently dependent on hyperscalers, proprietary datacentre vendors, or closed management platforms.

The goal is not to build a fragile "cheap datacentre."

The goal is to make reliable, maintainable, auditable datacentre infrastructure that can be built in stages, operated by local engineers, repaired with documented parts, powered partly by local renewable energy, and expanded from a 50 kW edge site to a national AI-ready sovereign cloud.

**Mission:** build sovereign cloud capacity that countries can own, understand, repair, expand, and power sustainably.

The project combines:

- Building systems for DC-powered HVAC auxiliaries, lighting, physical security, solar power, sodium-ion storage, and earth-based cooling.
- FreeCAD 1.1 design artifacts for mechanical parts, racks, adapters, cable paths, and serviceable assemblies.
- Flexible rack patterns for 19-inch EIA, Open19, OCP Open Rack V3, and Open Rack Wide where appropriate.
- A Rust-based unified control plane for inventory, observability, cost modelling, scheduling, workflow automation, and user/admin interfaces.
- Open-source infrastructure stacks for bare metal, Kubernetes, storage, networking, monitoring, identity, AI serving, and job queueing.
- Test harnesses and operational guidance for building systems, IT systems, AI workloads, and cost calculators.

## What This Project Is

- A reference architecture for sovereign, sustainable datacentres.
- A practical build path from 50 kW edge sites to national AI-ready infrastructure.
- A set of open BOMs, calculators, software adapters, mechanical designs, and commissioning tests.
- A way for countries to reduce dependency on closed cloud platforms and proprietary datacentre management stacks.

## What This Project Is Not

- It is not a certified datacentre design.
- It is not a substitute for licensed local engineers, fire engineers, electrical engineers, code approval, or authority review.
- It is not a promise that marketplace parts are safe, compliant, or suitable for life-safety and critical-power systems.
- It is not a hyperscaler clone.
- It is not a reason to skip commissioning, documentation, training, spare parts, security review, or disaster recovery.

## Design Principles for Sovereign Datacentres

1. **Sovereignty first** - data, encryption keys, logs, backups, identity, and admin control must remain under national or institutional control.
2. **Open where possible** - use open-source software, open standards, documented APIs, and exportable data models.
3. **Local maintainability** - every critical system needs a local spares plan, local training path, and second-source plan.
4. **Simplify topology, not safety** - reduce unnecessary vendor complexity, but do not reduce fire, electrical, commissioning, or operational safety.
5. **Energy-aware by design** - power availability, solar integration, storage, cooling efficiency, and workload scheduling are core design inputs.
6. **Upgradeable path** - start with 19-inch commodity racks and grow toward Open19, OCP Open Rack V3, or Open Rack Wide where the supply chain can support it.
7. **No false certification claims** - describe Tier II/Tier III-like design intent, but do not claim Uptime Tier certification unless formally audited.

## Deployment Ladder

| Stage | Use case | Main goal |
| --- | --- | --- |
| **50 kW edge micro** | University, hospital group, ministry edge, ISP edge | Prove local operation and sovereign services. |
| **250 kW regional pilot** | First serious public-sector cloud | Validate power, cooling, cloud stack, operators, costs, and commissioning discipline. |
| **1 MW regional production** | National health, education, government apps, research | Deliver reliable sovereign cloud capacity. |
| **5 MW national/AI-ready** | National AI, HPC, larger public cloud | Build strategic compute infrastructure. |

The flagship developing-world pilot is the 250 kW regional design: 10 racks at 25 kW average, N+1 pumps and controls, DC-first solar sodium-ion microgrid, fallback generator path, open-source management stack, and local fabrication where safe.

## Local Maintainability Doctrine

A sovereign datacentre is not sovereign if it cannot be repaired locally.

Every critical component must have:

- a documented function;
- wiring or interface drawings;
- at least one second-source option;
- local spare-part classification;
- maintenance interval;
- failure symptom list;
- safe replacement procedure;
- commissioning or post-replacement test.

## Software Position

The software goal is a sovereign cloud user experience over open infrastructure, not a new AWS clone.

```text
User experience:
  AWS-like portal and API

Underneath:
  OpenStack
  Ceph
  Kubernetes
  Kueue
  Slurm
  Keycloak
  OPA
  NetBox/openDCIM
  OpenTelemetry
  Grafana
  Rust adapters
```

Rust owns APIs, adapters, calculators, workflow automation, policy checks, and the unified interface. Mature infrastructure projects own the low-level cloud, storage, identity, telemetry, and scheduling systems.

## Sovereign Edge and Security Fabric

OSDC Edge Shield is the repository's open-source edge and security pillar. It provides a nationally operated regional equivalent for the same classes of service commonly bought from Cloudflare, Akamai, Fastly, Okta, Datadog, Splunk, Vault, Terraform Cloud, and similar proprietary infrastructure platforms.

It is not a claim to replace Cloudflare's global anycast network, global DDoS absorption, commercial threat intelligence, or 24/7 managed security operations. It is a sovereign fabric for DNS, TLS, CDN cache, WAF, rate limiting, private tunnels, identity-aware access, secrets, logs, metrics, policies, keys, audit trails, and GitOps-controlled rollout.

## Repository Map

- [Sovereign Datacentre Mission](docs/strategy/sovereign-datacentre-mission.md) - mission, audience, and national implementation outputs.
- [Developing-World Deployment Model](docs/strategy/developing-world-deployment-model.md) - adoption ladder and planning assumptions.
- [What This Is and Is Not](docs/strategy/what-this-is-and-is-not.md) - scope guardrails and safety boundaries.
- [Sovereign Edge Security Stack](docs/security/sovereign-edge-security-stack.md) - OSDC Edge Shield as the security fabric around sovereign datacentres.
- [Cloudflare Equivalent Open Tooling](docs/security/cloudflare-equivalent-open-tooling.md) - open-source mapping for DNS, CDN, WAF, access, tunnels, telemetry, secrets, IaC, and SOC tools.
- [DDoS Realistic Threat Model](docs/security/ddos-realistic-threat-model.md) - what Edge Shield can and cannot absorb locally.
- [Zero Trust Access](docs/security/zero-trust-access.md), [WAF and API Protection](docs/security/waf-and-api-protection.md), and [SIEM/SOC Open Source Stack](docs/security/siem-soc-open-source-stack.md) - operating guides for the edge/security pillar.
- [Country Site Profile Guide](docs/deployment/country-site-profile-guide.md) - country-pack schema, planning fields, and profile examples.
- [50 kW Edge Micro](docs/deployment/50kw-edge-micro.md), [250 kW Regional Pilot](docs/deployment/250kw-regional-pilot.md), [1 MW Regional Production](docs/deployment/1mw-regional-production.md), and [5 MW National AI-Ready](docs/deployment/5mw-national-ai-ready.md) - staged reference deployment patterns.
- [Commissioning Overview](docs/commissioning/commissioning-overview.md) - L1-L5 commissioning model and critical integrated tests.
- [Data Residency](docs/sovereignty/data-residency.md), [Key Management](docs/sovereignty/key-management.md), and [Backup and Disaster Recovery](docs/sovereignty/backup-and-disaster-recovery.md) - sovereignty controls for public infrastructure.
- [Local Fabrication Guide](docs/procurement/local-fabrication-guide.md) and [Second-Source Requirements](docs/procurement/second-source-requirements.md) - maintainability and procurement doctrine.
- [Operator Training](docs/operations/operator-training.md), [Spares and Tools](docs/operations/spares-and-tools.md), and [Emergency Runbooks](docs/operations/emergency-runbooks.md) - local operations pack.
- [Technology Stack Research](docs/research/technology-stack-2027.md) - sourced recommendations for 2027+ builds.
- [Compute Hardware Baseline](docs/research/compute-hardware-baseline-2026.md) - default SBC/GPU choices for open Linux, low-cost, low-power deployments.
- [Reference Architecture](docs/architecture/reference-architecture.md) - system layers and integration boundaries.
- [Open Cloud Service Map](docs/architecture/open-cloud-service-map.md) - AWS-like service domains mapped to open-source tools.
- [Core Cloud Services Baseline](docs/architecture/core-cloud-services.md) - chosen AWS/Azure-like services implemented first.
- [OSDC Edge Shield](docs/architecture/edge-shield-cloudflare-alternative.md) - Radxa-based open alternative to Cloudflare-style DNS, CDN, WAF, tunnels, and access.
- [Portal API Prototype](docs/architecture/portal-api.md) - current Rust GUI/API routes and next adapter path.
- [Data Model](docs/architecture/data-model.md) - initial domain objects for Rust services and APIs.
- [Rack Thermal Spine Cooling](docs/design/rack-thermal-spine-cooling.md) - priority design for rack heat capture, underfloor heat transport, and heat-driven cooling.
- [Solar Sodium-Ion DC Microgrid Power](docs/design/solar-sodium-inverter-power.md) - UPS-less DC-first topology from PV and sodium-ion storage to racks, cooling auxiliaries, and single fallback boundary input.
- [Systems and BOM Strategy](docs/design/systems-simplification-bom.md) - state-of-the-art components simplified for developing-world deployments.
- [FreeCAD Guidelines](docs/design/freecad-1.1-guidelines.md) - mechanical and design-artifact conventions.
- [Test Harnesses](docs/process/test-harnesses.md) - verification strategy across facilities, IT, AI, and cost tools.
- [Cost Calculators](docs/process/cost-calculators.md) - calculator scope, formulas, and validation rules.
- [Alibaba/AliExpress Cost Scenarios](docs/costing/alibaba-aliexpress-scenarios.md) - marketplace price basis, scale scenarios, and build-time estimates.
- [Open AI Governance](docs/process/open-ai-governance.md) - model selection and queueing guidance.
- [Rust Workspace](crates/) - calculator, model, CLI, portal, and edge service crates.
- [Portal Crate](crates/osdc-portal/) - Rust-served tenant, operator, and Edge Shield GUI/API prototype.
- [Edge Crate](crates/osdc-edge/) - Radxa-local Edge Shield status and config-preview service.
- [BOM Data](data/bom/) - component catalogue and starter 250 kW bill of materials.
- [Costing Data](data/costing/) - current marketplace price basis and scenario cost ranges.
- [Hardware Data](data/hardware/) - chosen SBC/GPU baseline profiles.
- [Software Service Data](data/software/) - open cloud service catalogue mappings.
- [Country Profiles](data/country-profiles/) - example country-planning packs for grid, climate, energy, procurement, and sovereignty assumptions.

## Initial Technical Position

The project should not try to rewrite mature infrastructure tools. The core software should be Rust, but the platform should integrate best-in-class open-source systems through typed adapters:

- Rust for the unified API, policy-aware automation, calculators, adapters, and eventually the web/control interface.
- A Rust-served tenant portal and operator console for user provisioning and datacentre operations.
- NetBox or openDCIM as inventory/DCIM sources of truth, with Rust services adding cost, workflow, and facility-aware orchestration.
- Kubernetes, Kueue, Slurm, and open model-serving engines for AI and batch workloads.
- BACnet, Modbus, OPC UA, MQTT, Project Haystack, and Brick-compatible semantics for building-system integrations.
- OpenTelemetry, Prometheus/VictoriaMetrics, Grafana OSS, and structured logs for observability.

## Rust Quick Start

```bash
cargo fmt --check
cargo test
scripts/verify.sh
cargo run -p osdcctl -- examples/site-profile.json
cargo run -p osdc-portal -- 127.0.0.1:8787
cargo run -p osdc-edge -- 127.0.0.1:8790
```

The first CLI calculates high-level energy, water, carbon, and cost metrics from an example site profile. It is intentionally small: the value is establishing tested formulas and typed inputs early.

The first portal serves three GUI surfaces:

- Tenant portal: `http://127.0.0.1:8787/user`
- Operator console: `http://127.0.0.1:8787/operator`
- Edge Shield console: `http://127.0.0.1:8787/edge`
- Cost planner: `http://127.0.0.1:8787/planner`

The portal GUI exposes tenant provisioning previews, service-catalog filtering, tenant resource CSV export, operator power/cooling/cloud-stack views, Edge Shield service/config rollout previews, and scale/cost planning from the marketplace scenario data. The local edge service exposes a Radxa-ready dashboard at `http://127.0.0.1:8790/` plus JSON APIs at `/api/status` and `/api/config-preview`.

For CSV fixture checks:

```bash
find data -name '*.csv' -print0 | xargs -0 -n1 sh -c 'awk -F, "NR == 1 { cols = NF } NR > 1 && NF != cols { exit 1 }" "$0"'
```

## License

No project license has been selected yet. The recommended default for this repository is Apache-2.0 or MIT for Rust code and CC-BY-4.0 for documentation/design material, unless the project wants stronger copyleft guarantees.
