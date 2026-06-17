# NetBox Read Adapter

Last reviewed: 2026-06-17.

The NetBox read adapter is the second concrete live integration in `osdc-adapters`. It reads authoritative DCIM and IPAM state from NetBox and converts it into an OSDC inventory snapshot that provisioning, billing, assurance, and drift workflows can consume later.

It is read-only. It does not create sites, racks, devices, prefixes, IP addresses, circuits, tenants, or custom fields. It does not yet persist evidence, write drift reports, or update the portal state database.

## Scope

The adapter currently reads:

- `/api/dcim/sites/`
- `/api/dcim/racks/`
- `/api/dcim/devices/`
- `/api/ipam/ip-addresses/`

It supports token authentication through the `Authorization: Token <token>` header and follows paginated `next` links returned by NetBox list endpoints.

## Run A Live Read

Create a read-only NetBox API token for the operator or service account that owns this smoke test. Then run:

```bash
OSDC_NETBOX_URL=https://netbox.example.gov \
OSDC_NETBOX_TOKEN=change-me \
cargo run -p osdc-adapters --example netbox-read
```

The command prints a JSON `NetBoxInventorySnapshot` with sites, racks, devices, and IP addresses.

## Offline Test

The adapter crate includes a fixture-backed HTTP test that checks token authentication, pagination, nested DCIM fields, and IP assignment parsing:

```bash
cargo test -p osdc-adapters netbox_http_adapter_reads_inventory_snapshot
```

## Production Boundary

Before this becomes a production provisioning dependency, OSDC still needs:

- PostgreSQL persistence for inventory snapshots, adapter runs, and drift reports;
- workbench display of the latest NetBox evidence object;
- scoped filters for site, tenant, region, and role instead of full-estate reads;
- circuit, prefix, VLAN, tenant, platform, manufacturer, and custom-field read models;
- failure-mode tests for expired tokens, permission-denied responses, malformed pages, and large estates;
- approval and GitOps rules before any future write path is added.

## References

- NetBox REST API overview: <https://netboxlabs.com/docs/netbox/integrations/rest-api/>
- NetBox API and integration overview: <https://netboxlabs.com/docs/netbox/features/api-integration/>
