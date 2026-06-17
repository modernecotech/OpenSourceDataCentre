# Commercial Readiness Gap Register

The gap register is the commercial-readiness control surface for the repository.

Source data:

- [commercial-gap-register.csv](../../data/commercial/commercial-gap-register.csv)

Each row should answer:

| Field | Meaning |
| --- | --- |
| `gap_id` | Stable identifier for review and issue tracking. |
| `domain` | Certification, electrical, cooling, interconnection, operations, compliance, commercial, or adjacent domain. |
| `commercial_expectation` | What a serious colocation, bank, insurer, public-sector buyer, or engineering reviewer expects. |
| `current_repo_state` | Honest statement of the repo's current maturity. |
| `priority` | `critical`, `high`, `medium`, or `low`. |
| `next_artifact` | The next file that should close or narrow the gap. |
| `status` | `open`, `in-progress`, `blocked`, `review`, or `closed`. |

Rules:

- Do not close a gap because a concept is documented.
- Close a gap only when there is reviewable evidence, an owner, and a test or acceptance path.
- Certification-related gaps require external professional review before any public compliance claim.
