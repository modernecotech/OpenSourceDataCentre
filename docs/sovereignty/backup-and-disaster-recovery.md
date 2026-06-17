# Backup and Disaster Recovery

Backups are a sovereignty feature, not only an operations feature. A public institution needs to know it can recover critical services without permission from a foreign platform.

## Service Classes

| Class | Example | Recovery target |
| --- | --- | --- |
| Critical public service | Identity, health, payments, emergency services | Tested restore with low RPO/RTO approved by the owner. |
| Important service | Education, research, public portals | Scheduled restore tests and documented manual fallback. |
| Archive | Records, logs, research data | Integrity, retention, and retrieval tests. |

## Requirements

- Maintain at least one offline or logically isolated backup path for critical data.
- Encrypt backups under locally controlled keys.
- Document restore order for identity, network, storage, control plane, tenant services, and application data.
- Test backup integrity and complete restore procedures.
- Keep printed or offline emergency instructions for identity and backup recovery.

## Evidence

- Backup inventory.
- Retention schedule.
- Restore-test reports.
- Failure notes and corrective actions.
- Owner approval for recovery objectives.
