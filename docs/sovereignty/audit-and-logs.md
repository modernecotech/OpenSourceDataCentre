# Audit and Logs

Logs prove who did what, when, from where, and against which system. They are also sensitive data and must follow residency and retention rules.

## Log Classes

- Identity and privileged access logs.
- Cloud API and tenant action logs.
- Kubernetes audit logs.
- Storage, backup, and restore logs.
- Facility events from power, cooling, fire, access, and security systems.
- Change-management and commissioning records.

## Requirements

- Keep logs in open formats with documented retention.
- Protect logs from tenant and administrator tampering.
- Avoid exporting sensitive logs outside approved residency boundaries.
- Alert on privileged actions, failed login spikes, backup failures, key changes, and policy bypass.
- Test retrieval during incident exercises.

## Evidence

- Retention matrix.
- Dashboard and alert list.
- Audit sample.
- Incident exercise log retrieval.
- Access review for log readers and administrators.
