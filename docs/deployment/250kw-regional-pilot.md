# 250 kW Regional Pilot

The 250 kW regional pilot is the flagship developing-world reference design. It is the first serious public-sector cloud pattern, large enough to validate the full stack while still small enough to commission carefully.

## Design Target

- 10 racks at 25 kW average IT load.
- Headroom for selected 40-60 kW liquid-cooled racks.
- N+1 critical pumps and controls, without full Tier IV duplication.
- DC-first solar sodium-ion microgrid with 380-400 VDC facility backbone and 48 VDC rack or row buses.
- Boundary rectifiers for grid and fallback generator input.
- Rack thermal spine cooling with measured heat capture and final heat rejection.
- Open-source management stack with Rust adapters and operator portal.
- Local fabrication for safe structural and mechanical items.

## What It Proves

- Whether local contractors can build the shell, service trench, cable paths, racks, and utility areas to the required standard.
- Whether local operators can run OpenStack, Ceph, Kubernetes, Keycloak, OPA, NetBox, telemetry, and backup workflows.
- Whether the DC microgrid can ride through grid faults and transfer to fallback power.
- Whether cooling failover and pump/control redundancy work under load.
- Whether BOM landed cost, lead time, spares, and second-source assumptions are realistic.

## Commissioning Must Include

- L1 component inspection.
- L2 pre-functional checks.
- L3 functional tests.
- L4 integrated systems test with load bank or staged IT load.
- L5 production-readiness review.
- Grid-loss, DC-bus ride-through, cooling-failover, generator-start, and backup-restore tests.
