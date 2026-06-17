# Sovereign Datacentre Mission

Open Source Data Centre exists to help countries build cloud capacity that they can own, understand, repair, expand, and power sustainably.

The project is aimed at governments, universities, public-sector clouds, national AI programmes, hospitals, telecom operators, research networks, and regional infrastructure companies that need local hosting without permanent hyperscaler dependency or foreign vendor lock-in.

## National Implementation Outputs

The repository should produce six practical outputs:

| Output | Purpose |
| --- | --- |
| Reference architectures | 50 kW, 250 kW, 1 MW, and 5 MW sovereign datacentre patterns. |
| BOM and cost calculators | Country-specific CAPEX, OPEX, import duty, local labour, spares, energy cost, and replacement planning. |
| FreeCAD / IFC / STEP designs | Buildable structures, racks, ducts, thermal spines, cable trays, and serviceable parts. |
| Commissioning and reliability tests | Grid-loss tests, generator tests, cooling failover, DC-bus ride-through, backup restore, and production readiness. |
| Operator training pack | Local skills, runbooks, spares lists, maintenance schedules, emergency procedures, and escalation paths. |
| Sovereign cloud software stack | OpenStack, Ceph, Kubernetes, Keycloak, OPA, NetBox, OpenTelemetry, Grafana, and Rust adapters. |

## Positioning

Open Source Data Centre is a sovereign datacentre build kit for countries that need reliable, sustainable, locally maintainable cloud infrastructure without hyperscaler dependency or permanent foreign vendor lock-in.

The project should be judged on whether a local engineering team can operate the facility safely after handover, recover it after common failures, inspect its software and data flows, and expand it without replacing the whole platform.

## Success Criteria

- Sensitive workloads can remain under national or institutional control.
- Operators can explain the facility topology, cloud stack, data flows, and failure modes.
- Every critical component has a spares plan, second-source path, maintenance interval, and post-replacement test.
- The power and cooling design reflects local grid reliability, climate, water stress, fuel supply, and renewable potential.
- Cost calculators show landed cost, operating cost, import exposure, local labour assumptions, and replacement cycles.
- Commissioning records prove the site can survive expected failures before production launch.

## Tone

The repository should not promise a cheap shortcut to a certified hyperscale facility. It should offer a serious path to affordable, maintainable, sovereign public infrastructure.
