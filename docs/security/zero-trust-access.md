# Zero Trust Access

Zero-trust access protects admin and tenant applications before they reach an origin service.

## Baseline Stack

- Keycloak for identity, realms, groups, MFA, and federation.
- OPA for policy decisions.
- Caddy, Envoy, HAProxy, Authelia, or Authentik as the access enforcement layer.
- NetBird, OpenZiti, Headscale, or WireGuard for private origin and operator networks.
- OpenTelemetry, Loki, and Grafana for access logs and dashboards.

## Rules

- No public admin dashboard without identity-aware access.
- No shared administrator accounts.
- Admin sessions must be logged with user, source, target, decision, and ticket or change reference.
- Emergency access must be documented, time-limited, and reviewed.
- Tenant access exceptions must expire.

## Policy Inputs

- User identity.
- Group or agency.
- Device or source network trust.
- Requested application.
- Route sensitivity.
- Time window.
- Change ticket or emergency flag.

## Evidence

- Identity realm export.
- OPA policy version.
- Access review.
- Break-glass test.
- Deny-log sample.
- Incident drill record.
