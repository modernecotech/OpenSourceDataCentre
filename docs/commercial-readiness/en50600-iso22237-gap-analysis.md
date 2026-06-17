# EN 50600 / ISO 22237 Gap Analysis

EN 50600 and ISO/IEC 22237-style reviews are useful because they look beyond a single equipment topology. They force the project to define site, construction, power, cooling, telecommunications, security, operations, and energy-efficiency evidence.

OSDC should use this file as a readiness map only. It must not claim compliance until a project-specific engineer, local authority, and qualified assessor review the actual design.

| Domain | Expected evidence | OSDC response | Next evidence |
| --- | --- | --- | --- |
| Site and building | Site risk assessment, flood/seismic/geotechnical review, authority constraints, expansion plan. | Site-selection scorecard and authority checklist are templates. | [Site-selection scorecard](../site-selection/site-selection-scorecard.md) |
| Power distribution | Utility interface, generator/fallback boundary, DC bus design, earthing, protection, isolation, maintenance switching. | DC-first topology is defined as design intent only. | [Electrical single-line](../engineering/electrical-single-line-250kw.md) |
| Environmental control | Cooling mode, thermal envelope, redundancy, leak detection, water treatment, controls, operating limits. | Thermal spine and cooling options are documented. | [Cooling P&ID](../engineering/cooling-piping-and-instrumentation.md) |
| Telecommunications | Carrier entry, route diversity, meet-me room, cross-connect, demarcation, maintenance windows. | Network-commercial pack exists as a template. | [Meet-me room design](../network-commercial/meet-me-room-design.md) |
| Security systems | Physical zones, visitor controls, CCTV, access review, rack/cage locks, loading dock chain of custody. | Physical-security controls exist as templates. | [Security zones](../security-physical/security-zones.md) |
| Operations | MOP/SOP/EOP, staffing, shift handover, permit-to-work, incident command, drills, maintenance records. | Operations procedure catalogue exists. | [MOP template](../operations/mop-template.md) |
| Energy efficiency | PUE/WUE/CUE boundaries, metering plan, renewable energy, carbon, water strategy, reporting cadence. | Sustainability pack exists as a reporting template. | [Measurement boundary](../sustainability/pue-wue-cue-measurement-boundary.md) |

Minimum acceptance for a funded project:

- Every applicable domain has a named owner.
- Every critical item points to a drawing, calculation, procedure, test record, or authority submission.
- Every non-applicable item has a written rationale.
- Every open gap is tracked in `data/commercial/commercial-gap-register.csv` or the project risk register.
