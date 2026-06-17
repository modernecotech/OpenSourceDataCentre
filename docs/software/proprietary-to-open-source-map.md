# Proprietary to Open-Source Map

The service catalogue should make it obvious which proprietary cloud class is being replaced, which open tools are responsible, and which OSDC UI surface owns the workflow.

The canonical machine-readable map is `data/software/proprietary-open-source-equivalents.csv`.

## Mapping Rules

- Map service classes, not vendor branding.
- Prefer mature open-source systems over custom OSDC implementations.
- Make the operator source of truth explicit.
- Keep security controls and upgrade method visible.
- Mark optional systems as optional so the platform does not become unmaintainable on day one.

## Core Mappings

| Proprietary class | OSDC open equivalent |
| --- | --- |
| IAM / Entra ID / Google IAM | Keycloak + OPA + OSDC tenancy model |
| Secrets Manager / Key Vault / KMS | OpenBao + Barbican + HSM where available |
| EC2 / Azure VM / Compute Engine | OpenStack Nova/KVM or CloudStack |
| EKS / AKS / GKE | Kubernetes + Cluster API + Cilium |
| S3 / Blob Storage / Cloud Storage | Ceph RGW |
| EBS / Managed Disk | Ceph RBD + Cinder |
| EFS / Azure Files | CephFS + Manila |
| Route 53 / Azure DNS | PowerDNS + dnsdist |
| Cloud CDN / Front Door / Cloudflare | OSDC Edge Shield |
| GitHub Enterprise / Azure DevOps | Forgejo/Gitea + Woodpecker/Tekton + Argo CD/Flux |
| Terraform Cloud | OpenTofu + GitOps runner + Ceph/S3 state backend |
| Artifact Registry / ECR / ACR | Harbor + cosign + Syft + Grype |
| CloudWatch / Azure Monitor | OpenTelemetry + Prometheus/VictoriaMetrics + Grafana + Loki/Tempo |
| Security Hub / Defender / Sentinel | Wazuh + Falco + Kyverno + OpenSearch + Zeek + Suricata |
| RDS / Cloud SQL | CloudNativePG + Percona/MariaDB operators |
| SageMaker / Azure ML / Vertex AI | KServe + MLflow + Ray/Kubeflow + vLLM/SGLang |

## Boundary

OSDC should not claim feature parity with all commercial products. It should claim local control of the critical service categories, with transparent tooling, auditable policy, and an upgrade workflow.
