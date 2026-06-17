use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

const STYLE_CSS: &str = include_str!("views/style.css");
const PORTAL_JS: &str = include_str!("views/portal.js");
const USER_HTML: &str = include_str!("views/user.html");
const OPERATOR_HTML: &str = include_str!("views/operator.html");
const EDGE_HTML: &str = include_str!("views/edge.html");
const PLANNER_HTML: &str = include_str!("views/planner.html");
const RACK_IMAGE: &[u8] = include_bytes!("../../../docs/assets/rack-thermal-spine-cutaway.png");
const EXTERIOR_IMAGE: &[u8] =
    include_bytes!("../../../docs/assets/prefab-panel-datacentre-exterior-02.png");
const EDGE_CADDYFILE: &str = include_str!("../../../examples/config-scripts/edge/Caddyfile");
const EDGE_POWERDNS_CONF: &str =
    include_str!("../../../examples/config-scripts/edge/pdns-osdc.conf");
const EDGE_CORAZA_CONF: &str =
    include_str!("../../../examples/config-scripts/edge/coraza-crs.conf");
const EDGE_CROWDSEC_ACQUIS: &str =
    include_str!("../../../examples/config-scripts/edge/crowdsec-acquis.yaml");
const EDGE_WIREGUARD_CONF: &str =
    include_str!("../../../examples/config-scripts/edge/wireguard-osdc-edge.conf");
const SOVEREIGN_SERVICE_CATALOGUE_CSV: &str =
    include_str!("../../../data/software/service-catalogue-v1.csv");
const COMMERCIAL_GAP_REGISTER_CSV: &str =
    include_str!("../../../data/commercial/commercial-gap-register.csv");
const STANDARDS_CONTROL_MATRIX_CSV: &str =
    include_str!("../../../data/commercial/standards-control-matrix.csv");
const SLA_CLASSES_CSV: &str = include_str!("../../../data/commercial/sla-classes.csv");
const COLOCATION_PRODUCTS_CSV: &str =
    include_str!("../../../data/commercial/colocation-products.csv");
const CROSS_CONNECT_PRODUCTS_CSV: &str =
    include_str!("../../../data/commercial/cross-connect-products.csv");
const REMOTE_HANDS_PRODUCTS_CSV: &str =
    include_str!("../../../data/commercial/remote-hands-products.csv");
const AUDIT_EVIDENCE_CSV: &str = include_str!("../../../data/commercial/audit-evidence.csv");

fn main() -> std::io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8787".to_string());
    let listener = TcpListener::bind(&addr)?;

    println!("osdc-portal listening on http://{addr}");
    println!("tenant portal: http://{addr}/user");
    println!("operator console: http://{addr}/operator");
    println!("edge shield console: http://{addr}/edge");
    println!("planning console: http://{addr}/planner");
    println!("catalog API: http://{addr}/api/catalog/hardware");

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
        ("GET", "/") => redirect("/user"),
        ("GET", "/user") => html(USER_HTML),
        ("GET", "/operator") => html(OPERATOR_HTML),
        ("GET", "/edge") => html(EDGE_HTML),
        ("GET", "/planner") => html(PLANNER_HTML),
        ("GET", "/styles.css") => bytes("200 OK", "text/css; charset=utf-8", STYLE_CSS.as_bytes()),
        ("GET", "/portal.js") => bytes(
            "200 OK",
            "text/javascript; charset=utf-8",
            PORTAL_JS.as_bytes(),
        ),
        ("GET", "/api/catalog/hardware") => json(&hardware_catalog()),
        ("GET", "/api/catalog/services") => json(&service_catalog()),
        ("GET", "/api/catalog/core-services") => json(&core_cloud_services()),
        ("GET", "/api/catalog/sovereign-services") => json(&sovereign_cloud_services()),
        ("GET", "/api/catalog/upgrade-policy") => json(&upgrade_policy()),
        ("GET", "/api/catalog/blueprints") => json(&provisioning_blueprints()),
        ("GET", "/api/config/scripts") => json(&config_scripts()),
        ("GET", "/api/edge/services") => json(&edge_shield_services()),
        ("GET", "/api/edge/status") => json(&edge_shield_status()),
        ("GET", "/api/edge/config-preview") => json(&edge_config_preview()),
        ("GET", "/api/provisioning/options") => json(&provisioning_options()),
        ("GET", "/api/provisioning/preview") => json(&provisioning_preview()),
        ("GET", "/api/tenant/summary") => json(&tenant_summary()),
        ("GET", "/api/operator/status") => json(&operator_status()),
        ("GET", "/api/cost/planning") => json(&cost_planning()),
        ("GET", "/api/cost/scenarios") => json(&cost_scenarios()),
        ("GET", "/api/cost/categories") => json(&cost_categories()),
        ("GET", "/api/cost/price-basis") => json(&price_basis()),
        ("GET", "/api/commercial/gaps") => json(&commercial_gaps()),
        ("GET", "/api/commercial/standards") => json(&commercial_standards()),
        ("GET", "/api/commercial/sla-classes") => json(&sla_classes()),
        ("GET", "/api/commercial/colocation-products") => json(&colocation_products()),
        ("GET", "/api/commercial/cross-connect-products") => json(&cross_connect_products()),
        ("GET", "/api/commercial/remote-hands-products") => json(&remote_hands_products()),
        ("GET", "/api/commercial/audit-evidence") => json(&audit_evidence()),
        ("GET", "/assets/rack-thermal-spine-cutaway.png") => {
            bytes("200 OK", "image/png", RACK_IMAGE)
        }
        ("GET", "/assets/prefab-panel-datacentre-exterior-02.png") => {
            bytes("200 OK", "image/png", EXTERIOR_IMAGE)
        }
        ("GET", "/health") => bytes("200 OK", "text/plain; charset=utf-8", b"ok\n"),
        _ => bytes("404 Not Found", "text/plain; charset=utf-8", b"not found\n"),
    }
}

