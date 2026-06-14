use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use serde::Serialize;

const STYLE_CSS: &str = include_str!("views/style.css");
const PORTAL_JS: &str = include_str!("views/portal.js");
const USER_HTML: &str = include_str!("views/user.html");
const OPERATOR_HTML: &str = include_str!("views/operator.html");
const RACK_IMAGE: &[u8] = include_bytes!("../../../docs/assets/rack-thermal-spine-cutaway.png");
const EXTERIOR_IMAGE: &[u8] =
    include_bytes!("../../../docs/assets/prefab-panel-datacentre-exterior-02.png");

fn main() -> std::io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8787".to_string());
    let listener = TcpListener::bind(&addr)?;

    println!("osdc-portal listening on http://{addr}");
    println!("tenant portal: http://{addr}/user");
    println!("operator console: http://{addr}/operator");
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

    let response = match (method, path) {
        ("GET", "/") => redirect("/user"),
        ("GET", "/user") => html(USER_HTML),
        ("GET", "/operator") => html(OPERATOR_HTML),
        ("GET", "/styles.css") => bytes("200 OK", "text/css; charset=utf-8", STYLE_CSS.as_bytes()),
        ("GET", "/portal.js") => bytes(
            "200 OK",
            "text/javascript; charset=utf-8",
            PORTAL_JS.as_bytes(),
        ),
        ("GET", "/api/catalog/hardware") => json(&hardware_catalog()),
        ("GET", "/api/catalog/services") => json(&service_catalog()),
        ("GET", "/api/provisioning/options") => json(&provisioning_options()),
        ("GET", "/api/tenant/summary") => json(&tenant_summary()),
        ("GET", "/api/operator/status") => json(&operator_status()),
        ("GET", "/assets/rack-thermal-spine-cutaway.png") => {
            bytes("200 OK", "image/png", RACK_IMAGE)
        }
        ("GET", "/assets/prefab-panel-datacentre-exterior-02.png") => {
            bytes("200 OK", "image/png", EXTERIOR_IMAGE)
        }
        ("GET", "/health") => bytes("200 OK", "text/plain; charset=utf-8", b"ok\n"),
        _ => bytes("404 Not Found", "text/plain; charset=utf-8", b"not found\n"),
    };

    stream.write_all(&response)?;
    stream.flush()
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
struct TenantSummary {
    metrics: Vec<Metric>,
    site_flow: Vec<FlowStep>,
    services: Vec<ServiceTile>,
    resources: Vec<TenantResource>,
}

#[derive(Debug, Serialize)]
struct OperatorStatus {
    metrics: Vec<Metric>,
    power_flow: Vec<FlowStep>,
    thermal_flow: Vec<FlowStep>,
    hardware_pools: Vec<HardwarePool>,
    cloud_stack: Vec<StackHealth>,
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

fn provisioning_options() -> ProvisioningOptions {
    ProvisioningOptions {
        services: vec![
            "VM instance",
            "Kubernetes cluster",
            "AI model endpoint",
            "PostgreSQL database",
            "Object bucket",
            "Bare-metal reservation",
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
