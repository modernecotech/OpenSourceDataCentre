# Keycloak Read Adapter

Last reviewed: 2026-06-17.

The Keycloak read adapter is the third concrete live integration in `osdc-adapters`. It reads identity control-plane state from the Keycloak Admin REST API and converts it into an OSDC identity snapshot for tenant provisioning, MFA, approval, and audit workflows.

It is read-only. It does not create realms, groups, roles, clients, users, mappers, identity providers, or role mappings. Tenant creation and role assignment remain future guarded write paths that need approval, rollback, audit, and GitOps evidence.

## Scope

The adapter currently reads:

- `/admin/realms`
- `/admin/realms/{realm}/groups`
- `/admin/realms/{realm}/roles`

It uses `Authorization: Bearer <token>` and expects the token to have read access to the target realm.

## Run A Live Read

Create or obtain a short-lived admin API token with the minimum read permissions needed for realm, group, and role discovery. Then run:

```bash
OSDC_KEYCLOAK_URL=https://keycloak.example.gov \
OSDC_KEYCLOAK_TOKEN=change-me \
OSDC_KEYCLOAK_REALM=osdc \
cargo run -p osdc-adapters --example keycloak-read
```

The command prints a JSON `KeycloakIdentitySnapshot` with realms, groups, and realm roles.

## Offline Test

The adapter crate includes a fixture-backed HTTP test that checks bearer-token authentication and realm-scoped group and role reads:

```bash
cargo test -p osdc-adapters keycloak_http_adapter_reads_identity_snapshot
```

## Production Boundary

Before this becomes a production tenant-provisioning dependency, OSDC still needs:

- token acquisition and rotation through Keycloak or a secret manager instead of manually supplied bearer tokens;
- PostgreSQL persistence for identity snapshots, adapter runs, and drift reports;
- realm and tenant filters so operators do not read more identity data than needed;
- user, client, client-role, identity-provider, MFA-required-action, and role-mapping read models;
- failure-mode tests for expired tokens, insufficient permissions, disabled realms, and malformed role payloads;
- approval and GitOps rules before any future write path is added.

## References

- Keycloak Admin REST API: <https://www.keycloak.org/docs-api/latest/rest-api/index.html>
- Keycloak server administration guide: <https://www.keycloak.org/docs/latest/server_admin/index.html>
