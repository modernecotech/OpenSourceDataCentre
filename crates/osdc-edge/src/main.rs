use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use serde::Serialize;

const EDGE_HTML: &str = r##"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>OSDC Edge Node</title>
    <style>
      :root { color-scheme: light; --ink:#111827; --muted:#5b6473; --line:#d8dee8; --page:#eef3f6; --surface:#fff; --teal:#0f766e; }
      * { box-sizing: border-box; }
      body { margin: 0; color: var(--ink); background: var(--page); font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; font-size: 14px; line-height: 1.45; letter-spacing: 0; }
      header { min-height: 72px; background: var(--surface); border-bottom: 1px solid var(--line); padding: 16px 22px; display:flex; justify-content:space-between; gap:16px; align-items:center; }
      h1 { margin: 0; font-size: 22px; }
      p { margin: 4px 0 0; color: var(--muted); }
      main { padding: 22px; display:grid; gap:16px; }
      .grid { display:grid; grid-template-columns: repeat(4, minmax(150px, 1fr)); gap:12px; }
      .panel, .metric { background:var(--surface); border:1px solid var(--line); border-radius:8px; overflow:hidden; }
      .metric { min-height:90px; padding:14px; display:grid; align-content:space-between; }
      .metric span { color:var(--muted); font-size:12px; }
      .metric strong { font-size:24px; }
      .links, .tools { display:flex; flex-wrap:wrap; gap:8px; align-items:center; }
      .panel-header { min-height:52px; padding:12px 14px; border-bottom:1px solid var(--line); display:flex; justify-content:space-between; gap:10px; align-items:center; }
      .panel h2 { margin:0; font-size:15px; }
      input { min-height:36px; width:min(240px, 48vw); border:1px solid var(--line); border-radius:8px; padding:0 10px; font:inherit; }
      table { width:100%; border-collapse:collapse; }
      th, td { border-bottom:1px solid var(--line); padding:10px 12px; text-align:left; }
      th { color:var(--muted); background:#f8fafc; font-size:12px; }
      .status { display:inline-flex; min-height:24px; align-items:center; border-radius:999px; padding:0 8px; background:#ecfdf5; color:#166534; font-size:12px; font-weight:700; }
      .status.warn { background:#fff7ed; color:#b45309; }
      @media (max-width: 760px) { .grid { grid-template-columns:1fr 1fr; } header { align-items:flex-start; flex-direction:column; } table { display:block; overflow-x:auto; white-space:nowrap; } }
    </style>
  </head>
  <body>
    <header>
      <div>
        <h1>OSDC Edge Node</h1>
        <p>Radxa/RK3588 local edge controller for DNS, cache, WAF, tunnel, access, and telemetry services</p>
      </div>
      <div class="links">
        <a href="/api/status">JSON status</a>
        <a href="/api/config-preview">Config preview</a>
      </div>
    </header>
    <main>
      <section class="grid" id="metrics"></section>
      <section class="panel">
        <div class="panel-header">
          <h2>Services</h2>
          <input id="service-filter" type="search" placeholder="Filter services">
        </div>
        <table>
          <thead><tr><th>Service</th><th>Stack</th><th>Port</th><th>Status</th></tr></thead>
          <tbody id="services"></tbody>
        </table>
      </section>
      <section class="panel">
        <div class="panel-header"><h2>Generated Config Preview</h2><span class="status" id="node-role">radxa-edge-gateway</span></div>
        <table>
          <thead><tr><th>File</th><th>Owner</th><th>Purpose</th></tr></thead>
          <tbody id="config-files"></tbody>
        </table>
        <table>
          <thead><tr><th>Rollout check</th></tr></thead>
          <tbody id="checks"></tbody>
        </table>
      </section>
    </main>
    <script>
      const badge = (status) => `<span class="status ${status === "degraded" ? "warn" : ""}">${status}</span>`;
      const filterRows = () => {
        const query = document.getElementById("service-filter").value.trim().toLowerCase();
        for (const row of document.querySelectorAll("#services tr")) {
          row.hidden = query && !row.textContent.toLowerCase().includes(query);
        }
      };
      Promise.all([
        fetch("/api/status").then(r => r.json()),
        fetch("/api/config-preview").then(r => r.json())
      ]).then(([data, config]) => {
        document.getElementById("metrics").innerHTML = data.metrics.map(m => `<div class="metric"><span>${m.label}</span><strong>${m.value}</strong><span>${m.detail}</span></div>`).join("");
        document.getElementById("services").innerHTML = data.services.map(s => `<tr><td>${s.name}</td><td>${s.stack}</td><td>${s.port}</td><td>${badge(s.status)}</td></tr>`).join("");
        document.getElementById("node-role").textContent = config.node_role;
        document.getElementById("config-files").innerHTML = config.generated_files.map(f => `<tr><td>${f.path}</td><td>${f.owner}</td><td>${f.purpose}</td></tr>`).join("");
        document.getElementById("checks").innerHTML = config.rollout_checks.map(check => `<tr><td>${check}</td></tr>`).join("");
        document.getElementById("service-filter").addEventListener("input", filterRows);
      });
    </script>
  </body>
</html>
"##;

fn main() -> std::io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8790".to_string());
    let listener = TcpListener::bind(&addr)?;

    println!("osdc-edge listening on http://{addr}");
    println!("edge dashboard: http://{addr}/");
    println!("edge status API: http://{addr}/api/status");
    println!("edge config preview: http://{addr}/api/config-preview");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_connection(stream) {
                    eprintln!("request failed: {err}");
                }
            }
            Err(err) => eprintln!("connection failed: {err}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 4096];
    let read = stream.read(&mut buffer)?;
    if read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..read]);
    let mut parts = request
        .lines()
        .next()
        .unwrap_or_default()
        .split_whitespace();
    let method = parts.next().unwrap_or_default();
    let raw_path = parts.next().unwrap_or("/");
    let path = raw_path.split('?').next().unwrap_or("/");

    let response = route_response(method, path);

    stream.write_all(&response)?;
    stream.flush()
}

fn route_response(method: &str, path: &str) -> Vec<u8> {
    match (method, path) {
        ("GET", "/") => html(EDGE_HTML),
        ("GET", "/api/status") => json(&edge_status()),
        ("GET", "/api/config-preview") => json(&config_preview()),
        ("GET", "/health") => text("ok\n"),
        _ => bytes("404 Not Found", "text/plain; charset=utf-8", b"not found\n"),
    }
}

fn html(body: &str) -> Vec<u8> {
    bytes("200 OK", "text/html; charset=utf-8", body.as_bytes())
}

fn text(body: &str) -> Vec<u8> {
    bytes("200 OK", "text/plain; charset=utf-8", body.as_bytes())
}

fn json<T: Serialize>(body: &T) -> Vec<u8> {
    match serde_json::to_vec(body) {
        Ok(raw) => bytes("200 OK", "application/json; charset=utf-8", &raw),
        Err(err) => bytes(
            "500 Internal Server Error",
            "text/plain; charset=utf-8",
            format!("failed to serialize response: {err}\n").as_bytes(),
        ),
    }
}

fn bytes(status: &str, content_type: &str, body: &[u8]) -> Vec<u8> {
    let mut response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    response.extend_from_slice(body);
    response
}

#[derive(Debug, Serialize)]
struct EdgeStatus {
    node: &'static str,
    board: &'static str,
    os: &'static str,
    power_source: &'static str,
    metrics: Vec<EdgeMetric>,
    services: Vec<EdgeService>,
}

#[derive(Debug, Serialize)]
struct EdgeMetric {
    label: &'static str,
    value: &'static str,
    detail: &'static str,
}

#[derive(Debug, Serialize)]
struct EdgeService {
    id: &'static str,
    name: &'static str,
    stack: &'static str,
    port: &'static str,
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct ConfigPreview {
    node_role: &'static str,
    generated_files: Vec<GeneratedFile>,
    rollout_checks: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct GeneratedFile {
    path: &'static str,
    owner: &'static str,
    purpose: &'static str,
}

fn edge_status() -> EdgeStatus {
    EdgeStatus {
        node: "edge-a.regional-pilot-01",
        board: "Radxa ROCK 5B+ / RK3588 / 16-32GB RAM",
        os: "Debian or Armbian",
        power_source: "48 VDC rack bus to USB-C PD or PoE",
        metrics: vec![
            EdgeMetric {
                label: "Cache hit",
                value: "74%",
                detail: "Varnish/Nginx sample",
            },
            EdgeMetric {
                label: "Blocked",
                value: "128",
                detail: "CrowdSec + WAF today",
            },
            EdgeMetric {
                label: "Tunnel",
                value: "3",
                detail: "WireGuard origins",
            },
            EdgeMetric {
                label: "Power",
                value: "18 W",
                detail: "controller only",
            },
        ],
        services: edge_services(),
    }
}

fn edge_services() -> Vec<EdgeService> {
    vec![
        EdgeService {
            id: "edge_dns_authoritative",
            name: "Authoritative DNS",
            stack: "PowerDNS Authoritative + dnsdist",
            port: "53/tcp+udp",
            status: "ready",
        },
        EdgeService {
            id: "edge_reverse_proxy",
            name: "TLS reverse proxy",
            stack: "Caddy or Traefik",
            port: "80/443",
            status: "ready",
        },
        EdgeService {
            id: "edge_cache",
            name: "CDN cache",
            stack: "Varnish/Vinyl Cache or Nginx cache",
            port: "local",
            status: "ready",
        },
        EdgeService {
            id: "edge_waf",
            name: "WAF",
            stack: "OWASP Coraza + OWASP CRS",
            port: "http filter",
            status: "detection",
        },
        EdgeService {
            id: "edge_rate_limit",
            name: "Rate limit and abuse block",
            stack: "CrowdSec + nftables",
            port: "L3/L4/L7",
            status: "ready",
        },
        EdgeService {
            id: "edge_tunnel",
            name: "Origin tunnel",
            stack: "WireGuard + NetBird/OpenZiti",
            port: "udp",
            status: "ready",
        },
        EdgeService {
            id: "edge_access",
            name: "Zero-trust access",
            stack: "Keycloak + OPA + Authelia/Authentik",
            port: "443",
            status: "ready",
        },
        EdgeService {
            id: "edge_observability",
            name: "Observability",
            stack: "Prometheus + OpenTelemetry + Loki",
            port: "9100/4317",
            status: "ready",
        },
    ]
}

fn config_preview() -> ConfigPreview {
    ConfigPreview {
        node_role: "radxa-edge-gateway",
        generated_files: vec![
            GeneratedFile {
                path: "/etc/caddy/Caddyfile",
                owner: "caddy",
                purpose: "TLS reverse proxy routes and origin pools",
            },
            GeneratedFile {
                path: "/etc/powerdns/pdns.d/osdc.conf",
                owner: "pdns",
                purpose: "authoritative DNS backend and API settings",
            },
            GeneratedFile {
                path: "/etc/crowdsec/acquis.yaml",
                owner: "crowdsec",
                purpose: "log acquisition for WAF/proxy abuse detection",
            },
            GeneratedFile {
                path: "/etc/wireguard/osdc-edge.conf",
                owner: "root",
                purpose: "private origin tunnel",
            },
            GeneratedFile {
                path: "/etc/osdc-edge/policy.json",
                owner: "osdc-edge",
                purpose: "route cache WAF access and DDoS claim policy",
            },
        ],
        rollout_checks: vec![
            "validate generated configs",
            "confirm DNS secondary reachable",
            "run WAF in detection mode before blocking",
            "confirm tunnel key rotation",
            "confirm emergency bypass route",
            "confirm no volumetric DDoS claim without upstream plan",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn response_text(response: &[u8]) -> String {
        String::from_utf8_lossy(response).into_owned()
    }

    fn body(response: &[u8]) -> String {
        let text = response_text(response);
        text.split_once("\r\n\r\n")
            .map(|(_, body)| body.to_string())
            .unwrap_or_default()
    }

    fn json_body(path: &str) -> Value {
        let response = route_response("GET", path);
        assert!(response_text(&response).starts_with("HTTP/1.1 200 OK"));
        serde_json::from_str(&body(&response)).expect("route should return valid JSON")
    }

    #[test]
    fn serves_local_dashboard() {
        let response = route_response("GET", "/");
        let text = response_text(&response);

        assert!(text.starts_with("HTTP/1.1 200 OK"));
        assert!(text.contains("OSDC Edge Node"));
        assert!(text.contains("/api/config-preview"));
    }

    #[test]
    fn status_reports_radxa_node_and_edge_services() {
        let status = json_body("/api/status");
        let services = status["services"].as_array().unwrap();

        assert_eq!(status["node"], "edge-a.regional-pilot-01");
        assert_eq!(status["os"], "Debian or Armbian");
        assert!(services.len() >= 8);
        assert!(services.iter().any(|service| service["stack"]
            .as_str()
            .unwrap_or_default()
            .contains("PowerDNS")));
        assert!(services.iter().any(|service| service["stack"]
            .as_str()
            .unwrap_or_default()
            .contains("Coraza")));
    }

    #[test]
    fn config_preview_lists_generated_files_and_rollout_checks() {
        let config = json_body("/api/config-preview");

        assert_eq!(config["node_role"], "radxa-edge-gateway");
        assert!(config["generated_files"]
            .as_array()
            .unwrap()
            .iter()
            .any(|file| file["path"] == "/etc/caddy/Caddyfile"));
        assert!(config["generated_files"]
            .as_array()
            .unwrap()
            .iter()
            .any(|file| file["path"] == "/etc/wireguard/osdc-edge.conf"));
        assert!(config["rollout_checks"].as_array().unwrap().len() >= 6);
    }

    #[test]
    fn health_and_missing_routes_return_expected_status() {
        let health = response_text(&route_response("GET", "/health"));
        let missing = response_text(&route_response("GET", "/missing"));

        assert!(health.starts_with("HTTP/1.1 200 OK"));
        assert!(health.ends_with("ok\n"));
        assert!(missing.starts_with("HTTP/1.1 404 Not Found"));
    }
}
