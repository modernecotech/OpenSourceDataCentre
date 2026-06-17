# Sovereign Edge Security Stack

OSDC Edge Shield is the software and security fabric around a sovereign datacentre. It gives a country or institution local control over DNS, TLS, cache, WAF, rate limiting, tunnels, identity-aware access, logs, metrics, secrets, policy, and audit trails.

It should not be described as a Cloudflare clone. It is a sovereign regional equivalent for the same service categories, built from open-source systems and operated by local teams.

## Reference Flow

```text
Internet user
   ↓
Authoritative DNS / geo steering
   ↓
Edge reverse proxy + TLS
   ↓
WAF + rate limit + bot friction
   ↓
CDN/static/API cache
   ↓
Origin load balancer
   ↓
Private origin over WireGuard / NetBird / OpenZiti
   ↓
Sovereign datacentre services
```

## Minimum Node Profile

- PowerDNS Authoritative + dnsdist.
- Caddy for TLS and reverse proxy.
- Varnish/Vinyl Cache for cache.
- Coraza WAF with OWASP Core Rule Set.
- CrowdSec + nftables for abuse response.
- WireGuard + Headscale or NetBird for origin tunnels.
- Keycloak + OPA for zero-trust access.
- Prometheus + Loki + Grafana + OpenTelemetry for telemetry.
- `osdc-edge` Rust agent for config, health, audit, and status.

## Regional Profile

Run at least three nodes per region where possible:

```text
edge-a: active proxy/cache/DNS
edge-b: active proxy/cache/DNS
edge-c: management secondary DNS cold failover
```

## Rust Responsibility

Rust should not rewrite DNS, WAF, proxy, tunnel, or observability engines. The Rust layer should own:

- service inventory;
- config generation;
- health checks;
- safe rollouts;
- policy decisions;
- audit events;
- one operator API/UI.

## Operating Boundary

OSDC Edge Shield provides sovereign edge protection, not unlimited DDoS protection. Volumetric attacks must still be handled with upstream ISPs, IXPs, scrubbing providers, BGP blackhole/FlowSpec, and national telecom coordination.
