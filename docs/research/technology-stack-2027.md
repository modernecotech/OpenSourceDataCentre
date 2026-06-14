# Technology Stack Research for 2027+ Open Source Datacentres

Last reviewed: 2026-06-14.

This document turns the project vision into a stack recommendation for datacentres designed from 2027 onward. The practical strategy is a Rust control plane that integrates mature open-source systems instead of replacing them. Rust should own the project-specific code: APIs, adapters, calculators, workflow automation, policy checks, and the unified interface.

## Decision Principles

1. Prefer open standards and open-source implementations over proprietary control planes.
2. Keep a vendor-neutral hardware envelope: 19-inch EIA first, Open19 for modular 19-inch deployments, OCP Open Rack V3/Open Rack Wide for high-density AI and hyperscale deployments.
3. Treat building systems as operational technology with safety boundaries. Rust software can supervise, model, schedule, and alert, but final control loops should remain in certified field controllers where required by local law and engineering practice.
4. Separate source-of-truth data from runtime telemetry. Inventory belongs in DCIM/IPAM tooling; telemetry belongs in time-series/log/tracing systems; orchestration belongs in the Rust control plane.
5. Require license review before adding any "open" AI model. Prefer Apache-2.0, MIT, or OSI-compatible terms where possible, and classify open-weight models separately from fully open-source AI systems.

## Recommended Stack by Domain

