# osdc-edge

`osdc-edge` is the Radxa-local Edge Shield node service. It is intended to run on low-cost RK3588 boards such as Radxa ROCK 5B+ and expose enough local status/config information for DNS, reverse proxy, cache, WAF, rate limiting, tunnels, zero-trust access, and observability.

It is the local control/status layer for the sovereign edge fabric. It does not replace mature edge software or claim global DDoS absorption.

## APIs

- `/` - local HTML dashboard.
- `/api/status` - node, board, power source, service status, and metrics.
- `/api/config-preview` - generated file list and rollout checks for the node.
- `/health` - readiness check.

## Run

```bash
cargo run -p osdc-edge -- 127.0.0.1:8790
```

Open `http://127.0.0.1:8790/` on the node or through an operator tunnel.

## Test

```bash
cargo test -p osdc-edge
```

The tests verify dashboard routing, JSON status, generated config paths, rollout checks, health checks, and 404 behavior.

## Deployment Note

This crate should coordinate open-source components such as PowerDNS, dnsdist, Caddy, Envoy, HAProxy, Varnish/Vinyl Cache, Coraza, CrowdSec, WireGuard, NetBird, OpenZiti, Keycloak, OPA, Prometheus, Loki, Grafana, and OpenTelemetry.

Related crates:

- `osdc-edge-config` validates deployment profiles from `examples/edge-shield/`.
- `osdc-edge-policy` models cache, access, and DDoS claim policy decisions.
