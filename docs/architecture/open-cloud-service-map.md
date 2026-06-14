# Open Cloud Service Map

Last reviewed: 2026-06-14.

The project should expose an AWS-like experience without cloning AWS internally. The right pattern is a unified Rust portal and API that orchestrates mature open-source systems underneath.

## Baseline Stack

| Cloud domain | User-facing service | Open-source implementation baseline | Notes |
| --- | --- | --- | --- |
| Identity and projects | Accounts, projects, roles, API tokens | Keycloak, OpenStack Keystone where OpenStack is deployed, OPA | Keycloak should be the main human identity provider; Keystone can remain the IaaS service identity layer. |
| Virtual machines | Instances, images, keypairs, security groups | OpenStack Nova, Glance, Neutron, KVM, libvirt | Use Apache CloudStack only as a simpler alternative for small sites that cannot operate OpenStack. |
| Bare metal | Dedicated servers, GPU nodes, storage nodes | OpenStack Ironic, Metal3, Redfish, OpenBMC | Keep dedicated GPU nodes explicit; do not hide scarce accelerators behind vague instance names. |
| Block storage | Volumes, snapshots | Ceph RBD, OpenStack Cinder | Default tenant block storage path. |
| Object storage | Buckets, lifecycle, S3-compatible API | Ceph RGW | Avoid proprietary S3 dependencies while keeping common tooling compatibility. |
| File storage | Shared file systems | CephFS, OpenStack Manila | Useful for research groups, model stores, and shared datasets. |
| Networking | Private networks, routers, floating IPs, firewalls | OpenStack Neutron with OVN, FRRouting, nftables | Keep network state auditable and exportable. |
| Load balancing | L4/L7 load balancers | OpenStack Octavia, HAProxy, Envoy | HAProxy remains the simple local option. |
| DNS | Hosted zones and records | OpenStack Designate, PowerDNS | Integrate with tenant projects and audit logs. |
| Kubernetes | Managed clusters, node pools | Cluster API, Metal3, kubeadm or Talos, Cilium | Offer a small number of tested cluster profiles. |
| Containers | Namespaces, deployments, ingress | Kubernetes, Cilium, cert-manager, ingress-nginx or Envoy Gateway | Keep this separate from VM tenancy but use the same identity/policy layer. |
| Serverless | Functions and scale-to-zero services | Knative Serving/Eventing | Good for small public-sector apps and bursty APIs. |
| AI serving | Model endpoints, GPU queues | KServe, vLLM, SGLang, llama.cpp, Kueue | Expose queue position, energy cost, and model license class. |
| Batch and HPC | Jobs, arrays, fair-share queues | Kueue, Slurm, Volcano where needed | Use Slurm for HPC users and Kueue for Kubernetes-native jobs. |
| Databases | PostgreSQL, MySQL/MariaDB, cache | CloudNativePG, Percona Operator, MariaDB Operator, Valkey | Prefer operator-managed databases over a custom database service. |
| Messaging | Queues, streams, pub/sub | NATS, Apache Kafka with Strimzi, Apache Pulsar | Choose one default per site to avoid operational sprawl. |
| Secrets and keys | Secrets, transit encryption, certs | OpenBao, cert-manager, Smallstep, OpenStack Barbican | Avoid proprietary Vault licensing dependency. |
| Container registry | OCI images, SBOMs, signing | Harbor, cosign, Syft, Grype | Require signed base images for platform workloads. |
| Infrastructure as code | Declarative provisioning | OpenTofu, Ansible, Crossplane where useful | OpenTofu is the user-facing IaC path. |
| GitOps and CI/CD | Repos, pipelines, deploys | Forgejo, Woodpecker CI, Argo CD or Flux | Keep platform changes versioned and reviewable. |
| Backup and recovery | VM, volume, Kubernetes, object backup | Velero, Restic, Kopia, Borgmatic, Ceph snapshots | Every service must show restore status in the operator console. |
| Observability | Metrics, logs, traces, alerts | OpenTelemetry, Prometheus, VictoriaMetrics, Grafana OSS, Loki, Tempo | Tenant and operator views use the same telemetry but different permissions. |
| Cost and chargeback | Usage, budgets, energy cost | OpenCost, CloudKitty, OSDC Rust calculators | Include power, cooling, carbon, and GPU queue cost, not just CPU/RAM. |
| DCIM and IPAM | Racks, devices, circuits, IPs | NetBox, openDCIM | NetBox remains the source of truth for network and rack inventory. |

## Portal Split

Build two user interfaces over one Rust API:

- Tenant portal: provision VMs, Kubernetes clusters, object buckets, volumes, AI jobs, model endpoints, databases, DNS, and networks.
- Operator console: manage sites, racks, DC power, cooling, OS images, firmware, networks, quotas, capacity, alerts, and maintenance windows.

The portal should not replace OpenStack Horizon, Grafana, NetBox, or Kubernetes dashboards. It should be the common operational surface for the workflows that cross those tools.

## First Service Catalog

The first deployable catalog should contain:

| Service | Backing system | Initial shapes |
| --- | --- | --- |
| VM instances | OpenStack Nova/KVM | `cpu.tiny`, `cpu.small`, `cpu.standard`, `edge-arm.small` |
| GPU instances | OpenStack Nova PCI passthrough or Kubernetes device plugins | `gpu-open.1x16g`, `gpu-lowpower.1x16g` |
| Kubernetes clusters | Cluster API + Cilium | `k8s.edge`, `k8s.standard`, `k8s.gpu-queue` |
| Object buckets | Ceph RGW | Standard, archive, public-read gated by policy |
| Block volumes | Cinder + Ceph RBD | 50 GB, 250 GB, 1 TB |
| AI queues | Kueue and Slurm | Inference, training, batch, low-priority solar-surplus |
| Databases | CloudNativePG and Valkey | PostgreSQL small/medium, Valkey small |
| Networks | Neutron/OVN | Private network, router, floating IP, firewall group |

## Source Notes

- OpenStack describes itself as open-source cloud infrastructure with services for compute, storage, and networking managed through APIs and dashboards: https://www.openstack.org/
- Apache CloudStack is an open-source IaaS platform with compute orchestration, network-as-a-service, user/account management, API, accounting, and UI: https://cloudstack.apache.org/ and https://www.shapeblue.com/apache-cloudstack-overview/
- Kubernetes is an open-source system for automating deployment, scaling, and management of containerized applications: https://kubernetes.io/
- OpenTofu is an open-source infrastructure-as-code tool under Linux Foundation stewardship: https://opentofu.org/
