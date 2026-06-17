# Secrets and Key Management

The edge layer handles TLS keys, tunnel keys, API tokens, signing keys, registry credentials, and administrator access. Those are national control points.

## Baseline Stack

- OpenBao for secrets, dynamic credentials, leasing, revocation, and transit encryption.
- Keycloak for human identity.
- OPA for policy decisions.
- cert-manager, Caddy, and Smallstep for certificate workflows.
- OpenStack Barbican where OpenStack service integration is required.

## Rules

- Root keys and recovery material remain under national or institutional control.
- Tunnel keys rotate on a schedule and after operator changes.
- Public TLS and private CA workflows are separated.
- Edge agents receive least-privilege tokens.
- Secrets are never embedded in generated configs stored in Git.

## Evidence

- Key ceremony or initialization record.
- Recovery test.
- Rotation log.
- Access review.
- Revocation test.
