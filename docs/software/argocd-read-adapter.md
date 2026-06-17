# Argo CD Read Adapter

Last reviewed: 2026-06-17.

The Argo CD read adapter is the fifth concrete live integration in `osdc-adapters`. It reads application sync and health status from the Argo CD API and converts it into an OSDC GitOps sync snapshot for deployment assurance, upgrade gates, tenant operations, and rollback planning.

It is read-only. It does not create applications, sync applications, change projects, write repositories, open Forgejo pull requests, or mutate Kubernetes resources.

## Scope

The adapter currently reads:

- `GET /api/v1/applications`

It uses `Authorization: Bearer <token>` and expects the token to have read access to the applications that OSDC operators should see.

## Run A Live Read

Create or obtain an Argo CD API token with application read permission. Then run:

```bash
OSDC_ARGOCD_URL=https://argocd.example.gov \
OSDC_ARGOCD_TOKEN=change-me \
cargo run -p osdc-adapters --example argocd-read
```

The command prints a JSON `GitOpsSyncSnapshot` with application names, namespaces, sync statuses, health statuses, and revisions.

## Offline Test

The adapter crate includes a fixture-backed HTTP test that checks bearer-token authentication and application sync/health parsing:

```bash
cargo test -p osdc-adapters argocd_http_adapter_reads_sync_snapshot
```

## Production Boundary

Before this becomes a production deployment-control dependency, OSDC still needs:

- Forgejo/Gitea pull-request creation for reviewed GitOps changes;
- Argo CD project, cluster, repo, operation-state, and resource-diff read models;
- filtered application reads by customer, project, environment, and namespace;
- PostgreSQL persistence for sync snapshots, adapter runs, drift reports, and upgrade-gate evidence;
- failure-mode tests for expired tokens, RBAC-denied applications, degraded apps, and malformed status payloads;
- approval and rollback gates before any future sync or write path is added.

## References

- Argo CD API docs: <https://argo-cd.readthedocs.io/en/latest/developer-guide/api-docs/>
- Argo CD resource health docs: <https://argo-cd.readthedocs.io/en/latest/operator-manual/health/>
