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

## What This Repository Produces

| Output | Purpose |
| --- | --- |
| Reference architectures | 50 kW, 250 kW, 1 MW, and 5 MW sovereign datacentre deployment patterns. |
| BOM and cost calculators | Country-specific CAPEX/OPEX, import duty, local labour, spares, energy, water, carbon, outage, and fallback-fuel planning. |
| FreeCAD / IFC / STEP design system | Buildable structures, racks, adapters, service trenches, thermal spines, cable trays, and serviceable parts. |
| Commissioning and reliability pack | L1-L5 commissioning, grid-loss, DC-bus ride-through, cooling failover, generator-start, and backup-restore tests. |
| Operator training pack | Local skills, runbooks, spares lists, maintenance schedules, emergency procedures, staffing, and escalation paths. |
| Sovereign cloud service catalogue | Open-source cloud, edge, security, developer, data, AI, observability, backup, and operations services under one portal/API. |
| Edge Shield security fabric | Sovereign DNS, TLS, CDN cache, WAF, rate limiting, private tunnels, zero-trust access, logs, metrics, secrets, policy, and audit. |
| Browser config management | Web-based editing of real tool config scripts with validation, GitOps review, staged rollout, rollback checks, and audit. |

## What This Project Is

- A reference architecture for sovereign, sustainable datacentres.
- A practical build path from 50 kW edge sites to national AI-ready infrastructure.
- A set of open BOMs, calculators, software adapters, mechanical designs, and commissioning tests.
- A way for countries to reduce dependency on closed cloud platforms and proprietary datacentre management stacks.
- A national implementation manual for cloud capacity that can be operated and repaired locally.

## What This Project Is Not

- It is not a certified datacentre design.
- It is not a substitute for licensed local engineers, fire engineers, electrical engineers, code approval, or authority review.
- It is not a promise that marketplace parts are safe, compliant, or suitable for life-safety and critical-power systems.
- It is not a hyperscaler clone.
- It is not a Cloudflare clone or a claim to reproduce global anycast/DDoS capacity.
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

## Architecture At A Glance

```text
             OSDC Unified Portal
        Rust API / policy / audit / cost
                    |
 +------------------+------------------+
 |                  |                  |
Tenant UI       Operator UI        Security UI
 |                  |                  |
VMs/K8s/AI      racks/power/DCIM    SIEM/WAF/IAM
 |                  |                  |
OpenStack       NetBox/openDCIM     Wazuh/Falco/OPA
Kubernetes      Prometheus          Keycloak/OpenBao
Ceph            Grafana             Edge Shield
Harbor          Velero              Coraza/CrowdSec
Argo/Flux       Cilium              Suricata/Zeek
```

The Rust layer does not replace mature infrastructure systems. It provides the unified portal/API, policy checks, cost and sustainability calculations, approval flows, config generation, health checks, audit events, and GitOps rollout orchestration.

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

## Sovereign Cloud Service Catalogue

The repository defines an open-source sovereign cloud service catalogue: identity, compute, storage, networking, edge, developer platform, observability, SOC, data, AI, DCIM, ITSM, backup, and upgrade workflows.

The portal should expose workflows, not every low-level setting:

- create a government tenant;
- issue a secure application endpoint;
- create a VM, Kubernetes cluster, bucket, volume, or PostgreSQL database;
- approve GPU queue access;
- deploy a model endpoint;
- approve an OpenTofu plan;
- patch Keycloak or Cilium through GitOps;
- restore last night's backup;
- show externally exposed services;
- show vulnerable images and non-compliant workloads;
- show which systems are behind on security patches.

The catalogue is built as bundles:

- **Bundle A - Sovereign Cloud Core:** OpenStack or CloudStack, Kubernetes, Ceph, Cilium, Keycloak, OpenBao, OPA/Kyverno, Harbor, OpenTofu, Argo CD or Flux, Prometheus/VictoriaMetrics, Grafana, Loki/Tempo, Velero, and NetBox.
- **Bundle B - Sovereign Edge and Security:** PowerDNS, dnsdist, Caddy/Envoy/HAProxy, Varnish, Coraza, CrowdSec, WireGuard/NetBird/OpenZiti, Wazuh, Falco, Suricata, Zeek, and MISP/OpenCTI.
- **Bundle C - Developer Platform:** Forgejo/Gitea, Woodpecker/Tekton, Harbor, Backstage option, Argo CD/Flux, Renovate, OpenTofu, Ansible/AWX, Trivy, Grype, Syft, and cosign.
- **Bundle D - Data and AI Platform:** CloudNativePG, Valkey, NATS, Kafka, Trino, Iceberg, ClickHouse, Superset, Airflow/Argo Workflows, MLflow, KServe, vLLM/SGLang, JupyterHub, and vector stores.

