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
- Do not rewrite CloudStack, OpenNebula, or Proxmox.
- Do not rewrite Kubernetes.
- Do not rewrite Ceph.
- Do not rewrite NetBox, MAAS, Foreman, Ironic, Metal3, Tinkerbell, SONiC, OpenBMC, Keycloak, OpenBao, Grafana, Harbor, Argo CD, Flux, Wazuh, or Edge Shield components.

## API Surfaces

Current and planned catalogue APIs:

- `/api/catalog/core-services`
- `/api/catalog/sovereign-services`
- `/api/catalog/upgrade-policy`
- `/api/catalog/blueprints`
- `/api/deployment/stack-profiles`
- `/api/edge/services`
- `/api/config/scripts`
- `/api/operator/status`

## Adapter Contracts

The first adapter skeletons live in `crates/osdc-adapters/`.

They define contracts for:

- Keycloak-style identity and tenant setup.
- PowerDNS-style DNS zone creation.
- NetBox-style inventory registration.
- OpenBao-style secret policy application.
- Argo CD or Flux-style GitOps change submission.
- OpenStack-style VM provisioning.
- Kubernetes-style workload submission.
- Ceph-style storage allocation.
- Redfish/OpenBMC and MAAS/Ironic/Metal3/Tinkerbell-style bare-metal lifecycle.
- Harbor-style registry project setup.
- DefectDojo/Dependency-Track/Wazuh-style evidence and finding ingestion.

These traits are deliberately small. They let the portal model real integrations without pretending that a live production adapter exists before credentials, staging clusters, failure modes, and audit requirements are known.

## Deployment Substrate Selection

OSDC should select mature substrates by deployment size:

| Deployment size | Preferred substrate |
| --- | --- |
| 50 kW edge micro | Proxmox VE or CloudStack, Ceph/ZFS, NetBox/openDCIM, PowerDNS, Edge Shield. |
| 250 kW regional pilot | CloudStack or OpenStack, Ceph, Kubernetes, NetBox, MAAS or Ironic/Metal3, Keycloak, OpenBao, Forgejo, Harbor, Argo CD or Flux. |
| 1 MW regional production | OpenStack, Ceph, Kubernetes with Kueue/Slurm, NetBox, Wazuh, Falco, Suricata, Zeek, commercial-readiness and lifecycle evidence registers. |
| 5 MW national AI-ready | OpenStack/Ironic/Metal3, Ceph NVMe tiers, Kubernetes, Slurm/Kueue, OpenBMC/Redfish, SONiC where supportable, OCP/Open19 rack profiles. |

The machine-readable source is `data/software/deployment-stack-profiles.csv`, exposed through `/api/deployment/stack-profiles`.

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
