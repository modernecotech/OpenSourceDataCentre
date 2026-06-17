# osdc-portal

`osdc-portal` is the Rust-served GUI and JSON API prototype for the Open Source Data Centre control plane. It uses only the Rust standard library plus `serde`/`serde_json` so the first version stays easy to audit and portable.

## Surfaces

- Infrastructure workbench: `/infrastructure`
- Tenant portal: `/user`
- Operator console: `/operator`
- Edge Shield console: `/edge`
- Cost planner: `/planner`
- Lifecycle console: `/lifecycle`
- Hardware provisioning console: `/hardware`
- Commercial console: `/commercial`
- Customer operations console: `/customers`
- Assurance console: `/assurance`
- Developer console: `/developer`
- Data platform console: `/data-platform`

The GUI supports infrastructure command records, tenant provisioning preview recalculation, customer onboarding/MFA/provisioning/billing previews, service and resource filters, hardware request previews, assurance controls, developer workflows, data-platform catalogues, and visible action feedback. Every page mounts the shared active command queue.

## APIs

- `/api/catalog/hardware`
- `/api/infrastructure/workbench`
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
- `/api/customers/overview`
- `/api/cost/planning`
- `/api/cost/scenarios`
- `/api/cost/categories`
- `/api/cost/price-basis`
- `/health`

## Run

```bash
cargo run -p osdc-portal -- 127.0.0.1:8787
```

Open `http://127.0.0.1:8787/infrastructure` or `http://127.0.0.1:8787/customers` to start from the operator-facing workflows.

## Test

```bash
cargo test -p osdc-portal
```

The tests exercise page routing, static assets, the AWS/Azure-like service catalog, provisioning contracts, customer operations catalogues, operator power/thermal status, Edge Shield config-preview JSON, assurance, developer, data-platform, persistence, and cost-planning APIs.

## Next Adapter Path

Replace the in-process sample data with typed adapters for NetBox/openDCIM, OpenStack, CloudStack, Proxmox, Ceph, Kubernetes/Kueue, OpenBao, PowerDNS, Keycloak, privacyIDEA, CloudKitty, OpenMeter, Kill Bill, Lago, Caddy/Traefik, Coraza, CrowdSec, and facility gateways over Modbus, BACnet, OPC UA, or MQTT.
