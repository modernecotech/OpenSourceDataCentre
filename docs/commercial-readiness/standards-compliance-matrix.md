# Standards Compliance Matrix

This matrix maps candidate standards and control families to OSDC design responses and evidence.

Source data:

- [standards-control-matrix.csv](../../data/commercial/standards-control-matrix.csv)

Template:

| Requirement | Applies? | OSDC design response | Evidence file | Responsible party | Status |
| --- | --- | --- | --- | --- | --- |
| Reliability classification | Yes | Map Tier-like intent without claiming certification. | [Uptime/Tier gap analysis](uptime-tier-gap-analysis.md) | Owner engineer | Open |
| Telecom/site infrastructure | Yes | Track telecom rooms, pathways, power, security, and environmental evidence. | [TIA-942 gap analysis](tia-942-gap-analysis.md) | Owner engineer | Open |
| ISMS controls | Yes | Maintain risk treatment, statement of applicability, access reviews, incident records, and management review. | [ISO27001 outline](../compliance/iso27001-isms-outline.md) | Security owner | Open |
| OT cybersecurity | Yes | Define zones, conduits, remote access, patching, and backup for facility control systems. | [OT security zones](../security/ot-security-zones-and-conduits.md) | OT owner | Open |
| Life safety | Yes | Require local fire engineer and authority review. | [Fire strategy](../engineering/fire-strategy.md) | Life-safety engineer | Open |

Status values:

- `open`: evidence not yet sufficient.
- `in-progress`: owner assigned and artifact under development.
- `review`: evidence ready for qualified review.
- `accepted`: reviewed for a specific project boundary.
- `not-applicable`: written reason recorded.

No matrix row creates certification by itself.
