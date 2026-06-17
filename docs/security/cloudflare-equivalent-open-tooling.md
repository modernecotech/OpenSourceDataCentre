# Cloudflare Equivalent Open Tooling

This map describes service categories, not global-network equivalence. OSDC Edge Shield should offer local control of the same categories of service, while being honest that it cannot reproduce a commercial global anycast network.

| Proprietary function | Open-source equivalent | Role in OSDC |
| --- | --- | --- |
| Cloudflare DNS | PowerDNS Authoritative + dnsdist, optionally Knot/NSD | Sovereign authoritative DNS, DNSSEC, health-aware responses. |
| Cloudflare CDN | Varnish/Vinyl Cache, Nginx cache, Apache Traffic Server | Static and API response caching. |
| Cloudflare WAF | OWASP Coraza + OWASP Core Rule Set | Web/API request inspection. |
| Cloudflare reverse proxy/TLS | Caddy, Envoy, HAProxy, Traefik | TLS termination, routing, origin proxy. |
| Cloudflare Tunnel | WireGuard, Headscale, NetBird, OpenZiti | Publish private origins without exposing inbound ports. |
| Cloudflare Access / Zero Trust | Keycloak, Authentik/Authelia, OPA | Identity-aware access to admin and tenant apps. |
| Cloudflare rate limiting/bot blocking | CrowdSec, nftables, HAProxy stick tables, Envoy rate limit | Abuse blocking and dynamic bans. |
| Cloudflare Load Balancing | HAProxy, Envoy, Traefik, dnsdist | Health-aware regional routing. |
| Cloudflare Workers | WasmEdge, Wasmtime, Spin, wasmCloud | Small edge functions and transforms. |
| Cloudflare Logs/Analytics | OpenTelemetry, Prometheus, VictoriaMetrics, Loki, Grafana | Logs, metrics, tracing, dashboards. |
| Cloudflare config dashboard | OSDC Rust edge agent + GitOps | Sovereign UI/API/config rollout layer. |

## Broader Proprietary Replacement Map

| Proprietary system | OSDC open-source replacement |
| --- | --- |
| Okta / Auth0 | Keycloak + Authentik/Authelia where useful |
| HashiCorp Vault Enterprise | OpenBao |
| Terraform Cloud | OpenTofu + GitOps runner + S3/Ceph state backend |
| DockerHub / ECR / Artifactory | Harbor + cosign + Syft + Grype |
| Datadog / New Relic | OpenTelemetry + Prometheus/VictoriaMetrics + Grafana + Loki/Tempo |
| Splunk / Sentinel | Wazuh + OpenSearch + Zeek + Suricata + MISP |
| GitHub Enterprise / GitLab SaaS | Forgejo + Woodpecker CI + Renovate |
| AWS KMS / Secrets Manager | OpenBao + Barbican where OpenStack is used |
| AWS IAM-like policy | Keycloak + OPA + OpenStack Keystone |
| AWS CloudWatch | OpenTelemetry + Prometheus + Loki + Grafana |

## Tooling Defaults

- Default DNS: PowerDNS Authoritative with PostgreSQL backend and dnsdist in front.
- Default TLS/proxy: Caddy for MVP; Envoy or HAProxy for high-scale production.
- Default cache: Varnish/Vinyl Cache.
- Default WAF: Coraza with OWASP CRS.
- Default tunnel: NetBird or OpenZiti for production; Headscale/plain WireGuard for simple deployments.
- Default registry: Harbor with cosign, Syft, and Grype.
- Default secrets: OpenBao.
- Default IaC: OpenTofu plus GitOps runner.