Machine-readable catalogue data lives in:

- [service-catalogue-v1.csv](data/software/service-catalogue-v1.csv)
- [proprietary-open-source-equivalents.csv](data/software/proprietary-open-source-equivalents.csv)
- [security-controls.csv](data/software/security-controls.csv)
- [upgrade-policy.csv](data/software/upgrade-policy.csv)
- [config-script-catalogue.csv](data/software/config-script-catalogue.csv)
- [service catalogue examples](examples/service-catalogue/)

## Sovereign Edge and Security Fabric

OSDC Edge Shield is the repository's open-source edge and security pillar. It provides a nationally operated regional equivalent for the same classes of service commonly bought from Cloudflare, Akamai, Fastly, Okta, Datadog, Splunk, Vault, Terraform Cloud, and similar proprietary infrastructure platforms.

It is not a claim to replace Cloudflare's global anycast network, global DDoS absorption, commercial threat intelligence, or 24/7 managed security operations. It is a sovereign fabric for DNS, TLS, CDN cache, WAF, rate limiting, private tunnels, identity-aware access, secrets, logs, metrics, policies, keys, audit trails, and GitOps-controlled rollout.

Minimum OSDC Edge Shield node:

- PowerDNS Authoritative + dnsdist.
- Caddy for TLS and reverse proxy.
- Varnish/Vinyl Cache for HTTP caching.
- Coraza WAF with OWASP Core Rule Set.
- CrowdSec + nftables for abuse response.
- WireGuard + Headscale or NetBird for private origin tunnels.
- Keycloak + OPA for zero-trust access.
- Prometheus + Loki + Grafana + OpenTelemetry for telemetry.
- `osdc-edge` Rust agent for config, health, audit, and status.

Regional profile:

```text
edge-a: active proxy/cache/DNS
edge-b: active proxy/cache/DNS
edge-c: management secondary DNS cold failover
```

OSDC Edge Shield provides sovereign edge protection, not unlimited DDoS protection. Volumetric attacks still require upstream ISPs, IXPs, scrubbing providers, BGP blackhole/FlowSpec, and national telecom coordination.

## Browser-Based Config Management

The UI should expose the real configuration scripts of individual tools rather than trying to capture every upstream option as a custom form field.

```text
Browser editor
   |
validate script
   |
open GitOps change
   |
review and approve
   |
staging rollout
   |
health checks and rollback test
   |
production rollout
   |
audit record
```

This keeps the browser as the management surface for the whole system while preserving the operational truth that mature tools have their own configuration languages. The portal can still provide guided forms for common workflows, but complete management should remain available through config-as-code.

The current prototype exposes editable sample configs for:

- Caddy: `/etc/caddy/Caddyfile`
- PowerDNS: `/etc/powerdns/pdns.d/osdc.conf`
- Coraza WAF: `/etc/coraza/osdc-crs.conf`
- CrowdSec: `/etc/crowdsec/acquis.yaml`
- WireGuard: `/etc/wireguard/osdc-edge.conf`

Safety rules:

- Browser edits stage GitOps changes; they do not rewrite live files directly.
- Secrets appear as references or placeholders, never raw secret values.
- High-risk files require service-owner review.
- DNS, WAF, tunnel, key, and certificate changes require validation and rollback checks.

## Managed Upgrade Path

A sovereign cloud must not become a frozen pile of old open-source tools.

```text
Upstream releases
   |
Renovate watches charts containers Rust crates packages OS images
   |
Pull request created
   |
SBOM + vulnerability scan + licence scan + policy check
   |
Staging rollout through Argo CD / Flux
   |
Smoke tests + backup/restore test + rollback test
   |
Approval window
   |
Production rollout
   |
Health verification
   |
Audit record stored
```

| Update class | Frequency | Process |
| --- | ---: | --- |
| Critical CVE | 24-72 hours | Emergency staging test, fast approval, production patch. |
| High security | Weekly | Normal PR, staging, rollout. |
| Normal patch | Monthly | Maintenance window. |
| Minor feature release | Quarterly | Compatibility test. |
| Major version | 6-12 months | Migration plan, backup, dry run. |
| Firmware/BMC | Quarterly or emergency | Lab test first, staged by rack. |
| Kubernetes/OpenStack/Ceph | Planned release train | Never ad hoc. |

