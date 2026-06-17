# Portal Persistence

Last reviewed: 2026-06-17.

The OSDC portal needs durable state when it moves from prototype catalogue browsing into real operations. PostgreSQL is the first persistence target because it is open source, widely operated, easy to back up, and suitable for approval, evidence, audit, and request workflows.

The first migration lives at `crates/osdc-portal/migrations/0001_osdc_portal_state.sql`.

## Boundary

PostgreSQL stores OSDC-owned workflow state. It does not become the source of truth for every infrastructure fact.

External systems remain authoritative for their own domains:

- NetBox or openDCIM owns sites, racks, devices, circuits, cable paths, and IPAM.
- PowerDNS owns DNS zones and records.
- Keycloak owns realms, groups, roles, identity federation, and user sessions.
- privacyIDEA and authentik own MFA token state, enrolment flows, and authenticator validation state where deployed.
- OpenBao owns secret mounts, policies, and key material.
- Forgejo, Argo CD, and Flux own GitOps changes and sync state.
- Proxmox, CloudStack, OpenStack, Kubernetes, Ceph, MAAS, Ironic, Metal3, Tinkerbell, Redfish/OpenBMC, and facility gateways own live infrastructure state.
- CloudKitty, OpenMeter, Lago, Kill Bill, OpenCost, and local finance systems own detailed rating, invoice, payment, and cost records.

The portal stores the workflow envelope around those systems: who requested a change, what was approved, what evidence was produced, which proof or test ran, what rollback plan was attached, and what audit events were emitted.

## Initial Tables

| Table | Purpose |
| --- | --- |
| `osdc_portal.change_requests` | GitOps or guarded-API change envelope with target system, environment, risk, validation plan, rollout plan, rollback plan, and status. |
| `osdc_portal.approval_records` | Approval decisions, approvers, evidence references, and notes linked to a change request. |
| `osdc_portal.evidence_bundles` | Test, scan, commissioning, adapter-proof, and deployment evidence attached to workflows. |
| `osdc_portal.audit_events` | Append-style operational events for request creation, validation, approval, rollout, rollback, and closeout. |
| `osdc_portal.infrastructure_requests` | Front-door workbench requests for tenants, VMs, Kubernetes, storage, hardware, edge endpoints, upgrades, scans, and data products. |
| `osdc_portal.adapter_proof_runs` | Local and future live adapter proof executions with target, milestone, mode, status, evidence path, and summary. |
| `osdc_portal.customer_accounts` | Customer account workflow state with residency zone, identity realm, billing account, support tier, owner, status, and payload summary. |
| `osdc_portal.customer_site_instances` | Customer site-instance state for deployment stage, load, substrate, provisioner, residency zone, source of truth, owner, and status. |
| `osdc_portal.identity_mfa_policies` | Portal-visible MFA policy state for open-source 2FA factors, recovery, enforcement point, evidence, owner, and status. |
| `osdc_portal.usage_meter_snapshots` | Customer usage snapshots imported from metering systems for billing period, quantity, source system, evidence, and rating status. |
| `osdc_portal.invoice_previews` | Draft invoice previews with customer, period, plan, amount, credits, tax, approval reference, and release status. |

## Production Rules

- The portal must read from external systems before it writes anything.
- Writes must be GitOps-first or guarded by approval, validation, quota, rollback, and audit.
- Evidence paths should point to immutable artifacts in the assurance bundle, object storage, or a signed repository commit.
- Request status should be updated by workflow execution, not by manual database edits.
- Secret values must never be stored in these tables. Store references to OpenBao paths or encrypted GitOps artifacts instead.
- Backup and restore tests for this database are blocking gates before production use.

## Adapter Proof

The PostgreSQL roadmap proof is `PROOF_POSTGRES_MIGRATION`. In plan mode, `scripts/adapter-proof.sh --milestone ADAPT_009 --mode plan` inspects the migration file and writes a local evidence note under `target/assurance/adapter-proofs/latest/postgresql.md`.
