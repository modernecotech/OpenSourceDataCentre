# Browser-Based Config Management

The OSDC UI should expose the real configuration scripts of individual tools instead of trying to capture every upstream option as a custom form field.

This makes the web browser the management surface for the whole system while preserving the operational truth that mature open-source tools still use their own configuration languages.

## Principle

```text
Browser editor
   |
validate script
   |
open GitOps change
   |
review and approve
   |
staging rollout
   |
health checks and rollback test
   |
production rollout
   |
audit record
```

The browser should edit source-controlled config artifacts. It should not SSH into machines and rewrite live files directly.

## Why This Beats Giant Forms

- Caddy, PowerDNS, Coraza, CrowdSec, WireGuard, OpenBao, Keycloak, Argo CD, Flux, Prometheus, and many other tools already have expressive config formats.
- A UI that models every option will be incomplete, stale, and risky.
- Operators need to see the exact config that will run.
- Git diffs, review, validation, and rollback are better safety controls than hidden UI state.
- Advanced operators can still use the browser from low-power laptops or local management stations.

## UI Pattern

The portal should expose:

- file path;
- owner;
- tool;
- language;
- validation command;
- rollout target;
- risk level;
- editable text;
- rendered diff;
- policy warnings;
- validation result;
- pull request or change request status;
- rollout checks;
- audit trail.

## Safety Rules

- Secrets must appear as references or placeholders, not raw secret values.
- High-risk files require review from the service owner.
- WAF rules start in detection mode unless explicitly approved.
- DNS changes require zone validation and rollback plan.
- Tunnel changes require key rotation and emergency bypass check.
- Certificate and key material must be controlled through OpenBao or an approved CA workflow.

## Data

- `data/software/config-script-catalogue.csv` lists editable scripts and validation commands.
- `examples/config-scripts/` contains sample source artifacts.
- `/api/config/scripts` exposes sample scripts through the portal prototype.

## Scope

This model should cover tool configuration for:

- Edge Shield: Caddy, PowerDNS, Coraza, CrowdSec, WireGuard, NetBird, OpenZiti.
- Cloud core: OpenStack service configs, Kubernetes manifests, Cilium policy, Ceph pools.
- Security: OPA, Kyverno, Wazuh, Falco, Suricata, OpenBao policy.
- Developer platform: Argo CD, Flux, Harbor, OpenTofu, Ansible.
- Observability: Prometheus, Alertmanager, Grafana dashboards, Loki retention.

The UI can still provide guided forms for common workflows, but the escape hatch and source of truth should be the config-as-code editor.
