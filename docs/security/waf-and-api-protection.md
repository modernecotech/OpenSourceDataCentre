# WAF and API Protection

The WAF layer should protect public services without hiding rule behavior from local operators.

## Baseline

- Coraza with OWASP Core Rule Set.
- Caddy, Envoy, HAProxy, or Traefik integration depending on deployment profile.
- Detection mode first, then staged blocking.
- Rule exceptions stored in Git and linked to tickets.
- Logs shipped to Loki/OpenSearch and summarized in Grafana.

## Cache Policy

```text
Cache public static files aggressively.
Cache public API responses only when explicitly marked cacheable.
Default to no-store for identity, health, legal, payment, admin, and sensitive public-sector services.
```

## Abuse Controls

- CrowdSec local decisions for known abuse patterns.
- nftables or proxy-level bouncers for enforcement.
- HAProxy stick tables or Envoy rate-limit service where needed.
- Bot friction only on risky paths, with accessibility and public-service impact reviewed.

## Rollout Rules

- Start new WAF rules in detection mode.
- Measure false positives before blocking.
- Keep emergency bypass for critical public services.
- Never cache sensitive authenticated responses by default.
- Review all rule exceptions periodically.