| Domain | Recommended baseline | Why | Watchouts |
| --- | --- | --- | --- |
| Mechanical design | FreeCAD 1.1, IFC, STEP, CSV BOM exports | FreeCAD 1.1 is current and suitable for open mechanical artifacts. IFC keeps building design exchangeable. | Treat FreeCAD files as source; export neutral formats for review and manufacturing. |
| Building energy modelling | EnergyPlus + OpenStudio | OpenStudio is an open-source SDK for building energy modelling with EnergyPlus. | Datacentre loads require careful schedules and heat-rejection modelling. |
| CFD and airflow | OpenFOAM | OpenFOAM is mature open-source CFD with heat-transfer capability. | Requires validated boundary conditions and mesh QA; do not use CFD results without commissioning data. |
| Primary cooling architecture | Rack thermal spine: rack heat capture, underfloor/service-trench heat bus, ground/free rejection, sorption cooling, backup CDU/chiller | Captures heat close to source, keeps vendors interchangeable, and creates a path to recycle waste heat into chilled water where temperature allows. | Must not assume heat can cool itself fully; final heat rejection and backup cooling remain mandatory. |
| Earth-based cooling | Ground-source heat pumps, closed-loop ground heat exchangers, Cold UTES where geology allows | DOE notes geothermal/Cold UTES can reduce peak datacentre cooling demand and energy cost. | Site geology, groundwater law, water rights, corrosion, and maintenance drive feasibility. |
| Free cooling | Water-side economizers; air-side only where filtration/humidity and pollution are acceptable | ENERGY STAR notes water-side economizers can bypass chillers when conditions allow. | Requires climate analysis and redundancy for liquid-cooled AI loads. |
| Building integration | BACnet/IP, Modbus TCP, OPC UA, MQTT, Project Haystack, Brick | BACnet is the building automation norm; OPC UA provides secure industrial data modelling; MQTT is useful for telemetry; Haystack/Brick provide semantics. | Protocol gateways should be isolated from IT networks and authenticated. |
| Distributed building control | Eclipse VOLTTRON where agent-based control is useful | VOLTTRON is open-source and designed for distributed sensing/control in buildings and grids. | Python ecosystem; integrate with Rust over APIs/message bus rather than rewriting immediately. |
| Solar/storage/DC microgrid EMS | OpenEMS, NREL REopt for planning, sodium-ion BESS, MPPT DC/DC converters, bidirectional battery converters, 380-400 VDC backbone, and 48 VDC rack power | OpenEMS targets PV, battery storage, EV charging, heat pumps, and tariffs. REopt supports cost-optimal solar/storage planning. Sodium-ion is increasingly aimed at stationary storage. 380/400 VDC distribution reduces repeated AC/DC conversion while 48 V aligns with rack and telecom-style equipment. | OpenEMS licensing includes AGPL/EPL components; review before embedding. Sodium-ion suppliers need extra validation because the market is younger than LFP. DC protection, arc-fault behavior, isolation monitoring, and local code approval are major design gates. |
| DCIM/source of truth | NetBox for network/IP/DCIM; openDCIM for simpler inventory/floor/rack mapping | NetBox has strong APIs and models networks/DCIM; openDCIM is simpler and GPL-based. | Avoid making Rust services the manual inventory system. Sync, validate, and extend. |
| Racks and hardware standards | 19-inch EIA, Open19, OCP Open Rack V3, OCP Open Rack Wide, OCP DC-MHS | Open19 keeps standard 19-inch fit; OCP ORV3/ORW target high-density and AI; DC-MHS improves server modularity. | ORW is very new; keep adapters and aisle/service clearances modular. |
| Low-power SBC baseline | Radxa ROCK 5B+ / RK3588 class boards with Debian or Armbian | RK3588 gives 8 ARM cores, useful NVMe/network I/O, and a 6 TOPS NPU for edge/control tasks at low power. | Use for gateways and edge services, not memory-heavy cloud services. Mainline/community kernel readiness must be checked per board revision. |
| Open Linux GPU baseline | AMD Radeon RX 9060 XT 16GB for default ROCm GPU lane; Intel Arc Pro B50 16GB for low-power XPU lane | Both keep the project closer to open Linux graphics/compute stacks than a CUDA-only baseline. AMD is the default for ROCm/HIP; Intel is attractive where 70 W and 16 GB VRAM matter. | ROCm and oneAPI support must be tested per kernel/runtime release. NVIDIA can be an optional compatibility pool, not the default. |
| Bare metal lifecycle | OpenStack Ironic, Metal3, Redfish, OpenBMC where hardware supports it | Ironic manages bare-metal lifecycle; Metal3 brings bare-metal hosts into Kubernetes Cluster API; OpenBMC reduces firmware lock-in. | Hardware compatibility and firmware supply-chain support matter more than logos. |
| Compute orchestration | Kubernetes on Talos Linux or k0s; Slurm for HPC; Kueue for Kubernetes-native queues | Kubernetes is the open baseline for container orchestration; Kueue handles batch/HPC/AI queueing; Slurm remains important for HPC/training. | Use Slurm when MPI/HPC users expect it; use Kueue for cloud-native AI/platform jobs. |
| Networking | SONiC on compatible switches, FRRouting, Cilium, Open vSwitch/OVN | SONiC runs on multiple switch vendors/ASICs; Cilium gives eBPF networking/security/observability. | SONiC operations need strong network engineering skill and tested hardware lists. |
| Storage | Ceph + Rook for block/file/object; local NVMe with OpenEBS for latency-sensitive Kubernetes volumes | Ceph provides distributed object/block/file storage; Rook manages Ceph in Kubernetes. | Ceph needs operational discipline; model power, rebuild time, and failure domains. |
| Observability | OpenTelemetry, Prometheus or VictoriaMetrics, Grafana OSS, Loki-compatible logs | OpenTelemetry is now a vendor-neutral standard; Prometheus is the open metrics baseline; Grafana OSS visualizes metrics/logs/traces. | Define retention, cardinality budgets, and facility telemetry sampling early. |
| Identity and policy | Keycloak, OPA, OpenZiti for zero-trust overlays | Keycloak provides open-source IAM; OPA centralizes policy; OpenZiti provides zero-trust connectivity. | Physical-security systems need separate break-glass procedures. |
| Physical security | Leosac for open physical access control pilots; Frigate for local NVR/object detection; Keycloak for application IAM | Leosac is open source physical access control; Frigate runs local camera inference. | Door control is safety-critical. Pilot before production and meet local codes. |
| AI serving | vLLM and SGLang for inference; Hugging Face tooling for packaging; OCI images for reproducibility | vLLM and SGLang are high-throughput open serving engines for open models. | Model licenses, safety filters, data retention, and GPU isolation must be governed. |
| AI queueing | Kueue for Kubernetes-native AI batch jobs; Slurm for HPC/training; optional Ray for distributed Python jobs | Kueue is explicitly for Kubernetes batch/HPC/AI/ML queueing. | Keep fair-share and priority rules visible to users. |
| Open cloud service layer | OpenStack, Ceph, Kubernetes, Kueue, Knative, OpenBao, Harbor, OpenTofu, CloudNativePG, NATS/Kafka, OpenCost, CloudKitty | Covers the broad domains users expect from AWS-style infrastructure while staying open-source and self-hostable. | The Rust portal should orchestrate these tools, not replace them. Start with a small service catalog. |
| Unified interface | Rust services using axum/Tokio or a lightweight Rust HTTP server, PostgreSQL, OpenAPI, gRPC/tonic where needed, Leptos or a modest web UI | Rust keeps the project efficient and portable while integrating existing systems. | Avoid framework churn; API stability matters more than UI novelty. |

