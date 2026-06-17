# OpenBao Read Adapter

Last reviewed: 2026-06-17.

The OpenBao read adapter is the fourth concrete live integration in `osdc-adapters`. It reads secret-engine mounts and ACL policy names from the OpenBao HTTP API and converts them into an OSDC secret snapshot for tenant provisioning, secrets governance, approval, and audit workflows.

It is read-only. It does not enable mounts, write policies, create namespaces, create tokens, rotate secrets, create transit keys, or read secret values.

## Scope

The adapter currently reads:

- `GET /v1/sys/mounts`
- `LIST /v1/sys/policies/acl`

It uses `X-Vault-Token: <token>` and expects the token to have only the read/list capabilities needed for these system paths.

## Run A Live Read

Create or obtain a scoped OpenBao token with read/list access to the mounted secrets engines and ACL policy list. Then run:

```bash
OSDC_OPENBAO_ADDR=https://openbao.example.gov \
OSDC_OPENBAO_TOKEN=change-me \
cargo run -p osdc-adapters --example openbao-read
```

The command prints a JSON `OpenBaoSecretSnapshot` with mount summaries and policy names.

## Offline Test

The adapter crate includes a fixture-backed HTTP test that checks `X-Vault-Token`, mounted secrets-engine reads, and ACL policy listing with the `LIST` verb:

```bash
cargo test -p osdc-adapters openbao_http_adapter_reads_secret_snapshot
```

## Production Boundary

Before this becomes a production secrets-governance dependency, OSDC still needs:

- token acquisition and rotation through a bootstrap identity flow instead of manually supplied tokens;
- PostgreSQL persistence for mount snapshots, policy snapshots, adapter runs, and drift reports;
- namespace and tenant scoping where the deployed OpenBao version and policy model support it;
- policy-detail, auth-method, transit-key, audit-device, and seal-status read models;
- failure-mode tests for sealed clusters, expired tokens, insufficient policy, and malformed mount payloads;
- approval and GitOps rules before any future write path is added.

## References

- OpenBao HTTP API overview: <https://openbao.org/api-docs/>
- OpenBao mounted secrets engines API: <https://openbao.org/api-docs/system/mounts/>
- OpenBao ACL policies API: <https://openbao.org/api-docs/system/policies/>