The unified portal should show current version, available version, risk, staging result, backup status, rollback status, approval owner, scheduled window, and audit record. It should trigger GitOps; it should not SSH into machines randomly.

## Repository Map

- [Sovereign Datacentre Mission](docs/strategy/sovereign-datacentre-mission.md) - mission, audience, and national implementation outputs.
- [Developing-World Deployment Model](docs/strategy/developing-world-deployment-model.md) - adoption ladder and planning assumptions.
- [What This Is and Is Not](docs/strategy/what-this-is-and-is-not.md) - scope guardrails and safety boundaries.
- [Sovereign Edge Security Stack](docs/security/sovereign-edge-security-stack.md) - OSDC Edge Shield as the security fabric around sovereign datacentres.
- [Cloudflare Equivalent Open Tooling](docs/security/cloudflare-equivalent-open-tooling.md) - open-source mapping for DNS, CDN, WAF, access, tunnels, telemetry, secrets, IaC, and SOC tools.
- [DDoS Realistic Threat Model](docs/security/ddos-realistic-threat-model.md) - what Edge Shield can and cannot absorb locally.
- [Zero Trust Access](docs/security/zero-trust-access.md), [WAF and API Protection](docs/security/waf-and-api-protection.md), and [SIEM/SOC Open Source Stack](docs/security/siem-soc-open-source-stack.md) - operating guides for the edge/security pillar.
- [Sovereign Cloud Service Catalogue](docs/software/sovereign-cloud-service-catalogue.md) - broad open-source cloud service catalogue and bundles.
- [Unified Portal Integration Model](docs/software/unified-portal-integration-model.md) - Rust API and workflow layer over mature open-source systems.
- [Browser-Based Config Management](docs/software/browser-config-management.md) - expose real tool config scripts through browser editing, validation, GitOps, and audit.
- [Patching and Upgrade Policy](docs/software/patching-and-upgrade-policy.md) - GitOps-based managed upgrade path.
- [Developer Platform](docs/software/developer-platform.md) and [Data and AI Platform](docs/software/data-and-ai-platform.md) - service catalogue pillars beyond basic compute/storage.
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
- [Rust Workspace](crates/) - calculator, model, CLI, portal, Edge Shield, Edge config, and Edge policy crates.
- [Portal Crate](crates/osdc-portal/) - Rust-served tenant, operator, and Edge Shield GUI/API prototype.
- [Edge Crate](crates/osdc-edge/) - Radxa-local Edge Shield status and config-preview service.
- [BOM Data](data/bom/) - component catalogue and starter 250 kW bill of materials.
- [Costing Data](data/costing/) - current marketplace price basis and scenario cost ranges.
- [Hardware Data](data/hardware/) - chosen SBC/GPU baseline profiles.
- [Software Service Data](data/software/) - open cloud service catalogue mappings.
- [Country Profiles](data/country-profiles/) - example country-planning packs for grid, climate, energy, procurement, and sovereignty assumptions.
- [Service Catalogue Examples](examples/service-catalogue/) - scale-specific service bundle selections.
- [Config Script Examples](examples/config-scripts/) - sample tool configuration artifacts for browser-based editing.

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

The first portal serves four GUI surfaces:

- Tenant portal: `http://127.0.0.1:8787/user`
- Operator console: `http://127.0.0.1:8787/operator`
- Edge Shield console: `http://127.0.0.1:8787/edge`
- Cost planner: `http://127.0.0.1:8787/planner`

The portal GUI exposes tenant provisioning previews, service-catalog filtering, tenant resource CSV export, operator power/cooling/cloud-stack views, Edge Shield service/config rollout previews, browser-based config-script editing, and scale/cost planning from the marketplace scenario data.

Useful portal APIs:

- `/api/catalog/core-services`
- `/api/catalog/sovereign-services`
- `/api/catalog/upgrade-policy`
- `/api/catalog/blueprints`
- `/api/config/scripts`
- `/api/edge/services`
- `/api/edge/status`
- `/api/edge/config-preview`
- `/api/cost/planning`

The local edge service exposes a Radxa-ready dashboard at `http://127.0.0.1:8790/` plus JSON APIs at `/api/status` and `/api/config-preview`.

For CSV fixture checks:

```bash
find data -name '*.csv' -print0 | xargs -0 -n1 sh -c 'awk -F, "NR == 1 { cols = NF } NR > 1 && NF != cols { exit 1 }" "$0"'
```

## License

No project license has been selected yet. The recommended default for this repository is Apache-2.0 or MIT for Rust code and CC-BY-4.0 for documentation/design material, unless the project wants stronger copyleft guarantees.
