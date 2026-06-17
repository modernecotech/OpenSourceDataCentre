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

## Source Notes

- NetBox REST API documentation: https://netboxlabs.com/docs/netbox/integrations/rest-api/
- OpenStack Ironic documentation: https://docs.openstack.org/ironic/latest/
- MAAS documentation: https://canonical.com/maas/docs/latest/
- Metal3 documentation: https://metal3.io/documentation.html
- Tinkerbell documentation: https://tinkerbell.org/
- Redfish standard: https://www.dmtf.org/standards/redfish