fn html(body: &str) -> Vec<u8> {
    bytes("200 OK", "text/html; charset=utf-8", body.as_bytes())
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

fn csv_rows<T: DeserializeOwned>(raw: &str, source: &str) -> Vec<T> {
    csv::Reader::from_reader(raw.as_bytes())
        .deserialize::<T>()
        .map(|row| {
            row.unwrap_or_else(|err| panic!("embedded CSV {source} must deserialize: {err}"))
        })
        .collect()
}

fn redirect(location: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 302 Found\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    )
    .into_bytes()
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
struct HardwareProfile {
    id: &'static str,
    role: &'static str,
    hardware: &'static str,
    linux_stack: &'static str,
    accelerator: &'static str,
    memory: &'static str,
    power_w: u16,
    price_low_usd: u16,
    price_high_usd: u16,
    default_use: &'static str,
}

#[derive(Debug, Serialize)]
struct CloudService {
    domain: &'static str,
    user_service: &'static str,
    stack: &'static str,
    source_of_truth: &'static str,
    tenant_visible: bool,
}

#[derive(Debug, Serialize)]
struct CoreCloudService {
    id: &'static str,
    display_name: &'static str,
    priority: &'static str,
    aws_equivalent: &'static str,
    azure_equivalent: &'static str,
    open_source_stack: &'static str,
    tenant_visible: bool,
    operator_visible: bool,
    provisionable: bool,
    default_shape: &'static str,
    status: &'static str,
    status_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct SovereignCloudService {
    id: String,
    proprietary_service: String,
    open_equivalent: String,
    category: String,
    bundle: String,
    priority: String,
    maturity: String,
    ui_surface: String,
    upgrade_method: String,
    security_controls: String,
    workflow: String,
}

#[derive(Debug, Deserialize)]
struct SovereignCloudServiceRow {
    service_id: String,
    proprietary_service: String,
    open_equivalent: String,
    category: String,
    bundle: String,
    priority: String,
    ui_surface: String,
    upgrade_method: String,
    security_controls: String,
    workflow: String,
    maturity: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommercialGap {
    gap_id: String,
    domain: String,
    commercial_expectation: String,
    current_repo_state: String,
    priority: String,
    next_artifact: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StandardsControl {
    requirement_id: String,
    standard_family: String,
    control_area: String,
    applies: String,
    osdc_design_response: String,
    evidence_file: String,
    responsible_party: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlaClass {
    sla_class_id: String,
    service_scope: String,
    target: String,
    measurement_window: String,
    credit_model: String,
    exclusions: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColocationProduct {
    product_id: String,
    product_type: String,
    unit: String,
    default_commitment: String,
    required_controls: String,
    demarcation: String,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrossConnectProduct {
    product_id: String,
    product_type: String,
    media: String,
    demarcation: String,
    workflow: String,
    required_evidence: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RemoteHandsProduct {
    product_id: String,
    task_class: String,
    response_target: String,
    requires_approval: String,
    scope_boundary: String,
    required_evidence: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuditEvidence {
    evidence_id: String,
    domain: String,
    evidence_name: String,
    evidence_path: String,
    owner: String,
    cadence: String,
    status: String,
}

impl From<SovereignCloudServiceRow> for SovereignCloudService {
    fn from(row: SovereignCloudServiceRow) -> Self {
        Self {
            id: row.service_id,
            proprietary_service: row.proprietary_service,
            open_equivalent: row.open_equivalent,
            category: row.category,
            bundle: row.bundle,
            priority: row.priority,
            maturity: row.maturity,
            ui_surface: row.ui_surface,
            upgrade_method: row.upgrade_method,
            security_controls: row.security_controls,
            workflow: row.workflow,
        }
    }
}

#[derive(Debug, Serialize)]
struct UpgradePolicy {
    update_class: &'static str,
    frequency: &'static str,
    target_window: &'static str,
    required_gates: &'static str,
    approval_owner: &'static str,
    rollback_requirement: &'static str,
}

#[derive(Debug, Serialize)]
struct ConfigScript {
    id: &'static str,
    tool: &'static str,
    path: &'static str,
    owner: &'static str,
    language: &'static str,
    edit_mode: &'static str,
    validation_command: &'static str,
    rollout_target: &'static str,
    risk: &'static str,
    notes: &'static str,
    content: &'static str,
}

#[derive(Debug, Serialize)]
struct ProvisioningBlueprint {
    id: &'static str,
    display_name: &'static str,
    service_id: &'static str,
    default_shape: &'static str,
    api_surface: &'static str,
    backing_stack: &'static str,
    operator_checks: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct ProvisioningPreview {
    request_id: &'static str,
    service_id: &'static str,
    shape: &'static str,
    linux_image: &'static str,
    backing_stack: &'static str,
    estimated_power_w: u16,
    estimated_monthly_usd: u16,
    operator_checks: Vec<&'static str>,
    next_api_path: &'static str,
}

#[derive(Debug, Serialize)]
struct TenantSummary {
    metrics: Vec<Metric>,
    site_flow: Vec<FlowStep>,
    services: Vec<ServiceTile>,
    resources: Vec<TenantResource>,
    core_services: Vec<CoreCloudService>,
}

#[derive(Debug, Serialize)]
struct OperatorStatus {
    metrics: Vec<Metric>,
    power_flow: Vec<FlowStep>,
    thermal_flow: Vec<FlowStep>,
    hardware_pools: Vec<HardwarePool>,
    cloud_stack: Vec<StackHealth>,
    core_services: Vec<CoreCloudService>,
    operations: Vec<OperationItem>,
}

#[derive(Debug, Serialize)]
struct ProvisioningOptions {
    services: Vec<&'static str>,
    shapes: Vec<ShapeOption>,
    linux_images: Vec<&'static str>,
    storage: Vec<&'static str>,
    networks: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct ShapeOption {
    id: &'static str,
    label: &'static str,
    hardware_profile: &'static str,
    power_class: &'static str,
}

#[derive(Debug, Serialize)]
struct Metric {
    label: &'static str,
    value: &'static str,
    detail: &'static str,
    kind: &'static str,
}

#[derive(Debug, Serialize)]
struct FlowStep {
    label: &'static str,
    value: &'static str,
}

#[derive(Debug, Serialize)]
struct ServiceTile {
    name: &'static str,
    stack: &'static str,
}

#[derive(Debug, Serialize)]
struct TenantResource {
    name: &'static str,
    resource_type: &'static str,
    backing_stack: &'static str,
    power_class: &'static str,
    status: &'static str,
    status_kind: &'static str,
    monthly_estimate: &'static str,
}

#[derive(Debug, Serialize)]
struct HardwarePool {
    pool: &'static str,
    hardware: &'static str,
    power: &'static str,
    allocated: &'static str,
    status: &'static str,
    status_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct StackHealth {
    domain: &'static str,
    stack: &'static str,
    health: &'static str,
    health_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct OperationItem {
    time: &'static str,
    system: &'static str,
    action: &'static str,
    owner: &'static str,
    risk: &'static str,
    status: &'static str,
    status_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct EdgeShieldService {
    id: &'static str,
    cloudflare_equivalent: &'static str,
    open_source_stack: &'static str,
    radxa_role: &'static str,
    criticality: &'static str,
    protocol: &'static str,
    status: &'static str,
    status_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct EdgeShieldStatus {
    metrics: Vec<Metric>,
    services: Vec<EdgeShieldService>,
    nodes: Vec<EdgeNode>,
    rollout: Vec<OperationItem>,
}

#[derive(Debug, Serialize)]
struct EdgeNode {
    name: &'static str,
    board: &'static str,
    role: &'static str,
    power: &'static str,
    status: &'static str,
    status_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct CostPlanning {
    metrics: Vec<Metric>,
    scenarios: Vec<CostScenario>,
    categories: Vec<CostCategory>,
    price_basis: Vec<PriceBasis>,
}

#[derive(Debug, Serialize)]
struct CostScenario {
    id: &'static str,
    name: &'static str,
    it_load_kw: u32,
    racks: u16,
    building_area_m2: u32,
    core_facility_low_usd: u32,
    core_facility_high_usd: u32,
    starter_it_low_usd: u32,
    starter_it_high_usd: u32,
    total_with_it_low_usd: u32,
    total_with_it_high_usd: u32,
    build_time_low_weeks: u16,
    build_time_high_weeks: u16,
    default_building_system: &'static str,
    notes: &'static str,
}

#[derive(Debug, Serialize)]
struct CostCategory {
    scenario_id: &'static str,
    category: &'static str,
    low_usd: u32,
    high_usd: u32,
    notes: &'static str,
}

#[derive(Debug, Serialize)]
struct PriceBasis {
    item_family: &'static str,
    unit: &'static str,
    low_usd: f64,
    high_usd: f64,
    planning_selected_usd: f64,
    source_marketplace: &'static str,
    project_use: &'static str,
}

#[derive(Debug, Serialize)]
struct EdgeConfigPreview {
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

fn hardware_catalog() -> Vec<HardwareProfile> {
    vec![
        HardwareProfile {
            id: "sbc.rk3588.control",
            role: "facility_gateway",
            hardware: "Radxa ROCK 5B+ RK3588 16-32GB",
            linux_stack: "Debian or Armbian",
            accelerator: "RK3588 NPU 6 TOPS",
            memory: "16-32GB system RAM",
            power_w: 30,
            price_low_usd: 140,
            price_high_usd: 260,
            default_use: "facility telemetry, OT gateways, local console, edge management",
        },
        HardwareProfile {
            id: "sbc.rk3588.edge",
            role: "tenant_edge",
            hardware: "Radxa ROCK 5B+ or Orange Pi 5 Plus RK3588",
            linux_stack: "Debian or Armbian",
            accelerator: "RK3588 NPU 6 TOPS",
            memory: "16-32GB system RAM",
            power_w: 30,
            price_low_usd: 120,
            price_high_usd: 240,
            default_use: "low-power tenant services and small edge clusters",
        },
        HardwareProfile {
            id: "gpu.rocm.16g.default",
            role: "tenant_gpu",
            hardware: "AMD Radeon RX 9060 XT 16GB",
            linux_stack: "x86 Linux with Mesa, AMDGPU, and ROCm",
            accelerator: "ROCm, HIP, OpenCL, Vulkan",
            memory: "16GB GDDR6",
            power_w: 160,
            price_low_usd: 350,
            price_high_usd: 430,
            default_use: "default open-driver GPU instance and AI inference lane",
        },
        HardwareProfile {
            id: "gpu.xpu.16g.lowpower",
            role: "tenant_gpu_low_power",
            hardware: "Intel Arc Pro B50 16GB",
            linux_stack: "x86 Linux with Mesa, Xe driver, Level Zero, OpenCL",
            accelerator: "oneAPI Level Zero, OpenCL, OpenVINO",
            memory: "16GB GDDR6",
            power_w: 70,
            price_low_usd: 350,
            price_high_usd: 450,
            default_use: "low-power GPU visualization, media, and inference lane",
        },
    ]
}

fn service_catalog() -> Vec<CloudService> {
    vec![
        CloudService {
            domain: "compute",
            user_service: "VM instances, images, keypairs, security groups",
            stack: "OpenStack Nova, Glance, Neutron, KVM, libvirt",
            source_of_truth: "OpenStack and NetBox",
            tenant_visible: true,
        },
        CloudService {
            domain: "bare_metal",
            user_service: "Dedicated servers and GPU nodes",
            stack: "OpenStack Ironic, Metal3, Redfish, OpenBMC",
            source_of_truth: "NetBox and Ironic",
            tenant_visible: true,
        },
        CloudService {
            domain: "object_storage",
            user_service: "Buckets, lifecycle, S3-compatible API",
            stack: "Ceph RGW",
            source_of_truth: "Ceph",
            tenant_visible: true,
        },
        CloudService {
            domain: "kubernetes",
            user_service: "Managed clusters and node pools",
            stack: "Cluster API, Metal3, Cilium, Talos or kubeadm",
            source_of_truth: "Kubernetes and GitOps",
            tenant_visible: true,
        },
        CloudService {
            domain: "ai_serving",
            user_service: "Model endpoints and GPU queues",
            stack: "KServe, vLLM, SGLang, llama.cpp, Kueue",
            source_of_truth: "Kubernetes, Kueue, and model registry",
            tenant_visible: true,
        },
        CloudService {
            domain: "dcim",
            user_service: "Racks, devices, circuits, IPAM",
            stack: "NetBox, openDCIM",
            source_of_truth: "NetBox",
            tenant_visible: false,
        },
    ]
}

fn edge_shield_services() -> Vec<EdgeShieldService> {
    vec![
        EdgeShieldService {
            id: "edge_dns_authoritative",
            cloudflare_equivalent: "Cloudflare DNS",
            open_source_stack: "PowerDNS Authoritative, dnsdist",
            radxa_role: "authoritative DNS edge and DNSSEC host",
            criticality: "critical",
            protocol: "DNS 53 TCP/UDP",
            status: "ready",
            status_kind: "normal",
        },
        EdgeShieldService {
            id: "edge_reverse_proxy",
            cloudflare_equivalent: "Cloudflare CDN proxy",
            open_source_stack: "Caddy, Envoy, HAProxy, Traefik",
            radxa_role: "TLS termination and origin routing",
            criticality: "critical",
            protocol: "HTTP 80 HTTPS 443",
            status: "ready",
            status_kind: "normal",
        },
        EdgeShieldService {
            id: "edge_cache",
            cloudflare_equivalent: "Cloudflare CDN cache",
            open_source_stack: "Varnish/Vinyl Cache or Nginx cache",
            radxa_role: "static and API response cache",
            criticality: "critical",
            protocol: "HTTP local",
            status: "ready",
            status_kind: "normal",
        },
        EdgeShieldService {
            id: "edge_waf",
            cloudflare_equivalent: "Cloudflare WAF",
            open_source_stack: "OWASP Coraza with OWASP CRS",
            radxa_role: "request inspection and blocking",
            criticality: "critical",
            protocol: "HTTP filter",
            status: "detection",
            status_kind: "info",
        },
        EdgeShieldService {
            id: "edge_rate_limit",
            cloudflare_equivalent: "Rate limiting and DDoS rules",
            open_source_stack: "CrowdSec, nftables, HAProxy stick tables",
            radxa_role: "abuse detection and local blocking",
            criticality: "critical",
            protocol: "L3/L4/L7",
            status: "ready",
            status_kind: "normal",
        },
        EdgeShieldService {
            id: "edge_load_balance",
            cloudflare_equivalent: "Cloudflare Load Balancing",
            open_source_stack: "HAProxy, Traefik, Envoy, dnsdist",
            radxa_role: "origin failover and regional routing",
            criticality: "critical",
            protocol: "HTTP TCP DNS",
            status: "ready",
            status_kind: "normal",
        },
        EdgeShieldService {
            id: "edge_tunnel",
            cloudflare_equivalent: "Cloudflare Tunnel",
            open_source_stack: "WireGuard, Headscale, NetBird, OpenZiti",
            radxa_role: "private origin tunnel endpoint",
            criticality: "important",
            protocol: "WireGuard UDP",
            status: "ready",
            status_kind: "normal",
        },
        EdgeShieldService {
            id: "edge_access",
            cloudflare_equivalent: "Cloudflare Access",
            open_source_stack: "Keycloak, OPA, Authelia, Authentik",
            radxa_role: "zero-trust app access",
            criticality: "critical",
            protocol: "HTTPS OIDC",
            status: "ready",
            status_kind: "normal",
        },
        EdgeShieldService {
            id: "edge_functions",
            cloudflare_equivalent: "Cloudflare Workers",
            open_source_stack: "WASI, Wasmtime, Spin, wasmCloud",
            radxa_role: "small edge functions and transforms",
            criticality: "optional",
            protocol: "WASI HTTP",
            status: "preview",
            status_kind: "info",
        },
        EdgeShieldService {
            id: "edge_observability",
            cloudflare_equivalent: "Cloudflare Analytics and logs",
            open_source_stack: "Prometheus, OpenTelemetry, Loki, Grafana",
            radxa_role: "node and service telemetry",
            criticality: "critical",
            protocol: "Prometheus 9100 OTLP",
            status: "ready",
            status_kind: "normal",
        },
    ]
}

fn edge_shield_status() -> EdgeShieldStatus {
    EdgeShieldStatus {
        metrics: vec![
            Metric {
                label: "Radxa nodes",
                value: "3",
                detail: "edge-a/b/c",
                kind: "normal",
            },
            Metric {
                label: "Cache hit",
                value: "74%",
                detail: "Varnish/Nginx",
                kind: "normal",
            },
            Metric {
                label: "Blocked today",
                value: "128",
                detail: "CrowdSec + WAF",
                kind: "normal",
            },
            Metric {
                label: "Power draw",
                value: "54 W",
                detail: "3 boards",
                kind: "normal",
            },
        ],
        services: edge_shield_services(),
        nodes: vec![
            EdgeNode {
                name: "edge-a",
                board: "Radxa ROCK 5B+",
                role: "primary DNS/cache/proxy",
                power: "18 W",
                status: "ready",
                status_kind: "normal",
            },
            EdgeNode {
                name: "edge-b",
                board: "Radxa ROCK 5B+",
                role: "secondary DNS/cache/proxy",
                power: "17 W",
                status: "ready",
                status_kind: "normal",
            },
            EdgeNode {
                name: "edge-c",
                board: "Radxa ROCK 5B+",
                role: "management and failover",
                power: "19 W",
                status: "standby",
                status_kind: "info",
            },
        ],
        rollout: vec![
            OperationItem {
                time: "08:30",
                system: "DNS",
                action: "PowerDNS zone serial audit",
                owner: "edge",
                risk: "low",
                status: "complete",
                status_kind: "normal",
            },
            OperationItem {
                time: "09:10",
                system: "WAF",
                action: "Coraza CRS update in detection mode",
                owner: "security",
                risk: "medium",
                status: "testing",
                status_kind: "info",
            },
            OperationItem {
                time: "10:00",
                system: "Tunnel",
                action: "WireGuard key rotation",
                owner: "network",
                risk: "medium",
                status: "scheduled",
                status_kind: "warn",
            },
        ],
    }
}

fn edge_config_preview() -> EdgeConfigPreview {
    EdgeConfigPreview {
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
                path: "/etc/prometheus/file_sd/osdc-edge.json",
                owner: "prometheus",
                purpose: "edge service scrape targets",
            },
        ],
        rollout_checks: vec![
            "validate generated configs",
            "confirm DNS secondary reachable",
            "run WAF in detection mode before blocking",
            "confirm tunnel key rotation",
            "confirm emergency bypass route",
            "record rollback file hashes",
        ],
    }
}

fn cost_planning() -> CostPlanning {
    CostPlanning {
        metrics: vec![
            Metric {
                label: "Scale scenarios",
                value: "4",
                detail: "50 kW to 5 MW IT",
                kind: "normal",
            },
            Metric {
                label: "Total range",
                value: "$420k-$66.5M",
                detail: "facility plus starter IT",
                kind: "normal",
            },
            Metric {
                label: "Fastest build",
                value: "8-14 wk",
                detail: "edge micro",
                kind: "normal",
            },
            Metric {
                label: "Power baseline",
                value: "DC",
                detail: "solar sodium-ion microgrid",
                kind: "info",
            },
        ],
        scenarios: cost_scenarios(),
        categories: cost_categories(),
        price_basis: price_basis(),
    }
}

fn cost_scenarios() -> Vec<CostScenario> {
    vec![
        CostScenario {
            id: "S1",
            name: "Edge micro",
            it_load_kw: 50,
            racks: 4,
            building_area_m2: 120,
            core_facility_low_usd: 340_000,
            core_facility_high_usd: 580_000,
            starter_it_low_usd: 80_000,
            starter_it_high_usd: 220_000,
            total_with_it_low_usd: 420_000,
            total_with_it_high_usd: 800_000,
            build_time_low_weeks: 8,
            build_time_high_weeks: 14,
            default_building_system:
                "insulated sandwich panel building with solar sodium-ion DC microgrid bus",
            notes: "excludes land, major utility upgrade, and fibre buildout",
        },
        CostScenario {
            id: "S2",
            name: "Regional pilot",
            it_load_kw: 250,
            racks: 10,
            building_area_m2: 300,
            core_facility_low_usd: 1_020_000,
            core_facility_high_usd: 1_920_000,
            starter_it_low_usd: 180_000,
            starter_it_high_usd: 550_000,
            total_with_it_low_usd: 1_200_000,
            total_with_it_high_usd: 2_470_000,
            build_time_low_weeks: 14,
            build_time_high_weeks: 24,
            default_building_system:
                "insulated panel hall with service trench and solar sodium-ion DC microgrid bus",
            notes: "includes adsorption chiller as optional pilot allowance",
        },
        CostScenario {
            id: "S3",
            name: "Regional production",
            it_load_kw: 1_000,
            racks: 40,
            building_area_m2: 1_000,
            core_facility_low_usd: 3_650_000,
            core_facility_high_usd: 6_650_000,
            starter_it_low_usd: 900_000,
            starter_it_high_usd: 3_200_000,
            total_with_it_low_usd: 4_550_000,
            total_with_it_high_usd: 9_850_000,
            build_time_low_weeks: 28,
            build_time_high_weeks: 44,
            default_building_system:
                "multi-zone insulated panel halls with solar sodium-ion DC microgrid bus",
            notes: "requires stronger local engineering and authority review",
        },
        CostScenario {
            id: "S4",
            name: "National AI-ready",
            it_load_kw: 5_000,
            racks: 160,
            building_area_m2: 4_500,
            core_facility_low_usd: 18_500_000,
            core_facility_high_usd: 36_500_000,
            starter_it_low_usd: 5_000_000,
            starter_it_high_usd: 30_000_000,
            total_with_it_low_usd: 23_500_000,
            total_with_it_high_usd: 66_500_000,
            build_time_low_weeks: 52,
            build_time_high_weeks: 90,
            default_building_system:
                "large insulated panel campus with solar sodium-ion DC microgrid blocks",
            notes: "marketplace sourcing is useful for commodities only at this scale",
        },
    ]
}

fn cost_categories() -> Vec<CostCategory> {
    vec![
        CostCategory {
            scenario_id: "S1",
            category: "panel_building_civil_trench_security_shell",
            low_usd: 70_000,
            high_usd: 130_000,
            notes: "120 m2 insulated panel shell with service trench and simple perimeter",
        },
        CostCategory {
            scenario_id: "S1",
            category: "dc_microgrid_solar_sodium_single_fallback_power",
            low_usd: 115_000,
            high_usd: 190_000,
            notes: "75 kWp PV, 150 kWh sodium-ion BESS, 75 kW DC microgrid converters, 380-400 VDC bus, 48 V racks, and one fallback generator path",
        },
        CostCategory {
            scenario_id: "S1",
            category: "rack_thermal_spine_cooling",
            low_usd: 70_000,
            high_usd: 140_000,
            notes: "4 rear-door heat exchangers, small pump skid, dry cooler, backup chiller, and controls",
        },
        CostCategory {
            scenario_id: "S1",
            category: "fire_access_cctv",
            low_usd: 30_000,
            high_usd: 65_000,
            notes: "local-code approved detection, suppression, and basic security",
        },
        CostCategory {
            scenario_id: "S1",
            category: "racks_cabling_basic_network",
            low_usd: 35_000,
            high_usd: 75_000,
            notes: "4 racks, DC rack power, OOB, and modest switching",
        },
        CostCategory {
            scenario_id: "S1",
            category: "commissioning_spares_tools_docs",
            low_usd: 45_000,
            high_usd: 90_000,
            notes: "load testing, spares, operator tools, and runbooks",
        },
        CostCategory {
            scenario_id: "S2",
            category: "panel_building_civil_trench_perimeter",
            low_usd: 160_000,
            high_usd: 300_000,
            notes: "300 m2 panel hall with electrical room and thermal plant room",
        },
        CostCategory {
            scenario_id: "S2",
            category: "dc_microgrid_solar_sodium_single_fallback_power",
            low_usd: 430_000,
            high_usd: 720_000,
            notes: "300 kWp PV, 500 kWh sodium-ion BESS, 350 kW DC converters, 380-400 VDC bus, 48 V racks, and one fallback generator path",
        },
        CostCategory {
            scenario_id: "S2",
            category: "rack_thermal_spine_and_backup_cooling",
            low_usd: 230_000,
            high_usd: 470_000,
            notes: "10 rear-door heat exchangers, thermal spine, dry cooler, chiller, and adsorption pilot allowance",
        },
        CostCategory {
            scenario_id: "S2",
            category: "fire_security_monitoring",
            low_usd: 70_000,
            high_usd: 150_000,
            notes: "aspirating detection, fire alarm, suppression, access, CCTV, and monitoring",
        },
        CostCategory {
            scenario_id: "S2",
            category: "racks_structured_cabling_network",
            low_usd: 90_000,
            high_usd: 220_000,
            notes: "10 racks with 25G/100G-capable network and cabling",
        },
        CostCategory {
            scenario_id: "S2",
            category: "commissioning_spares_tools_docs",
            low_usd: 90_000,
            high_usd: 180_000,
            notes: "integrated systems testing, spares, local runbooks, and training",
        },
        CostCategory {
            scenario_id: "S3",
            category: "panel_building_civil_roads_perimeter_trenches",
            low_usd: 550_000,
            high_usd: 1_100_000,
            notes: "1000 m2 multi-zone panel facility and plant yard",
        },
        CostCategory {
            scenario_id: "S3",
            category: "dc_microgrid_solar_sodium_single_fallback_power",
            low_usd: 1_450_000,
            high_usd: 2_550_000,
            notes: "1.2 MWp PV, 2 MWh sodium-ion BESS, 1.4 MW DC converters, HVDC distribution, and single fallback plant",
        },
        CostCategory {
            scenario_id: "S3",
            category: "cooling_plant_rack_heat_capture_thermal_spine",
            low_usd: 850_000,
            high_usd: 1_700_000,
            notes: "40 racks, multiple dry coolers, chillers, thermal spine zones, and pump skids",
        },
        CostCategory {
            scenario_id: "S3",
            category: "fire_security_monitoring",
            low_usd: 180_000,
            high_usd: 360_000,
            notes: "multi-zone life safety and security systems",
        },
        CostCategory {
            scenario_id: "S3",
            category: "racks_cabling_network_fabric",
            low_usd: 380_000,
            high_usd: 750_000,
            notes: "40 racks, redundant fabric, optics allowance, and structured cabling",
        },
        CostCategory {
            scenario_id: "S3",
            category: "commissioning_spares_tools_docs",
            low_usd: 240_000,
            high_usd: 520_000,
            notes: "load banks, spares, procedures, and training",
        },
        CostCategory {
            scenario_id: "S4",
            category: "panel_buildings_civil_campus_security",
            low_usd: 2_500_000,
            high_usd: 5_000_000,
            notes: "large panel campus with roads, fencing, plant yards, and security perimeter",
        },
        CostCategory {
            scenario_id: "S4",
            category: "dc_microgrid_solar_sodium_single_fallback_power",
            low_usd: 7_500_000,
            high_usd: 14_500_000,
            notes: "5 MWp PV, 10 MWh sodium-ion BESS, 6 MW DC converter blocks, MV boundary gear, HVDC distribution, and single fallback plant",
        },
        CostCategory {
            scenario_id: "S4",
            category: "cooling_plant_thermal_spine_zones",
            low_usd: 4_000_000,
            high_usd: 8_500_000,
            notes: "multiple cooling zones, dry coolers, backup chillers, liquid cooling, and thermal spine plant",
        },
        CostCategory {
            scenario_id: "S4",
            category: "fire_security_monitoring",
            low_usd: 700_000,
            high_usd: 1_600_000,
            notes: "campus-scale life safety and security systems",
        },
        CostCategory {
            scenario_id: "S4",
            category: "racks_cabling_network_fabric",
            low_usd: 2_000_000,
            high_usd: 4_500_000,
            notes: "160 racks, AI-ready network, optics, and structured cabling",
        },
        CostCategory {
            scenario_id: "S4",
            category: "commissioning_spares_tools_docs",
            low_usd: 1_800_000,
            high_usd: 3_500_000,
            notes: "full integrated systems testing, spares, documentation, and training",
        },
    ]
}

fn price_basis() -> Vec<PriceBasis> {
    vec![
        PriceBasis {
            item_family: "100mm_pu_pir_panel",
            unit: "m2_fob",
            low_usd: 8.0,
            high_usd: 18.0,
            planning_selected_usd: 12.0,
            source_marketplace: "Alibaba",
            project_use: "fast insulated building envelope",
        },
        PriceBasis {
            item_family: "installed_panel_envelope",
            unit: "m2_installed",
            low_usd: 35.0,
            high_usd: 90.0,
            planning_selected_usd: 55.0,
            source_marketplace: "Derived",
            project_use: "scenario building shell",
        },
        PriceBasis {
            item_family: "prefab_steel_frame",
            unit: "m2_fob",
            low_usd: 45.0,
            high_usd: 100.0,
            planning_selected_usd: 65.0,
            source_marketplace: "Alibaba/China prefab market",
            project_use: "fast panel building structure",
        },
        PriceBasis {
            item_family: "concrete_plinth_and_floor",
            unit: "m2_local",
            low_usd: 100.0,
            high_usd: 240.0,
            planning_selected_usd: 160.0,
            source_marketplace: "Derived",
            project_use: "building foundation",
        },
        PriceBasis {
            item_family: "42u_server_rack",
            unit: "each_landed",
            low_usd: 450.0,
            high_usd: 900.0,
            planning_selected_usd: 650.0,
            source_marketplace: "Alibaba",
            project_use: "rack structure",
        },
        PriceBasis {
            item_family: "dc_rack_power_shelf_or_pdu",
            unit: "each_landed",
            low_usd: 300.0,
            high_usd: 900.0,
            planning_selected_usd: 450.0,
            source_marketplace: "Alibaba/OCP-style suppliers",
            project_use: "rack DC power",
        },
        PriceBasis {
            item_family: "rear_door_heat_exchanger",
            unit: "each_landed",
            low_usd: 1800.0,
            high_usd: 6500.0,
            planning_selected_usd: 2800.0,
            source_marketplace: "Alibaba",
            project_use: "rack heat capture",
        },
        PriceBasis {
            item_family: "thermal_spine_pipework",
            unit: "m_installed",
            low_usd: 50.0,
            high_usd: 180.0,
            planning_selected_usd: 85.0,
            source_marketplace: "Derived",
            project_use: "warm-water thermal spine",
        },
        PriceBasis {
            item_family: "dry_cooler_250kw",
            unit: "each_installed",
            low_usd: 12_000.0,
            high_usd: 45_000.0,
            planning_selected_usd: 35_000.0,
            source_marketplace: "Alibaba",
            project_use: "primary heat rejection",
        },
        PriceBasis {
            item_family: "backup_chiller_250kw",
            unit: "each_installed",
            low_usd: 20_000.0,
            high_usd: 70_000.0,
            planning_selected_usd: 40_000.0,
            source_marketplace: "Alibaba",
            project_use: "backup cooling",
        },
        PriceBasis {
            item_family: "adsorption_chiller_pilot",
            unit: "each_installed",
            low_usd: 40_000.0,
            high_usd: 140_000.0,
            planning_selected_usd: 60_000.0,
            source_marketplace: "RFQ/Derived",
            project_use: "heat-to-cooling pilot",
        },
        PriceBasis {
            item_family: "sodium_ion_bess",
            unit: "kwh_installed",
            low_usd: 200.0,
            high_usd: 420.0,
            planning_selected_usd: 230.0,
            source_marketplace: "Alibaba/Derived",
            project_use: "critical ride-through and solar shifting",
        },
        PriceBasis {
            item_family: "dc_microgrid_converter_controller",
            unit: "kw_installed",
            low_usd: 85.0,
            high_usd: 210.0,
            planning_selected_usd: 135.0,
            source_marketplace: "Alibaba/Derived",
            project_use: "380-400 VDC bus control",
        },
        PriceBasis {
            item_family: "ac_boundary_rectifier_inverter",
            unit: "kw_installed",
            low_usd: 55.0,
            high_usd: 150.0,
            planning_selected_usd: 95.0,
            source_marketplace: "Alibaba/Derived",
            project_use: "boundary adapter",
        },
        PriceBasis {
            item_family: "hvdc_distribution_protection",
            unit: "kw_installed",
            low_usd: 25.0,
            high_usd: 90.0,
            planning_selected_usd: 45.0,
            source_marketplace: "Derived",
            project_use: "DC distribution safety",
        },
        PriceBasis {
            item_family: "solar_pv_system",
            unit: "w_installed",
            low_usd: 0.55,
            high_usd: 0.95,
            planning_selected_usd: 0.65,
            source_marketplace: "Alibaba/Derived",
            project_use: "core solar input",
        },
        PriceBasis {
            item_family: "25g_leaf_switch",
            unit: "each_landed",
            low_usd: 2500.0,
            high_usd: 9000.0,
            planning_selected_usd: 5500.0,
            source_marketplace: "Alibaba",
            project_use: "network fabric",
        },
        PriceBasis {
            item_family: "100g_spine_switch",
            unit: "each_landed",
            low_usd: 6500.0,
            high_usd: 15_000.0,
            planning_selected_usd: 9000.0,
            source_marketplace: "Alibaba/SONiC vendors",
            project_use: "network fabric",
        },
        PriceBasis {
            item_family: "standard_compute_server",
            unit: "each_landed",
            low_usd: 1400.0,
            high_usd: 3500.0,
            planning_selected_usd: 1800.0,
            source_marketplace: "Alibaba/used market",
            project_use: "compute nodes",
        },
        PriceBasis {
            item_family: "storage_server_node",
            unit: "each_landed",
            low_usd: 5000.0,
            high_usd: 15_000.0,
            planning_selected_usd: 8000.0,
            source_marketplace: "Alibaba",
            project_use: "Ceph storage",
        },
        PriceBasis {
            item_family: "gpu_server_pilot",
            unit: "each_landed",
            low_usd: 15_000.0,
            high_usd: 50_000.0,
            planning_selected_usd: 20_000.0,
            source_marketplace: "Alibaba",
            project_use: "AI pilot",
        },
        PriceBasis {
            item_family: "commissioning_load_bank",
            unit: "lot",
            low_usd: 8000.0,
            high_usd: 80_000.0,
            planning_selected_usd: 25_000.0,
            source_marketplace: "Rental/Derived",
            project_use: "integrated systems testing",
        },
    ]
}

fn core_cloud_services() -> Vec<CoreCloudService> {
    vec![
        CoreCloudService {
            id: "identity",
            display_name: "Identity and projects",
            priority: "foundation",
            aws_equivalent: "IAM, Organizations",
            azure_equivalent: "Microsoft Entra ID, resource groups",
            open_source_stack: "Keycloak, OPA, OpenStack Keystone",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "project.small",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "compute_vm",
            display_name: "Virtual machines",
            priority: "foundation",
            aws_equivalent: "EC2",
            azure_equivalent: "Azure Virtual Machines",
            open_source_stack: "OpenStack Nova, KVM, libvirt",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "cpu.standard",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "compute_bare_metal",
            display_name: "Bare metal and GPU nodes",
            priority: "foundation",
            aws_equivalent: "EC2 bare metal, accelerated instances",
            azure_equivalent: "Bare metal and GPU VM families",
            open_source_stack: "OpenStack Ironic, Metal3, Redfish, Kubernetes device plugins",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "gpu-open.1x16g",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "object_storage",
            display_name: "Object storage",
            priority: "foundation",
            aws_equivalent: "S3",
            azure_equivalent: "Blob Storage",
            open_source_stack: "Ceph RGW",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "bucket.standard",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "block_storage",
            display_name: "Block volumes",
            priority: "foundation",
            aws_equivalent: "EBS",
            azure_equivalent: "Managed Disks",
            open_source_stack: "Ceph RBD, OpenStack Cinder",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "volume.250gb",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "networking",
            display_name: "Private networking",
            priority: "foundation",
            aws_equivalent: "VPC, subnets, security groups",
            azure_equivalent: "Virtual Network, NSG",
            open_source_stack: "OpenStack Neutron, OVN, FRRouting, nftables",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "network.private",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "load_dns",
            display_name: "Load balancing and DNS",
            priority: "foundation",
            aws_equivalent: "ELB, Route 53",
            azure_equivalent: "Load Balancer, Application Gateway, Azure DNS",
            open_source_stack: "Octavia, HAProxy, Envoy, PowerDNS, Designate",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "public-api.standard",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "kubernetes",
            display_name: "Managed Kubernetes",
            priority: "foundation",
            aws_equivalent: "EKS",
            azure_equivalent: "AKS",
            open_source_stack: "Cluster API, Metal3, Cilium, Talos/kubeadm",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "k8s.standard",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "serverless",
            display_name: "Serverless and app hosting",
            priority: "important",
            aws_equivalent: "Lambda, ECS/Fargate, App Runner",
            azure_equivalent: "Functions, Container Apps, App Service",
            open_source_stack: "Knative, Kubernetes, KEDA",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "function.small",
            status: "preview",
            status_kind: "info",
        },
        CoreCloudService {
            id: "database",
            display_name: "Managed databases",
            priority: "foundation",
            aws_equivalent: "RDS, DynamoDB, ElastiCache",
            azure_equivalent: "Azure SQL, Cosmos DB, Cache for Redis",
            open_source_stack: "CloudNativePG, MariaDB/Percona operators, Valkey, FerretDB",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "postgres.small",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "messaging",
            display_name: "Messaging and events",
            priority: "important",
            aws_equivalent: "SQS, SNS, EventBridge, MSK",
            azure_equivalent: "Service Bus, Event Grid, Event Hubs",
            open_source_stack: "NATS, Kafka/Strimzi, RabbitMQ",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "queue.standard",
            status: "preview",
            status_kind: "info",
        },
        CoreCloudService {
            id: "secrets",
            display_name: "Secrets, keys, certificates",
            priority: "foundation",
            aws_equivalent: "Secrets Manager, KMS, ACM",
            azure_equivalent: "Key Vault",
            open_source_stack: "OpenBao, cert-manager, Smallstep, Barbican",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "secrets.project",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "observability",
            display_name: "Observability and audit",
            priority: "foundation",
            aws_equivalent: "CloudWatch, CloudTrail",
            azure_equivalent: "Azure Monitor, Log Analytics",
            open_source_stack: "OpenTelemetry, Prometheus/VictoriaMetrics, Grafana, Loki, Tempo",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "metrics.standard",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "backup",
            display_name: "Backup and disaster recovery",
            priority: "foundation",
            aws_equivalent: "AWS Backup, snapshots, Glacier",
            azure_equivalent: "Azure Backup, Archive Storage",
            open_source_stack: "Velero, Restic, Kopia, Borgmatic, Ceph snapshots, offline media",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "backup.standard",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "devops_iac",
            display_name: "DevOps and IaC",
            priority: "important",
            aws_equivalent: "CodeBuild, CodePipeline, CloudFormation",
            azure_equivalent: "Azure DevOps, ARM/Bicep",
            open_source_stack: "Forgejo, Woodpecker CI, Argo CD/Flux, OpenTofu, Ansible",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "repo.project",
            status: "preview",
            status_kind: "info",
        },
        CoreCloudService {
            id: "ai_batch",
            display_name: "AI batch and model serving",
            priority: "foundation",
            aws_equivalent: "SageMaker, Bedrock, Batch",
            azure_equivalent: "Azure AI, Azure ML, Batch",
            open_source_stack: "Kueue, Slurm, KServe, vLLM, SGLang, llama.cpp, MLflow",
            tenant_visible: true,
            operator_visible: true,
            provisionable: true,
            default_shape: "gpu-open.1x16g",
            status: "implemented",
            status_kind: "normal",
        },
        CoreCloudService {
            id: "costing",
            display_name: "Cost and sustainability",
            priority: "foundation",
            aws_equivalent: "Cost Explorer, Budgets",
            azure_equivalent: "Cost Management",
            open_source_stack: "OpenCost, CloudKitty, OSDC Rust calculators",
            tenant_visible: true,
            operator_visible: true,
            provisionable: false,
            default_shape: "cost.view",
            status: "implemented",
            status_kind: "normal",
        },
    ]
}

fn sovereign_cloud_services() -> Vec<SovereignCloudService> {
    csv_rows::<SovereignCloudServiceRow>(
        SOVEREIGN_SERVICE_CATALOGUE_CSV,
        "data/software/service-catalogue-v1.csv",
    )
    .into_iter()
    .map(Into::into)
    .collect()
}

fn commercial_gaps() -> Vec<CommercialGap> {
    csv_rows(
        COMMERCIAL_GAP_REGISTER_CSV,
        "data/commercial/commercial-gap-register.csv",
    )
}

fn commercial_standards() -> Vec<StandardsControl> {
    csv_rows(
        STANDARDS_CONTROL_MATRIX_CSV,
        "data/commercial/standards-control-matrix.csv",
    )
}

fn sla_classes() -> Vec<SlaClass> {
    csv_rows(SLA_CLASSES_CSV, "data/commercial/sla-classes.csv")
}

fn colocation_products() -> Vec<ColocationProduct> {
    csv_rows(
        COLOCATION_PRODUCTS_CSV,
        "data/commercial/colocation-products.csv",
    )
}

fn cross_connect_products() -> Vec<CrossConnectProduct> {
    csv_rows(
        CROSS_CONNECT_PRODUCTS_CSV,
        "data/commercial/cross-connect-products.csv",
    )
}

fn remote_hands_products() -> Vec<RemoteHandsProduct> {
    csv_rows(
        REMOTE_HANDS_PRODUCTS_CSV,
        "data/commercial/remote-hands-products.csv",
    )
}

fn audit_evidence() -> Vec<AuditEvidence> {
    csv_rows(AUDIT_EVIDENCE_CSV, "data/commercial/audit-evidence.csv")
}

fn upgrade_policy() -> Vec<UpgradePolicy> {
    vec![
        UpgradePolicy {
            update_class: "critical_cve",
            frequency: "24-72 hours",
            target_window: "emergency",
            required_gates: "staging-test+scan+backup-check+fast-approval",
            approval_owner: "platform-security-owner",
            rollback_requirement: "rollback-plan-required",
        },
        UpgradePolicy {
            update_class: "high_security",
            frequency: "weekly",
            target_window: "scheduled-security-window",
            required_gates: "pr+sbom+scan+staging+smoke-test",
            approval_owner: "platform-owner",
            rollback_requirement: "rollback-tested",
        },
        UpgradePolicy {
            update_class: "normal_patch",
            frequency: "monthly",
            target_window: "maintenance-window",
            required_gates: "pr+scan+staging+smoke-test",
            approval_owner: "service-owner",
            rollback_requirement: "rollback-available",
        },
        UpgradePolicy {
            update_class: "minor_feature",
            frequency: "quarterly",
            target_window: "planned-window",
            required_gates: "compatibility-test+docs-review+staging",
            approval_owner: "service-owner",
            rollback_requirement: "rollback-tested",
        },
        UpgradePolicy {
            update_class: "major_version",
            frequency: "6-12 months",
            target_window: "migration-window",
            required_gates: "migration-plan+backup+dry-run+rollback-test",
            approval_owner: "architecture-board",
            rollback_requirement: "rollback-tested",
        },
        UpgradePolicy {
            update_class: "firmware_bmc",
            frequency: "quarterly-or-emergency",
            target_window: "rack-by-rack-window",
            required_gates: "lab-test+vendor-notes+spare-node",
            approval_owner: "hardware-owner",
            rollback_requirement: "firmware-rollback-or-spare",
        },
        UpgradePolicy {
            update_class: "kubernetes_openstack_ceph",
            frequency: "planned-release-train",
            target_window: "platform-release-window",
            required_gates: "release-plan+staging+backup-restore+rollback-test",
            approval_owner: "platform-owner",
            rollback_requirement: "rollback-tested",
        },
    ]
}

fn config_scripts() -> Vec<ConfigScript> {
    vec![
        ConfigScript {
            id: "edge_caddyfile",
            tool: "Caddy",
            path: "/etc/caddy/Caddyfile",
            owner: "caddy",
            language: "caddyfile",
            edit_mode: "gitops-pr",
            validation_command: "caddy validate --config /etc/caddy/Caddyfile",
            rollout_target: "edge-a edge-b",
            risk: "medium",
            notes: "TLS routes reverse proxy and access middleware",
            content: EDGE_CADDYFILE,
        },
        ConfigScript {
            id: "edge_powerdns",
            tool: "PowerDNS",
            path: "/etc/powerdns/pdns.d/osdc.conf",
            owner: "pdns",
            language: "ini",
            edit_mode: "gitops-pr",
            validation_command: "pdnsutil check-all-zones",
            rollout_target: "edge-a edge-b edge-c",
            risk: "high",
            notes: "authoritative DNS backend API and DNSSEC settings",
            content: EDGE_POWERDNS_CONF,
        },
        ConfigScript {
            id: "edge_coraza",
            tool: "Coraza WAF",
            path: "/etc/coraza/osdc-crs.conf",
            owner: "root",
            language: "modsecurity",
            edit_mode: "gitops-pr",
            validation_command: "coraza --validate /etc/coraza/osdc-crs.conf",
            rollout_target: "edge-a edge-b",
            risk: "high",
            notes: "WAF CRS include file starts in detection mode",
            content: EDGE_CORAZA_CONF,
        },
        ConfigScript {
            id: "edge_crowdsec",
            tool: "CrowdSec",
            path: "/etc/crowdsec/acquis.yaml",
            owner: "crowdsec",
            language: "yaml",
            edit_mode: "gitops-pr",
            validation_command: "crowdsec hubtest run",
            rollout_target: "edge-a edge-b",
            risk: "medium",
            notes: "log acquisition for WAF and proxy decisions",
            content: EDGE_CROWDSEC_ACQUIS,
        },
        ConfigScript {
            id: "edge_wireguard",
            tool: "WireGuard",
            path: "/etc/wireguard/osdc-edge.conf",
            owner: "root",
            language: "ini",
            edit_mode: "secret-aware-pr",
            validation_command: "wg-quick strip /etc/wireguard/osdc-edge.conf",
            rollout_target: "edge-a edge-b",
            risk: "high",
            notes: "private origin tunnel with secret placeholders",
            content: EDGE_WIREGUARD_CONF,
        },
    ]
}

fn provisioning_blueprints() -> Vec<ProvisioningBlueprint> {
    vec![
        ProvisioningBlueprint {
            id: "vm-with-volume",
            display_name: "Linux VM with block volume",
            service_id: "compute_vm",
            default_shape: "cpu.standard",
            api_surface: "POST /api/provisioning/requests",
            backing_stack: "OpenStack Nova + Cinder + Neutron",
            operator_checks: vec!["quota", "image signature", "rack power", "Ceph health"],
        },
        ProvisioningBlueprint {
            id: "gpu-model-endpoint",
            display_name: "GPU model endpoint",
            service_id: "ai_batch",
            default_shape: "gpu-open.1x16g",
            api_surface: "POST /api/provisioning/requests",
            backing_stack: "Kueue + KServe + vLLM/SGLang + Ceph model cache",
            operator_checks: vec![
                "GPU reset support",
                "model license class",
                "thermal headroom",
                "queue policy",
            ],
        },
        ProvisioningBlueprint {
            id: "managed-postgres",
            display_name: "Managed PostgreSQL",
            service_id: "database",
            default_shape: "postgres.small",
            api_surface: "POST /api/provisioning/requests",
            backing_stack: "CloudNativePG + Ceph RBD + OpenBao",
            operator_checks: vec![
                "backup policy",
                "restore test",
                "storage quota",
                "secret rotation",
            ],
        },
        ProvisioningBlueprint {
            id: "private-kubernetes",
            display_name: "Managed Kubernetes cluster",
            service_id: "kubernetes",
            default_shape: "k8s.standard",
            api_surface: "POST /api/provisioning/requests",
            backing_stack: "Cluster API + Metal3 + Cilium + Harbor",
            operator_checks: vec![
                "node capacity",
                "image policy",
                "network quota",
                "backup policy",
            ],
        },
        ProvisioningBlueprint {
            id: "object-bucket",
            display_name: "Object bucket",
            service_id: "object_storage",
            default_shape: "bucket.standard",
            api_surface: "POST /api/provisioning/requests",
            backing_stack: "Ceph RGW + Keycloak/OPA policy",
            operator_checks: vec![
                "bucket policy",
                "quota",
                "replication",
                "public access review",
            ],
        },
    ]
}

fn provisioning_preview() -> ProvisioningPreview {
    ProvisioningPreview {
        request_id: "preview-0001",
        service_id: "ai_batch",
        shape: "gpu-open.1x16g",
        linux_image: "Debian stable",
        backing_stack: "Kueue + KServe + vLLM/SGLang + Ceph model cache",
        estimated_power_w: 220,
        estimated_monthly_usd: 96,
        operator_checks: vec![
            "GPU pool capacity",
            "model license class",
            "thermal headroom",
            "tenant quota",
        ],
        next_api_path: "POST /api/provisioning/requests",
    }
}

fn provisioning_options() -> ProvisioningOptions {
    ProvisioningOptions {
        services: vec![
            "VM instance",
            "Kubernetes cluster",
            "AI model endpoint",
            "PostgreSQL database",
            "Object bucket",
            "Bare-metal reservation",
            "Serverless function",
            "Message queue",
            "Secrets vault",
            "Backup policy",
        ],
        shapes: vec![
            ShapeOption {
                id: "gpu-open.1x16g",
                label: "AMD RX 9060 XT 16GB",
                hardware_profile: "gpu.rocm.16g.default",
                power_class: "default GPU",
            },
            ShapeOption {
                id: "gpu-lowpower.1x16g",
                label: "Intel Arc Pro B50 16GB",
                hardware_profile: "gpu.xpu.16g.lowpower",
                power_class: "low-power GPU",
            },
            ShapeOption {
                id: "edge-arm.small",
                label: "RK3588 edge SBC",
                hardware_profile: "sbc.rk3588.edge",
                power_class: "low-power ARM",
            },
            ShapeOption {
                id: "cpu.standard",
                label: "x86 KVM compute",
                hardware_profile: "cpu.standard",
                power_class: "general compute",
            },
        ],
        linux_images: vec![
            "Debian stable",
            "Ubuntu Server LTS",
            "Fedora Cloud",
            "Rocky Linux",
            "Talos Kubernetes node",
            "Armbian RK3588",
        ],
        storage: vec![
            "Ceph RBD 250 GB",
            "Ceph RBD 1 TB",
            "CephFS shared dataset",
            "Object bucket only",
        ],
        networks: vec!["research-private", "public-api", "training-isolated"],
    }
}

fn tenant_summary() -> TenantSummary {
    TenantSummary {
        metrics: vec![
            Metric {
                label: "Running instances",
                value: "28",
                detail: "4 GPU attached",
                kind: "normal",
            },
            Metric {
                label: "Object storage",
                value: "18.4 TB",
                detail: "Ceph RGW",
                kind: "normal",
            },
            Metric {
                label: "GPU queue",
                value: "11 min",
                detail: "solar surplus window",
                kind: "normal",
            },
            Metric {
                label: "Month estimate",
                value: "$412",
                detail: "power aware",
                kind: "normal",
            },
        ],
        site_flow: vec![
            FlowStep {
                label: "PV",
                value: "186 kW now",
            },
            FlowStep {
                label: "BESS",
                value: "71% SOC",
            },
            FlowStep {
                label: "DC bus",
                value: "392 V",
            },
            FlowStep {
                label: "Rack bus",
                value: "48.4 V",
            },
            FlowStep {
                label: "Queue",
                value: "14 jobs",
            },
        ],
        services: vec![
            ServiceTile {
                name: "Compute",
                stack: "OpenStack Nova, Ironic, Incus edge",
            },
            ServiceTile {
                name: "Storage",
                stack: "Ceph RBD, RGW, CephFS",
            },
            ServiceTile {
                name: "Kubernetes",
                stack: "Cluster API, Cilium, Kueue",
            },
            ServiceTile {
                name: "AI",
                stack: "vLLM, SGLang, KServe, Slurm",
            },
            ServiceTile {
                name: "Data",
                stack: "CloudNativePG, Valkey, Kafka",
            },
            ServiceTile {
                name: "Security",
                stack: "Keycloak, OPA, OpenBao",
            },
        ],
        core_services: core_cloud_services()
            .into_iter()
            .filter(|service| service.tenant_visible)
            .collect(),
        resources: vec![
            TenantResource {
                name: "clinic-model-api",
                resource_type: "AI endpoint",
                backing_stack: "vLLM on Kueue",
                power_class: "gpu-open.1x16g",
                status: "Running",
                status_kind: "normal",
                monthly_estimate: "$96",
            },
            TenantResource {
                name: "records-db",
                resource_type: "PostgreSQL",
                backing_stack: "CloudNativePG",
                power_class: "cpu.standard",
                status: "Running",
                status_kind: "normal",
                monthly_estimate: "$42",
            },
            TenantResource {
                name: "imaging-bucket",
                resource_type: "Object bucket",
                backing_stack: "Ceph RGW",
                power_class: "storage.standard",
                status: "Healthy",
                status_kind: "normal",
                monthly_estimate: "$61",
            },
            TenantResource {
                name: "edge-screening",
                resource_type: "ARM service",
                backing_stack: "RK3588 Incus",
                power_class: "edge-arm.small",
                status: "Deploying",
                status_kind: "warn",
                monthly_estimate: "$11",
            },
        ],
    }
}

fn operator_status() -> OperatorStatus {
    OperatorStatus {
        metrics: vec![
            Metric {
                label: "380-400 VDC bus",
                value: "392 V",
                detail: "stable",
                kind: "normal",
            },
            Metric {
                label: "Sodium-ion BESS",
                value: "71%",
                detail: "2.2 h critical",
                kind: "normal",
            },
            Metric {
                label: "Rack thermal spine",
                value: "214 kW",
                detail: "86% captured",
                kind: "normal",
            },
            Metric {
                label: "PUE now",
                value: "1.22",
                detail: "dry cooler active",
                kind: "normal",
            },
        ],
        power_flow: vec![
            FlowStep {
                label: "PV",
                value: "186 kW",
            },
            FlowStep {
                label: "MPPT",
                value: "98%",
            },
            FlowStep {
                label: "BESS",
                value: "356 kWh",
            },
            FlowStep {
                label: "DC bus",
                value: "392 V",
            },
            FlowStep {
                label: "48 V rows",
                value: "48.4 V",
            },
        ],
        thermal_flow: vec![
            FlowStep {
                label: "Rack heat",
                value: "250 kW",
            },
            FlowStep {
                label: "Captured",
                value: "214 kW",
            },
            FlowStep {
                label: "Hot loop",
                value: "63 C",
            },
            FlowStep {
                label: "Sorption",
                value: "38 kW",
            },
            FlowStep {
                label: "Reject",
                value: "309 kW",
            },
        ],
        hardware_pools: vec![
            HardwarePool {
                pool: "edge-arm",
                hardware: "5x RK3588 SBC",
                power: "112 W",
                allocated: "3 / 5",
                status: "Ready",
                status_kind: "normal",
            },
            HardwarePool {
                pool: "gpu-open",
                hardware: "4x RX 9060 XT 16GB",
                power: "486 W",
                allocated: "3 / 4",
                status: "Ready",
                status_kind: "normal",
            },
            HardwarePool {
                pool: "gpu-lowpower",
                hardware: "2x Arc Pro B50 16GB",
                power: "118 W",
                allocated: "1 / 2",
                status: "Testing",
                status_kind: "info",
            },
            HardwarePool {
                pool: "cpu-standard",
                hardware: "20x x86 servers",
                power: "5.4 kW",
                allocated: "61%",
                status: "Ready",
                status_kind: "normal",
            },
        ],
        cloud_stack: vec![
            StackHealth {
                domain: "IaaS",
                stack: "OpenStack Nova Neutron Cinder",
                health: "OK",
                health_kind: "normal",
            },
            StackHealth {
                domain: "Storage",
                stack: "Ceph RBD RGW CephFS",
                health: "OK",
                health_kind: "normal",
            },
            StackHealth {
                domain: "Containers",
                stack: "Kubernetes Cilium Kueue",
                health: "2 upgrades",
                health_kind: "warn",
            },
            StackHealth {
                domain: "Identity",
                stack: "Keycloak OPA OpenBao",
                health: "OK",
                health_kind: "normal",
            },
        ],
        core_services: core_cloud_services()
            .into_iter()
            .filter(|service| service.operator_visible)
            .collect(),
        operations: vec![
            OperationItem {
                time: "09:20",
                system: "DC protection",
                action: "Isolation monitor calibration",
                owner: "electrical",
                risk: "low",
                status: "Scheduled",
                status_kind: "warn",
            },
            OperationItem {
                time: "10:00",
                system: "GPU pool",
                action: "ROCm reset test",
                owner: "platform",
                risk: "medium",
                status: "Queued",
                status_kind: "info",
            },
            OperationItem {
                time: "11:30",
                system: "Ceph",
                action: "OSD replacement drill",
                owner: "storage",
                risk: "low",
                status: "Approved",
                status_kind: "normal",
            },
            OperationItem {
                time: "13:00",
                system: "Cooling",
                action: "Dry cooler fan failover",
                owner: "thermal",
                risk: "medium",
                status: "Pending",
                status_kind: "warn",
            },
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
    fn root_redirects_to_tenant_console() {
        let response = route_response("GET", "/");
        let text = response_text(&response);

        assert!(text.starts_with("HTTP/1.1 302 Found"));
        assert!(text.contains("Location: /user"));
    }

    #[test]
    fn serves_all_gui_pages_with_expected_controls() {
        let user = body(&route_response("GET", "/user"));
        let operator = body(&route_response("GET", "/operator"));
        let edge = body(&route_response("GET", "/edge"));
        let planner = body(&route_response("GET", "/planner"));

        assert!(user.contains("Tenant Cloud"));
        assert!(user.contains("tenant-service-filter"));
        assert!(user.contains("tenant-action-output"));
        assert!(operator.contains("Operator Console"));
        assert!(operator.contains("operator-service-filter"));
        assert!(edge.contains("Edge Shield"));
        assert!(edge.contains("edge-service-filter"));
        assert!(edge.contains("edge-config-preview"));
        assert!(edge.contains("edge-script-editor"));
        assert!(planner.contains("Cost Planner"));
        assert!(planner.contains("planner-scenarios"));
        assert!(planner.contains("planner-price-basis"));
    }

    #[test]
    fn exposes_core_cloud_services_matching_aws_and_azure_baseline() {
        let services = json_body("/api/catalog/core-services");
        let services = services
            .as_array()
            .expect("service catalog should be array");

        assert!(services.len() >= 16);
        assert!(services.iter().any(|service| service["id"] == "compute_vm"
            && service["aws_equivalent"] == "EC2"
            && service["azure_equivalent"] == "Azure Virtual Machines"));
        assert!(services
            .iter()
            .any(|service| service["id"] == "object_storage"
                && service["aws_equivalent"] == "S3"
                && service["azure_equivalent"] == "Blob Storage"));
        assert!(services.iter().any(|service| service["id"] == "ai_batch"
            && service["open_source_stack"]
                .as_str()
                .unwrap_or_default()
                .contains("Kueue")));
    }

    #[test]
    fn exposes_sovereign_service_catalogue_and_upgrade_policy() {
        let services = json_body("/api/catalog/sovereign-services");
        let upgrade_policy = json_body("/api/catalog/upgrade-policy");
        let services = services
            .as_array()
            .expect("sovereign service catalog should be array");
        let upgrade_policy = upgrade_policy
            .as_array()
            .expect("upgrade policy should be array");

        assert!(services.len() >= 50);
        assert!(services.iter().any(|service| service["id"] == "identity"
            && service["open_equivalent"]
                .as_str()
                .unwrap_or_default()
                .contains("Keycloak")
            && service["workflow"] == "create tenant and assign roles"));
        assert!(services.iter().any(|service| service["id"] == "registry"
            && service["security_controls"]
                .as_str()
                .unwrap_or_default()
                .contains("sbom")
            && service["maturity"] == "production-baseline"));
        assert!(services
            .iter()
            .any(|service| service["id"] == "fleet_os" && service["maturity"] == "pilot"));
        assert!(upgrade_policy
            .iter()
            .any(|policy| policy["update_class"] == "critical_cve"
                && policy["frequency"] == "24-72 hours"));
        assert!(upgrade_policy.iter().any(|policy| {
            policy["update_class"] == "kubernetes_openstack_ceph"
                && policy["target_window"] == "platform-release-window"
        }));
    }

    #[test]
    fn exposes_browser_editable_config_scripts() {
        let scripts = json_body("/api/config/scripts");
        let scripts = scripts
            .as_array()
            .expect("config scripts should be an array");

        assert!(scripts.len() >= 5);
        assert!(scripts.iter().any(|script| script["id"] == "edge_caddyfile"
            && script["content"]
                .as_str()
                .unwrap_or_default()
                .contains("public.example.gov")
            && script["validation_command"]
                .as_str()
                .unwrap_or_default()
                .contains("caddy validate")));
        assert!(scripts.iter().any(|script| script["id"] == "edge_wireguard"
            && script["edit_mode"] == "secret-aware-pr"
            && script["content"]
                .as_str()
                .unwrap_or_default()
                .contains("${OSDC_WIREGUARD_PRIVATE_KEY}")));
    }

    #[test]
    fn exposes_tenant_provisioning_contract() {
        let options = json_body("/api/provisioning/options");
        let preview = json_body("/api/provisioning/preview");
        let blueprints = json_body("/api/catalog/blueprints");

        assert!(options["services"]
            .as_array()
            .unwrap()
            .iter()
            .any(|service| service == "AI model endpoint"));
        assert!(options["shapes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|shape| shape["id"] == "gpu-open.1x16g"));
        assert_eq!(preview["next_api_path"], "POST /api/provisioning/requests");
        assert!(blueprints.as_array().unwrap().iter().any(|blueprint| {
            blueprint["id"] == "gpu-model-endpoint" && blueprint["service_id"] == "ai_batch"
        }));
    }

    #[test]
    fn exposes_edge_shield_services_and_config_preview() {
        let services = json_body("/api/edge/services");
        let config = json_body("/api/edge/config-preview");
        let services = services.as_array().expect("edge services should be array");

        assert!(services.len() >= 10);
        assert!(services.iter().any(|service| {
            service["open_source_stack"]
                .as_str()
                .unwrap_or_default()
                .contains("PowerDNS")
                && service["cloudflare_equivalent"] == "Cloudflare DNS"
        }));
        assert!(services.iter().any(|service| {
            service["open_source_stack"]
                .as_str()
                .unwrap_or_default()
                .contains("Coraza")
                && service["cloudflare_equivalent"] == "Cloudflare WAF"
        }));
        assert!(config["generated_files"]
            .as_array()
            .unwrap()
            .iter()
            .any(|file| file["path"] == "/etc/caddy/Caddyfile"));
        assert!(config["rollout_checks"].as_array().unwrap().len() >= 5);
    }

    #[test]
    fn operator_status_includes_dc_power_thermal_and_cloud_stack() {
        let status = json_body("/api/operator/status");

        assert!(status["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|metric| metric["label"] == "380-400 VDC bus"));
        assert!(status["thermal_flow"]
            .as_array()
            .unwrap()
            .iter()
            .any(|step| step["label"] == "Rack heat"));
        assert!(status["core_services"]
            .as_array()
            .unwrap()
            .iter()
            .any(|service| service["id"] == "networking"));
    }

    #[test]
    fn exposes_cost_planning_scenarios_categories_and_price_basis() {
        let planning = json_body("/api/cost/planning");
        let scenarios = json_body("/api/cost/scenarios");
        let categories = json_body("/api/cost/categories");
        let price_basis = json_body("/api/cost/price-basis");

        assert_eq!(planning["metrics"].as_array().unwrap().len(), 4);
        assert!(scenarios.as_array().unwrap().iter().any(|scenario| {
            scenario["id"] == "S2"
                && scenario["it_load_kw"] == 250
                && scenario["total_with_it_low_usd"] == 1_200_000
                && scenario["total_with_it_high_usd"] == 2_470_000
        }));
        assert!(categories.as_array().unwrap().iter().any(|category| {
            category["scenario_id"] == "S2"
                && category["category"] == "dc_microgrid_solar_sodium_single_fallback_power"
                && category["notes"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("sodium-ion BESS")
        }));
        assert!(price_basis.as_array().unwrap().iter().any(|item| {
            item["item_family"] == "solar_pv_system"
                && item["unit"] == "w_installed"
                && item["source_marketplace"] == "Alibaba/Derived"
        }));
    }

    #[test]
    fn exposes_commercial_readiness_catalogues() {
        let gaps = json_body("/api/commercial/gaps");
        let standards = json_body("/api/commercial/standards");
        let slas = json_body("/api/commercial/sla-classes");
        let colocation = json_body("/api/commercial/colocation-products");
        let cross_connects = json_body("/api/commercial/cross-connect-products");
        let remote_hands = json_body("/api/commercial/remote-hands-products");
        let evidence = json_body("/api/commercial/audit-evidence");

        assert!(gaps.as_array().unwrap().iter().any(|gap| {
            gap["gap_id"] == "G002"
                && gap["next_artifact"] == "docs/engineering/electrical-single-line-250kw.md"
        }));
        assert!(standards
            .as_array()
            .unwrap()
            .iter()
            .any(|control| control["requirement_id"] == "STD006"
                && control["standard_family"] == "IEC62443"));
        assert!(slas
            .as_array()
            .unwrap()
            .iter()
            .any(|sla| sla["sla_class_id"] == "SLA_POWER_A"));
        assert!(colocation
            .as_array()
            .unwrap()
            .iter()
            .any(|product| product["product_id"] == "COLO_FULL_CAB"));
        assert!(cross_connects
            .as_array()
            .unwrap()
            .iter()
            .any(|product| product["product_id"] == "XC_IXP"));
        assert!(remote_hands
            .as_array()
            .unwrap()
            .iter()
            .any(|product| product["product_id"] == "RH_SMART_HANDS"));
        assert!(evidence
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["evidence_id"] == "EV007"));
    }

    #[test]
    fn static_assets_have_content_types_and_unknown_routes_404() {
        let css = response_text(&route_response("GET", "/styles.css"));
        let png = response_text(&route_response(
            "GET",
            "/assets/rack-thermal-spine-cutaway.png",
        ));
        let missing = response_text(&route_response("GET", "/missing"));

        assert!(css.contains("Content-Type: text/css; charset=utf-8"));
        assert!(png.contains("Content-Type: image/png"));
        assert!(missing.starts_with("HTTP/1.1 404 Not Found"));
    }
}
