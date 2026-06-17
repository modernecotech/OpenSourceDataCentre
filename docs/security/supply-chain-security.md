# Supply Chain Security

The edge/security pillar should be reproducible and auditable. Operators need to know what software is running, where it came from, and whether it can be rebuilt.

## Baseline Stack

- Forgejo for source repositories.
- Woodpecker CI for pipelines.
- Harbor for OCI registry.
- cosign for image signing.
- Syft for SBOM generation.
- Grype for vulnerability scanning.
- Renovate for dependency update proposals.
- OpenTofu and Ansible for infrastructure changes.

## Rules

- Platform images are signed before deployment.
- SBOMs are stored with releases.
- Critical images are scanned before rollout.
- Base images are pinned and reviewed.
- Edge config changes are pull-requested, reviewed, and auditable.
- Rollbacks must be tested.

## Evidence

- Signed release.
- SBOM.
- Vulnerability scan.
- Deployment diff.
- Rollback test.
- Exception record for known unresolved vulnerabilities.

Supply-chain evidence should be ingested into the open threat-management platform:

- Dependency-Track tracks SBOM/component risk over time.
- DefectDojo aggregates scanner findings, deduplicates results, and tracks remediation.
- The OSDC assurance console summarizes whether an upgrade can move to the next ring.

Related docs:

- [Assurance Test and Upgrade Fabric](../software/assurance-test-and-upgrade-fabric.md)
- [Open Threat Management and Scanner Platform](open-threat-management-and-scanner.md)
