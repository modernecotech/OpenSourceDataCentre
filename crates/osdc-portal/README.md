# osdc-portal

`osdc-portal` is the Rust-served GUI and JSON API prototype for the Open Source Data Centre control plane. It uses only the Rust standard library plus `serde`/`serde_json` so the first version stays easy to audit and portable.

## Surfaces

- Tenant portal: `/user`
- Operator console: `/operator`
- Edge Shield console: `/edge`
- Cost planner: `/planner`

The tenant GUI supports provisioning preview recalculation, service and resource filters, and visible action feedback. The operator GUI exposes DC power, thermal flow, hardware pools, open cloud implementation status, and the operations queue. The edge GUI exposes Cloudflare-equivalent open-source services, Radxa nodes, rollout state, and generated config previews. The planner GUI exposes scale scenarios, category costs, build times, and marketplace price bases.

## APIs

- `/api/catalog/hardware`
- `/api/catalog/services`
- `/api/catalog/core-services`
- `/api/catalog/blueprints`
- `/api/provisioning/options`
- `/api/provisioning/preview`
- `/api/tenant/summary`
- `/api/operator/status`
- `/api/edge/services`
- `/api/edge/status`
- `/api/edge/config-preview`
- `/api/cost/planning`
- `/api/cost/scenarios`
- `/api/cost/categories`
- `/api/cost/price-basis`
- `/health`

## Run

```bash
cargo run -p osdc-portal -- 127.0.0.1:8787
```

Open `http://127.0.0.1:8787/user`, `http://127.0.0.1:8787/operator`, `http://127.0.0.1:8787/edge`, or `http://127.0.0.1:8787/planner`.

## Test

```bash
cargo test -p osdc-portal
```

The tests exercise page routing, static assets, the AWS/Azure-like service catalog, provisioning contracts, operator power/thermal status, Edge Shield config-preview JSON, and cost-planning APIs.

## Next Adapter Path

Replace the in-process sample data with typed adapters for NetBox/openDCIM, OpenStack, Ceph, Kubernetes/Kueue, OpenBao, PowerDNS, Caddy/Traefik, Coraza, CrowdSec, and facility gateways over Modbus, BACnet, OPC UA, or MQTT.
