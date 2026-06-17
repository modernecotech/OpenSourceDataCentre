# Public-Sector Tenant Isolation

Public-sector clouds often mix ministries, hospitals, universities, agencies, and research groups. Isolation must be explicit rather than assumed.

## Isolation Layers

- Identity realms, groups, roles, and administrator scopes.
- Network segmentation, security groups, firewall policy, and DNS boundaries.
- Storage pools, bucket policy, volume encryption, snapshots, and backup access.
- Kubernetes namespaces, network policies, admission policy, quotas, and secrets.
- Logging and audit access by tenant and service owner.

## Operating Rules

- No shared administrator accounts.
- No cross-tenant support access without ticket, approval, and audit record.
- No default public exposure for tenant services.
- Quotas and budgets are visible to tenant owners.
- Exceptions expire and are reviewed.

## Evidence

- Tenant onboarding checklist.
- Access review.
- Network policy export.
- Backup access test.
- Incident drill showing cross-tenant containment.
