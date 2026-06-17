# Patching and Upgrade Policy

A sovereign datacentre must not become a frozen pile of old open-source tools. The OSDC portal should expose a managed upgrade path with testing, approval, rollback, and audit.

## Upgrade Flow

```text
Upstream releases
   |
Renovate watches charts containers Rust crates packages OS images
   |
Pull request created
   |
SBOM + vulnerability scan + licence scan + policy check
   |
Staging rollout through Argo CD / Flux
   |
Smoke tests + backup/restore test + rollback test
   |
Approval window
   |
Production rollout
   |
Health verification
   |
Audit record stored
```

## Policy Classes

| Update class | Frequency | Process |
| --- | ---: | --- |
| Critical CVE | 24-72 hours | Emergency staging test, fast approval, production patch. |
| High security | Weekly | Normal PR, staging, rollout. |
| Normal patch | Monthly | Maintenance window. |
| Minor feature release | Quarterly | Compatibility test. |
| Major version | 6-12 months | Migration plan, backup, dry run. |
| Firmware/BMC | Quarterly or emergency | Lab test first, staged by rack. |
| Kubernetes/OpenStack/Ceph | Planned release train | Never ad hoc. |

Machine-readable policy rows live in:

- [upgrade-policy.csv](../../data/software/upgrade-policy.csv)
- [upgrade-rings.csv](../../data/software/upgrade-rings.csv)
- [upgrade-test-gates.csv](../../data/software/upgrade-test-gates.csv)

The broader automated testing fabric is defined in [Assurance Test and Upgrade Fabric](assurance-test-and-upgrade-fabric.md).

## Portal View

The unified UI should not perform blind upgrades. It should show:

```text
Service: Keycloak
Current version: 26.x
Available version: 26.y
Risk: security patch
Test status: passed in staging
Backup status: valid
Rollback status: available
Approval: required from platform owner
Scheduled window: Friday 23:00
```

Then the portal should trigger GitOps. It should not SSH into machines randomly.

## Required Gates

- Dependency PR exists.
- SBOM generated.
- Vulnerability scan completed.
- Licence/policy check completed.
- Open-source threat-management findings are ingested into DefectDojo or Dependency-Track.
- Config scripts validated where the change touches service configuration.
- Staging rollout completed.
- Smoke test passed.
- Backup/restore test passed for stateful services.
- Rollback tested or documented.
- Owner approval recorded.
- Audit event stored.

## Source Notes

- Renovate documentation: https://docs.renovatebot.com/
- Argo CD documentation: https://argo-cd.readthedocs.io/en/stable/
- Flux documentation: https://fluxcd.io/flux/
