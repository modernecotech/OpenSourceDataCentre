# Backup-Restore Test

## Purpose

Prove that critical services and tenant data can be restored under local control.

## Scope

At minimum, test restore for:

- Identity and access configuration.
- Portal/API configuration.
- Kubernetes or OpenStack control data.
- Ceph object, block, or file data sample.
- Tenant workload sample.
- Logs or audit sample.

## Procedure

1. Select backup set and restore target.
2. Verify keys, credentials, and offline recovery instructions are available.
3. Restore into the approved test environment.
4. Confirm service starts and data integrity checks pass.
5. Record elapsed time, missing dependencies, operator steps, and defects.

## Pass Criteria

- Restore completes without unapproved external dependency.
- Data integrity checks pass.
- Recovery time and recovery point are recorded.
- Owner accepts or records corrective action.
