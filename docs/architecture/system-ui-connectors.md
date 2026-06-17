# System UI Connectors

The portal should expose one coherent operator interface without pretending every backend has the same risk profile. Some systems are safe to query directly. Some should be changed only through GitOps. A few need guarded, audited API actions because they touch power, firmware, secrets, identity, or facility controls.

Machine-readable connector data lives in [system-ui-connectors.csv](../../data/software/system-ui-connectors.csv).

## Connector Modes

| Mode | Use when | Examples |
| --- | --- | --- |
| Read mostly | The portal displays state and links to the owning system. | Wazuh alerts, Ceph health, BMS telemetry, Trino query status. |
| GitOps manifest | The portal opens a repository change that another controller reconciles. | Kubernetes workloads, Metal3 BareMetalHost, Flux Kustomization, Argo CD application changes. |
| Config script GitOps | The portal edits a known config file and opens a change request. | Caddyfile, PowerDNS zone snippets, WireGuard edge config. |
| API with approval | The portal calls the backend API only after an owner approval or ticket. | MAAS deployment, Keycloak tenant role changes, OpenStack quota changes. |
| Guarded API action | The portal requires break-glass controls, evidence, and rollback notes. | Redfish power or firmware action, Ceph disruptive maintenance, OpenVAS active scans. |
| API ingest | The portal or CI sends evidence into the tool. | DefectDojo findings, Dependency-Track SBOMs. |
| Local persistence | The portal stores its own workflow state without becoming the source of truth for external systems. | PostgreSQL-backed lifecycle state, approvals, evidence bundles, audit events, and request history. |

## Minimum Connector Contract

Every connector exposed in the UI needs:

- system owner;
- portal surface;
- read endpoint;
- write mode;
- auth model;
- allowed actions;
- dry-run or preview path;
- evidence path;
- rollback or reversal notes;
- test harness.

## Implementation Direction

The first production adapter layer should be small and typed:

```text
portal action
  -> policy check
  -> dry-run preview
  -> change request or guarded API call
  -> evidence bundle
  -> status poll
  -> audit event
```

The portal should prefer GitOps for anything declarative and repeatable. It should reserve direct API calls for discovery, state reads, and actions that cannot reasonably be represented as a manifest.

## Hardware Connector Pattern

Hardware provisioning needs several connectors to cooperate:

1. NetBox reserves the asset, rack, power, IPAM, cable, and role identity.
2. Redfish or OpenBMC validates BMC reachability, power control, boot mode, virtual media, and firmware inventory.
3. MAAS, Ironic, Metal3, or Tinkerbell handles commissioning and OS deployment.
4. Wazuh, OpenBao, Prometheus, and logging systems enroll the host.
5. OpenStack, Kubernetes, Ceph, or the data platform receives the node.
6. The assurance system closes the request only when evidence exists.

## Deployment Substrate Connector Pattern

The portal should expose Proxmox, CloudStack, and OpenStack as deployment substrates, not as competing OSDC replacements.

| Substrate | Best fit | Connector posture |
| --- | --- | --- |
| Proxmox VE | 50 kW edge micro and small institutional deployments. | Guarded API actions for VM/LXC lifecycle, storage, backup, and cluster status with simple operator approval. |
| Apache CloudStack | 50 kW to 250 kW sites that need a simpler IaaS operating model. | API-with-quota-policy for accounts, zones, templates, networks, and VMs. |
| OpenStack | 250 kW+ regional and national sovereign clouds. | API-with-quota-policy plus GitOps/audit for Nova, Neutron, Glance, Cinder, Keystone, and Ironic workflows. |

OSDC should choose the substrate by site scale and operator maturity, then provide the same sovereign workflow surface: tenant request, quota check, evidence, approval, deployment, health, cost, and audit.

## Portal State Connector Pattern

PostgreSQL is the first persistence target for OSDC-owned state. It should store change requests, approvals, evidence bundles, audit events, lifecycle work items, and infrastructure request history. It must not become a shadow source of truth for racks, IPAM, DNS, identity, cloud inventory, or secrets; those remain owned by NetBox, PowerDNS, Keycloak, OpenBao, Proxmox, CloudStack, OpenStack, Kubernetes, Ceph, and the other mature systems.

## Source Notes

- NetBox REST API documentation: https://netboxlabs.com/docs/netbox/integrations/rest-api/
- OpenStack Ironic documentation: https://docs.openstack.org/ironic/latest/
- MAAS documentation: https://canonical.com/maas/docs/latest/
- Metal3 documentation: https://metal3.io/documentation.html
- Tinkerbell documentation: https://tinkerbell.org/
- Redfish standard: https://www.dmtf.org/standards/redfish
