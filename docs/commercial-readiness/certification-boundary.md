# Certification Boundary

Certification boundaries prevent accidental overclaiming.

OSDC is a reference architecture and planning toolkit. A real project must define what is inside and outside any certification, audit, insurance, contractual, or authority-review boundary.

## Boundary Fields

| Field | Project answer |
| --- | --- |
| Site name and address | To be filled by project. |
| Building boundary | To be filled by project. |
| Critical load covered | IT load, edge/security load, facility controls, or other. |
| Power systems included | Utility, generator, BESS, DC bus, rack bus, converters, PDUs. |
| Cooling systems included | Pumps, dry coolers, chillers, rear-door heat exchangers, liquid loops. |
| Network/interconnection included | Meet-me room, carriers, cross-connects, edge nodes, customer ports. |
| Security systems included | Perimeter, entry, visitor flow, CCTV, access control, cages/racks. |
| Operations included | Staffing, procedures, monitoring, incident response, maintenance. |
| Software platform included | OpenStack, Kubernetes, Ceph, identity, GitOps, data platform, Edge Shield. |
| Exclusions | Anything not operated, controlled, or evidenced by the project. |

## Rules

- Do not use "Tier certified", "TIA certified", "EN compliant", "SOC 2 compliant", or similar language until the relevant third party has signed the claim.
- Keep design intent language separate from certification language.
- Keep tenant responsibility separate from facility responsibility.
- Keep national data-sovereignty controls separate from facility availability controls.
- Keep pilot/lab services separate from production-baseline services.

The boundary should be approved before procurement release and re-approved before handover.
