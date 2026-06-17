# Developer Platform

The developer platform is the sovereign equivalent of GitHub Enterprise, Azure DevOps, Cloud Build, CodePipeline, Artifact Registry, and Terraform Cloud.

It should let local developers build and deploy software without learning every infrastructure subsystem. The OSDC portal owns the workflow and audit surface. Forgejo, CI, Harbor, GitOps controllers, OpenTofu, and VS Code do the day-to-day work underneath.

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

## Default Recommendation

Use this first stack for the 250 kW regional pilot:

| Layer | Default | Reason |
| --- | --- | --- |
| Source forge | Forgejo | Lightweight self-hosted GitHub-like forge. |
| CI | Woodpecker CI | Simple container-native pipelines connected to Forgejo. |
| Registry | Harbor | Local OCI registry with vulnerability scanning, SBOM support, robot accounts, and replication options. |
| App GitOps | Argo CD | Visual deployment status, manual sync, rollback, and audit-friendly operator UX. |
| Controller GitOps | Flux | Lightweight controller-first sync for edge, platform, and cluster config. |
| IaC | OpenTofu + Ansible/AWX | Plan/apply workflow for infrastructure, VM, network, DNS, and platform changes. |
| Templates | OSDC templates, optional Backstage later | Start simple in the OSDC portal, then add Backstage if a larger developer portal is needed. |
| Dependency updates | Renovate | Scheduled dependency and image update pull requests. |

## Developer Workflow

```text
Developer Console
   |
create service from template
   |
Forgejo repository
   |
open in VS Code
   |
Dev Container
   |
push code
   |
Woodpecker/Tekton pipeline
   |
test + scan + SBOM + sign
   |
Harbor artifact
   |
GitOps pull request
   |
Argo CD or Flux rollout
   |
OSDC health, cost, audit, rollback
```

The browser should expose the whole flow while still letting developers work naturally in VS Code. The portal should create the repo, show the clone URI, link the devcontainer, show CI status, show Harbor image/security state, and open or stage GitOps changes.

## VS Code Integration

The first implementation should not require a custom VS Code extension. It should work through standard Git and Dev Containers:

- Forgejo exposes a normal Git remote.
- The portal exposes a `vscode://vscode.git/clone?...` URI for each generated repository.
- Templates include `.devcontainer/devcontainer.json` so developers can reopen the project in a consistent toolchain.
- Templates include `.woodpecker.yml` so CI is versioned next to code.
- Templates include `deploy/` or `OpenTofu` paths so GitOps and infrastructure changes are reviewable in source control.
- The portal links to pipeline status, image scan state, SBOMs, deployment sync state, and rollback records.

An optional later extension can add a native OSDC activity view for tenants, pipeline status, deployment environments, and approval requests. That extension should call the OSDC API rather than bypassing GitOps.

## Data Files

- `data/software/developer-platform-services.csv` defines the developer stack services and controls.
- `data/software/developer-templates.csv` defines starter templates, devcontainers, CI files, deployment manifests, owners, and VS Code clone URIs.
- `data/software/deployment-environments.csv` defines dev, staging, production, edge, and IaC targets.
- `data/software/developer-promotion-gates.csv` defines checks and approvers for promotion.
- `data/software/vscode-workflows.csv` defines the first VS Code-facing actions.
- `examples/developer-platform/` contains starter template artifacts.

## Guardrails

- Production changes go through GitOps.
- Platform images must be signed.
- SBOMs must be stored for platform releases.
- IaC plans require approval.
- Registry vulnerabilities must be visible before promotion.
- Rollbacks must be tested.
- Dev containers must not embed production secrets.
- CI robot accounts must be scoped to project or environment.
- Developers can deploy to dev directly, but staging and production require promotion gates.
- Emergency production changes must still leave a Git and audit trail.

## Source Notes

- Forgejo: https://forgejo.org/
- Woodpecker CI: https://woodpecker-ci.org/docs/intro
- Argo CD documentation: https://argo-cd.readthedocs.io/
- Flux documentation: https://fluxcd.io/flux/
- Backstage overview: https://backstage.io/docs/overview/what-is-backstage/
- Harbor documentation: https://goharbor.io/docs/
- OpenTofu documentation: https://opentofu.org/docs/intro/
- Renovate documentation: https://docs.renovatebot.com/
- VS Code Dev Containers: https://code.visualstudio.com/docs/devcontainers/containers
- VS Code Source Control: https://code.visualstudio.com/docs/sourcecontrol/overview
