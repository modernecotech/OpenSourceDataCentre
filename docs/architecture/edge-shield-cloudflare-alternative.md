# OSDC Edge Shield: Sovereign Edge and Security Fabric

Last reviewed: 2026-06-17.

`OSDC Edge Shield` is an open-source, nationally operated alternative to the core classes of service provided by Cloudflare, Akamai, Fastly, Okta, Datadog, Splunk, Vault, Terraform Cloud, and other proprietary infrastructure platforms.

It is not a claim to replace Cloudflare's global network. Cloudflare currently lists 337 cities across 8 regions on its network page, which is not something a new open-source repo can replicate. The OSDC version is a sovereign, locally operated regional edge fabric that gives developing-world operators the same *classes* of service with open tools, local ownership, and low power consumption.

## Request Path

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

## Target Radxa Node

| Item | Baseline |
| --- | --- |
| Board | Radxa ROCK 5B+ or equivalent RK3588 board |
| RAM | 16-32 GB |
| Storage | NVMe SSD for cache and logs |
| Network | 2.5GbE minimum; USB3 or PCIe NIC where a second port is required |
| Power | 48 VDC rack/row bus to USB-C PD or PoE HAT |
| OS | Debian or Armbian |
| Role | edge reverse proxy, DNS, cache, WAF, tunnel endpoint, access gateway, telemetry node |

Run at least three nodes per region where possible:

- `edge-a`: primary route and cache.
- `edge-b`: redundant route and cache.
- `edge-c`: management, DNS secondary, and cold failover.

## Service Map

| Cloudflare-like function | OSDC open-source stack | Radxa role |
| --- | --- | --- |
| Authoritative DNS | PowerDNS Authoritative, dnsdist | Host zones, DNSSEC, health-aware responses, API-managed records |
| Recursive/secure DNS | Unbound or PowerDNS Recursor | Local secure resolver for tenants and operators |
| Reverse proxy and TLS | Caddy, Envoy, HAProxy, or Traefik | Automatic TLS, origin routing, HTTP/2/3 where supported |
| CDN/static cache | Varnish/Vinyl Cache, Nginx cache, Caddy cache module where suitable | Cache static assets and API responses close to users |
| WAF | OWASP Coraza with OWASP Core Rule Set | Protect HTTP apps and APIs |
| Rate limiting and abuse blocking | CrowdSec, nftables, HAProxy stick tables, Envoy local rate limit | Block aggressive clients and propagate local blocklists |
| Load balancing | HAProxy, Traefik, Envoy, dnsdist | Route to healthy origins and regional datacentres |
| Tunnels | WireGuard, Headscale, NetBird, OpenZiti where suitable | Publish private origins without exposing inbound ports |
| Zero-trust access | Keycloak, OPA, Authelia, or Authentik | Protect admin apps and tenant services |
| Bot friction | Anubis-style proof-of-work, mCaptcha, Friendly Captcha self-hosted where acceptable | Apply lightweight checks only to risky paths |
| Edge functions | WASI runtimes such as Wasmtime, Spin, or wasmCloud | Small request/response transforms and local functions |
| Observability | Prometheus node exporter, OpenTelemetry, Loki, Grafana | Measure cache hit ratio, blocked requests, latency, and origin health |
| Config and GitOps | OSDC Rust edge agent, OpenTofu, Ansible, Forgejo, Flux/Argo CD | Generate configs, audit changes, and roll back |

## Rust Components

Two Rust surfaces are included:

- `osdc-portal /edge`: operator GUI for the unified Edge Shield stack.
- `osdc-edge`: Radxa-runnable local edge node service with a small status API and dashboard.

The Rust layer should not rewrite mature packet-processing tools. It should own:

- service inventory,
- config generation,
- health checks,
- safe rollouts,
- policy decisions,
- audit events,
- and a single API/UI for operators.

## First Deployment Profile

Minimum viable deployment:

