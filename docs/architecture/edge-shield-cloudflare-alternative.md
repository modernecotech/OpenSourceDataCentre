# OSDC Edge Shield: Open Cloudflare Alternative

Last reviewed: 2026-06-14.

`OSDC Edge Shield` is a low-cost open-source alternative to the common Cloudflare bundle, designed to run on Radxa ROCK 5B+/RK3588 boards at schools, cities, ISPs, hospitals, datacentres, and regional exchange points.

It is not a claim to replace Cloudflare's global network. Cloudflare's value is partly its planet-scale anycast network. The OSDC version is a sovereign, locally operated edge fabric that gives developing-world operators the same *classes* of service with open tools, local ownership, and low power consumption.

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
| Reverse proxy and TLS | Caddy or Traefik | Automatic TLS, origin routing, HTTP/2/3 where supported |
| CDN/static cache | Varnish Cache, Nginx cache, Caddy cache module where suitable | Cache static assets and API responses close to users |
| WAF | OWASP Coraza with OWASP Core Rule Set | Protect HTTP apps and APIs |
| Rate limiting and abuse blocking | CrowdSec, nftables, HAProxy stick tables, Envoy local rate limit | Block aggressive clients and propagate local blocklists |
| Load balancing | HAProxy, Traefik, Envoy, dnsdist | Route to healthy origins and regional datacentres |
| Tunnels | WireGuard, Headscale, NetBird, zrok/OpenZiti where suitable | Publish private origins without exposing inbound ports |
| Zero-trust access | Authelia or Authentik, Keycloak, OPA | Protect admin apps and tenant services |
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
2. Caddy for TLS and reverse proxy.
3. Varnish for HTTP caching.
4. Coraza WAF for API/web protection.
5. CrowdSec plus nftables for abuse response.
6. WireGuard/Headscale for private origin tunnels.
7. Prometheus and Loki exporters for metrics/logs.
8. `osdc-edge` for local status, service inventory, and config/audit surface.

## Safety Boundaries

- Do not expose admin dashboards publicly without zero-trust access.
- Do not centralize all country traffic through one board or one city.
- Do not advertise DDoS protection beyond the upstream bandwidth actually available.
- Keep bypass paths so emergency public-service sites can be served directly if the edge layer fails.
- Cache only content that policy allows; health, legal, and identity systems may require no-store defaults.

## Source Notes

- Cloudflare describes its platform as combining DDoS protection, WAF, bot management, and Zero Trust controls on one network: https://www.cloudflare.com/
- Cloudflare application services include DDoS/bot blocking, vulnerability protection, caching/acceleration, and API management: https://www.cloudflare.com/application-services/products/
- Cloudflare product categories include compute, storage, AI, media, security, network, and SASE/Zero Trust: https://www.cloudflare.com/products/
- Caddy is an open-source web server with automatic HTTPS and reverse-proxy support: https://caddyserver.com/ and https://caddyserver.com/docs/quick-starts/reverse-proxy
- PowerDNS Authoritative and Recursor are available in source form and Linux packages: https://www.powerdns.com/downloads
- OWASP Coraza is an open-source WAF compatible with the OWASP Core Rule Set: https://www.coraza.io/
- CrowdSec is an open-source collaborative cybersecurity solution for detecting and blocking malicious behavior: https://www.crowdsec.net/
- Radxa ROCK 5B/5B+ supports Debian and Armbian images: https://docs.radxa.com/en/rock5/rock5b/download
