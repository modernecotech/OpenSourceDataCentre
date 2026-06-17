# Hardware Provisioning

OSDC hardware provisioning should feel cloud-like to users while staying honest about physical constraints. A user should request capacity through the portal, but the platform must still prove rack space, power, cooling, network, firmware, security, burn-in, and ownership before a node is handed to OpenStack, Kubernetes, Ceph, Edge Shield, or the data platform.

Machine-readable provisioning data:

- [provisioning-pipeline.csv](../../data/hardware/provisioning-pipeline.csv)
- [provisioning-profiles.csv](../../data/hardware/provisioning-profiles.csv)
- [provisioning-requests.csv](../../data/hardware/provisioning-requests.csv)
- [system-ui-connectors.csv](../../data/software/system-ui-connectors.csv)

## Recommended Provisioning Model

```text
user request
  -> profile and quota preview
  -> NetBox reservation
  -> Redfish or OpenBMC BMC validation
  -> commissioning and burn-in
  -> firmware baseline
  -> network attachment
  -> OS image deployment
  -> security enrolment
  -> OpenStack Kubernetes Ceph Edge or data-platform handoff
  -> assurance closeout
```

## Provisioner Choice

| Target | Preferred tool | Reason |
| --- | --- | --- |
| OpenStack bare metal and VM cloud | Ironic or MAAS | Ironic integrates with OpenStack services; MAAS is simple for data-centre-style bare-metal lifecycle. |
| Kubernetes-native bare metal | Metal3 | Hosts are represented as Kubernetes resources and can be managed through GitOps. |
| Edge nodes and heterogeneous sites | Tinkerbell | Declarative workflows fit small sites, edge devices, and mixed hardware. |
| Simple Ubuntu-oriented regional cloud | MAAS | Strong discovery, commissioning, networking, storage, and image-deployment workflow. |
| Facility or OT gateway | MAAS with manual approval | Keeps OT deployment controlled and evidence-heavy. |

The portal should expose one request form and choose the backend based on the selected profile. Operators can override the backend when the site has a different installed provisioner.

## User-Facing Fields

The request form should ask for:

- profile;
- quantity;
- site;
- target environment;
- network zone;
- rack policy;
- storage class;
- accelerator need;
- maintenance window;
- approval reference;
- owner and cost centre.

The portal should preview:

- expected backend provisioner;
- likely rack and power impact;
- security enrolment requirements;
- approval gate;
- estimated lead time;
- evidence bundle path.

## Operator Controls

Operators need:

- reserve in NetBox;
- validate BMC;
- run commissioning;
- stage firmware baseline;
- deploy OS image;
- enroll security agents and keys;
- hand off to platform pool;
- rollback or quarantine node;
- attach evidence and close request.

## Safety Rules

- Never power-cycle or firmware-update a production host without an approval and rollback note.
- Never scan OT/facility gateways actively outside an approved maintenance window.
- Never hand off a node that has not passed burn-in and security enrolment.
- Never allocate hardware directly from the UI without source-of-truth reservation.
- Every hardware request must leave an evidence bundle and owner trail.

## Source Notes

- OpenStack Ironic provisions bare-metal machines and integrates with Keystone, Nova, Neutron, Glance, and Swift: https://docs.openstack.org/ironic/latest/
- MAAS discovers hardware and supports API-driven deployment workflows: https://canonical.com/maas/docs/latest/
- Metal3 provides bare-metal host management through Kubernetes interfaces: https://metal3.io/documentation.html
- Tinkerbell provides API-centric declarative bare-metal provisioning: https://tinkerbell.org/
- DMTF Redfish is a RESTful standard for scalable platform management: https://www.dmtf.org/standards/redfish
