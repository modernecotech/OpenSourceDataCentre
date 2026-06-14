# Open Source Data Centre

Open Source Data Centre is a reference architecture and implementation path for sustainable, vendor-neutral datacentres that can be built and operated from 2027 onward without tying operators to proprietary hardware, licensing, or management platforms.

The project combines:

- Building systems for DC-powered HVAC auxiliaries, lighting, physical security, solar power, sodium-ion storage, and earth-based cooling.
- FreeCAD 1.1 design artifacts for mechanical parts, racks, adapters, cable paths, and serviceable assemblies.
- Flexible rack patterns for 19-inch EIA, Open19, OCP Open Rack V3, and Open Rack Wide where appropriate.
- A Rust-based unified control plane for inventory, observability, cost modelling, scheduling, workflow automation, and user/admin interfaces.
- Open-source infrastructure stacks for bare metal, Kubernetes, storage, networking, monitoring, identity, AI serving, and job queueing.
- Test harnesses and operational guidance for building systems, IT systems, AI workloads, and cost calculators.

## Repository Map

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
