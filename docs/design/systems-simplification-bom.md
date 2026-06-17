# Datacentre Systems, Simplification, and BOM Strategy

This document identifies the major systems in a state-of-the-art datacentre and reduces them into a practical bill-of-materials strategy for developing-world deployments. The goal is not to build a fragile "cheap datacentre." The goal is a maintainable, vendor-neutral facility where critical systems are simple enough to service locally and advanced enough to support modern compute.

## Design Target

The first reference BOM is a 250 kW IT-load regional pilot:

- 10 racks at 25 kW average, with headroom for selected 40-60 kW liquid-cooled racks.
- Rack thermal spine cooling as the differentiator.
- N+1 critical pumps and controls, but not full Tier IV duplication.
- DC-first solar sodium-ion microgrid instead of a separate UPS layer: PV DC, sodium-ion BESS DC, 380-400 VDC facility backbone, and 48 VDC rack/row power.
- One fallback generator/rental-generator system for extended outage.
- Open-source management stack and vendor-neutral hardware interfaces.
- Local fabrication where safe: trench modules, brackets, cable trays, enclosures, skids, and non-pressure structural parts.

## State-of-the-Art System Map

| System | State-of-the-art datacentre pattern | Developing-world simplification |
| --- | --- | --- |
| Site and civil | Large campus with multiple substations, redundant fibre paths, hardened perimeter, separate utility yards | Start with one hardened building/pod and leave expansion corridors for power, cooling, fibre, and solar. Use standard civil works and drainage that local contractors can maintain. |
| Building shell | Purpose-built hall, high fire rating, blast/security zones, modular electrical/mechanical galleries | Smaller modular hall with fire-rated IT room, electrical room, thermal plant room, and service trench. Avoid over-custom architectural elements. |
| Power utility | Dual utility feeds, MV switchgear, redundant transformers, static transfer, 2N or distributed redundant UPS | One utility feed plus a solar sodium-ion DC microgrid and one fallback generator/rental-generator system. Utility and generator AC enter only through boundary rectifier gear. Meter every major DC and AC-boundary branch. Design second-feed space but do not require it on day one. |
| Backup power | Multiple diesel or gas generators, bulk fuel systems, paralleling switchgear | One generator sized for critical load plus connection point for temporary rental generator. Add fuel polishing only if local fuel quality requires it. |
| UPS / ride-through | Large centralized double-conversion UPS or distributed rack UPS | Delete the standalone UPS layer. Use sodium-ion BESS, MPPT DC/DC, bidirectional battery converters, and DC bus controls as the ride-through system, with validated no-break behavior at the 48 V rack bus. |
| DC distribution | AC busway, intelligent rack PDUs, branch circuit monitoring, high-density 415 V/240 V or HVDC exploration | Use a protected 380-400 VDC backbone for plant and row distribution, then convert to 48 VDC rack/row buses. Keep connectors documented, second-sourceable, and aligned with OCP-style rack power where possible. |
| Rack power | Busway, intelligent rack PDUs, branch circuit monitoring, high-density 415 V/240 V or HVDC exploration | Prefer 48 VDC rack power shelves or metered DC PDUs with A/B feeds. Use AC-output PDUs only as an explicit compatibility exception for legacy equipment. |
| Cooling terminal | Direct-to-chip, immersion, rear-door heat exchangers, high-density CDUs, AI liquid-cooling manifolds | Hybrid rack capture: rear-door HX for ordinary servers, direct-to-chip manifold for dense nodes, and warm-water thermal spine. Avoid proprietary-only couplings. |
| Heat transport | Advanced CDUs, facility water loops, two-phase cooling, heat reuse, digital twins | Warm-water/glycol thermal spine first. Add two-phase thermosyphon row modules only after local serviceability and regulation are proven. |
| Heat rejection | Chillers, cooling towers, dry coolers, water-side economizers, ground storage, waste-heat export | Dry coolers and ground loop first where climate allows. Use water-side economizer and sorption chiller as optional modules. Minimize evaporative water use in water-stressed regions. |
| Heat-to-cooling | Absorption/adsorption chillers, experimental heat-driven cooling, heat pumps | Adsorption/absorption as baseline research/prototype module. Stirling/thermoacoustic modules stay experimental until low-grade rack heat performance is proven. |
| Fire detection | Aspirating smoke detection, multi-zone fire alarm, clean agent, pre-action sprinklers | Aspirating detection in IT/electrical rooms, conventional addressable detection elsewhere. Use locally code-approved clean agent or inert gas only where refill/service supply exists. |
| Fire suppression | Clean agent, inert gas, water mist, pre-action sprinkler, battery fire strategy | Meet local authority requirements first. Prefer compartmentation, early detection, emergency power-off logic, and serviceable suppression agents. |
| Physical security | Security operations centre, mantraps, biometrics, multi-layer CCTV, access analytics | Perimeter lighting/fence, two-door controlled entry, open access-control pilots where safe, local NVR, strong audit logs. Biometrics optional. |
| Network | Spine-leaf fabric, 100/400/800G, SONiC, EVPN/VXLAN, out-of-band network | Two-tier leaf/spine or collapsed core for 250 kW. SONiC on supported switches where skills exist; otherwise open APIs and standard L2/L3. Always include out-of-band management. |
| Compute | OCP/Open Rack Wide AI systems, liquid-cooled GPU racks, modular servers | Mixed 19-inch EIA/Open19-ready racks. Keep adapters for OCP/Open Rack later. Buy commodity servers first unless AI density justifies liquid nodes. |
| Storage | Ceph, NVMe-oF, object storage, erasure coding, backup tiers | Ceph/Rook for shared block/object/file. Local NVMe for scratch and model cache. Offline backup media for sovereignty and disaster recovery. |
| Software | DCIM, telemetry, AI schedulers, policy, IAM, automation, digital twin | NetBox/openDCIM, Prometheus/VictoriaMetrics, Grafana OSS, Keycloak, OPA, Kubernetes/Kueue, Slurm profile where needed, Rust unified API. |
| Commissioning | Factory tests, L1-L5 commissioning, integrated systems tests, load banks | Keep the same discipline at smaller scale: component inspection, pre-functional checks, functional tests, integrated failover/load-bank tests. Do not skip commissioning to save money. |
| Spares and tools | Vendor service contracts, remote operations centre, predictive maintenance | Local spare kits for pumps, sensors, fans, PDUs, filters, valves, NICs, disks, PSUs, fibre, and common fasteners. Train operators and keep printed runbooks. |

