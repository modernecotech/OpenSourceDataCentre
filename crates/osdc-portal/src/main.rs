use std::{
    env, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

const STYLE_CSS: &str = include_str!("views/style.css");
const PORTAL_JS: &str = include_str!("views/portal.js");
const USER_HTML: &str = include_str!("views/user.html");
const OPERATOR_HTML: &str = include_str!("views/operator.html");
const EDGE_HTML: &str = include_str!("views/edge.html");
const PLANNER_HTML: &str = include_str!("views/planner.html");
const INFRASTRUCTURE_HTML: &str = include_str!("views/infrastructure.html");
const LIFECYCLE_HTML: &str = include_str!("views/lifecycle.html");
const HARDWARE_HTML: &str = include_str!("views/hardware.html");
const DEVELOPER_HTML: &str = include_str!("views/developer.html");
const DATA_PLATFORM_HTML: &str = include_str!("views/data-platform.html");
const COMMERCIAL_HTML: &str = include_str!("views/commercial.html");
const CUSTOMERS_HTML: &str = include_str!("views/customers.html");
const ASSURANCE_HTML: &str = include_str!("views/assurance.html");
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
const EDGE_SHIELD_SERVICES_CSV: &str =
    include_str!("../../../data/software/edge-shield-services.csv");
const SOVEREIGN_SERVICE_CATALOGUE_CSV: &str =
    include_str!("../../../data/software/service-catalogue-v1.csv");
const CONFIG_SCRIPT_CATALOGUE_CSV: &str =
    include_str!("../../../data/software/config-script-catalogue.csv");
const UPGRADE_POLICY_CSV: &str = include_str!("../../../data/software/upgrade-policy.csv");
const SCENARIO_COSTS_CSV: &str = include_str!("../../../data/costing/scenario-costs-2026.csv");
const SCENARIO_CATEGORY_COSTS_CSV: &str =
    include_str!("../../../data/costing/scenario-category-costs-2026.csv");
const MARKETPLACE_PRICE_BASIS_CSV: &str =
    include_str!("../../../data/costing/marketplace-price-basis-2026.csv");
const DEPLOYMENT_STACK_PROFILES_CSV: &str =
    include_str!("../../../data/software/deployment-stack-profiles.csv");
const INFRASTRUCTURE_WORKFLOWS_CSV: &str =
    include_str!("../../../data/software/infrastructure-workflows.csv");
const LIVE_ADAPTER_ROADMAP_CSV: &str =
    include_str!("../../../data/software/live-adapter-roadmap.csv");
const LIVE_ADAPTER_PROOF_CATALOGUE_CSV: &str =
    include_str!("../../../data/software/live-adapter-proof-catalogue.csv");
const PORTAL_STATE_MIGRATION_SQL: &str = include_str!("../migrations/0001_osdc_portal_state.sql");
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
const REMOTE_HANDS_PRICEBOOK_CSV: &str =
    include_str!("../../../data/commercial/remote-hands-pricebook.csv");
const CUSTOMER_ACCOUNTS_CSV: &str = include_str!("../../../data/commercial/customer-accounts.csv");
const CUSTOMER_SITES_CSV: &str = include_str!("../../../data/commercial/customer-sites.csv");
const BILLING_PLANS_CSV: &str = include_str!("../../../data/commercial/billing-plans.csv");
const USAGE_METERS_CSV: &str = include_str!("../../../data/commercial/usage-meters.csv");
const INVOICE_PREVIEW_CSV: &str = include_str!("../../../data/commercial/invoice-preview.csv");
const ACCESS_ROLES_CSV: &str = include_str!("../../../data/commercial/access-roles.csv");
const AUDIT_EVIDENCE_CSV: &str = include_str!("../../../data/commercial/audit-evidence.csv");
const SITE_SELECTION_SCORECARD_CSV: &str =
    include_str!("../../../data/site-selection/site-selection-scorecard.csv");
const PHYSICAL_SECURITY_CONTROLS_CSV: &str =
    include_str!("../../../data/security/physical-security-controls.csv");
const SUSTAINABILITY_METRICS_CSV: &str =
    include_str!("../../../data/sustainability/sustainability-metrics.csv");
const HARDWARE_COMPUTE_BASELINE_CSV: &str =
    include_str!("../../../data/hardware/compute-baseline-2026.csv");
const AI_RACK_CLASSES_CSV: &str =
    include_str!("../../../data/ai-ready/high-density-rack-classes.csv");
const HARDWARE_PROVISIONING_PIPELINE_CSV: &str =
    include_str!("../../../data/hardware/provisioning-pipeline.csv");
const HARDWARE_PROVISIONING_PROFILES_CSV: &str =
    include_str!("../../../data/hardware/provisioning-profiles.csv");
const HARDWARE_PROVISIONING_REQUESTS_CSV: &str =
    include_str!("../../../data/hardware/provisioning-requests.csv");
const ENGINEERING_EVIDENCE_CSV: &str =
    include_str!("../../../data/engineering/engineering-evidence-register.csv");
const OPERATIONS_PROCEDURES_CSV: &str =
    include_str!("../../../data/operations/procedure-catalogue.csv");
const PROJECT_GATES_CSV: &str = include_str!("../../../data/delivery/project-gates.csv");
const AUTHORITY_PERMITS_CSV: &str = include_str!("../../../data/delivery/authority-permits.csv");
const DELIVERY_RISKS_CSV: &str = include_str!("../../../data/delivery/risk-register.csv");
const DELIVERY_ACTIONS_CSV: &str = include_str!("../../../data/delivery/action-tracker.csv");
const COMMISSIONING_EVIDENCE_CSV: &str =
    include_str!("../../../data/commissioning/commissioning-evidence-register.csv");
const DEVELOPER_PLATFORM_SERVICES_CSV: &str =
    include_str!("../../../data/software/developer-platform-services.csv");
const DEVELOPER_TEMPLATES_CSV: &str =
    include_str!("../../../data/software/developer-templates.csv");
const DEPLOYMENT_ENVIRONMENTS_CSV: &str =
    include_str!("../../../data/software/deployment-environments.csv");
const DEVELOPER_PROMOTION_GATES_CSV: &str =
    include_str!("../../../data/software/developer-promotion-gates.csv");
const VSCODE_WORKFLOWS_CSV: &str = include_str!("../../../data/software/vscode-workflows.csv");
const CUSTOMER_OPERATIONS_WORKFLOWS_CSV: &str =
    include_str!("../../../data/software/customer-operations-workflows.csv");
const IDENTITY_MFA_POLICIES_CSV: &str =
    include_str!("../../../data/software/identity-mfa-policies.csv");
const DATA_PLATFORM_SERVICES_CSV: &str =
    include_str!("../../../data/software/data-platform-services.csv");
const DATA_PRODUCTS_CSV: &str = include_str!("../../../data/software/data-products.csv");
const DATA_PIPELINES_CSV: &str = include_str!("../../../data/software/data-pipelines.csv");
const DATA_ONTOLOGY_OBJECTS_CSV: &str =
    include_str!("../../../data/software/data-ontology-objects.csv");
const DATA_ACCESS_POLICIES_CSV: &str =
    include_str!("../../../data/software/data-access-policies.csv");
const DATA_PLATFORM_TEMPLATES_CSV: &str =
    include_str!("../../../data/software/data-platform-templates.csv");
const OPEN_CLOUD_SERVICE_MAP_CSV: &str =
    include_str!("../../../data/software/open-cloud-service-map.csv");
const CORE_CLOUD_SERVICES_CSV: &str =
    include_str!("../../../data/software/core-cloud-services.csv");
const ASSURANCE_AUTOMATION_JOBS_CSV: &str =
    include_str!("../../../data/software/assurance-automation-jobs.csv");
const SYSTEM_UI_CONNECTORS_CSV: &str =
    include_str!("../../../data/software/system-ui-connectors.csv");
const TEST_HARNESS_CATALOGUE_CSV: &str =
    include_str!("../../../data/software/test-harness-catalogue.csv");
const UPGRADE_RINGS_CSV: &str = include_str!("../../../data/software/upgrade-rings.csv");
const UPGRADE_TEST_GATES_CSV: &str = include_str!("../../../data/software/upgrade-test-gates.csv");
const THREAT_MANAGEMENT_STACK_CSV: &str =
    include_str!("../../../data/security/threat-management-stack.csv");
const SCANNER_COVERAGE_CSV: &str = include_str!("../../../data/security/scanner-coverage.csv");

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
    println!("infrastructure workbench: http://{addr}/infrastructure");
    println!("lifecycle console: http://{addr}/lifecycle");
    println!("hardware provisioning console: http://{addr}/hardware");
    println!("commercial console: http://{addr}/commercial");
    println!("customer operations console: http://{addr}/customers");
    println!("assurance console: http://{addr}/assurance");
    println!("developer console: http://{addr}/developer");
    println!("data platform console: http://{addr}/data-platform");
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
        ("GET", "/") => redirect("/infrastructure"),
        ("GET", "/user") => html(USER_HTML),
        ("GET", "/operator") => html(OPERATOR_HTML),
        ("GET", "/edge") => html(EDGE_HTML),
        ("GET", "/planner") => html(PLANNER_HTML),
        ("GET", "/infrastructure") => html(INFRASTRUCTURE_HTML),
        ("GET", "/lifecycle") => html(LIFECYCLE_HTML),
        ("GET", "/hardware") => html(HARDWARE_HTML),
        ("GET", "/developer") => html(DEVELOPER_HTML),
        ("GET", "/data-platform") => html(DATA_PLATFORM_HTML),
        ("GET", "/commercial") => html(COMMERCIAL_HTML),
        ("GET", "/customers") => html(CUSTOMERS_HTML),
        ("GET", "/assurance") => html(ASSURANCE_HTML),
        ("GET", "/styles.css") => bytes("200 OK", "text/css; charset=utf-8", STYLE_CSS.as_bytes()),
        ("GET", "/portal.js") => bytes(
            "200 OK",
            "text/javascript; charset=utf-8",
            PORTAL_JS.as_bytes(),
        ),
        ("GET", "/api/catalog/hardware") => json(&hardware_catalog()),
        ("GET", "/api/connectors/systems") => json(&system_ui_connectors()),
        ("GET", "/api/hardware/provisioning") => json(&hardware_provisioning_overview()),
        ("GET", "/api/hardware/provisioning-pipeline") => json(&hardware_provisioning_pipeline()),
        ("GET", "/api/hardware/provisioning-profiles") => json(&hardware_provisioning_profiles()),
        ("GET", "/api/hardware/provisioning-requests") => json(&hardware_provisioning_requests()),
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
        ("GET", "/api/deployment/stack-profiles") => json(&deployment_stack_profiles()),
        ("GET", "/api/commercial/gaps") => json(&commercial_gaps()),
        ("GET", "/api/commercial/standards") => json(&commercial_standards()),
        ("GET", "/api/commercial/sla-classes") => json(&sla_classes()),
        ("GET", "/api/commercial/colocation-products") => json(&colocation_products()),
        ("GET", "/api/commercial/cross-connect-products") => json(&cross_connect_products()),
        ("GET", "/api/commercial/remote-hands-products") => json(&remote_hands_products()),
        ("GET", "/api/commercial/remote-hands-pricebook") => json(&remote_hands_pricebook()),
        ("GET", "/api/customers/overview") => json(&customer_operations_overview()),
        ("GET", "/api/customers/accounts") => json(&customer_accounts()),
        ("GET", "/api/customers/sites") => json(&customer_sites()),
        ("GET", "/api/customers/workflows") => json(&customer_operations_workflows()),
        ("GET", "/api/customers/mfa-policies") => json(&identity_mfa_policies()),
        ("GET", "/api/customers/billing-plans") => json(&billing_plans()),
        ("GET", "/api/customers/usage-meters") => json(&usage_meters()),
        ("GET", "/api/customers/invoice-preview") => json(&invoice_preview()),
        ("GET", "/api/commercial/access-roles") => json(&access_roles()),
        ("GET", "/api/commercial/audit-evidence") => json(&audit_evidence()),
        ("GET", "/api/site-selection/scorecard") => json(&site_selection_scorecard()),
        ("GET", "/api/security/physical-controls") => json(&physical_security_controls()),
        ("GET", "/api/sustainability/metrics") => json(&sustainability_metrics()),
        ("GET", "/api/ai-ready/rack-classes") => json(&ai_rack_classes()),
        ("GET", "/api/engineering/evidence") => json(&engineering_evidence()),
        ("GET", "/api/operations/procedures") => json(&operations_procedures()),
        ("GET", "/api/delivery/gates") => json(&project_gates()),
        ("GET", "/api/delivery/permits") => json(&authority_permits()),
        ("GET", "/api/delivery/risks") => json(&delivery_risks()),
        ("GET", "/api/delivery/actions") => json(&delivery_actions()),
        ("GET", "/api/commissioning/evidence") => json(&commissioning_evidence()),
        ("GET", "/api/lifecycle/overview") => json(&lifecycle_overview()),
        ("GET", "/api/developer/platform") => json(&developer_platform()),
        ("GET", "/api/data-platform/overview") => json(&data_platform_overview()),
        ("GET", "/api/assurance/overview") => json(&assurance_overview()),
        ("GET", "/api/assurance/test-harnesses") => json(&test_harnesses()),
        ("GET", "/api/assurance/upgrade-rings") => json(&upgrade_rings()),
        ("GET", "/api/assurance/upgrade-gates") => json(&upgrade_test_gates()),
        ("GET", "/api/assurance/threat-stack") => json(&threat_management_stack()),
        ("GET", "/api/assurance/scanner-coverage") => json(&scanner_coverage()),
        ("GET", "/api/assurance/automation-jobs") => json(&assurance_automation_jobs()),
        ("GET", "/api/infrastructure/workbench") => json(&infrastructure_workbench()),
        ("GET", "/api/infrastructure/adapter-roadmap") => json(&live_adapter_roadmap()),
        ("GET", "/api/infrastructure/adapter-proofs") => json(&live_adapter_proofs()),
        ("GET", "/api/infrastructure/persistence-schema") => json(&portal_persistence_schema()),
        ("GET", "/assets/rack-thermal-spine-cutaway.png") => {
            bytes("200 OK", "image/png", RACK_IMAGE)
        }
        ("GET", "/assets/prefab-panel-datacentre-exterior-02.png") => {
            bytes("200 OK", "image/png", EXTERIOR_IMAGE)
        }
        ("GET", "/health") => bytes("200 OK", "text/plain; charset=utf-8", b"ok\n"),
        _ if method == "GET" && is_repo_text_path(path) => repo_text(path),
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

fn yes_no_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    match value.trim().to_ascii_lowercase().as_str() {
        "yes" | "true" | "1" => Ok(true),
        "no" | "false" | "0" => Ok(false),
        other => Err(serde::de::Error::custom(format!(
            "expected yes/no boolean, got {other:?}"
        ))),
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

fn is_repo_text_path(path: &str) -> bool {
    path.starts_with("/docs/")
        || path.starts_with("/crates/osdc-portal/migrations/")
        || path.starts_with("/data/")
        || path.starts_with("/examples/")
        || path == "/README.md"
        || path == "/LICENSE.md"
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn repo_text(path: &str) -> Vec<u8> {
    let relative = path.trim_start_matches('/');
    if relative
        .split('/')
        .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return bytes("404 Not Found", "text/plain; charset=utf-8", b"not found\n");
    }

    let target = repo_root().join(relative);
    if !target.is_file() {
        return bytes("404 Not Found", "text/plain; charset=utf-8", b"not found\n");
    }

    match fs::read(&target) {
        Ok(body) => bytes("200 OK", text_content_type(&target), &body),
        Err(err) => bytes(
            "500 Internal Server Error",
            "text/plain; charset=utf-8",
            format!("failed to read repository file: {err}\n").as_bytes(),
        ),
    }
}

fn text_content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("csv") => "text/csv; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("md") => "text/markdown; charset=utf-8",
        Some("toml") => "text/plain; charset=utf-8",
        Some("yaml") | Some("yml") => "text/yaml; charset=utf-8",
        _ => "text/plain; charset=utf-8",
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HardwareProfile {
    #[serde(rename(deserialize = "profile_id"))]
    id: String,
    role: String,
    #[serde(rename(deserialize = "baseline_hardware"))]
    hardware: String,
    linux_stack: String,
    accelerator: String,
    #[serde(rename(deserialize = "ram_or_vram"))]
    memory: String,
    #[serde(rename(deserialize = "power_design_w"))]
    power_w: u16,
    #[serde(rename(deserialize = "unit_price_low_usd"))]
    price_low_usd: u16,
    #[serde(rename(deserialize = "unit_price_high_usd"))]
    price_high_usd: u16,
    default_use: String,
}

#[derive(Debug, Serialize)]
struct HardwareProvisioningOverview {
    metrics: Vec<LifecycleMetric>,
    connectors: Vec<SystemUiConnector>,
    pipeline: Vec<HardwareProvisioningStage>,
    profiles: Vec<HardwareProvisioningProfile>,
    requests: Vec<HardwareProvisioningRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemUiConnector {
    connector_id: String,
    system_name: String,
    domain: String,
    portal_surface: String,
    capability: String,
    adapter_pattern: String,
    endpoint_pattern: String,
    auth_model: String,
    write_mode: String,
    evidence_path: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HardwareProvisioningStage {
    stage_id: String,
    stage_name: String,
    purpose: String,
    primary_system: String,
    ui_action: String,
    automation_hook: String,
    evidence_path: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HardwareProvisioningProfile {
    profile_id: String,
    workload_class: String,
    node_role: String,
    hardware_profile: String,
    provisioner: String,
    os_image: String,
    network_profile: String,
    storage_profile: String,
    accelerator_profile: String,
    post_install: String,
    target_pool: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HardwareProvisioningRequest {
    request_id: String,
    requester: String,
    profile_id: String,
    count: String,
    site: String,
    rack_policy: String,
    network_zone: String,
    approval_gate: String,
    current_stage: String,
    target_environment: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudService {
    #[serde(rename(deserialize = "cloud_domain"))]
    domain: String,
    user_service: String,
    #[serde(rename(deserialize = "baseline_open_source_stack"))]
    stack: String,
    #[serde(rename(deserialize = "operator_source_of_truth"))]
    source_of_truth: String,
    #[serde(deserialize_with = "yes_no_bool")]
    tenant_visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoreCloudService {
    #[serde(rename(deserialize = "service_id"))]
    id: String,
    display_name: String,
    priority: String,
    aws_equivalent: String,
    azure_equivalent: String,
    open_source_stack: String,
    #[serde(deserialize_with = "yes_no_bool")]
    tenant_visible: bool,
    #[serde(deserialize_with = "yes_no_bool")]
    operator_visible: bool,
    #[serde(deserialize_with = "yes_no_bool")]
    provisionable: bool,
    default_shape: String,
    #[serde(rename(deserialize = "notes"), skip_serializing)]
    _notes: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    status_kind: String,
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
struct RemoteHandsPricebook {
    pricebook_id: String,
    task_class: String,
    billing_unit: String,
    included_response: String,
    target_response: String,
    requires_approval: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct CustomerOperationsOverview {
    metrics: Vec<LifecycleMetric>,
    accounts: Vec<CustomerAccount>,
    site_instances: Vec<CustomerSiteInstance>,
    workflows: Vec<CustomerOperationsWorkflow>,
    mfa_policies: Vec<IdentityMfaPolicy>,
    billing_plans: Vec<BillingPlan>,
    usage_meters: Vec<UsageMeter>,
    invoice_preview: Vec<InvoicePreview>,
    connectors: Vec<SystemUiConnector>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomerAccount {
    customer_id: String,
    display_name: String,
    customer_type: String,
    residency_zone: String,
    primary_region: String,
    identity_realm: String,
    billing_account: String,
    support_tier: String,
    service_owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomerSiteInstance {
    site_id: String,
    customer_id: String,
    country: String,
    city: String,
    deployment_stage: String,
    it_load_kw: u32,
    substrate: String,
    provisioner: String,
    data_residency_zone: String,
    source_of_truth: String,
    ops_owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CustomerOperationsWorkflow {
    workflow_id: String,
    workflow_name: String,
    customer_goal: String,
    connector_ids: String,
    required_mfa_policy: String,
    provisioning_system: String,
    billing_event: String,
    evidence_path: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IdentityMfaPolicy {
    policy_id: String,
    scope: String,
    provider_stack: String,
    factors: String,
    enrollment_flow: String,
    recovery_method: String,
    enforcement_point: String,
    evidence_path: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BillingPlan {
    plan_id: String,
    customer_segment: String,
    services_included: String,
    rating_engine: String,
    invoice_engine: String,
    currency: String,
    minimum_commit_usd: u32,
    tax_policy: String,
    approval_owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsageMeter {
    meter_id: String,
    service_domain: String,
    source_system: String,
    metric_name: String,
    unit: String,
    collection_cadence: String,
    rating_plan: String,
    evidence_path: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InvoicePreview {
    invoice_id: String,
    customer_id: String,
    billing_period: String,
    plan_id: String,
    usage_summary: String,
    amount_usd: f64,
    credits_usd: f64,
    tax_usd: f64,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessRole {
    access_role_id: String,
    role_name: String,
    scope: String,
    approval_owner: String,
    review_cadence: String,
    allowed_areas: String,
    evidence_path: String,
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

#[derive(Debug, Serialize, Deserialize)]
struct SiteSelectionCriterion {
    criterion_id: String,
    domain: String,
    criterion: String,
    minimum_expectation: String,
    preferred_expectation: String,
    weight: u8,
    red_flag: String,
    next_evidence: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PhysicalSecurityControl {
    control_id: String,
    zone: String,
    control: String,
    minimum_expectation: String,
    evidence: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SustainabilityMetric {
    metric_id: String,
    metric: String,
    boundary: String,
    cadence: String,
    evidence: String,
    stage: String,
    owner: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AiRackClass {
    rack_class_id: String,
    power_kw_range: String,
    cooling_mode: String,
    network_target: String,
    storage_target: String,
    facility_requirements: String,
    evidence_required: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EngineeringEvidence {
    evidence_id: String,
    domain: String,
    evidence_name: String,
    evidence_path: String,
    owner: String,
    stage: String,
    priority: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OperationsProcedure {
    procedure_id: String,
    procedure_type: String,
    procedure_name: String,
    doc_path: String,
    owner: String,
    review_cadence: String,
    criticality: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectGate {
    gate_id: String,
    phase: String,
    gate_name: String,
    exit_criteria: String,
    required_evidence: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorityPermit {
    permit_id: String,
    authority_area: String,
    permit_or_approval: String,
    applies: String,
    evidence_path: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeliveryRisk {
    risk_id: String,
    domain: String,
    risk: String,
    impact: String,
    likelihood: String,
    treatment: String,
    owner: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeliveryAction {
    action_id: String,
    source: String,
    action: String,
    owner: String,
    due_phase: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommissioningEvidence {
    evidence_id: String,
    level: String,
    test_name: String,
    evidence_path: String,
    owner: String,
    acceptance: String,
    criticality: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct LifecycleOverview {
    metrics: Vec<LifecycleMetric>,
    stages: Vec<LifecycleStage>,
    work_items: Vec<LifecycleWorkItem>,
    evidence: Vec<LifecycleEvidenceItem>,
    services: Vec<LifecycleServiceItem>,
    documents: Vec<LifecycleDocument>,
}

#[derive(Debug, Serialize)]
struct LifecycleMetric {
    label: String,
    value: String,
    detail: String,
    kind: &'static str,
}

#[derive(Debug, Serialize)]
struct LifecycleStage {
    phase: &'static str,
    gate_id: String,
    gate_name: String,
    owner: String,
    status: String,
    status_kind: &'static str,
    evidence_path: String,
    focus: &'static str,
}

#[derive(Debug, Serialize)]
struct LifecycleWorkItem {
    item_type: &'static str,
    id: String,
    phase: String,
    title: String,
    owner: String,
    priority: String,
    status: String,
    status_kind: &'static str,
    evidence_path: String,
}

#[derive(Debug, Serialize)]
struct LifecycleEvidenceItem {
    source: &'static str,
    id: String,
    domain: String,
    title: String,
    owner: String,
    status: String,
    status_kind: &'static str,
    artifact: String,
}

#[derive(Debug, Serialize)]
struct LifecycleServiceItem {
    service_id: String,
    category: String,
    interface: String,
    implementation: String,
    workflow: String,
    status: String,
    status_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct LifecycleDocument {
    area: &'static str,
    title: &'static str,
    path: &'static str,
    purpose: &'static str,
}

#[derive(Debug, Serialize)]
struct DeveloperPlatform {
    metrics: Vec<LifecycleMetric>,
    services: Vec<DeveloperPlatformService>,
    templates: Vec<DeveloperTemplate>,
    environments: Vec<DeploymentEnvironment>,
    promotion_gates: Vec<DeveloperPromotionGate>,
    vscode_workflows: Vec<VsCodeWorkflow>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeveloperPlatformService {
    service_id: String,
    function: String,
    default_stack: String,
    operator_surface: String,
    developer_surface: String,
    vs_code_integration: String,
    control: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeveloperTemplate {
    template_id: String,
    name: String,
    language: String,
    repo_path: String,
    devcontainer_path: String,
    pipeline_path: String,
    deploy_path: String,
    runtime: String,
    owner: String,
    status: String,
    vscode_clone_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeploymentEnvironment {
    environment_id: String,
    name: String,
    cluster: String,
    namespace_pattern: String,
    gitops_tool: String,
    approval_policy: String,
    rollback_policy: String,
    observability_url: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeveloperPromotionGate {
    gate_id: String,
    from_environment: String,
    to_environment: String,
    required_checks: String,
    approver: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VsCodeWorkflow {
    workflow_id: String,
    action: String,
    vs_code_surface: String,
    command_or_uri: String,
    artifact_path: String,
    portal_action: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct DataPlatformOverview {
    metrics: Vec<LifecycleMetric>,
    services: Vec<DataPlatformService>,
    products: Vec<DataProduct>,
    pipelines: Vec<DataPipeline>,
    ontology: Vec<DataOntologyObject>,
    access_policies: Vec<DataAccessPolicy>,
    templates: Vec<DataPlatformTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataPlatformService {
    service_id: String,
    function: String,
    default_stack: String,
    portal_surface: String,
    user_surface: String,
    control: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataProduct {
    product_id: String,
    name: String,
    domain: String,
    owner: String,
    source_systems: String,
    lakehouse_table: String,
    ontology_object: String,
    quality_gate: String,
    access_policy: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataPipeline {
    pipeline_id: String,
    name: String,
    engine: String,
    source: String,
    target: String,
    schedule: String,
    owner: String,
    quality_gate: String,
    gitops_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataOntologyObject {
    object_id: String,
    name: String,
    domain: String,
    description: String,
    source_products: String,
    relationships: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataAccessPolicy {
    policy_id: String,
    scope: String,
    subject: String,
    allowed_actions: String,
    conditions: String,
    enforcement_point: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataPlatformTemplate {
    template_id: String,
    name: String,
    template_type: String,
    repo_path: String,
    devcontainer_path: String,
    pipeline_path: String,
    preview_surface: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct AssuranceOverview {
    metrics: Vec<LifecycleMetric>,
    automation_jobs: Vec<AssuranceAutomationJob>,
    test_harnesses: Vec<TestHarness>,
    upgrade_rings: Vec<UpgradeRing>,
    upgrade_gates: Vec<UpgradeTestGate>,
    threat_stack: Vec<ThreatManagementComponent>,
    scanner_coverage: Vec<ScannerCoverage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssuranceAutomationJob {
    job_id: String,
    purpose: String,
    command: String,
    trigger: String,
    required_inputs: String,
    evidence_output: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestHarness {
    test_id: String,
    function_area: String,
    target: String,
    test_type: String,
    tool_stack: String,
    trigger: String,
    required_evidence: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpgradeRing {
    ring_id: String,
    scope: String,
    cadence: String,
    entry_criteria: String,
    automated_tests: String,
    promotion_gate: String,
    rollback_strategy: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpgradeTestGate {
    gate_id: String,
    stage: String,
    applies_to: String,
    required_checks: String,
    automation_tool: String,
    evidence_path: String,
    blocking: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ThreatManagementComponent {
    component_id: String,
    capability: String,
    wiz_like_function: String,
    open_source_stack: String,
    integration_surface: String,
    evidence_path: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScannerCoverage {
    scanner_id: String,
    scan_domain: String,
    target: String,
    default_tool: String,
    trigger: String,
    output: String,
    aggregation: String,
    evidence_path: String,
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

#[derive(Debug, Serialize, Deserialize)]
struct UpgradePolicy {
    update_class: String,
    frequency: String,
    target_window: String,
    required_gates: String,
    approval_owner: String,
    rollback_requirement: String,
    #[serde(rename(deserialize = "notes"), skip_serializing)]
    _notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigScript {
    #[serde(rename(deserialize = "script_id"))]
    id: String,
    tool: String,
    path: String,
    owner: String,
    language: String,
    edit_mode: String,
    validation_command: String,
    rollout_target: String,
    risk: String,
    notes: String,
    #[serde(default)]
    content: String,
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

#[derive(Debug, Serialize, Deserialize)]
struct EdgeShieldService {
    #[serde(rename(deserialize = "service_id"))]
    id: String,
    cloudflare_equivalent: String,
    open_source_stack: String,
    radxa_role: String,
    criticality: String,
    #[serde(rename(deserialize = "default_port_or_protocol"))]
    protocol: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    status_kind: String,
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
    deployment_profiles: Vec<DeploymentStackProfile>,
}

#[derive(Debug, Serialize)]
struct InfrastructureWorkbench {
    metrics: Vec<LifecycleMetric>,
    workflows: Vec<InfrastructureWorkflow>,
    adapter_milestones: Vec<LiveAdapterMilestone>,
    adapter_proofs: Vec<LiveAdapterProof>,
    persistence_schema: PortalPersistenceSchema,
    stack_profiles: Vec<DeploymentStackProfile>,
    connectors: Vec<SystemUiConnector>,
    test_harnesses: Vec<TestHarness>,
    upgrade_gates: Vec<UpgradeTestGate>,
    automation_jobs: Vec<AssuranceAutomationJob>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InfrastructureWorkflow {
    workflow_id: String,
    workflow_name: String,
    user_goal: String,
    primary_surface: String,
    service_domain: String,
    default_substrate: String,
    connector_ids: String,
    required_test_ids: String,
    required_gate_ids: String,
    automation_job_id: String,
    evidence_path: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LiveAdapterMilestone {
    milestone_id: String,
    priority: String,
    adapter_target: String,
    backend_system: String,
    connector_id: String,
    initial_mode: String,
    production_write_path: String,
    first_capability: String,
    workflow_ids: String,
    proof_command: String,
    evidence_path: String,
    owner: String,
    status: String,
    next_step: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LiveAdapterProof {
    proof_id: String,
    milestone_id: String,
    adapter_target: String,
    proof_command: String,
    mode: String,
    scope: String,
    required_env: String,
    evidence_output: String,
    required_gate: String,
    owner: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct PortalPersistenceSchema {
    migration_path: &'static str,
    docs_path: &'static str,
    schema_name: &'static str,
    boundary: &'static str,
    metrics: Vec<LifecycleMetric>,
    tables: Vec<PortalPersistenceTable>,
    indexes: Vec<String>,
}

#[derive(Debug, Serialize)]
struct PortalPersistenceTable {
    table_name: String,
    purpose: &'static str,
    column_count: usize,
    columns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CostScenario {
    #[serde(rename(deserialize = "scenario_id"))]
    id: String,
    name: String,
    it_load_kw: u32,
    racks: u16,
    building_area_m2: u32,
    core_facility_low_usd: u32,
    core_facility_high_usd: u32,
    starter_it_low_usd: u32,
    starter_it_high_usd: u32,
    #[serde(rename(deserialize = "dc_microgrid_included_usd"), skip_serializing)]
    _dc_microgrid_included_usd: String,
    total_with_it_low_usd: u32,
    total_with_it_high_usd: u32,
    build_time_low_weeks: u16,
    build_time_high_weeks: u16,
    default_building_system: String,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CostCategory {
    scenario_id: String,
    category: String,
    low_usd: u32,
    high_usd: u32,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceBasis {
    item_family: String,
    unit: String,
    low_usd: f64,
    high_usd: f64,
    planning_selected_usd: f64,
    source_marketplace: String,
    #[serde(rename(deserialize = "source_note"), skip_serializing)]
    _source_note: String,
    project_use: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeploymentStackProfile {
    profile_id: String,
    stage: String,
    it_load_kw: u32,
    default_cloud_substrate: String,
    alternate_cloud_substrate: String,
    storage_substrate: String,
    container_substrate: String,
    bare_metal_lifecycle: String,
    source_of_truth: String,
    edge_security: String,
    developer_gitops: String,
    operations_focus: String,
    when_to_use: String,
    maturity: String,
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
    csv_rows(
        HARDWARE_COMPUTE_BASELINE_CSV,
        "data/hardware/compute-baseline-2026.csv",
    )
}

fn system_ui_connectors() -> Vec<SystemUiConnector> {
    csv_rows(
        SYSTEM_UI_CONNECTORS_CSV,
        "data/software/system-ui-connectors.csv",
    )
}

fn hardware_provisioning_pipeline() -> Vec<HardwareProvisioningStage> {
    csv_rows(
        HARDWARE_PROVISIONING_PIPELINE_CSV,
        "data/hardware/provisioning-pipeline.csv",
    )
}

fn hardware_provisioning_profiles() -> Vec<HardwareProvisioningProfile> {
    csv_rows(
        HARDWARE_PROVISIONING_PROFILES_CSV,
        "data/hardware/provisioning-profiles.csv",
    )
}

fn hardware_provisioning_requests() -> Vec<HardwareProvisioningRequest> {
    csv_rows(
        HARDWARE_PROVISIONING_REQUESTS_CSV,
        "data/hardware/provisioning-requests.csv",
    )
}

fn hardware_provisioning_overview() -> HardwareProvisioningOverview {
    let connectors: Vec<SystemUiConnector> = system_ui_connectors()
        .into_iter()
        .filter(|connector| {
            connector.domain == "hardware" || connector.portal_surface.contains("/hardware")
        })
        .collect();
    let pipeline = hardware_provisioning_pipeline();
    let profiles = hardware_provisioning_profiles();
    let requests = hardware_provisioning_requests();
    let active_requests = requests
        .iter()
        .filter(|request| !matches!(request.status.as_str(), "closed" | "template"))
        .count();
    let guarded_connectors = connectors
        .iter()
        .filter(|connector| connector.write_mode.contains("guarded"))
        .count();
    let provisioner_count = profiles
        .iter()
        .map(|profile| profile.provisioner.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len();

    HardwareProvisioningOverview {
        metrics: vec![
            LifecycleMetric {
                label: "Profiles".to_string(),
                value: profiles.len().to_string(),
                detail: "edge cloud storage gpu data ot".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Provisioners".to_string(),
                value: provisioner_count.to_string(),
                detail: "MAAS Ironic Metal3 Tinkerbell".to_string(),
                kind: "info",
            },
            LifecycleMetric {
                label: "Active requests".to_string(),
                value: active_requests.to_string(),
                detail: "review pilot in-progress".to_string(),
                kind: "warn",
            },
            LifecycleMetric {
                label: "Guarded actions".to_string(),
                value: guarded_connectors.to_string(),
                detail: "power firmware scan controls".to_string(),
                kind: "danger",
            },
        ],
        connectors,
        pipeline,
        profiles,
        requests,
    }
}

fn service_catalog() -> Vec<CloudService> {
    csv_rows(
        OPEN_CLOUD_SERVICE_MAP_CSV,
        "data/software/open-cloud-service-map.csv",
    )
}

fn edge_shield_services() -> Vec<EdgeShieldService> {
    let mut services: Vec<EdgeShieldService> = csv_rows(
        EDGE_SHIELD_SERVICES_CSV,
        "data/software/edge-shield-services.csv",
    );
    for service in &mut services {
        match service.id.as_str() {
            "edge_waf" => {
                service.status = "detection".to_string();
                service.status_kind = "info".to_string();
            }
            _ if service.criticality == "optional" => {
                service.status = "preview".to_string();
                service.status_kind = "info".to_string();
            }
            _ => {
                service.status = "ready".to_string();
                service.status_kind = "normal".to_string();
            }
        }
    }
    services
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
        deployment_profiles: deployment_stack_profiles(),
    }
}

fn cost_scenarios() -> Vec<CostScenario> {
    csv_rows(SCENARIO_COSTS_CSV, "data/costing/scenario-costs-2026.csv")
}
fn cost_categories() -> Vec<CostCategory> {
    csv_rows(
        SCENARIO_CATEGORY_COSTS_CSV,
        "data/costing/scenario-category-costs-2026.csv",
    )
}
fn price_basis() -> Vec<PriceBasis> {
    csv_rows(
        MARKETPLACE_PRICE_BASIS_CSV,
        "data/costing/marketplace-price-basis-2026.csv",
    )
}

fn deployment_stack_profiles() -> Vec<DeploymentStackProfile> {
    csv_rows(
        DEPLOYMENT_STACK_PROFILES_CSV,
        "data/software/deployment-stack-profiles.csv",
    )
}

fn infrastructure_workflows() -> Vec<InfrastructureWorkflow> {
    csv_rows(
        INFRASTRUCTURE_WORKFLOWS_CSV,
        "data/software/infrastructure-workflows.csv",
    )
}

fn live_adapter_roadmap() -> Vec<LiveAdapterMilestone> {
    csv_rows(
        LIVE_ADAPTER_ROADMAP_CSV,
        "data/software/live-adapter-roadmap.csv",
    )
}

fn live_adapter_proofs() -> Vec<LiveAdapterProof> {
    csv_rows(
        LIVE_ADAPTER_PROOF_CATALOGUE_CSV,
        "data/software/live-adapter-proof-catalogue.csv",
    )
}

fn portal_persistence_schema() -> PortalPersistenceSchema {
    let tables = parse_migration_tables(PORTAL_STATE_MIGRATION_SQL);
    let indexes = parse_migration_indexes(PORTAL_STATE_MIGRATION_SQL);
    let jsonb_tables = tables
        .iter()
        .filter(|table| table.columns.iter().any(|column| column.contains("jsonb")))
        .count();

    PortalPersistenceSchema {
        migration_path: "crates/osdc-portal/migrations/0001_osdc_portal_state.sql",
        docs_path: "docs/architecture/portal-persistence.md",
        schema_name: "osdc_portal",
        boundary: "OSDC-owned workflow state only; external systems remain authoritative for their own domains.",
        metrics: vec![
            LifecycleMetric {
                label: "State tables".to_string(),
                value: tables.len().to_string(),
                detail: "requests approvals evidence audit".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Indexes".to_string(),
                value: indexes.len().to_string(),
                detail: "status owner workflow lookup".to_string(),
                kind: "info",
            },
            LifecycleMetric {
                label: "JSONB tables".to_string(),
                value: jsonb_tables.to_string(),
                detail: "plans summaries payloads".to_string(),
                kind: "info",
            },
        ],
        tables,
        indexes,
    }
}

fn parse_migration_tables(sql: &str) -> Vec<PortalPersistenceTable> {
    let mut tables = Vec::new();
    let mut current: Option<(String, Vec<String>)> = None;

    for line in sql.lines() {
        let trimmed = line.trim();
        if trimmed
            .to_ascii_uppercase()
            .starts_with("CREATE TABLE IF NOT EXISTS ")
        {
            let table_name = trimmed
                .split_whitespace()
                .nth(5)
                .unwrap_or_default()
                .trim_end_matches('(')
                .to_string();
            current = Some((table_name, Vec::new()));
            continue;
        }

        let Some((table_name, columns)) = current.as_mut() else {
            continue;
        };

        if trimmed.starts_with(");") {
            let table_name = table_name.clone();
            let columns = columns.clone();
            tables.push(PortalPersistenceTable {
                purpose: portal_table_purpose(&table_name),
                column_count: columns.len(),
                table_name,
                columns,
            });
            current = None;
            continue;
        }

        if trimmed.is_empty() || trimmed.starts_with("--") {
            continue;
        }

        let column_name = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .trim_matches('"')
            .trim_end_matches(',');
        if !column_name.is_empty() {
            columns.push(format!("{column_name} {}", column_type_summary(trimmed)));
        }
    }

    tables
}

fn parse_migration_indexes(sql: &str) -> Vec<String> {
    sql.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed
                .to_ascii_uppercase()
                .starts_with("CREATE INDEX IF NOT EXISTS ")
            {
                return None;
            }
            trimmed.split_whitespace().nth(5).map(str::to_string)
        })
        .collect()
}

fn column_type_summary(line: &str) -> String {
    line.split_whitespace()
        .skip(1)
        .take_while(|part| {
            !matches!(
                part.trim_end_matches(',').to_ascii_uppercase().as_str(),
                "PRIMARY" | "REFERENCES" | "NOT" | "DEFAULT"
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
        .trim_end_matches(',')
        .to_string()
}

fn portal_table_purpose(table_name: &str) -> &'static str {
    match table_name.rsplit('.').next().unwrap_or(table_name) {
        "change_requests" => {
            "GitOps or guarded-API change envelope, risk, validation, rollout, rollback, and status."
        }
        "approval_records" => {
            "Approval owner, decision, approver, decision timestamp, notes, and evidence reference."
        }
        "evidence_bundles" => {
            "Assurance, test, scanner, adapter-proof, and commissioning evidence attached to workflows."
        }
        "audit_events" => {
            "Operational audit trail for request creation, validation, approval, rollout, rollback, and closeout."
        }
        "infrastructure_requests" => {
            "Front-door workbench requests for infrastructure, hardware, edge, data, upgrade, and scan workflows."
        }
        "adapter_proof_runs" => {
            "Local and future live adapter proof executions, modes, statuses, summaries, and evidence paths."
        }
        "customer_accounts" => {
            "OSDC customer account workflow state, service owner, identity realm, billing account, support tier, and status."
        }
        "customer_site_instances" => {
            "Customer datacentre or site-instance state, deployment stage, substrate, provisioner, residency zone, and owner."
        }
        "identity_mfa_policies" => {
            "Open-source MFA policy catalogue state for factors, recovery, enforcement points, evidence, owner, and status."
        }
        "usage_meter_snapshots" => {
            "Customer usage snapshots imported from metering systems for rating, evidence, billing period, and approval."
        }
        "invoice_previews" => {
            "Draft invoice previews, customer period, plan, credits, taxes, approval reference, and release status."
        }
        _ => "Portal persistence table.",
    }
}

fn infrastructure_workbench() -> InfrastructureWorkbench {
    let workflows = infrastructure_workflows();
    let adapter_milestones = live_adapter_roadmap();
    let adapter_proofs = live_adapter_proofs();
    let connectors = system_ui_connectors();
    let test_harnesses = test_harnesses();
    let upgrade_gates = upgrade_test_gates();
    let automation_jobs = assurance_automation_jobs();
    let implemented_jobs = automation_jobs
        .iter()
        .filter(|job| job.status == "implemented")
        .count();
    let guarded_connectors = connectors
        .iter()
        .filter(|connector| {
            connector.write_mode.contains("guarded") || connector.write_mode.contains("approval")
        })
        .count();
    let blocking_gates = upgrade_gates
        .iter()
        .filter(|gate| gate.blocking == "yes")
        .count();

    InfrastructureWorkbench {
        metrics: vec![
            LifecycleMetric {
                label: "Workflows".to_string(),
                value: workflows.len().to_string(),
                detail: "create manage upgrade scan".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Guarded connectors".to_string(),
                value: guarded_connectors.to_string(),
                detail: "approval before risky writes".to_string(),
                kind: "warn",
            },
            LifecycleMetric {
                label: "Test harnesses".to_string(),
                value: test_harnesses.len().to_string(),
                detail: "mapped to user actions".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Adapter milestones".to_string(),
                value: adapter_milestones.len().to_string(),
                detail: "read-first integrations".to_string(),
                kind: "warn",
            },
            LifecycleMetric {
                label: "Adapter proofs".to_string(),
                value: adapter_proofs.len().to_string(),
                detail: "local plan-mode harnesses".to_string(),
                kind: "info",
            },
            LifecycleMetric {
                label: "Blocking gates".to_string(),
                value: blocking_gates.to_string(),
                detail: format!("{implemented_jobs} runnable jobs").to_string(),
                kind: "info",
            },
        ],
        workflows,
        adapter_milestones,
        adapter_proofs,
        persistence_schema: portal_persistence_schema(),
        stack_profiles: deployment_stack_profiles(),
        connectors,
        test_harnesses,
        upgrade_gates,
        automation_jobs,
    }
}

fn core_cloud_services() -> Vec<CoreCloudService> {
    let mut services: Vec<CoreCloudService> = csv_rows(
        CORE_CLOUD_SERVICES_CSV,
        "data/software/core-cloud-services.csv",
    );
    for service in &mut services {
        if service.priority == "foundation" {
            service.status = "implemented".to_string();
            service.status_kind = "normal".to_string();
        } else {
            service.status = "preview".to_string();
            service.status_kind = "info".to_string();
        }
    }
    services
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

fn remote_hands_pricebook() -> Vec<RemoteHandsPricebook> {
    csv_rows(
        REMOTE_HANDS_PRICEBOOK_CSV,
        "data/commercial/remote-hands-pricebook.csv",
    )
}

fn customer_accounts() -> Vec<CustomerAccount> {
    csv_rows(
        CUSTOMER_ACCOUNTS_CSV,
        "data/commercial/customer-accounts.csv",
    )
}

fn customer_sites() -> Vec<CustomerSiteInstance> {
    csv_rows(CUSTOMER_SITES_CSV, "data/commercial/customer-sites.csv")
}

fn customer_operations_workflows() -> Vec<CustomerOperationsWorkflow> {
    csv_rows(
        CUSTOMER_OPERATIONS_WORKFLOWS_CSV,
        "data/software/customer-operations-workflows.csv",
    )
}

fn identity_mfa_policies() -> Vec<IdentityMfaPolicy> {
    csv_rows(
        IDENTITY_MFA_POLICIES_CSV,
        "data/software/identity-mfa-policies.csv",
    )
}

fn billing_plans() -> Vec<BillingPlan> {
    csv_rows(BILLING_PLANS_CSV, "data/commercial/billing-plans.csv")
}

fn usage_meters() -> Vec<UsageMeter> {
    csv_rows(USAGE_METERS_CSV, "data/commercial/usage-meters.csv")
}

fn invoice_preview() -> Vec<InvoicePreview> {
    csv_rows(INVOICE_PREVIEW_CSV, "data/commercial/invoice-preview.csv")
}

fn customer_operations_overview() -> CustomerOperationsOverview {
    let accounts = customer_accounts();
    let site_instances = customer_sites();
    let workflows = customer_operations_workflows();
    let mfa_policies = identity_mfa_policies();
    let billing_plans = billing_plans();
    let usage_meters = usage_meters();
    let invoice_preview = invoice_preview();
    let connector_ids = workflows
        .iter()
        .flat_map(|workflow| workflow.connector_ids.split('+'))
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    let connectors = system_ui_connectors()
        .into_iter()
        .filter(|connector| {
            connector.portal_surface.contains("/customers")
                || connector_ids.contains(connector.connector_id.as_str())
        })
        .collect::<Vec<_>>();
    let active_accounts = accounts
        .iter()
        .filter(|account| matches!(account.status.as_str(), "active" | "onboarding" | "pilot"))
        .count();
    let enforced_mfa = mfa_policies
        .iter()
        .filter(|policy| matches!(policy.status.as_str(), "pilot" | "implemented"))
        .count();
    let open_invoice_total = invoice_preview
        .iter()
        .filter(|invoice| !is_closed_status(&invoice.status))
        .map(|invoice| invoice.amount_usd - invoice.credits_usd + invoice.tax_usd)
        .sum::<f64>();

    CustomerOperationsOverview {
        metrics: vec![
            LifecycleMetric {
                label: "Customers".to_string(),
                value: active_accounts.to_string(),
                detail: "active onboarding pilot".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Customer sites".to_string(),
                value: site_instances.len().to_string(),
                detail: "regional edge research".to_string(),
                kind: "info",
            },
            LifecycleMetric {
                label: "MFA policies".to_string(),
                value: enforced_mfa.to_string(),
                detail: "pilot or implemented".to_string(),
                kind: "warn",
            },
            LifecycleMetric {
                label: "Billable meters".to_string(),
                value: usage_meters.len().to_string(),
                detail: "compute storage AI network".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Draft invoices".to_string(),
                value: format!("${open_invoice_total:.0}"),
                detail: "before credits approval".to_string(),
                kind: "info",
            },
        ],
        accounts,
        site_instances,
        workflows,
        mfa_policies,
        billing_plans,
        usage_meters,
        invoice_preview,
        connectors,
    }
}

fn access_roles() -> Vec<AccessRole> {
    csv_rows(ACCESS_ROLES_CSV, "data/commercial/access-roles.csv")
}

fn audit_evidence() -> Vec<AuditEvidence> {
    csv_rows(AUDIT_EVIDENCE_CSV, "data/commercial/audit-evidence.csv")
}

fn site_selection_scorecard() -> Vec<SiteSelectionCriterion> {
    csv_rows(
        SITE_SELECTION_SCORECARD_CSV,
        "data/site-selection/site-selection-scorecard.csv",
    )
}

fn physical_security_controls() -> Vec<PhysicalSecurityControl> {
    csv_rows(
        PHYSICAL_SECURITY_CONTROLS_CSV,
        "data/security/physical-security-controls.csv",
    )
}

fn sustainability_metrics() -> Vec<SustainabilityMetric> {
    csv_rows(
        SUSTAINABILITY_METRICS_CSV,
        "data/sustainability/sustainability-metrics.csv",
    )
}

fn ai_rack_classes() -> Vec<AiRackClass> {
    csv_rows(
        AI_RACK_CLASSES_CSV,
        "data/ai-ready/high-density-rack-classes.csv",
    )
}

fn engineering_evidence() -> Vec<EngineeringEvidence> {
    csv_rows(
        ENGINEERING_EVIDENCE_CSV,
        "data/engineering/engineering-evidence-register.csv",
    )
}

fn operations_procedures() -> Vec<OperationsProcedure> {
    csv_rows(
        OPERATIONS_PROCEDURES_CSV,
        "data/operations/procedure-catalogue.csv",
    )
}

fn project_gates() -> Vec<ProjectGate> {
    csv_rows(PROJECT_GATES_CSV, "data/delivery/project-gates.csv")
}

fn authority_permits() -> Vec<AuthorityPermit> {
    csv_rows(AUTHORITY_PERMITS_CSV, "data/delivery/authority-permits.csv")
}

fn delivery_risks() -> Vec<DeliveryRisk> {
    csv_rows(DELIVERY_RISKS_CSV, "data/delivery/risk-register.csv")
}

fn delivery_actions() -> Vec<DeliveryAction> {
    csv_rows(DELIVERY_ACTIONS_CSV, "data/delivery/action-tracker.csv")
}

fn commissioning_evidence() -> Vec<CommissioningEvidence> {
    csv_rows(
        COMMISSIONING_EVIDENCE_CSV,
        "data/commissioning/commissioning-evidence-register.csv",
    )
}

fn developer_platform_services() -> Vec<DeveloperPlatformService> {
    csv_rows(
        DEVELOPER_PLATFORM_SERVICES_CSV,
        "data/software/developer-platform-services.csv",
    )
}

fn developer_templates() -> Vec<DeveloperTemplate> {
    csv_rows(
        DEVELOPER_TEMPLATES_CSV,
        "data/software/developer-templates.csv",
    )
}

fn deployment_environments() -> Vec<DeploymentEnvironment> {
    csv_rows(
        DEPLOYMENT_ENVIRONMENTS_CSV,
        "data/software/deployment-environments.csv",
    )
}

fn developer_promotion_gates() -> Vec<DeveloperPromotionGate> {
    csv_rows(
        DEVELOPER_PROMOTION_GATES_CSV,
        "data/software/developer-promotion-gates.csv",
    )
}

fn vscode_workflows() -> Vec<VsCodeWorkflow> {
    csv_rows(VSCODE_WORKFLOWS_CSV, "data/software/vscode-workflows.csv")
}

fn developer_platform() -> DeveloperPlatform {
    let services = developer_platform_services();
    let templates = developer_templates();
    let environments = deployment_environments();
    let promotion_gates = developer_promotion_gates();
    let vscode_workflows = vscode_workflows();

    let production_ready = services
        .iter()
        .filter(|service| service.status == "production-baseline")
        .count();

    DeveloperPlatform {
        metrics: vec![
            LifecycleMetric {
                label: "Developer services".to_string(),
                value: services.len().to_string(),
                detail: "forge CI registry GitOps IaC".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Templates".to_string(),
                value: templates.len().to_string(),
                detail: "VS Code ready starters".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Environments".to_string(),
                value: environments.len().to_string(),
                detail: "dev staging prod edge IaC".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Production baselines".to_string(),
                value: production_ready.to_string(),
                detail: "registry GitOps IaC controls".to_string(),
                kind: "info",
            },
        ],
        services,
        templates,
        environments,
        promotion_gates,
        vscode_workflows,
    }
}

fn data_platform_services() -> Vec<DataPlatformService> {
    csv_rows(
        DATA_PLATFORM_SERVICES_CSV,
        "data/software/data-platform-services.csv",
    )
}

fn data_products() -> Vec<DataProduct> {
    csv_rows(DATA_PRODUCTS_CSV, "data/software/data-products.csv")
}

fn data_pipelines() -> Vec<DataPipeline> {
    csv_rows(DATA_PIPELINES_CSV, "data/software/data-pipelines.csv")
}

fn data_ontology_objects() -> Vec<DataOntologyObject> {
    csv_rows(
        DATA_ONTOLOGY_OBJECTS_CSV,
        "data/software/data-ontology-objects.csv",
    )
}

fn data_access_policies() -> Vec<DataAccessPolicy> {
    csv_rows(
        DATA_ACCESS_POLICIES_CSV,
        "data/software/data-access-policies.csv",
    )
}

fn data_platform_templates() -> Vec<DataPlatformTemplate> {
    csv_rows(
        DATA_PLATFORM_TEMPLATES_CSV,
        "data/software/data-platform-templates.csv",
    )
}

fn data_platform_overview() -> DataPlatformOverview {
    let services = data_platform_services();
    let products = data_products();
    let pipelines = data_pipelines();
    let ontology = data_ontology_objects();
    let access_policies = data_access_policies();
    let templates = data_platform_templates();

    let production_baseline = services
        .iter()
        .filter(|service| service.status == "production-baseline")
        .count();

    DataPlatformOverview {
        metrics: vec![
            LifecycleMetric {
                label: "Platform services".to_string(),
                value: services.len().to_string(),
                detail: "ingest lakehouse catalog apps AI".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Data products".to_string(),
                value: products.len().to_string(),
                detail: "governed domain products".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Pipelines".to_string(),
                value: pipelines.len().to_string(),
                detail: "ingest transform index".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Production baselines".to_string(),
                value: production_baseline.to_string(),
                detail: "lakehouse query catalog".to_string(),
                kind: "info",
            },
        ],
        services,
        products,
        pipelines,
        ontology,
        access_policies,
        templates,
    }
}

fn test_harnesses() -> Vec<TestHarness> {
    csv_rows(
        TEST_HARNESS_CATALOGUE_CSV,
        "data/software/test-harness-catalogue.csv",
    )
}

fn upgrade_rings() -> Vec<UpgradeRing> {
    csv_rows(UPGRADE_RINGS_CSV, "data/software/upgrade-rings.csv")
}

fn upgrade_test_gates() -> Vec<UpgradeTestGate> {
    csv_rows(
        UPGRADE_TEST_GATES_CSV,
        "data/software/upgrade-test-gates.csv",
    )
}

fn threat_management_stack() -> Vec<ThreatManagementComponent> {
    csv_rows(
        THREAT_MANAGEMENT_STACK_CSV,
        "data/security/threat-management-stack.csv",
    )
}

fn scanner_coverage() -> Vec<ScannerCoverage> {
    csv_rows(SCANNER_COVERAGE_CSV, "data/security/scanner-coverage.csv")
}

fn assurance_automation_jobs() -> Vec<AssuranceAutomationJob> {
    csv_rows(
        ASSURANCE_AUTOMATION_JOBS_CSV,
        "data/software/assurance-automation-jobs.csv",
    )
}

fn assurance_overview() -> AssuranceOverview {
    let automation_jobs = assurance_automation_jobs();
    let test_harnesses = test_harnesses();
    let upgrade_rings = upgrade_rings();
    let upgrade_gates = upgrade_test_gates();
    let threat_stack = threat_management_stack();
    let scanner_coverage = scanner_coverage();

    let blocking_gates = upgrade_gates
        .iter()
        .filter(|gate| gate.blocking.eq_ignore_ascii_case("yes"))
        .count();
    let threat_components = threat_stack
        .iter()
        .filter(|component| component.status != "retired")
        .count();

    AssuranceOverview {
        metrics: vec![
            LifecycleMetric {
                label: "Test harnesses".to_string(),
                value: test_harnesses.len().to_string(),
                detail: "software security data facility".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Upgrade rings".to_string(),
                value: upgrade_rings.len().to_string(),
                detail: "dev staging canary prod edge".to_string(),
                kind: "normal",
            },
            LifecycleMetric {
                label: "Blocking gates".to_string(),
                value: blocking_gates.to_string(),
                detail: "automated promotion controls".to_string(),
                kind: "warn",
            },
            LifecycleMetric {
                label: "Threat coverage".to_string(),
                value: threat_components.to_string(),
                detail: "open Wiz-style components".to_string(),
                kind: "info",
            },
        ],
        automation_jobs,
        test_harnesses,
        upgrade_rings,
        upgrade_gates,
        threat_stack,
        scanner_coverage,
    }
}

fn is_closed_status(status: &str) -> bool {
    matches!(
        status.to_ascii_lowercase().as_str(),
        "approved" | "baseline" | "closed" | "complete" | "implemented" | "ready" | "retired"
    )
}

fn status_kind(status: &str) -> &'static str {
    match status.to_ascii_lowercase().as_str() {
        "blocked" | "danger" | "failed" => "danger",
        "open" | "pending" | "scheduled" | "template" => "warn",
        "draft" | "in-progress" | "pilot" | "preview" | "review" | "testing" => "info",
        _ => "normal",
    }
}

fn priority_kind(priority: &str) -> &'static str {
    match priority.to_ascii_lowercase().as_str() {
        "critical" | "high" => "danger",
        "medium" => "warn",
        "low" => "info",
        _ => "normal",
    }
}

fn stage_from_gate(
    gates: &[ProjectGate],
    gate_id: &str,
    phase: &'static str,
    focus: &'static str,
) -> LifecycleStage {
    let gate = gates
        .iter()
        .find(|gate| gate.gate_id == gate_id)
        .unwrap_or_else(|| panic!("lifecycle gate {gate_id} must exist"));

    LifecycleStage {
        phase,
        gate_id: gate.gate_id.clone(),
        gate_name: gate.gate_name.clone(),
        owner: gate.owner.clone(),
        status: gate.status.clone(),
        status_kind: status_kind(&gate.status),
        evidence_path: gate.required_evidence.clone(),
        focus,
    }
}

fn lifecycle_documents() -> Vec<LifecycleDocument> {
    vec![
        LifecycleDocument {
            area: "strategy",
            title: "Sovereign datacentre mission",
            path: "docs/strategy/sovereign-datacentre-mission.md",
            purpose: "national scope and project boundary",
        },
        LifecycleDocument {
            area: "planning",
            title: "Country site profile guide",
            path: "docs/deployment/country-site-profile-guide.md",
            purpose: "country pack and site-planning fields",
        },
        LifecycleDocument {
            area: "delivery",
            title: "Project lifecycle gates",
            path: "docs/delivery/project-lifecycle-gates.md",
            purpose: "phase gate control from concept to handover",
        },
        LifecycleDocument {
            area: "delivery",
            title: "Authority permit register",
            path: "docs/delivery/authority-permit-register.md",
            purpose: "approval and authority tracking",
        },
        LifecycleDocument {
            area: "design",
            title: "Design freeze readiness",
            path: "docs/delivery/design-freeze-readiness.md",
            purpose: "minimum evidence before procurement and construction",
        },
        LifecycleDocument {
            area: "engineering",
            title: "Electrical single-line",
            path: "docs/engineering/electrical-single-line-250kw.md",
            purpose: "first electrical design evidence target",
        },
        LifecycleDocument {
            area: "engineering",
            title: "Thermal design basis",
            path: "docs/engineering/thermal-design-basis.md",
            purpose: "cooling and rack heat-capture basis",
        },
        LifecycleDocument {
            area: "commissioning",
            title: "Commissioning overview",
            path: "docs/commissioning/commissioning-overview.md",
            purpose: "L1-L5 commissioning model",
        },
        LifecycleDocument {
            area: "commissioning",
            title: "Commissioning evidence register",
            path: "docs/commissioning/commissioning-evidence-register.md",
            purpose: "test evidence and acceptance record",
        },
        LifecycleDocument {
            area: "operations",
            title: "Operational readiness review",
            path: "docs/delivery/operational-readiness-review.md",
            purpose: "run-state acceptance gate",
        },
        LifecycleDocument {
            area: "operations",
            title: "Handover to operations",
            path: "docs/delivery/handover-to-operations.md",
            purpose: "as-built evidence and open-risk transfer",
        },
        LifecycleDocument {
            area: "software",
            title: "Unified portal integration model",
            path: "docs/software/unified-portal-integration-model.md",
            purpose: "Rust workflow layer over open systems",
        },
        LifecycleDocument {
            area: "security",
            title: "Sovereign edge security stack",
            path: "docs/security/sovereign-edge-security-stack.md",
            purpose: "Edge Shield DNS TLS WAF access and audit fabric",
        },
        LifecycleDocument {
            area: "commercial",
            title: "Commercial readiness",
            path: "docs/commercial-readiness/README.md",
            purpose: "customer, SLA, audit, and standards readiness",
        },
    ]
}

fn lifecycle_overview() -> LifecycleOverview {
    let gates = project_gates();
    let permits = authority_permits();
    let risks = delivery_risks();
    let actions = delivery_actions();
    let gaps = commercial_gaps();
    let engineering = engineering_evidence();
    let commissioning = commissioning_evidence();
    let operations = operations_procedures();
    let audit = audit_evidence();
    let standards = commercial_standards();
    let pricebook = remote_hands_pricebook();
    let access = access_roles();
    let site = site_selection_scorecard();
    let physical = physical_security_controls();
    let sustainability = sustainability_metrics();
    let ai = ai_rack_classes();
    let sovereign = sovereign_cloud_services();
    let core = core_cloud_services();
    let config = config_scripts();
    let documents = lifecycle_documents();

    let open_work_count = risks
        .iter()
        .filter(|item| !is_closed_status(&item.status))
        .count()
        + actions
            .iter()
            .filter(|item| !is_closed_status(&item.status))
            .count()
        + gaps
            .iter()
            .filter(|item| !is_closed_status(&item.status))
            .count();
    let evidence_count = engineering.len()
        + commissioning.len()
        + operations.len()
        + audit.len()
        + standards.len()
        + pricebook.len()
        + access.len();

    let metrics = vec![
        LifecycleMetric {
            label: "Lifecycle gates".to_string(),
            value: gates.len().to_string(),
            detail: "concept to handover".to_string(),
            kind: "normal",
        },
        LifecycleMetric {
            label: "Open work".to_string(),
            value: open_work_count.to_string(),
            detail: "risks actions gaps".to_string(),
            kind: if open_work_count > 0 {
                "warn"
            } else {
                "normal"
            },
        },
        LifecycleMetric {
            label: "Evidence records".to_string(),
            value: evidence_count.to_string(),
            detail: "engineering commissioning operations audit".to_string(),
            kind: "normal",
        },
        LifecycleMetric {
            label: "Managed services".to_string(),
            value: sovereign.len().to_string(),
            detail: "sovereign service catalogue".to_string(),
            kind: "normal",
        },
    ];

    let stages = vec![
        stage_from_gate(
            &gates,
            "GATE_00",
            "1. Initiate",
            "mission boundary funding certification scope",
        ),
        stage_from_gate(
            &gates,
            "GATE_01",
            "2. Select Site",
            "flood seismic geotechnical utility fibre authority due diligence",
        ),
        stage_from_gate(
            &gates,
            "GATE_03",
            "3. Freeze Design",
            "MEP evidence fire security controls network cost and risk baseline",
        ),
        stage_from_gate(
            &gates,
            "GATE_05",
            "4. Build",
            "permits procurement method statements and inspection test plan",
        ),
        stage_from_gate(
            &gates,
            "GATE_06",
            "5. Commission",
            "L1-L5 tests integrated systems tests and defect closeout",
        ),
        stage_from_gate(
            &gates,
            "GATE_07",
            "6. Operate",
            "staffing MOP SOP EOP monitoring spares incident command",
        ),
        stage_from_gate(
            &gates,
            "GATE_08",
            "7. Serve Tenants",
            "SLA onboarding services responsibility matrix and support",
        ),
        stage_from_gate(
            &gates,
            "GATE_09",
            "8. Handover",
            "as-builts evidence registers training and open-risk transfer",
        ),
    ];

    let mut work_items = Vec::new();
    for risk in risks {
        work_items.push(LifecycleWorkItem {
            item_type: "risk",
            id: risk.risk_id,
            phase: risk.domain,
            title: risk.risk,
            owner: risk.owner,
            priority: risk.impact,
            status_kind: status_kind(&risk.status),
            status: risk.status,
            evidence_path: risk.evidence_path,
        });
    }
    for action in actions {
        work_items.push(LifecycleWorkItem {
            item_type: "action",
            id: action.action_id,
            phase: action.due_phase,
            title: action.action,
            owner: action.owner,
            priority: action.source,
            status_kind: status_kind(&action.status),
            status: action.status,
            evidence_path: action.evidence_path,
        });
    }
    for gap in gaps {
        work_items.push(LifecycleWorkItem {
            item_type: "commercial-gap",
            id: gap.gap_id,
            phase: gap.domain,
            title: gap.commercial_expectation,
            owner: "commercial-readiness".to_string(),
            priority: gap.priority,
            status_kind: status_kind(&gap.status),
            status: gap.status,
            evidence_path: gap.next_artifact,
        });
    }

    let mut evidence = Vec::new();
    for item in engineering {
        evidence.push(LifecycleEvidenceItem {
            source: "engineering",
            id: item.evidence_id,
            domain: item.domain,
            title: item.evidence_name,
            owner: item.owner,
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.evidence_path,
        });
    }
    for item in commissioning {
        evidence.push(LifecycleEvidenceItem {
            source: "commissioning",
            id: item.evidence_id,
            domain: item.level,
            title: item.test_name,
            owner: item.owner,
            status_kind: priority_kind(&item.criticality),
            status: item.status,
            artifact: item.evidence_path,
        });
    }
    for item in operations {
        evidence.push(LifecycleEvidenceItem {
            source: "operations",
            id: item.procedure_id,
            domain: item.procedure_type,
            title: item.procedure_name,
            owner: item.owner,
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.doc_path,
        });
    }
    for item in audit {
        evidence.push(LifecycleEvidenceItem {
            source: "audit",
            id: item.evidence_id,
            domain: item.domain,
            title: item.evidence_name,
            owner: item.owner,
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.evidence_path,
        });
    }
    for item in standards {
        evidence.push(LifecycleEvidenceItem {
            source: "standards",
            id: item.requirement_id,
            domain: item.standard_family,
            title: item.control_area,
            owner: item.responsible_party,
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.evidence_file,
        });
    }
    for item in pricebook {
        evidence.push(LifecycleEvidenceItem {
            source: "remote-hands-pricebook",
            id: item.pricebook_id,
            domain: item.task_class,
            title: item.billing_unit,
            owner: item.requires_approval,
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.evidence_path,
        });
    }
    for item in access {
        evidence.push(LifecycleEvidenceItem {
            source: "access-role",
            id: item.access_role_id,
            domain: item.scope,
            title: item.role_name,
            owner: item.approval_owner,
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.evidence_path,
        });
    }
    for item in permits {
        evidence.push(LifecycleEvidenceItem {
            source: "permit",
            id: item.permit_id,
            domain: item.authority_area,
            title: item.permit_or_approval,
            owner: item.owner,
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.evidence_path,
        });
    }
    for item in site {
        evidence.push(LifecycleEvidenceItem {
            source: "site",
            id: item.criterion_id,
            domain: item.domain,
            title: item.criterion,
            owner: "owner-engineer".to_string(),
            status: "template".to_string(),
            status_kind: "warn",
            artifact: item.next_evidence,
        });
    }
    for item in physical {
        evidence.push(LifecycleEvidenceItem {
            source: "physical-security",
            id: item.control_id,
            domain: item.zone,
            title: item.control,
            owner: item.owner,
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.evidence,
        });
    }
    for item in sustainability {
        evidence.push(LifecycleEvidenceItem {
            source: "sustainability",
            id: item.metric_id,
            domain: item.stage,
            title: item.metric,
            owner: item.owner,
            status: "measurement".to_string(),
            status_kind: "info",
            artifact: item.evidence,
        });
    }
    for item in ai {
        evidence.push(LifecycleEvidenceItem {
            source: "ai-ready",
            id: item.rack_class_id,
            domain: item.power_kw_range,
            title: item.cooling_mode,
            owner: "facility-and-platform".to_string(),
            status_kind: status_kind(&item.status),
            status: item.status,
            artifact: item.evidence_required,
        });
    }

    let mut services = Vec::new();
    for service in core {
        services.push(LifecycleServiceItem {
            service_id: service.id.to_string(),
            category: service.priority.to_string(),
            interface: if service.tenant_visible {
                "tenant+operator".to_string()
            } else {
                "operator".to_string()
            },
            implementation: service.open_source_stack.to_string(),
            workflow: service.default_shape.to_string(),
            status: service.status.to_string(),
            status_kind: status_kind(&service.status),
        });
    }
    for service in sovereign {
        services.push(LifecycleServiceItem {
            service_id: service.id,
            category: service.category,
            interface: service.ui_surface,
            implementation: service.open_equivalent,
            workflow: service.workflow,
            status_kind: status_kind(&service.maturity),
            status: service.maturity,
        });
    }
    for script in config {
        services.push(LifecycleServiceItem {
            service_id: script.id.to_string(),
            category: "config-script".to_string(),
            interface: script.rollout_target.to_string(),
            implementation: format!("{} {}", script.tool, script.path),
            workflow: script.validation_command.to_string(),
            status: script.edit_mode.to_string(),
            status_kind: priority_kind(&script.risk),
        });
    }

    LifecycleOverview {
        metrics,
        stages,
        work_items,
        evidence,
        services,
        documents,
    }
}

fn upgrade_policy() -> Vec<UpgradePolicy> {
    csv_rows(UPGRADE_POLICY_CSV, "data/software/upgrade-policy.csv")
}

fn config_scripts() -> Vec<ConfigScript> {
    let mut scripts: Vec<ConfigScript> = csv_rows(
        CONFIG_SCRIPT_CATALOGUE_CSV,
        "data/software/config-script-catalogue.csv",
    );
    for script in &mut scripts {
        script.content = match script.id.as_str() {
            "edge_caddyfile" => EDGE_CADDYFILE,
            "edge_powerdns" => EDGE_POWERDNS_CONF,
            "edge_coraza" => EDGE_CORAZA_CONF,
            "edge_crowdsec" => EDGE_CROWDSEC_ACQUIS,
            "edge_wireguard" => EDGE_WIREGUARD_CONF,
            _ => "",
        }
        .to_string();
    }
    scripts
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
    fn root_redirects_to_infrastructure_workbench() {
        let response = route_response("GET", "/");
        let text = response_text(&response);

        assert!(text.starts_with("HTTP/1.1 302 Found"));
        assert!(text.contains("Location: /infrastructure"));
    }

    #[test]
    fn serves_all_gui_pages_with_expected_controls() {
        let infrastructure = body(&route_response("GET", "/infrastructure"));
        let user = body(&route_response("GET", "/user"));
        let operator = body(&route_response("GET", "/operator"));
        let edge = body(&route_response("GET", "/edge"));
        let planner = body(&route_response("GET", "/planner"));
        let lifecycle = body(&route_response("GET", "/lifecycle"));
        let hardware = body(&route_response("GET", "/hardware"));
        let developer = body(&route_response("GET", "/developer"));
        let data_platform = body(&route_response("GET", "/data-platform"));
        let commercial = body(&route_response("GET", "/commercial"));
        let customers = body(&route_response("GET", "/customers"));
        let assurance = body(&route_response("GET", "/assurance"));

        assert!(infrastructure.contains("Infrastructure Workbench"));
        assert!(infrastructure.contains("infra-workflow-select"));
        assert!(infrastructure.contains("infra-change-record"));
        assert!(infrastructure.contains("infra-execution-steps"));
        assert!(infrastructure.contains("infra-evidence-bundle"));
        assert!(infrastructure.contains("infra-download-change"));
        assert!(infrastructure.contains("infra-approve-change"));
        assert!(infrastructure.contains("infra-workflows"));
        assert!(infrastructure.contains("infra-connectors"));
        assert!(infrastructure.contains("infra-adapters"));
        assert!(infrastructure.contains("infra-adapter-proofs"));
        assert!(infrastructure.contains("infra-persistence-schema"));
        assert!(infrastructure.contains("infra-tests"));
        assert!(infrastructure.contains("infra-gates"));
        assert!(infrastructure.contains("href=\"/hardware\""));
        assert!(infrastructure.contains("href=\"/customers\""));
        assert!(infrastructure.contains("href=\"/assurance\""));
        assert!(infrastructure.contains("href=\"/developer\""));
        assert!(user.contains("Tenant Cloud"));
        assert!(user.contains("tenant-service-filter"));
        assert!(user.contains("tenant-action-output"));
        assert!(user.contains("href=\"/infrastructure\""));
        assert!(user.contains("href=\"/commercial\""));
        assert!(user.contains("href=\"/assurance\""));
        assert!(user.contains("href=\"/hardware\""));
        assert!(operator.contains("Operator Console"));
        assert!(operator.contains("operator-service-filter"));
        assert!(operator.contains("href=\"/infrastructure\""));
        assert!(operator.contains("href=\"/commercial\""));
        assert!(operator.contains("href=\"/assurance\""));
        assert!(operator.contains("href=\"/hardware\""));
        assert!(edge.contains("Edge Shield"));
        assert!(edge.contains("edge-service-filter"));
        assert!(edge.contains("edge-config-preview"));
        assert!(edge.contains("edge-script-editor"));
        assert!(edge.contains("href=\"/infrastructure\""));
        assert!(edge.contains("href=\"/commercial\""));
        assert!(edge.contains("href=\"/assurance\""));
        assert!(edge.contains("href=\"/hardware\""));
        assert!(planner.contains("Cost Planner"));
        assert!(planner.contains("planner-scenarios"));
        assert!(planner.contains("planner-stack-profiles"));
        assert!(planner.contains("planner-price-basis"));
        assert!(planner.contains("href=\"/infrastructure\""));
        assert!(planner.contains("href=\"/commercial\""));
        assert!(planner.contains("href=\"/assurance\""));
        assert!(planner.contains("href=\"/hardware\""));
        assert!(lifecycle.contains("Lifecycle Console"));
        assert!(lifecycle.contains("lifecycle-stages"));
        assert!(lifecycle.contains("lifecycle-evidence"));
        assert!(lifecycle.contains("lifecycle-services"));
        assert!(lifecycle.contains("lifecycle-commercial-gaps"));
        assert!(lifecycle.contains("lifecycle-commercial-remote-hands"));
        assert!(lifecycle.contains("lifecycle-commercial-access"));
        assert!(lifecycle.contains("href=\"/infrastructure\""));
        assert!(lifecycle.contains("href=\"/hardware\""));
        assert!(hardware.contains("Hardware Provisioning"));
        assert!(hardware.contains("hardware-profile-select"));
        assert!(hardware.contains("hardware-pipeline"));
        assert!(hardware.contains("hardware-connectors"));
        assert!(hardware.contains("href=\"/infrastructure\""));
        assert!(hardware.contains("href=\"/assurance\""));
        assert!(developer.contains("Developer Console"));
        assert!(developer.contains("developer-templates"));
        assert!(developer.contains("developer-vscode"));
        assert!(developer.contains("Forgejo"));
        assert!(developer.contains("href=\"/infrastructure\""));
        assert!(developer.contains("href=\"/commercial\""));
        assert!(developer.contains("href=\"/assurance\""));
        assert!(developer.contains("href=\"/hardware\""));
        assert!(data_platform.contains("Data Platform"));
        assert!(data_platform.contains("data-products"));
        assert!(data_platform.contains("data-ontology"));
        assert!(data_platform.contains("Open Data Platform Stack"));
        assert!(data_platform.contains("href=\"/infrastructure\""));
        assert!(data_platform.contains("href=\"/commercial\""));
        assert!(data_platform.contains("href=\"/assurance\""));
        assert!(data_platform.contains("href=\"/hardware\""));
        assert!(commercial.contains("Commercial Console"));
        assert!(commercial.contains("commercial-standards"));
        assert!(commercial.contains("commercial-pricebook"));
        assert!(commercial.contains("commercial-access"));
        assert!(commercial.contains("href=\"/customers\""));
        assert!(commercial.contains("href=\"/infrastructure\""));
        assert!(commercial.contains("href=\"/assurance\""));
        assert!(commercial.contains("href=\"/hardware\""));
        assert!(customers.contains("Customer Operations"));
        assert!(customers.contains("customer-command-record"));
        assert!(customers.contains("customer-execution-steps"));
        assert!(customers.contains("customer-mfa-policies"));
        assert!(customers.contains("customer-billing-plans"));
        assert!(customers.contains("customer-usage-meters"));
        assert!(customers.contains("customer-invoices"));
        assert!(customers.contains("customer-connectors"));
        assert!(customers.contains("href=\"/infrastructure\""));
        assert!(customers.contains("href=\"/commercial\""));
        assert!(customers.contains("href=\"/hardware\""));
        assert!(assurance.contains("Assurance Console"));
        assert!(assurance.contains("assurance-jobs"));
        assert!(assurance.contains("assurance-tests"));
        assert!(assurance.contains("assurance-gates"));
        assert!(assurance.contains("assurance-threat-stack"));
        assert!(assurance.contains("assurance-scanners"));
        assert!(assurance.contains("href=\"/infrastructure\""));
        assert!(assurance.contains("href=\"/hardware\""));
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
    fn exposes_hardware_provisioning_and_system_connectors() {
        let overview = json_body("/api/hardware/provisioning");
        let connectors = json_body("/api/connectors/systems");
        let pipeline = json_body("/api/hardware/provisioning-pipeline");
        let profiles = json_body("/api/hardware/provisioning-profiles");
        let requests = json_body("/api/hardware/provisioning-requests");

        assert!(overview["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|metric| metric["label"] == "Guarded actions"));
        assert!(overview["connectors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|connector| {
                connector["connector_id"] == "CONN_REDFISH"
                    && connector["write_mode"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("guarded")
            }));
        assert!(connectors.as_array().unwrap().iter().any(|connector| {
            connector["connector_id"] == "CONN_DEFECTDOJO"
                && connector["portal_surface"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("/assurance")
        }));
        assert!(connectors.as_array().unwrap().iter().any(|connector| {
            connector["connector_id"] == "CONN_PROXMOX"
                && connector["evidence_path"] == "docs/software/deployment-stack-profiles.md"
        }));
        assert!(pipeline.as_array().unwrap().iter().any(|stage| {
            stage["stage_id"] == "HP_STAGE_SOURCE_OF_TRUTH"
                && stage["primary_system"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("NetBox")
        }));
        assert!(profiles.as_array().unwrap().iter().any(|profile| {
            profile["profile_id"] == "HP_GPU_AI" && profile["provisioner"] == "Metal3"
        }));
        assert!(requests.as_array().unwrap().iter().any(|request| {
            request["request_id"] == "HREQ_001"
                && request["target_environment"] == "openstack-compute"
        }));
    }

    #[test]
    fn exposes_infrastructure_workbench_with_workflow_assurance_mapping() {
        let workbench = json_body("/api/infrastructure/workbench");
        let adapter_roadmap = json_body("/api/infrastructure/adapter-roadmap");
        let adapter_proofs = json_body("/api/infrastructure/adapter-proofs");
        let persistence_schema = json_body("/api/infrastructure/persistence-schema");

        assert!(workbench["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|metric| metric["label"] == "Test harnesses"));
        assert!(workbench["workflows"]
            .as_array()
            .unwrap()
            .iter()
            .any(|workflow| {
                workflow["workflow_id"] == "WF_PROVISION_VM"
                    && workflow["connector_ids"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("CONN_PROXMOX")
                    && workflow["required_test_ids"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("TEST_ENDPOINT_POSTURE")
                    && workflow["required_gate_ids"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("GATE_STAGING")
            }));
        assert!(workbench["stack_profiles"]
            .as_array()
            .unwrap()
            .iter()
            .any(|profile| profile["profile_id"] == "DSP_250KW_REGIONAL"));
        assert!(workbench["adapter_milestones"]
            .as_array()
            .unwrap()
            .iter()
            .any(|milestone| {
                milestone["milestone_id"] == "ADAPT_006"
                    && milestone["adapter_target"] == "Proxmox"
                    && milestone["workflow_ids"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("WF_PROVISION_VM")
            }));
        assert!(workbench["adapter_proofs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|proof| {
                proof["proof_id"] == "PROOF_PROXMOX_READ"
                    && proof["proof_command"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("scripts/adapter-proof.sh")
            }));
        assert_eq!(
            workbench["persistence_schema"]["migration_path"],
            "crates/osdc-portal/migrations/0001_osdc_portal_state.sql"
        );
        assert!(workbench["persistence_schema"]["tables"]
            .as_array()
            .unwrap()
            .iter()
            .any(|table| table["table_name"] == "osdc_portal.change_requests"));
        assert!(workbench["connectors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|connector| connector["connector_id"] == "CONN_POSTGRES"));
        assert!(workbench["connectors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|connector| connector["connector_id"] == "CONN_CLOUDSTACK"));
        assert!(workbench["test_harnesses"]
            .as_array()
            .unwrap()
            .iter()
            .any(|test| test["test_id"] == "TEST_PORTAL_CONTRACT"));
        assert!(workbench["upgrade_gates"]
            .as_array()
            .unwrap()
            .iter()
            .any(|gate| gate["gate_id"] == "GATE_AUDIT"));
        assert!(workbench["automation_jobs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|job| job["job_id"] == "JOB_ASSURANCE_RUN"));
        assert!(adapter_roadmap
            .as_array()
            .unwrap()
            .iter()
            .any(|milestone| milestone["milestone_id"] == "ADAPT_009"
                && milestone["backend_system"] == "PostgreSQL"));
        assert!(adapter_proofs
            .as_array()
            .unwrap()
            .iter()
            .any(|proof| proof["proof_id"] == "PROOF_POSTGRES_MIGRATION"
                && proof["required_env"] == "DATABASE_URL"));
        assert!(persistence_schema["tables"]
            .as_array()
            .unwrap()
            .iter()
            .any(
                |table| table["table_name"] == "osdc_portal.adapter_proof_runs"
                    && table["columns"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|column| column
                            .as_str()
                            .unwrap_or_default()
                            .contains("evidence_path"))
            ));
        assert!(persistence_schema["tables"]
            .as_array()
            .unwrap()
            .iter()
            .any(|table| table["table_name"] == "osdc_portal.invoice_previews"));
        assert_eq!(persistence_schema["indexes"].as_array().unwrap().len(), 13);
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
        let stack_profiles = json_body("/api/deployment/stack-profiles");

        assert_eq!(planning["metrics"].as_array().unwrap().len(), 4);
        assert!(planning["deployment_profiles"]
            .as_array()
            .unwrap()
            .iter()
            .any(|profile| profile["profile_id"] == "DSP_50KW_EDGE"));
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
        assert!(stack_profiles.as_array().unwrap().iter().any(|profile| {
            profile["profile_id"] == "DSP_50KW_EDGE"
                && profile["default_cloud_substrate"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("Proxmox")
                && profile["alternate_cloud_substrate"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("OpenNebula")
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
        let pricebook = json_body("/api/commercial/remote-hands-pricebook");
        let access_roles = json_body("/api/commercial/access-roles");
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
        assert!(pricebook
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["pricebook_id"] == "RHP_SMART_HANDS"));
        assert!(access_roles
            .as_array()
            .unwrap()
            .iter()
            .any(|role| role["access_role_id"] == "ACCESS_BREAK_GLASS"));
        assert!(evidence
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["evidence_id"] == "EV007"));
    }

    #[test]
    fn exposes_customer_operations_mfa_metering_and_billing_catalogues() {
        let overview = json_body("/api/customers/overview");
        let accounts = json_body("/api/customers/accounts");
        let sites = json_body("/api/customers/sites");
        let workflows = json_body("/api/customers/workflows");
        let mfa = json_body("/api/customers/mfa-policies");
        let plans = json_body("/api/customers/billing-plans");
        let meters = json_body("/api/customers/usage-meters");
        let invoices = json_body("/api/customers/invoice-preview");

        assert!(overview["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|metric| metric["label"] == "Billable meters"));
        assert!(overview["connectors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|connector| connector["connector_id"] == "CONN_PRIVACYIDEA"));
        assert!(overview["connectors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|connector| connector["connector_id"] == "CONN_KILLBILL"));
        assert!(accounts.as_array().unwrap().iter().any(|account| {
            account["customer_id"] == "CUST_HEALTH" && account["identity_realm"] == "health.gov"
        }));
        assert!(sites.as_array().unwrap().iter().any(|site| {
            site["site_id"] == "SITE_HEALTH_REGIONAL" && site["source_of_truth"] == "NetBox"
        }));
        assert!(workflows.as_array().unwrap().iter().any(|workflow| {
            workflow["workflow_id"] == "CUST_ENFORCE_MFA"
                && workflow["connector_ids"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("CONN_PRIVACYIDEA")
        }));
        assert!(mfa.as_array().unwrap().iter().any(|policy| {
            policy["policy_id"] == "MFA_OPERATOR_PRIVILEGED"
                && policy["provider_stack"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("privacyIDEA")
        }));
        assert!(plans.as_array().unwrap().iter().any(|plan| {
            plan["plan_id"] == "BILL_PUBLIC_CRITICAL" && plan["invoice_engine"] == "Kill-Bill"
        }));
        assert!(meters.as_array().unwrap().iter().any(|meter| {
            meter["meter_id"] == "METER_VM_HOURS"
                && meter["evidence_path"] == "docs/commercial/billing-and-metering.md"
        }));
        assert!(invoices.as_array().unwrap().iter().any(|invoice| {
            invoice["invoice_id"] == "INV_HEALTH_2026_06" && invoice["status"] == "draft"
        }));
    }

    #[test]
    fn exposes_facility_readiness_catalogues() {
        let site = json_body("/api/site-selection/scorecard");
        let physical = json_body("/api/security/physical-controls");
        let sustainability = json_body("/api/sustainability/metrics");
        let ai = json_body("/api/ai-ready/rack-classes");
        let engineering = json_body("/api/engineering/evidence");
        let operations = json_body("/api/operations/procedures");
        let gates = json_body("/api/delivery/gates");
        let permits = json_body("/api/delivery/permits");
        let risks = json_body("/api/delivery/risks");
        let actions = json_body("/api/delivery/actions");
        let commissioning = json_body("/api/commissioning/evidence");

        assert!(site.as_array().unwrap().iter().any(|criterion| {
            criterion["criterion_id"] == "SITE_FIBRE"
                && criterion["next_evidence"]
                    == "docs/site-selection/fibre-route-diversity-checklist.md"
        }));
        assert!(physical
            .as_array()
            .unwrap()
            .iter()
            .any(|control| control["control_id"] == "PHY_MANTRAP"));
        assert!(sustainability
            .as_array()
            .unwrap()
            .iter()
            .any(|metric| metric["metric_id"] == "SUS_PUE"));
        assert!(ai.as_array().unwrap().iter().any(|rack| {
            rack["rack_class_id"] == "AI_RACK_80KW"
                && rack["cooling_mode"] == "direct-to-chip-liquid"
        }));
        assert!(engineering.as_array().unwrap().iter().any(|evidence| {
            evidence["evidence_id"] == "ENG_SELECTIVITY"
                && evidence["evidence_path"] == "docs/engineering/breaker-fuse-coordination.md"
        }));
        assert!(operations.as_array().unwrap().iter().any(|procedure| {
            procedure["procedure_id"] == "OPS_LOTO"
                && procedure["doc_path"] == "docs/operations/lockout-tagout.md"
        }));
        assert!(gates.as_array().unwrap().iter().any(|gate| {
            gate["gate_id"] == "GATE_06"
                && gate["required_evidence"]
                    == "docs/commissioning/commissioning-evidence-register.md"
        }));
        assert!(permits
            .as_array()
            .unwrap()
            .iter()
            .any(|permit| permit["permit_id"] == "PERMIT_FIRE"));
        assert!(risks
            .as_array()
            .unwrap()
            .iter()
            .any(|risk| risk["risk_id"] == "RISK_DC_ARC"));
        assert!(actions
            .as_array()
            .unwrap()
            .iter()
            .any(|action| action["action_id"] == "ACT_005"));
        assert!(commissioning.as_array().unwrap().iter().any(|evidence| {
            evidence["evidence_id"] == "COM_GRID_LOSS"
                && evidence["evidence_path"] == "docs/commissioning/grid-loss-test.md"
        }));
    }

    #[test]
    fn exposes_unified_lifecycle_overview() {
        let overview = json_body("/api/lifecycle/overview");

        assert!(overview["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|metric| metric["label"] == "Lifecycle gates"));
        assert!(overview["stages"]
            .as_array()
            .unwrap()
            .iter()
            .any(|stage| stage["gate_id"] == "GATE_06" && stage["phase"] == "5. Commission"));
        assert!(overview["work_items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "RISK_DC_ARC"
                && item["evidence_path"] == "docs/engineering/dc-protection-and-arc-flash.md"));
        assert!(overview["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "COM_GRID_LOSS"
                && item["artifact"] == "docs/commissioning/grid-loss-test.md"));
        assert!(overview["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "ACCESS_BREAK_GLASS" && item["source"] == "access-role"));
        assert!(overview["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "RHP_SMART_HANDS"
                && item["source"] == "remote-hands-pricebook"));
        assert!(overview["services"]
            .as_array()
            .unwrap()
            .iter()
            .any(|service| service["service_id"] == "identity"
                && service["implementation"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("Keycloak")));
        assert!(overview["documents"]
            .as_array()
            .unwrap()
            .iter()
            .any(|document| document["path"] == "docs/delivery/project-lifecycle-gates.md"));
    }

    #[test]
    fn exposes_developer_platform_for_gitops_and_vscode() {
        let platform = json_body("/api/developer/platform");

        assert!(platform["services"]
            .as_array()
            .unwrap()
            .iter()
            .any(|service| {
                service["service_id"] == "dev_forge"
                    && service["default_stack"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("Forgejo")
            }));
        assert!(platform["templates"]
            .as_array()
            .unwrap()
            .iter()
            .any(|template| {
                template["template_id"] == "rust_axum_api"
                    && template["devcontainer_path"]
                        == "examples/developer-platform/rust-api/.devcontainer/devcontainer.json"
                    && template["vscode_clone_uri"]
                        .as_str()
                        .unwrap_or_default()
                        .starts_with("vscode://")
            }));
        assert!(platform["environments"]
            .as_array()
            .unwrap()
            .iter()
            .any(|environment| environment["environment_id"] == "prod"
                && environment["gitops_tool"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("Argo CD")));
        assert!(platform["promotion_gates"]
            .as_array()
            .unwrap()
            .iter()
            .any(|gate| gate["gate_id"] == "PROMOTE_STAGING_PROD"));
        assert!(platform["vscode_workflows"]
            .as_array()
            .unwrap()
            .iter()
            .any(|workflow| workflow["workflow_id"] == "VSCODE_DEVCONTAINER"));
    }

    #[test]
    fn exposes_data_platform_for_governed_data_products() {
        let overview = json_body("/api/data-platform/overview");

        assert!(overview["services"]
            .as_array()
            .unwrap()
            .iter()
            .any(|service| {
                service["service_id"] == "dp_lakehouse"
                    && service["default_stack"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("Apache Iceberg")
            }));
        assert!(overview["products"]
            .as_array()
            .unwrap()
            .iter()
            .any(|product| {
                product["product_id"] == "DP_DATACENTRE_TELEMETRY"
                    && product["ontology_object"]
                        .as_str()
                        .unwrap_or_default()
                        .contains("rack")
            }));
        assert!(overview["pipelines"]
            .as_array()
            .unwrap()
            .iter()
            .any(|pipeline| {
                pipeline["pipeline_id"] == "PIPE_FACILITY_TELEMETRY"
                    && pipeline["gitops_path"]
                        == "examples/data-platform/facility-telemetry/dagster_assets.py"
            }));
        assert!(overview["ontology"]
            .as_array()
            .unwrap()
            .iter()
            .any(|object| object["object_id"] == "ONT_FACILITY"));
        assert!(overview["access_policies"]
            .as_array()
            .unwrap()
            .iter()
            .any(|policy| policy["policy_id"] == "DATA_ROLE_AI_CURATOR"));
        assert!(overview["templates"]
            .as_array()
            .unwrap()
            .iter()
            .any(|template| {
                template["template_id"] == "DATA_TEMPLATE_DAGSTER"
                    && template["devcontainer_path"]
                        == "examples/data-platform/health-capacity/.devcontainer/devcontainer.json"
            }));
    }

    #[test]
    fn exposes_assurance_testing_upgrade_and_threat_management() {
        let overview = json_body("/api/assurance/overview");
        let jobs = json_body("/api/assurance/automation-jobs");
        let tests = json_body("/api/assurance/test-harnesses");
        let rings = json_body("/api/assurance/upgrade-rings");
        let gates = json_body("/api/assurance/upgrade-gates");
        let threat = json_body("/api/assurance/threat-stack");
        let scanners = json_body("/api/assurance/scanner-coverage");

        assert!(overview["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|metric| metric["label"] == "Threat coverage"));
        assert!(overview["automation_jobs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|job| job["job_id"] == "JOB_UPGRADE_DRY_RUN"));
        assert!(jobs.as_array().unwrap().iter().any(|job| {
            job["job_id"] == "JOB_ASSURANCE_RUN"
                && job["command"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("scripts/assurance-run.sh")
        }));
        assert!(tests.as_array().unwrap().iter().any(|test| {
            test["test_id"] == "TEST_SUPPLY_CHAIN"
                && test["tool_stack"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("trivy")
        }));
        assert!(rings
            .as_array()
            .unwrap()
            .iter()
            .any(|ring| ring["ring_id"] == "RING_CANARY"));
        assert!(gates.as_array().unwrap().iter().any(|gate| {
            gate["gate_id"] == "GATE_SCAN"
                && gate["automation_tool"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("OSV-Scanner")
        }));
        assert!(threat.as_array().unwrap().iter().any(|component| {
            component["component_id"] == "THREAT_ASPM"
                && component["open_source_stack"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("DefectDojo")
        }));
        assert!(scanners.as_array().unwrap().iter().any(|scanner| {
            scanner["scanner_id"] == "SCAN_K8S"
                && scanner["default_tool"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("Kubescape")
        }));
    }

    #[test]
    fn static_assets_have_content_types_and_unknown_routes_404() {
        let css = response_text(&route_response("GET", "/styles.css"));
        let js = response_text(&route_response("GET", "/portal.js"));
        let png = response_text(&route_response(
            "GET",
            "/assets/rack-thermal-spine-cutaway.png",
        ));
        let missing = response_text(&route_response("GET", "/missing"));

        assert!(css.contains("Content-Type: text/css; charset=utf-8"));
        assert!(js.contains("Content-Type: text/javascript; charset=utf-8"));
        assert!(js.contains("Active Command Queue"));
        assert!(js.contains("recordPortalCommand"));
        assert!(png.contains("Content-Type: image/png"));
        assert!(missing.starts_with("HTTP/1.1 404 Not Found"));
    }

    #[test]
    fn serves_repository_text_artifacts_for_lifecycle_links() {
        let doc = response_text(&route_response(
            "GET",
            "/docs/delivery/project-lifecycle-gates.md",
        ));
        let csv = response_text(&route_response("GET", "/data/delivery/project-gates.csv"));
        let devcontainer = response_text(&route_response(
            "GET",
            "/examples/developer-platform/rust-api/.devcontainer/devcontainer.json",
        ));
        let data_template = response_text(&route_response(
            "GET",
            "/examples/data-platform/health-capacity/dagster_assets.py",
        ));
        let migration = response_text(&route_response(
            "GET",
            "/crates/osdc-portal/migrations/0001_osdc_portal_state.sql",
        ));
        let escaped = response_text(&route_response("GET", "/docs/../Cargo.toml"));

        assert!(doc.contains("Content-Type: text/markdown; charset=utf-8"));
        assert!(doc.contains("Project Lifecycle Gates"));
        assert!(csv.contains("Content-Type: text/csv; charset=utf-8"));
        assert!(csv.contains("GATE_06"));
        assert!(devcontainer.contains("Content-Type: application/json; charset=utf-8"));
        assert!(devcontainer.contains("OSDC Rust API"));
        assert!(data_template.contains("Content-Type: text/plain; charset=utf-8"));
        assert!(data_template.contains("capacity_daily"));
        assert!(migration.contains("Content-Type: text/plain; charset=utf-8"));
        assert!(migration.contains("CREATE TABLE IF NOT EXISTS osdc_portal.change_requests"));
        assert!(escaped.starts_with("HTTP/1.1 404 Not Found"));
    }
}
