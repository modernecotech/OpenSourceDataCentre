# Key Management

Keys are part of sovereignty. A country or institution does not control its infrastructure if encryption keys, signing keys, recovery keys, and administrator credentials are controlled elsewhere.

## Principles

- Root keys remain under national or institutional control.
- Key custodians, recovery procedures, and emergency access are documented.
- Administrative access requires named accounts, strong authentication, and audit logs.
- Secrets are rotated on a schedule and after staff, vendor, or incident changes.
- Offline recovery material is sealed, inventoried, and periodically tested.

## Open Stack Baseline

- Keycloak for identity and federation.
- OpenBao or another auditable secrets manager for application and platform secrets.
- OPA for policy checks around privileged operations.
- Hardware security modules where required by law or risk model.

## Commissioning Evidence

- Key ceremony or initial key-generation record.
- Recovery test.
- Privileged-access review.
- Break-glass account test.
- Backup encryption and restore verification.