## Architecture Recommendation

The core implementation should be an `osdc-control-plane` composed of Rust services:

- `inventory-adapter`: reads NetBox/openDCIM, exposes normalized facility/IT inventory.
- `telemetry-ingest`: subscribes to Prometheus remote read, MQTT, OPC UA, BACnet gateways, and logs.
- `calculator`: computes CAPEX/OPEX, PUE, WUE, CUE, carbon, cooling capacity, rack power, and AI job cost.
- `cooling-design`: sizes rack heat capture, thermal-spine transport, sorption cooling offset, and heat-rejection load.
- `scheduler-adapter`: integrates Kueue, Slurm, Kubernetes, and AI serving endpoints.
- `policy`: calls OPA and enforces safety, queue, tenancy, and energy policies.
- `portal-api`: one Rust API for admins, operators, and users.

The Rust layer should own policy and workflow, not every data store. For example, it should not replace Prometheus or NetBox; it should validate and combine their data.

## Open AI Model Guidance

Prefer model families with clear terms allowing use, modification, derivative works, and sharing:

- Qwen3: open-weight models under Apache-2.0 according to Qwen documentation.
- Mistral 3 open models: announced under Apache-2.0.
- DeepSeek-R1: repository and weights under MIT, with commercial use and derivative works allowed.
- OLMo-style fully open research models where training data/process transparency is required.

Use the OSI Open Source AI Definition as the long-term target, but label most current models honestly as "open-weight" unless training data, code, parameters, and modification materials are all available.

Do not use non-commercial models in shared infrastructure without an explicit exception. Do not assume Meta Llama-style community licenses are open source under OSI terms.

## Physical and Sustainability Metrics

The calculator baseline should include:

- `PUE = total facility energy / IT equipment energy`
- `WUE = water use / IT equipment energy`
- `CUE = CO2 emissions caused by datacentre energy / IT equipment energy`
- Renewable fraction, grid import, storage loss, cooling overhead, embodied-carbon placeholders, and rack-level utilization.

These metrics are not enough on their own. For developing-world deployments, add reliability and sovereignty metrics:

- Hours of autonomy at critical load.
- Percentage of workloads that can run during islanded operation.
- Spare-parts locality score.
- Vendor lock-in score.
- Operator skill and maintainability score.

## Near-Term Build Choices

For the first public technical baseline:

1. Rust workspace with typed site profile, energy/carbon/water cost calculators, and JSON/CSV import/export.
2. NetBox-first source-of-truth integration design.
3. Tenant portal and operator console over one Rust API.
4. OpenStack/Ceph/Kubernetes service map for AWS-like user workflows.
5. Kubernetes + Kueue path for AI jobs, with Slurm as a second scheduler profile.
6. OCP/Open19 rack metadata model that can represent non-OCP local fabrication adapters.
7. FreeCAD 1.1 naming, BOM, and export conventions.
8. Rack thermal-spine cooling prototype: warm-water row loop first, then two-phase thermosyphon/loop heat-pipe modules if serviceability and regulation allow.
9. Component simplification catalogue and 250 kW pilot BOM, with local fabrication candidates and second-source notes.
10. Test harness using Rust unit tests, JSON golden files, containerized integration tests, and simulation fixtures.

## Source Notes