1. PowerDNS Authoritative for zones.
2. dnsdist in front for DNS steering and abuse handling.
3. Caddy for TLS and reverse proxy.
4. Varnish/Vinyl Cache for HTTP caching.
5. Coraza WAF with OWASP Core Rule Set for API/web protection.
6. CrowdSec plus nftables for abuse response.
7. WireGuard plus Headscale or NetBird for private origin tunnels.
8. Keycloak plus OPA for zero-trust access.
9. Prometheus, Loki, Grafana, and OpenTelemetry for telemetry.
10. `osdc-edge` for local status, service inventory, config rollout, audit, and health.

For regional deployment, run at least three nodes where possible:

```text
edge-a: active proxy/cache/DNS
edge-b: active proxy/cache/DNS
edge-c: management secondary DNS cold failover
```

## Open Replacement Map

| Proprietary system | OSDC open-source replacement |
| --- | --- |
| Cloudflare DNS | PowerDNS Authoritative + dnsdist |
| Cloudflare CDN | Varnish/Vinyl Cache + Caddy/Envoy + DNS steering |
| Cloudflare WAF | Coraza + OWASP CRS |
| Cloudflare Access | Keycloak + OPA + Caddy/Envoy auth middleware |
| Cloudflare Tunnel | WireGuard + NetBird/OpenZiti or Headscale/simple WireGuard |
| Okta / Auth0 | Keycloak + Authentik/Authelia where useful |
| Vault Enterprise | OpenBao |
| Terraform Cloud | OpenTofu + GitOps runner + S3/Ceph state backend |
| DockerHub / ECR / Artifactory | Harbor + cosign + Syft + Grype |
| Datadog / New Relic | OpenTelemetry + Prometheus/VictoriaMetrics + Grafana + Loki/Tempo |
| Splunk / Sentinel | Wazuh + OpenSearch + Zeek + Suricata + MISP |

## Safety Boundaries

- Do not expose admin dashboards publicly without zero-trust access.
- Do not centralize all country traffic through one board or one city.
- Do not advertise DDoS protection beyond the upstream bandwidth actually available.
- Volumetric attacks must still be handled with upstream ISPs, IXPs, scrubbing providers, BGP blackhole/FlowSpec, and national telecom coordination.
- Keep bypass paths so emergency public-service sites can be served directly if the edge layer fails.
- Cache only content that policy allows; health, legal, and identity systems may require no-store defaults.
- Support both CrowdSec community-intelligence mode and fully local/private intelligence mode for sovereign deployments.

## Source Notes

- Cloudflare describes its platform as combining DDoS protection, WAF, bot management, and Zero Trust controls on one network: https://www.cloudflare.com/
- Cloudflare network locations are listed at: https://www.cloudflare.com/network/
- PowerDNS Authoritative supports multiple backends including PostgreSQL and JSON APIs: https://doc.powerdns.com/authoritative/
- Caddy automatic HTTPS and reverse proxy docs: https://caddyserver.com/docs/automatic-https and https://caddyserver.com/docs/quick-starts/reverse-proxy
- Envoy describes itself as an L7 proxy with routing, rate limiting, health checking, and edge-proxy support: https://www.envoyproxy.io/docs/envoy/latest/intro/what_is_envoy
- Varnish/Vinyl Cache is a caching HTTP reverse proxy: https://varnish-cache.org/intro/index.html
- OWASP Coraza is an open-source WAF compatible with the OWASP Core Rule Set: https://www.coraza.io/
- CrowdSec documents log/request based detection and remediation components: https://docs.crowdsec.net/docs/intro/
- WireGuard describes itself as a simple and modern secure VPN tunnel: https://www.wireguard.com/
- NetBird is a self-hostable WireGuard-based zero-trust networking platform: https://docs.netbird.io/
- Radxa ROCK 5B/5B+ supports Debian and Armbian images: https://docs.radxa.com/en/rock5/rock5b/download
