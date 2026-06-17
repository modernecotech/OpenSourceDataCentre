# Unified Portal Integration Model

The OSDC portal should be a workflow console and policy layer, not a replacement for every upstream dashboard.

```text
             OSDC Unified Portal
        Rust API / policy / audit / cost
                    |
 +------------------+------------------+
 |                  |                  |
Tenant UI       Operator UI        Security UI
 |                  |                  |
VMs/K8s/AI      racks/power/DCIM    SIEM/WAF/IAM
 |                  |                  |
OpenStack       NetBox/openDCIM     Wazuh/Falco/OPA
Kubernetes      Prometheus          Keycloak/OpenBao
Ceph            Grafana             Edge Shield
Harbor          Velero              Coraza/CrowdSec
Argo/Flux       Cilium              Suricata/Zeek
```

## Portal Responsibilities

- Tenancy, roles, quotas, budgets, and approvals.
- Service catalogue workflows.
- Browser-based editing of source-controlled tool configuration scripts.
- Policy checks and audit records.
- Cost, energy, and carbon estimates.
- Upgrade and patch approval workflow.
- Status rollup across mature tools.
- Links or deep integrations into upstream dashboards where needed.

## Portal Non-Responsibilities

- Do not rewrite OpenStack.
- Do not rewrite Kubernetes.
- Do not rewrite Ceph.
- Do not rewrite Keycloak, OpenBao, Grafana, NetBox, Harbor, Argo CD, Flux, Wazuh, or Edge Shield components.

## API Surfaces

Current and planned catalogue APIs:

- `/api/catalog/core-services`
- `/api/catalog/sovereign-services`
- `/api/catalog/upgrade-policy`
- `/api/catalog/blueprints`
- `/api/edge/services`
- `/api/config/scripts`
- `/api/operator/status`

## Config Script Editing

The portal should expose the actual config scripts for tools such as Caddy, PowerDNS, Coraza, CrowdSec, WireGuard, OpenBao, OPA, Kyverno, Prometheus, Argo CD, Flux, and OpenTofu. The browser editor should validate and stage GitOps changes rather than directly rewriting live service files.

This keeps the unified UI realistic: common workflows can use forms, but advanced and complete management remains available through the exact tool configuration that operators already need to understand.

## Workflow Examples

- Create a tenant and assign ministry owners.
- Create a VM or Kubernetes cluster.
- Create a PostgreSQL database and backup policy.
- Issue a public endpoint through Edge Shield.
- Request GPU queue access.
- Approve an IaC plan.
- Patch Keycloak through GitOps.
- Restore a namespace from backup.
- Show externally exposed services.
- Show non-compliant workloads.