## Simplification Rules

1. Simplify topology, not safety.
2. Keep every critical path isolatable and testable.
3. Prefer modular N+1 over full 2N for the first pilot.
4. Use sodium-ion BESS and DC microgrid converters for ride-through rather than a separate UPS stack.
5. Use warm-water cooling before two-phase cooling.
6. Keep proprietary AI rack-scale systems optional, not foundational.
7. Prefer locally sourced structural/mechanical parts, but do not local-fabricate pressure vessels or life-safety equipment without certification.
8. Avoid components that need a single foreign technician to restart the site.
9. Every BOM line should have a second-source plan or a local substitution note.

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

## BOM Files

- [Component catalogue](../../data/bom/component-catalog.csv): full system catalogue mapping state-of-the-art components to simplified choices.
- [250 kW pilot BOM](../../data/bom/bom-250kw-open-regional.csv): starter BOM for a regional pilot facility.

These files are not procurement documents yet. They are planning baselines: quantities, local fabrication candidates, criticality, and pricing placeholders should be updated per country, code, climate, and supply chain.

## Component Criticality

- `critical`: loss can interrupt IT load, safety, or security.
- `important`: loss degrades operations but should not immediately interrupt IT load.
- `optional`: useful for efficiency, growth, or advanced capability.
- `future`: leave space/interface now, buy later.

## BOM Cost Method

For each line:

```text
extended_cost = quantity * unit_cost
landed_cost = extended_cost + shipping + import_duty + installation + commissioning
lifecycle_cost = landed_cost + maintenance + replacements + energy/water impact
```

The first real calculator should separate:

- CAPEX.
- Import/shipping/tax.
- Local labour.
- Commissioning.
- Annual maintenance.
- Replacement interval.
- Energy impact.
- Downtime risk.

## Notes From Current Research

- Uptime Institute tiers define maintainability, power, cooling, and fault-capability levels rather than a universal "best" tier. The pilot should target a practical Tier II/Tier III-like topology without claiming certification unless audited.
- OCP datacentre facility and cooling projects are useful guides because they focus on open, modular, scalable power/cooling/facility operations.
- ASHRAE TC 9.9 remains the main thermal-guideline reference for datacentre environmental envelopes.
- IEA expects datacentre electricity demand to grow strongly through 2030, making power availability and cooling efficiency central design constraints, especially in emerging economies.
- High-density AI racks are driving liquid cooling and higher power densities, but a developing-world open design should make AI density an upgrade path, not a day-one dependency.

## Sources

- Uptime Institute Tier Classification: https://uptimeinstitute.com/tiers
- ASHRAE datacentre resources: https://www.ashrae.org/technical-resources/bookstore/datacom-series
- Open Compute Project Data Center Facility: https://www.opencompute.org/community/data-center-facility
- OCP AI infrastructure standards call: https://www.opencompute.org/about/a-call-for-collaboration-on-ai-data-center-infrastructure-standards
- OCP Rack & Power: https://www.opencompute.org/community/rack-and-power
- OCP Rack & Power specifications: https://www.opencompute.org/wiki/Open_Rack/SpecsAndDesigns
- ETSI EN 300 132-3 400 VDC ICT power interface: https://www.etsi.org/deliver/etsi_en/300100_300199/30013203/02.03.01_60/en_30013203v020301p.pdf
- LBNL/EMerge Alliance 380 VDC datacentre architecture work: https://datacenters.lbl.gov/sites/default/files/380VdcArchitecturesfortheModernDataCenter.pdf
- IEA Energy and AI: https://www.iea.org/reports/energy-and-ai/energy-demand-from-ai
- Data centre electrical power components: https://www.csemag.com/know-the-electrical-power-components-in-a-data-center/
- Open19 Linux Foundation announcement: https://www.linuxfoundation.org/press/press-release/the-linux-foundation-hosts-open19-to-acceleratedata-center-and-edge-hardware-innovation
- ASHRAE commissioning resources: https://www.ashrae.org/technical-resources/bookstore/commissioning
