# Developer Platform Examples

These examples are lightweight starter artifacts for the OSDC developer platform.

They are not complete applications. They show the files the portal should generate when a developer creates a new service:

- `.devcontainer/devcontainer.json` for VS Code Dev Containers.
- `.woodpecker.yml` for CI.
- `deploy/` manifests for GitOps or OpenTofu review.

The production path is:

1. Create a Forgejo repository from a template.
2. Open the clone URL in VS Code.
3. Reopen the project in the dev container.
4. Commit and push application changes.
5. Let CI build, test, scan, sign, and publish artifacts.
6. Promote through GitOps after approval.
