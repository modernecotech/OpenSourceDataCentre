# Developing-World Deployment Model

The project uses a deployment ladder because most countries should not begin with a giant national cloud claim. The credible path is to prove local operation, scale the staff and supply chain, then expand capacity.

## Deployment Ladder

| Stage | Use case | Main goal | Default substrate posture |
| --- | --- | --- | --- |
| 50 kW edge micro | University, hospital group, ministry edge, ISP edge | Prove local operations, local identity, backup, monitoring, and basic sovereign services. | Proxmox VE or CloudStack before OpenStack. |
| 250 kW regional pilot | First serious public-sector cloud | Validate power, cooling, software stack, operator training, procurement assumptions, and commissioning discipline. | CloudStack or OpenStack with Ceph, Kubernetes, NetBox, MAAS/Ironic/Metal3. |
| 1 MW regional production | National health, education, government apps, research | Provide reliable sovereign cloud capacity for priority workloads. | OpenStack, Ceph, Kubernetes, Kueue/Slurm, full assurance and commercial evidence. |
| 5 MW national/AI-ready | National AI, HPC, larger public cloud | Deliver strategic compute infrastructure with AI and public-sector tenant isolation. | OpenStack/Ironic/Metal3, Ceph NVMe tiers, Kubernetes, Slurm/Kueue, OpenBMC/Redfish, SONiC where supportable. |

## Flagship Pilot

The 250 kW regional pilot is the reference point for developing-world adoption:

- 10 racks at 25 kW average IT load.
- Headroom for selected higher-density liquid-cooled racks.
- N+1 critical pumps and controls, without claiming full Tier IV duplication.
- Solar-first, DC-first power path using PV, sodium-ion BESS, 380-400 VDC facility backbone, and 48 VDC rack or row buses.
- Fallback generator or rental-generator connection for extended outage.
- Open-source management stack with Rust adapters.
- Deployment substrate selected from `data/software/deployment-stack-profiles.csv`.
- Local fabrication where safe and licensed review where required.

## Planning Assumptions

Each country deployment should start with a profile covering:

- Grid reliability, outage hours, voltage stability, fuel availability, and generator need.
- Climate, design dry-bulb temperature, dust, humidity, water stress, and filtration need.
- Energy tariffs, diesel price, solar capacity factor, carbon factor, and water price.
- Import duty, shipping multiplier, local labour multiplier, lead time, and customs risk.
- Data residency, national key management, offline backup, logs, and audit retention.
- Operator skill level, training path, spares locality, maintainability score, and vendor lock-in score.

## Reliability Position

The correct reliability claim is practical Tier II/Tier III-like topology for sovereign public infrastructure, with documented commissioning, N+1 where it matters, local spares, open telemetry, and disaster recovery.

Do not claim Uptime Tier certification unless the design has gone through the formal Uptime process.