- ASHRAE datacentre resources and TC 9.9 liquid-cooling resilience notes: https://www.ashrae.org/technical-resources/bookstore/datacom-series and https://tpc.ashrae.org/?cmtKey=fd4a4ee6-96a3-4f61-8b85-43418dfa988d
- FreeCAD 1.1 release notes: https://wiki.freecad.org/Release_notes_1.1
- OpenStudio and EnergyPlus: https://openstudio.net/ and https://energyplus.net/
- OpenFOAM: https://www.openfoam.com/ and https://openfoam.org/
- DOE geothermal and datacentres: https://www.energy.gov/hgeo/geothermal/geothermal-and-data-centers
- ENERGY STAR water-side economizers: https://www.energystar.gov/products/data_center_equipment/16-more-ways-cut-energy-waste-data-center/consider-water-side-economizers
- Project Haystack: https://www.project-haystack.org/
- BACnet open-source stack: https://github.com/bacnet-stack/bacnet-stack
- open62541 OPC UA: https://www.open62541.org/
- Eclipse VOLTTRON: https://volttron.org/ and https://eclipse-volttron.readthedocs.io/
- OpenEMS: https://openems.io/ and https://github.com/OpenEMS/openems
- NREL REopt notes: https://www.energy.gov/cmei/femp/events/reopt-web-tool-portfolio-analysis
- NetBox docs: https://netboxlabs.com/docs/netbox/
- openDCIM: https://opendcim.org/
- OCP rack and power, ORV3/ORW specs, and DC-MHS: https://www.opencompute.org/community/rack-and-power, https://www.opencompute.org/wiki/Open_Rack/SpecsAndDesigns, and https://www.opencompute.org/wiki/Server/MHS
- ETSI EN 300 132-3 400 VDC ICT power interface: https://www.etsi.org/deliver/etsi_en/300100_300199/30013203/02.03.01_60/en_30013203v020301p.pdf
- LBNL/EMerge Alliance 380 VDC datacentre architecture work: https://datacenters.lbl.gov/sites/default/files/380VdcArchitecturesfortheModernDataCenter.pdf
- Compute hardware baseline sources are captured in [Compute Hardware Baseline](compute-hardware-baseline-2026.md).
- Open cloud service sources are captured in [Open Cloud Service Map](../architecture/open-cloud-service-map.md).
- OCP Data Center Facility: https://www.opencompute.org/community/data-center-facility
- OCP Cooling Environments: https://www.opencompute.org/community/cooling-environments
- LBNL liquid-cooled rack specification work: https://datacenters.lbl.gov/development-liquid-cooled-rack-specification
- Heat-driven adsorption chillers: https://heatpumpingtechnologies.org/publications/heat-driven-adsorption-chiller-systems-for-sustainable-cooling-applications/
- Datacentre waste heat and sorption cooling review: https://www.mdpi.com/2071-1050/17/22/10101
- Two-phase thermosyphon datacentre cooling review: https://link.springer.com/article/10.1186/s44147-025-00833-3
- Heat-driven Stirling refrigerator research: https://pubs.aip.org/aip/apl/article/124/12/123905/3278014/Experimental-and-numerical-study-on-a-heat-driven
- Open19 overview: https://www.linuxfoundation.org/press/press-release/the-linux-foundation-hosts-open19-to-acceleratedata-center-and-edge-hardware-innovation
- Ironic and Metal3: https://ironicbaremetal.org/ and https://metal3.io/
- SONiC and OpenBMC: https://sonicfoundation.dev/ and https://openbmc.org/
- Kubernetes, Kueue, Slurm: https://kubernetes.io/, https://kueue.sigs.k8s.io/, and https://slurm.schedmd.com/overview.html
- Ceph and Rook: https://ceph.io/en/ and https://rook.io/
- OpenTelemetry, Prometheus, Grafana OSS, VictoriaMetrics: https://opentelemetry.io/, https://prometheus.io/, https://grafana.com/docs/grafana/latest/, and https://victoriametrics.com/
- Keycloak, OPA, OpenZiti: https://www.keycloak.org/, https://openpolicyagent.org/docs, and https://openziti.io/
- Leosac and Frigate: https://github.com/leosac/access-control and https://frigate.video/
- vLLM and SGLang: https://docs.vllm.ai/ and https://docs.sglang.ai/
- Qwen3, DeepSeek-R1, Mistral 3, and OSI Open Source AI Definition: https://github.com/qwenLM/qwen3, https://github.com/deepseek-ai/deepseek-r1, https://mistral.ai/news/mistral-3/, and https://opensource.org/ai/open-source-ai-definition
- PUE/WUE/CUE background: https://archive.thegreengrid.org/en/resources/library-and-tools/238-WP and https://www.thegreengrid.org/en/resources/library-and-tools/241-WP%2332---Carbon-Usage-Effectiveness-%28CUE%29%3A-A-Green-Grid-Data-Center-Sustainability-Metric
- OpenCost: https://opencost.io/
- Rust web stack references: https://github.com/tokio-rs/axum, https://leptos.dev/, and https://v2.tauri.app/
