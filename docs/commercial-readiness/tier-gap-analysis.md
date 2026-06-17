# Tier Gap Analysis

This is a compact roll-up of the more specific gap files:

- [Uptime/Tier gap analysis](uptime-tier-gap-analysis.md)
- [TIA-942 gap analysis](tia-942-gap-analysis.md)
- [EN 50600 / ISO 22237 gap analysis](en50600-iso22237-gap-analysis.md)
- [Certification boundary](certification-boundary.md)

## Current Position

OSDC can describe Tier-like design intent, but it must not claim certification. The reference design is intended to be reviewable by engineers, operators, insurers, financiers, and authorities, not to replace their work.

| Readiness area | Current status | Next action |
| --- | --- | --- |
| Certification boundary | Template added. | Fill for each project and freeze before procurement. |
| Power maintainability | Conceptual DC-first design. | Complete single-line, protection, maintenance bypass, and MOP evidence. |
| Cooling maintainability | Thermal-spine concept. | Complete P&ID, sequence of operations, leak detection, failure-mode test. |
| Fault tolerance | Not claimed. | Perform project-specific FMEA and third-party review. |
| Operations evidence | Procedure templates exist. | Run drills and capture signed records. |
| Customer SLA claim | SLA classes exist. | Tie SLA to actual measured evidence and exclusions. |

## Decision Rule

The project may move from "reference architecture" to "commercial readiness candidate" only when every critical gap has an owner, evidence file, target date, and sign-off path.
