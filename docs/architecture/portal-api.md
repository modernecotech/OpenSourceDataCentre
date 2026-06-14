# Portal API Prototype

Last reviewed: 2026-06-14.

`osdc-portal` is the first Rust-served prototype for the two OSDC GUI surfaces:

- Tenant portal: `/user`
- Operator console: `/operator`

It deliberately starts as a small standard-library HTTP server. That keeps the first interface easy to inspect, portable, and dependency-light. The production version can later move to `axum`, OpenAPI, auth middleware, PostgreSQL, and adapters for OpenStack, NetBox, Ceph, Kubernetes, Kueue, OpenBao, and the facility gateways.

## Current Routes

| Route | Type | Purpose |
| --- | --- | --- |
| `/user` | HTML | Tenant provisioning and resource-management GUI. |
| `/operator` | HTML | Datacentre operator GUI for power, cooling, hardware, cloud stack health, and operations. |
| `/api/catalog/hardware` | JSON | Chosen SBC/GPU hardware baseline. |
| `/api/catalog/services` | JSON | Open cloud service map. |
| `/api/provisioning/options` | JSON | Tenant form options for services, shapes, Linux images, storage, and networks. |
| `/api/tenant/summary` | JSON | Tenant metrics, service tiles, site-flow status, and resources. |
| `/api/operator/status` | JSON | DC power/cooling metrics, hardware pools, open cloud stack health, and operations queue. |
| `/health` | text | Readiness check. |

## Data Direction

The next production step is to replace the in-process sample data with adapters:

- Hardware catalog from `data/hardware/compute-baseline-2026.csv`.
- Open cloud service catalog from `data/software/open-cloud-service-map.csv`.
- Inventory and rack truth from NetBox/openDCIM.
- VM and bare-metal state from OpenStack Nova/Ironic.
- Storage state from Ceph.
- Kubernetes and queue state from Kubernetes, Kueue, and Slurm.
- Facility status from Modbus/BACnet/OPC UA gateways.
- Cost and carbon estimates from the Rust calculator crate.

## Run

```bash
cargo run -p osdc-portal -- 127.0.0.1:8787
```

Open:

- `http://127.0.0.1:8787/user`
- `http://127.0.0.1:8787/operator`
