# Open-Source Integration Positioning

Last reviewed: 2026-06-17.

OSDC should not present itself as a replacement for OpenStack, CloudStack, OpenNebula, Proxmox, Kubernetes, Ceph, NetBox, MAAS, Foreman, SONiC, OpenBMC, or OCP.

Its strongest position is different:

> OSDC is the sovereign datacentre lifecycle and integration layer that composes mature open-source systems into a country-scale build, operations, commercial-readiness, and governance model.

## Role

OSDC sits above and around specialist systems:

| Layer | Mature systems | OSDC role |
| --- | --- | --- |
| IaaS and virtualization | OpenStack, CloudStack, OpenNebula, Proxmox | Choose the right substrate by site scale, expose simple sovereign workflows, enforce quotas, approvals, evidence, and lifecycle controls. |
| Containers and AI | Kubernetes, Rancher, Kueue, Slurm, KServe | Coordinate tenant workflows, GPU access, policy, cost, energy, queue visibility, and audit. |
| Storage | Ceph, Rook, ZFS, Proxmox Backup | Treat storage as an integrated substrate with capacity, backup, restore, durability, and commercial evidence controls. |
| Source of truth | NetBox, openDCIM | Wrap authoritative rack, device, circuit, tenant, and IPAM data with project delivery, commissioning, and commercial readiness. |
| Bare metal | MAAS, Foreman, Ironic, Metal3, Tinkerbell | Coordinate rack-to-running workflows, BMC safety, firmware gates, OS imaging, security enrolment, and evidence closeout. |
| Network and firmware | SONiC, OpenBMC, Redfish, OCP, Open19 | Define where these standards fit, how they are patched and monitored, and when local skills can support them. |
| Edge and security | PowerDNS, Caddy, Coraza, CrowdSec, Wazuh, Falco, Suricata, Zeek, OpenBao, Keycloak, OPA | Provide a sovereign Edge Shield and security-control plane without claiming global anycast or hyperscaler DDoS equivalence. |

## What Makes OSDC Different

Most open-source datacentre tools start after the site and hardware already exist. OSDC starts earlier and runs longer:

- country and site profile;
- site selection, permits, logistics, grid, fibre, water, road, climate, and authority evidence;
- physical design, power, cooling, fire, life-safety, security, and commissioning evidence;
- sovereign service catalogue and deployment substrate selection;
- commercial readiness, SLA classes, cross-connects, access roles, audit evidence, and customer responsibility boundaries;
- hardware provisioning, developer platform, data platform, assurance, threat-management, upgrades, and operations.

That breadth is the differentiator. The tradeoff is maturity: OSDC is much less production-ready than the mature systems it composes.

## Guardrail

OSDC must not duplicate specialist platforms. It should:

- use OpenStack or CloudStack for IaaS where appropriate;
- use Proxmox or CloudStack for small sites where OpenStack would be too heavy;
- use Kubernetes for container and platform workloads;
- use Ceph or ZFS-backed profiles for storage;
- use NetBox or openDCIM as the source of truth;
- use MAAS, Foreman, Ironic, Metal3, or Tinkerbell for bare-metal lifecycle;
- use Keycloak, OpenBao, OPA, PowerDNS, GitOps, and open scanner/SOC tools as integrated services.

The Rust layer should provide typed adapters, policy checks, approval flows, evidence capture, cost and sustainability calculations, health rollups, browser config editing, GitOps orchestration, and audit records.

## Near-Term Milestone

The next milestone is not a full cloud replacement. It is a software-only control-plane slice for the 250 kW regional pilot:

1. PowerDNS adapter for zone and record workflows.
2. NetBox adapter for read-first site, rack, device, circuit, and IPAM truth.
3. Keycloak adapter for tenant, group, and role setup.
4. OpenBao adapter for secret namespaces and policy setup.
5. GitOps adapter for real pull requests or Argo CD/Flux changes.
6. Proxmox and CloudStack deployment profiles for 50 kW and 250 kW cases.
7. OpenStack profile for the 250 kW+ path.
8. PostgreSQL persistence for lifecycle state beyond embedded CSV catalogues.

The adapter crate currently defines plan-only contracts. Live clients should come after credentials, staging systems, failure modes, approval boundaries, and audit requirements are defined.
