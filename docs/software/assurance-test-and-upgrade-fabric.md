# Assurance Test and Upgrade Fabric

OSDC needs one assurance fabric for software, security, data, facility controls, and tenant-facing services. The goal is not only to run tests, but to make every release answer:

- what changed;
- what was tested;
- what was scanned;
- what failed or was waived;
- how rollback works;
- which evidence was attached.

## Operating Model

```text
Forgejo change or Renovate update
  -> build and unit tests
  -> SBOM and vulnerability scans
  -> IaC and policy checks
  -> staging GitOps sync
  -> smoke and synthetic tests
  -> backup and rollback test
  -> canary or site ring
  -> production rollout
  -> audit bundle and finding sync
```

The portal should be the control surface for this flow. It should not directly mutate systems. It should open GitOps changes, show evidence, block unsafe promotion, and link findings to DefectDojo, Dependency-Track, Wazuh, Harbor, NetBox, OpenSearch, and issue trackers.

## Test Coverage Catalogue

Machine-readable test coverage lives in:

- [test-harness-catalogue.csv](../../data/software/test-harness-catalogue.csv)
- [assurance-automation-jobs.csv](../../data/software/assurance-automation-jobs.csv)
- [upgrade-rings.csv](../../data/software/upgrade-rings.csv)
- [upgrade-test-gates.csv](../../data/software/upgrade-test-gates.csv)

The catalogue covers:

- Rust unit and contract tests;
- repository metadata and CSV checks;
- portal API route checks;
- config-script validation;
- GitOps and IaC policy tests;
- SBOM and dependency scanning;
- Kubernetes posture scanning;
- endpoint compliance scanning;
- network vulnerability scanning;
- runtime detection;
- data-platform workflow checks;
- facility commissioning evidence.

## Upgrade Rings

Every upgrade should move through rings, not jump directly to production:

| Ring | Purpose |
| --- | --- |
| Development | Unit tests, static analysis, SBOM, and dependency checks. |
| Staging | Full GitOps sync, integration tests, smoke tests, and backup checks. |
| Canary | Limited users, synthetic checks, runtime monitoring, and error-budget guardrails. |
| Production batch | Maintenance-window rollout with rollback window and audit evidence. |
| Facility control | Lab or offline test before touching BMS, EPMS, BESS, generator, cooling, or access control systems. |
| Edge Shield | Staged edge-node rollout for DNS, WAF, tunnel, rate-limit, and certificate changes. |
| Data platform | Schema, lineage, data-quality, access-policy, and rollback checks before publishing governed data products. |

## Required Blocking Gates

- Discovery gate: Renovate or owner change request, release notes, and impact analysis.
- Build gate: build, unit tests, image build, SBOM, and signature.
- Scan gate: vulnerability, licence, secret, IaC, container, dependency, and OSV checks.
- Posture gate: Kubernetes, host, compliance, and network posture checks.
- Staging gate: GitOps sync, smoke test, synthetic test, and metrics.
- State gate: backup validation, restore test, schema dry-run, and migration plan.
- Rollback gate: rollback plan and at least one tested recovery path.
- Approval gate: owner approval, tenant notice where needed, and maintenance window.
- Audit gate: evidence bundle, finding links, and change record.

## Tooling Baseline

| Need | Default open-source tooling |
| --- | --- |
| Dependency updates | Renovate |
| Build and CI | Woodpecker CI or Tekton |
| GitOps rollout | Argo CD or Flux |
| SBOM | Syft and CycloneDX/SPDX output |
| Image and filesystem scanning | Trivy and Grype |
| Dependency vulnerability scanning | OSV-Scanner, cargo-audit, Dependency-Track |
| IaC and policy checks | Checkov, KICS, Conftest, OPA, Kyverno |
| Kubernetes posture | Kubescape, kube-bench, Trivy Kubernetes |
| Endpoint compliance | Wazuh, OpenSCAP, osquery |
| Runtime detection | Falco, Wazuh, Suricata, Zeek |
| Finding management | DefectDojo, Dependency-Track, Forgejo/Zammad |

## Local Automation

The runnable operator entry point is [Assurance Automation Runbook](../process/assurance-automation-runbook.md).

Initial local commands:

```bash
scripts/assurance-run.sh --ring RING_DEV
scripts/upgrade-with-assurance.sh --ring RING_STAGING --service osdc-platform --change-ref PR-123
scripts/assurance-run.sh --ring RING_STAGING --strict-security
```

The scripts write evidence bundles under `target/assurance/`, which is intentionally ignored by git. CI workers should publish those bundles to the artifact store, DefectDojo, Dependency-Track, Wazuh/OpenSearch, or the final OSDC evidence service.

## Evidence Bundle

Every promotion should attach:

- Git commit and pull request;
- SBOM;
- scan reports;
- policy results;
- staging deployment result;
- smoke-test result;
- backup/restore result for stateful services;
- rollback result or exception;
- owner approval;
- tenant notice where applicable;
- audit event ID;
- open finding IDs and accepted-risk expiry.

## Production Rule

No automated upgrade is allowed to bypass a failed blocking gate. Emergency security updates may shorten the approval path, but they still need staging evidence, scan output, rollback evidence, and an auditable incident/change record.

## Source Notes

- Renovate documentation: https://docs.renovatebot.com/
- Argo CD sync phases and waves: https://argo-cd.readthedocs.io/en/stable/user-guide/sync-waves/
- OSV-Scanner documentation: https://google.github.io/osv-scanner/
- OpenSSF Scorecard: https://scorecard.dev/
