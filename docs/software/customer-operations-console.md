# Customer Operations Console

Last reviewed: 2026-06-17.

The customer operations console is the multi-customer control layer for OSDC. It lets a national operator or regional infrastructure company run several customer datacentres from the same sovereign platform without mixing identities, sites, billing accounts, or evidence records.

The console is backed by:

- `data/commercial/customer-accounts.csv`
- `data/commercial/customer-sites.csv`
- `data/software/customer-operations-workflows.csv`
- `data/software/identity-mfa-policies.csv`
- `data/commercial/billing-plans.csv`
- `data/commercial/usage-meters.csv`
- `data/commercial/invoice-preview.csv`

## Operating Model

Each customer gets a bounded account, identity realm or realm segment, residency zone, billing account, support tier, site instances, and evidence trail.

The portal should orchestrate the customer lifecycle, while authoritative state remains in the specialist systems:

| Domain | Open system |
| --- | --- |
| Identity and MFA | Keycloak, privacyIDEA, authentik |
| Source of truth | NetBox |
| Cloud substrate | Proxmox, CloudStack, OpenStack, Kubernetes, Ceph |
| Bare metal | MAAS, Ironic, Metal3, Tinkerbell, Redfish/OpenBMC |
| Metering and rating | CloudKitty, OpenMeter, Lago |
| Billing and invoices | Kill Bill or Lago |
| Portal workflow state | PostgreSQL `osdc_portal` schema |

## Customer Lifecycle

1. Create or import the customer account and support boundary.
2. Create the identity realm, groups, roles, and MFA policy.
3. Create the billing account, plan, meters, tax policy, and invoice approval owner.
4. Attach customer site instances to a residency zone and deployment stack.
5. Reserve hardware, network, address, rack, and project capacity in NetBox and the cloud substrate.
6. Run provisioning tests, security scans, backup checks, and metering validation.
7. Approve the command record and persist the evidence bundle.

## UI Contract

The `/customers` page must not be a read-only dashboard. It generates command records for:

- onboarding a customer;
- enforcing open-source MFA;
- provisioning a customer site instance;
- rating usage;
- previewing an invoice pack;
- exporting the catalogues used by the operator.

Every generated command should include the customer id, site id, workflow id, MFA policy, billing plan, connectors, evidence target, and persistence target.

## Production Adapter Direction

The first production implementation should remain read-first:

- read Keycloak realms, groups, and roles;
- read privacyIDEA/authentik MFA enrolment posture;
- read NetBox tenants, sites, racks, circuits, prefixes, and devices;
- read CloudKitty/OpenMeter usage records;
- read Kill Bill/Lago account and invoice previews;
- write only through GitOps or guarded API operations after validation and approval.

