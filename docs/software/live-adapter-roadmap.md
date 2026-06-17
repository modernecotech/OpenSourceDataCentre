# Live Adapter Roadmap

Last reviewed: 2026-06-17.

OSDC should be honest about its current maturity: the portal is a CSV-backed prototype and the adapter crate is still plan-only. The next engineering milestone is to turn that plan into read-first integrations with the mature open-source systems OSDC composes.

The source of truth for the roadmap is `data/software/live-adapter-roadmap.csv`. The proof harness source is `data/software/live-adapter-proof-catalogue.csv`. The infrastructure workbench exposes those rows so operators can see which live integrations are planned, which workflows they unlock, what proof command is expected, which environment variables are needed, where evidence lands, and what write path will be allowed in production.

## Integration Order

1. **PowerDNS**: read zones and records, then plan DNS changes for Edge Shield.
2. **NetBox**: read sites, racks, devices, circuits, IPAM, and asset truth before allowing provisioning.
3. **Keycloak**: read realms/groups/roles, then create tenant identity structures with approval.
4. **OpenBao**: read mounts and policies, then create tenant secret namespaces and guarded policies.
5. **GitOps**: turn `ChangeRequest` into Forgejo pull requests and track Argo CD or Flux sync health.
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
