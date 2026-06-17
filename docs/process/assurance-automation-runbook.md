# Assurance Automation Runbook

This runbook turns the assurance catalogues into repeatable operator commands. It is deliberately local-first: a laptop or CI worker can produce an evidence bundle before the full DefectDojo, Dependency-Track, Wazuh, Harbor, Argo CD, or Flux integrations exist.

## Commands

| Command | Purpose |
| --- | --- |
| `scripts/verify.sh` | Formatting, Rust tests, CSV checks, JSON checks, Markdown link checks, and portal route documentation checks. |
| `scripts/verify.sh --security` | Adds `cargo audit` and Syft SBOM generation when those tools are installed. |
| `scripts/verify.sh --live` | Smoke-tests a running portal and edge service. |
| `scripts/assurance-run.sh --ring RING_DEV` | Runs the broad local assurance gate set and writes an evidence bundle under `target/assurance/`. |
| `scripts/assurance-run.sh --ring RING_STAGING --strict-security` | Treats installed scanner failures as blocking release failures. |
| `scripts/upgrade-with-assurance.sh --ring RING_STAGING --service osdc-platform --change-ref PR-123` | Creates an upgrade plan, runs assurance, and writes a promotion decision bundle. |

## Evidence Layout

The default evidence directory is `target/assurance/<timestamp>/`.

Each assurance run writes:

- `README.md`: human-readable result summary;
- `evidence-manifest.tsv`: machine-readable step manifest;
- `commands.log`: executed command log;
- `git-status.txt`: preflight repository status;
- build and metadata outputs such as `cargo-test.txt` and `repository-verify.txt`;
- scanner outputs when installed, for example `trivy-fs.sarif`, `grype.json`, `osv-scanner.json`, `gitleaks.json`, `semgrep.json`, and SBOM files.

Upgrade dry runs write under `target/assurance/upgrades/<timestamp>/`:

- `upgrade-plan.md`;
- `ring-metadata.tsv`;
- `preflight-git-status.txt`;
- `assurance-run/`;
- `promotion-decision.md`.

## Gate Policy

Hard gates:

- `cargo fmt --check`;
- `cargo test`;
- `scripts/verify.sh`.

Optional local scanners:

- `cargo audit`;
- Syft;
- Trivy;
- Grype;
- OSV-Scanner;
- gitleaks;
- Semgrep;
- Checkov;
- Kubescape.

Optional scanner failures are warnings by default so new contributors can run the repo without installing a security lab. Use `--strict-security` in release gates and protected branches.

## Promotion Workflow

1. Renovate, a developer PR, or an operator change request opens a change.
2. CI runs `scripts/assurance-run.sh --ring RING_DEV`.
3. Staging promotion runs `scripts/upgrade-with-assurance.sh --ring RING_STAGING --service <service> --change-ref <ref> --strict-security`.
4. The evidence bundle is attached to the GitOps PR.
5. DefectDojo and Dependency-Track ingest scanner and SBOM outputs.
6. Wazuh/OpenSearch receives endpoint, runtime, and SOC events.
7. The portal shows status, blockers, waivers, and promotion decision.

## Production Rule

Production promotion needs a passing assurance bundle, rollback evidence, owner approval, and a record of every open critical/high finding. Waivers need an owner, expiry, compensating control, and approval evidence.
