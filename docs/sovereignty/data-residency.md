# Data Residency

Sovereign infrastructure must make the location and control of data explicit.

## Requirements

- Identify which workloads must remain inside the country, region, institution, or security boundary.
- Record storage locations for primary data, replicas, snapshots, logs, metrics, backups, and support bundles.
- Keep tenant metadata, audit logs, and identity records under the same residency rules as application data unless a legal review says otherwise.
- Require written approval before any managed support workflow exports logs, crash dumps, telemetry, configuration, or data samples.
- Prefer open export formats so a public institution can move data without vendor permission.

## Controls

- Tag workloads and storage classes with residency policy.
- Enforce placement through Kubernetes labels, OpenStack host aggregates, Ceph pool policy, and backup target policy.
- Audit administrator access and cross-boundary replication.
- Test restore from local and offline backup targets.

## Evidence

- Data-flow diagram.
- Storage and backup inventory.
- Tenant placement policy.
- Log retention and export policy.
- Restore-test records.
