# Open Source Data Centre

Open Source Data Centre is a reference architecture and implementation path for sustainable, vendor-neutral datacentres that can be built and operated from 2027 onward without tying operators to proprietary hardware, licensing, or management platforms.

The project combines:

- Building systems for HVAC, lighting, physical security, solar power, energy storage, and earth-based cooling.
- FreeCAD 1.1 design artifacts for mechanical parts, racks, adapters, cable paths, and serviceable assemblies.
- Flexible rack patterns for 19-inch EIA, Open19, OCP Open Rack V3, and Open Rack Wide where appropriate.
- A Rust-based unified control plane for inventory, observability, cost modelling, scheduling, workflow automation, and user/admin interfaces.
- Open-source infrastructure stacks for bare metal, Kubernetes, storage, networking, monitoring, identity, AI serving, and job queueing.
- Test harnesses and operational guidance for building systems, IT systems, AI workloads, and cost calculators.

## Repository Map

- [Technology Stack Research](docs/research/technology-stack-2027.md) - sourced recommendations for 2027+ builds.
- [Reference Architecture](docs/architecture/reference-architecture.md) - system layers and integration boundaries.
- [Data Model](docs/architecture/data-model.md) - initial domain objects for Rust services and APIs.
- [Rack Thermal Spine Cooling](docs/design/rack-thermal-spine-cooling.md) - priority design for rack heat capture, underfloor heat transport, and heat-driven cooling.
- [FreeCAD Guidelines](docs/design/freecad-1.1-guidelines.md) - mechanical and design-artifact conventions.
- [Test Harnesses](docs/process/test-harnesses.md) - verification strategy across facilities, IT, AI, and cost tools.
- [Cost Calculators](docs/process/cost-calculators.md) - calculator scope, formulas, and validation rules.
- [Open AI Governance](docs/process/open-ai-governance.md) - model selection and queueing guidance.
- [Rust Workspace](crates/) - initial calculator and model crates.

## Initial Technical Position

The project should not try to rewrite mature infrastructure tools. The core software should be Rust, but the platform should integrate best-in-class open-source systems through typed adapters:

- Rust for the unified API, policy-aware automation, calculators, adapters, and eventually the web/control interface.
- NetBox or openDCIM as inventory/DCIM sources of truth, with Rust services adding cost, workflow, and facility-aware orchestration.
- Kubernetes, Kueue, Slurm, and open model-serving engines for AI and batch workloads.
- BACnet, Modbus, OPC UA, MQTT, Project Haystack, and Brick-compatible semantics for building-system integrations.
- OpenTelemetry, Prometheus/VictoriaMetrics, Grafana OSS, and structured logs for observability.

## Rust Quick Start

```bash
cargo test
cargo run -p osdcctl -- examples/site-profile.json
```

The first CLI calculates high-level energy, water, carbon, and cost metrics from an example site profile. It is intentionally small: the value is establishing tested formulas and typed inputs early.

## License

No project license has been selected yet. The recommended default for this repository is Apache-2.0 or MIT for Rust code and CC-BY-4.0 for documentation/design material, unless the project wants stronger copyleft guarantees.
