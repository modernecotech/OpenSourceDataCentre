# Core Cloud Services Baseline

Last reviewed: 2026-06-14.

AWS and Azure expose hundreds of products, but an open datacentre should not copy every product name. The useful move is to implement the smallest service set that lets users build real applications while operators can still run it with local skill and open-source tooling.

## Selected Services

| OSDC service | AWS reference | Azure reference | Open-source implementation |
| --- | --- | --- | --- |
| Identity and projects | IAM, Organizations | Microsoft Entra ID, subscriptions/resource groups | Keycloak, OPA, OpenStack Keystone |
| Virtual machines | EC2 | Azure Virtual Machines | OpenStack Nova, KVM, libvirt |
| Bare metal and GPU nodes | EC2 bare metal, accelerated instances | Azure bare metal/GPU VM families | OpenStack Ironic, Metal3, Redfish, Kubernetes device plugins |
| Object storage | S3 | Blob Storage | Ceph RGW |
| Block volumes | EBS | Managed Disks | Ceph RBD, OpenStack Cinder |
| Shared file storage | EFS/FSx | Azure Files, Managed Lustre | CephFS, OpenStack Manila |
| Private networking | VPC, subnets, security groups | Virtual Network, NSG | OpenStack Neutron, OVN, FRRouting, nftables |
| Load balancing and DNS | ELB, Route 53 | Load Balancer, Application Gateway, Azure DNS | Octavia, HAProxy, Envoy, PowerDNS, Designate |
| Managed Kubernetes | EKS | AKS | Cluster API, Metal3, Cilium, Talos/kubeadm |
| Serverless/app hosting | Lambda, ECS/Fargate, App Runner | Functions, Container Apps, App Service | Knative, Kubernetes, KEDA |
| Managed databases | RDS, DynamoDB, ElastiCache | Azure SQL, Cosmos DB, Cache for Redis | CloudNativePG, MariaDB/Percona operators, Valkey, FerretDB where useful |
| Messaging and events | SQS, SNS, EventBridge, MSK | Service Bus, Event Grid, Event Hubs | NATS, Kafka/Strimzi, RabbitMQ |
| Secrets and keys | Secrets Manager, KMS, ACM | Key Vault | OpenBao, cert-manager, Smallstep, Barbican |
| Observability and audit | CloudWatch, CloudTrail | Azure Monitor, Log Analytics | OpenTelemetry, Prometheus/VictoriaMetrics, Grafana OSS, Loki, Tempo, OpenSearch |
| Backup and disaster recovery | AWS Backup, snapshots, Glacier | Azure Backup, Archive Storage | Velero, Restic, Kopia, Borgmatic, Ceph snapshots, tape/offline media |
| DevOps and infrastructure as code | CodeBuild/CodePipeline, CloudFormation | Azure DevOps, ARM/Bicep | Forgejo, Woodpecker CI, Argo CD/Flux, OpenTofu, Ansible |
| AI and batch | SageMaker, Bedrock, Batch | Azure AI, Azure ML, Batch | Kueue, Slurm, KServe, vLLM, SGLang, llama.cpp, MLflow |
| Cost and sustainability | Cost Explorer, Budgets | Cost Management | OpenCost, CloudKitty, OSDC Rust calculators |

## Implementation Rule

The tenant portal should expose these as simple products, while the operator console should show the underlying open-source backends and health:

- Tenants see shapes, quotas, cost, status, and APIs.
- Operators see service health, capacity, backing systems, maintenance risk, and power/cooling impact.
- Every service must map back to Git/IaC definitions so a deployment can be rebuilt without a proprietary control plane.

## Source Notes

- AWS documents major categories including compute, containers, databases, developer tools, machine learning, networking, security, storage, and management/governance: https://docs.aws.amazon.com/whitepapers/latest/aws-overview/amazon-web-services-cloud-platform.html
- AWS cloud overview highlights compute, file/block/object storage, SQL/NoSQL databases, AI/ML, networking/content delivery, and security/identity/compliance as core cloud service areas: https://aws.amazon.com/what-is-cloud-computing/
- Azure describes itself as offering cloud services including computing, analytics, storage, networking, and AI: https://azure.microsoft.com/en-us/resources/cloud-computing-dictionary/what-is-azure
- Azure products list includes Linux Virtual Machines, Azure Monitor, DevOps, Key Vault, security/network services, storage, Azure Files, and Backup: https://azure.microsoft.com/en-us/products
- Microsoft Learn lists Azure compute types including virtual machines, containers, functions, AI/ML, and IoT/Edge: https://learn.microsoft.com/en-us/training/modules/describe-azure-compute-networking-services/
