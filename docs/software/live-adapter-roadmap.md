# Live Adapter Roadmap

Last reviewed: 2026-06-17.

OSDC should be honest about its current maturity: the portal is a CSV-backed prototype and most adapter contracts are still plan-only. The first concrete exceptions are the PowerDNS, NetBox, Keycloak, OpenBao, and Argo CD read adapters in `osdc-adapters`. PowerDNS can read zones and record sets from the PowerDNS Authoritative HTTP API. NetBox can read sites, racks, devices, and IP addresses from the NetBox REST API. Keycloak can read realms, groups, and realm roles from the Keycloak Admin REST API. OpenBao can read mounted secret engines and ACL policy names from the OpenBao HTTP API. Argo CD can read application sync and health status from the Argo CD API. The next engineering milestone is to turn those narrow proofs into persisted evidence and then repeat the pattern for the mature open-source systems OSDC composes.

The source of truth for the roadmap is `data/software/live-adapter-roadmap.csv`. The proof harness source is `data/software/live-adapter-proof-catalogue.csv`. The infrastructure workbench exposes those rows so operators can see which live integrations are planned, which workflows they unlock, what proof command is expected, which environment variables are needed, where evidence lands, and what write path will be allowed in production.

## Integration Order

1. **PowerDNS**: read zones and records, then plan DNS changes for Edge Shield. The first live-read client is documented in [PowerDNS Read Adapter](powerdns-read-adapter.md).
2. **NetBox**: read sites, racks, devices, and IPAM truth before allowing provisioning. The second live-read client is documented in [NetBox Read Adapter](netbox-read-adapter.md).
3. **Keycloak**: read realms/groups/roles, then create tenant identity structures with approval. The third live-read client is documented in [Keycloak Read Adapter](keycloak-read-adapter.md).
4. **OpenBao**: read mounts and policies, then create tenant secret namespaces and guarded policies. The fourth live-read client is documented in [OpenBao Read Adapter](openbao-read-adapter.md).
5. **GitOps**: read Argo CD sync and health status first, then turn `ChangeRequest` into Forgejo pull requests later. The fifth live-read client is documented in [Argo CD Read Adapter](argocd-read-adapter.md).
6. **Proxmox**: support practical 50 kW edge micro deployments with read-first cluster and VM state.
7. **CloudStack**: support simpler 250 kW regional pilot IaaS where OpenStack is too heavy.
8. **OpenStack**: support 250 kW+ production IaaS through scoped Keystone reads before writes.
9. **PostgreSQL**: persist lifecycle state, approvals, evidence bundles, adapter proof runs, audit events, and request history.

## Rules

- Read-only adapters come before write adapters.
- Write actions must be GitOps-first or guarded by approval, quota, validation, rollback, and audit.
- Every adapter milestone must have a proof command and evidence path.
- The workbench should show adapter status beside workflows so users do not confuse a planned integration with a live one.
- CSV/catalogue state remains acceptable for planning, but production operation needs PostgreSQL-backed state and auditable change records.

## Completion Criteria

An adapter milestone is not complete until it has:

- typed request/response structures;
- endpoint configuration and secret handling;
- fixture tests or mocked client tests;
- a read-only smoke command;
- failure-mode handling;
- evidence written into the assurance bundle;
- portal/API status exposed to the workbench;
- documentation that names what the adapter does not control.

## Local Proof Harness

Before real credentials exist, run plan-mode adapter proofs:

```bash
scripts/adapter-proof.sh --all --mode plan
scripts/adapter-proof.sh --milestone ADAPT_001 --mode plan
```

The script writes evidence under `target/assurance/adapter-proofs/<timestamp>/` and updates `target/assurance/adapter-proofs/latest/`. Plan mode deliberately does not contact external systems; it verifies that each milestone has a command, owner, required environment, evidence output, gate, and next step.

For `ADAPT_009`, plan mode also inspects `crates/osdc-portal/migrations/0001_osdc_portal_state.sql` and records the portal-state tables found in `target/assurance/adapter-proofs/latest/postgresql.md`. That makes the persistence milestone testable before a live PostgreSQL instance exists.

For `ADAPT_001`, the offline unit test exercises a fake PowerDNS server:

```bash
cargo test -p osdc-adapters powerdns_http_adapter_reads_zones_and_records
```

The live read example uses real PowerDNS credentials:

```bash
OSDC_POWERDNS_URL=http://127.0.0.1:8081 \
OSDC_POWERDNS_API_KEY=change-me \
OSDC_POWERDNS_SERVER_ID=localhost \
OSDC_POWERDNS_ZONE=example.gov. \
cargo run -p osdc-adapters --example powerdns-read
```

For `ADAPT_002`, the offline unit test exercises a fake NetBox server with token authentication and pagination:

```bash
cargo test -p osdc-adapters netbox_http_adapter_reads_inventory_snapshot
```

The live read example uses real NetBox credentials:

```bash
OSDC_NETBOX_URL=https://netbox.example.gov \
OSDC_NETBOX_TOKEN=change-me \
cargo run -p osdc-adapters --example netbox-read
```

For `ADAPT_003`, the offline unit test exercises a fake Keycloak Admin REST API server with bearer-token authentication:

```bash
cargo test -p osdc-adapters keycloak_http_adapter_reads_identity_snapshot
```

The live read example uses real Keycloak credentials:

```bash
OSDC_KEYCLOAK_URL=https://keycloak.example.gov \
OSDC_KEYCLOAK_TOKEN=change-me \
OSDC_KEYCLOAK_REALM=osdc \
cargo run -p osdc-adapters --example keycloak-read
```

For `ADAPT_004`, the offline unit test exercises a fake OpenBao HTTP API server with `X-Vault-Token` authentication and ACL policy listing:

```bash
cargo test -p osdc-adapters openbao_http_adapter_reads_secret_snapshot
```

The live read example uses real OpenBao credentials:

```bash
OSDC_OPENBAO_ADDR=https://openbao.example.gov \
OSDC_OPENBAO_TOKEN=change-me \
cargo run -p osdc-adapters --example openbao-read
```

For `ADAPT_005`, the offline unit test exercises a fake Argo CD API server with bearer-token authentication:

```bash
cargo test -p osdc-adapters argocd_http_adapter_reads_sync_snapshot
```

The live read example uses real Argo CD credentials:

```bash
OSDC_ARGOCD_URL=https://argocd.example.gov \
OSDC_ARGOCD_TOKEN=change-me \
cargo run -p osdc-adapters --example argocd-read
```
