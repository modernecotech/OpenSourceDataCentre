# Developer Platform

The developer platform is the sovereign equivalent of GitHub Enterprise, Azure DevOps, Cloud Build, CodePipeline, Artifact Registry, and Terraform Cloud.

## Default Stack

| Function | Open-source tools | OSDC UI workflow |
| --- | --- | --- |
| Source repositories | Forgejo, Gitea, GitLab CE | Repo request, teams, branch protection, approvals. |
| CI | Woodpecker CI, Tekton, Jenkins | Build logs, test status, artifact promotion. |
| GitOps delivery | Argo CD, Flux | Deployment status, sync status, rollback. |
| Registry | Harbor | Image scan, signing, SBOM, project quota. |
| IaC | OpenTofu, Ansible, Crossplane | Plan, approval, apply status, drift. |
| Developer portal | Backstage as optional backend/plugin | Service catalogue, templates, docs, ownership. |
| Package registry | Harbor OCI, Gitea packages, Nexus alternatives | Internal packages and release evidence. |
| Feature flags | OpenFeature, Flipt, Unleash | Controlled rollout flags. |

## Guardrails

- Production changes go through GitOps.
- Platform images must be signed.
- SBOMs must be stored for platform releases.
- IaC plans require approval.
- Registry vulnerabilities must be visible before promotion.
- Rollbacks must be tested.

## Source Notes

- Backstage overview: https://backstage.io/docs/overview/what-is-backstage/
- Harbor documentation: https://goharbor.io/docs/
- OpenTofu documentation: https://opentofu.org/docs/intro/
