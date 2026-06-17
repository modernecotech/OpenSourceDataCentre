# Deployment Stack Profiles

Last reviewed: 2026-06-17.

OSDC should choose mature infrastructure substrates by deployment size. A 50 kW edge micro site should not be forced into the same operating model as a 5 MW national AI facility.

Machine-readable profiles live in [deployment-stack-profiles.csv](../../data/software/deployment-stack-profiles.csv). The portal exposes them through `/api/deployment/stack-profiles` and the planner console.

## 50 kW Edge Micro

Recommended pairing:

- Proxmox VE or Apache CloudStack for virtualization.
- Ceph small cluster, ZFS, or Proxmox Backup for storage depending on operator maturity.
- NetBox or openDCIM as the source of truth.
- PowerDNS, Edge Shield, Keycloak, OpenBao, Prometheus, Grafana, and Loki for the first sovereign services.
- Forgejo, Woodpecker, and Flux only where the operator can support GitOps.

Reason: OpenStack may be too heavy for a micro site. The goal is local operation, backup, identity, monitoring, DNS, and a small tenant service set.

## 250 kW Regional Pilot

Recommended pairing:

- Apache CloudStack or OpenStack as the cloud substrate.
- Ceph for block, object, and file storage.
- Kubernetes for platform services, data services, and AI-adjacent workloads.
- MAAS or Ironic/Metal3 for hardware lifecycle.
- NetBox as source of truth.
- Keycloak, OpenBao, OPA, PowerDNS, Edge Shield, Forgejo, Woodpecker, Harbor, Argo CD or Flux, OpenTofu, and Ansible.

Reason: this is the first scale where the full sovereign cloud service catalogue is credible, but the operator may still choose CloudStack when OpenStack operational weight is too high.

## 1 MW Regional Production

Recommended pairing:

- OpenStack as the default IaaS substrate.
- Ceph as the default storage substrate.
- Kubernetes with Kueue and/or Slurm for AI, batch, and platform services.
- MAAS, Ironic, and Metal3 for bare-metal lifecycle.
- NetBox as the source of truth.
- Wazuh, Falco, Suricata, Zeek, OpenBao, Keycloak, OPA, and Edge Shield for security and audit.
- Full commercial-readiness and lifecycle evidence registers.

Reason: this is where OSDC's lifecycle, commercial governance, tenant isolation, and audit fabric become as important as the cloud substrate.

## 5 MW National AI-Ready

Recommended pairing:

- OpenStack, Ironic, and Metal3 as the core cloud and bare-metal substrate.
- Ceph with high-throughput NVMe tiers.
- Kubernetes, Slurm, and Kueue for AI and HPC scheduling.
- SONiC where local skills and hardware support it.
- OpenBMC and Redfish for firmware and BMC operations.
- OCP, Open19, Open Rack V3, or Open Rack Wide profiles where the supply chain supports them.
- Liquid-cooling engineering evidence, AI rack classes, and commissioning evidence before production claims.

Reason: at this size, AI rack power, cooling, storage throughput, GPU network design, security operations, commissioning, and long-term maintainability dominate the risk picture.
