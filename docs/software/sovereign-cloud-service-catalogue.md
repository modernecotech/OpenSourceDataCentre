# Sovereign Cloud Service Catalogue

Open Source Data Centre should expose a national cloud service catalogue over mature open-source systems. The catalogue is not only compute and storage. It includes identity, policy, secrets, CI/CD, observability, SOC, data, AI, backup, and upgrade workflows.

## Core Principle

Do not build replacements for every system from scratch.

```text
OSDC Unified Portal / Rust API
        |
policy + tenancy + approval + audit + cost + upgrade workflow
        |
open-source tools underneath
```

OpenStack is the default IaaS substrate for compute, storage, and networking. Kubernetes is the common substrate for container workloads, automation, scaling, service discovery, and platform extensibility. The OSDC portal should become the single national cloud console while mature tools do the work underneath.

## Catalogue Pillars

| Pillar | Default open stack | OSDC UI workflows |
| --- | --- | --- |
| Identity and policy | Keycloak, OPA, Kyverno, SPIFFE/SPIRE, OpenBao | Create tenant, assign role, issue token, approve GPU access, rotate secret, view audit trail. |
| Compute and containers | OpenStack Nova/Ironic, KVM, Metal3, Kubernetes, Cluster API, Cilium, Knative | Create VM, create cluster, reserve GPU node, submit job, view queue position and energy class. |
| Storage and backup | Ceph RBD/RGW/CephFS, Velero, Restic, Kopia, Borgmatic, offline media | Create bucket, create volume, snapshot VM, restore namespace, test backup. |
| Networking and edge | Neutron, OVN, FRRouting, HAProxy, Envoy, PowerDNS, OSDC Edge Shield | Create DNS record, public endpoint, WAF mode, cache policy, origin tunnel, load balancer. |
| Developer platform | Forgejo, Woodpecker, Argo CD, Flux, Harbor, OpenTofu, Ansible, Backstage option | Request repo, approve IaC plan, view build logs, promote environment, rollback. |
| Observability and audit | OpenTelemetry, Prometheus, VictoriaMetrics, Grafana, Loki, Tempo, Wazuh | View service health, SLO, alert timeline, audit trail, rack/power correlation. |
| Security operations | Wazuh, Falco, Kyverno, Trivy, Grype, Zeek, Suricata, MISP, OpenCTI | View findings, vulnerable images, runtime alerts, policy violations, incidents. |
| Data and AI | CloudNativePG, Valkey, NATS, Kafka, Trino, Iceberg, ClickHouse, Superset, MLflow, KServe, vLLM | Create database, create queue, view lag, deploy model, approve model licence. |
| Asset and operations | NetBox, openDCIM, GLPI, Zammad, BookStack, MeshCentral, Ansible/AWX | View asset, rack, firmware, spares, warranty, maintenance window, runbook. |

## Service Bundles

| Bundle | Purpose | First services |
| --- | --- | --- |
| A - Sovereign Cloud Core | Minimum serious national cloud | OpenStack or CloudStack, Kubernetes, Ceph, Cilium, Keycloak, OpenBao, OPA/Kyverno, Harbor, OpenTofu, Argo CD or Flux, Prometheus/VictoriaMetrics, Grafana, Loki/Tempo, Velero, NetBox. |
| B - Sovereign Edge and Security | Cloudflare/Defender/Sentinel-like edge and SOC layer | PowerDNS, dnsdist, Caddy/Envoy/HAProxy, Varnish, Coraza, CrowdSec, WireGuard/NetBird/OpenZiti, Wazuh, Falco, Suricata, Zeek, MISP/OpenCTI. |
| C - Developer Platform | Azure DevOps/GitHub Enterprise/Cloud Build equivalent | Forgejo or Gitea, Woodpecker or Tekton, Harbor, Backstage option, Argo CD/Flux, Renovate, OpenTofu, Ansible/AWX, Trivy, Grype, Syft, cosign. |
| D - Data and AI Platform | Public-sector data and AI cloud | CloudNativePG, Valkey, NATS, Kafka, Trino, Iceberg, ClickHouse, Superset, Airflow/Argo Workflows, MLflow, KServe, vLLM/SGLang, JupyterHub, Qdrant/Milvus/pgvector. |

## Data Files

- `data/software/service-catalogue-v1.csv` is the main source-of-truth catalogue. It carries service ID, proprietary equivalent, open equivalent, category, bundle, priority, UI surface, upgrade method, controls, workflow, and maturity.
- `data/software/proprietary-open-source-equivalents.csv` is the user-facing commercial-service comparison view.
- `data/software/proprietary-to-open-source-map.csv` is the older security/edge-specific comparison view and should eventually become a generated view.
- `data/software/security-controls.csv` is the compliance/control catalogue for managed security controls and evidence.
- `data/software/security-control-map.csv` is the older control-map view and should remain aligned until it is retired or generated.
- `data/software/upgrade-policy.csv` defines patch and upgrade classes.
- `data/software/config-script-catalogue.csv` defines browser-editable configuration artifacts.

Maturity values in the service catalogue are:

- `experimental`
- `pilot`
- `production-baseline`
- `optional`
- `deprecated`

## Examples

Scale-specific service selections live in `examples/service-catalogue/`.

The UI should expose workflows rather than every low-level setting:

- create a government tenant;
- issue a secure application endpoint;
- create PostgreSQL database;
- deploy model endpoint;
- approve GPU queue request;
- patch Keycloak;
- restore last night's backup;
- show externally exposed services;
- show non-compliant workloads;
- show systems behind on security patches.

## Source Notes

- OpenStack software overview: https://www.openstack.org/software/
- Kubernetes overview: https://kubernetes.io/docs/concepts/overview/
- Keycloak: https://www.keycloak.org/
- OpenBao: https://openbao.org/docs/what-is-openbao/
- Open Policy Agent: https://www.openpolicyagent.org/docs/latest/
