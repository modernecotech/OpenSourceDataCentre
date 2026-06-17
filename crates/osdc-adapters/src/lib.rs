use std::{collections::BTreeMap, error::Error, fmt, time::Duration};

use osdc_models::ChangeRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterTarget {
    Keycloak,
    PowerDns,
    NetBox,
    OpenBao,
    ArgoCd,
    Flux,
    CloudStack,
    Proxmox,
    OpenNebula,
    OpenStack,
    PostgreSql,
    PrivacyIdea,
    Authentik,
    CloudKitty,
    OpenMeter,
    KillBill,
    Lago,
    OpenCost,
    Kubernetes,
    Ceph,
    Redfish,
    Maas,
    Foreman,
    Ironic,
    Metal3,
    Tinkerbell,
    Harbor,
    DefectDojo,
    DependencyTrack,
    Wazuh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterReceipt {
    pub target: AdapterTarget,
    pub external_id: String,
    pub status: AdapterStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterStatus {
    Planned,
    Submitted,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterError {
    pub target: AdapterTarget,
    pub message: String,
}

impl fmt::Display for AdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} adapter error: {}",
            self.target, self.message
        )
    }
}

impl Error for AdapterError {}

pub type AdapterResult<T> = Result<T, AdapterError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantRequest {
    pub tenant_id: String,
    pub display_name: String,
    pub data_residency_zone: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleAssignmentRequest {
    pub tenant_id: String,
    pub subject: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsZoneRequest {
    pub zone_name: String,
    pub owner_tenant: String,
    pub dnssec_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryDeviceRequest {
    pub name: String,
    pub role: String,
    pub rack: String,
    pub asset_tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretPolicyRequest {
    pub path: String,
    pub owner: String,
    pub rotation_days: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorHealth {
    pub target: AdapterTarget,
    pub endpoint: String,
    pub reachable: bool,
    pub mode: AdapterMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterMode {
    ReadOnly,
    GitOps,
    GuardedApi,
    EvidenceIngest,
    PlanOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BareMetalNodeRequest {
    pub request_id: String,
    pub hostname: String,
    pub profile_id: String,
    pub bmc_address: String,
    pub target_pool: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedfishPowerRequest {
    pub node_id: String,
    pub action: String,
    pub approval_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmwareBaselineRequest {
    pub node_id: String,
    pub baseline_id: String,
    pub rollback_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenStackProvisionRequest {
    pub project_id: String,
    pub flavor: String,
    pub image: String,
    pub network_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualMachineProvisionRequest {
    pub project_id: String,
    pub instance_name: String,
    pub template: String,
    pub network_id: String,
    pub storage_profile: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KubernetesWorkloadRequest {
    pub namespace: String,
    pub workload_name: String,
    pub manifest_path: String,
    pub git_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CephStorageRequest {
    pub tenant_id: String,
    pub storage_class: String,
    pub capacity_gib: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryProjectRequest {
    pub project: String,
    pub owner: String,
    pub require_signed_images: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindingIngestRequest {
    pub engagement: String,
    pub scanner: String,
    pub artifact_path: String,
    pub owner: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DnsZoneSummary {
    pub zone_name: String,
    pub owner_tenant: String,
    pub dnssec_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DnsRecordSummary {
    pub zone_name: String,
    pub record_name: String,
    pub record_type: String,
    pub ttl: u32,
    pub records: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventorySummary {
    pub site_count: u16,
    pub rack_count: u16,
    pub device_count: u16,
    pub circuit_count: u16,
    pub ip_address_count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetBoxSiteSummary {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetBoxRackSummary {
    pub id: u64,
    pub name: String,
    pub site: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetBoxDeviceSummary {
    pub id: u64,
    pub name: String,
    pub role: String,
    pub site: String,
    pub rack: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetBoxIpAddressSummary {
    pub id: u64,
    pub address: String,
    pub status: String,
    pub assigned_object: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetBoxInventorySnapshot {
    pub sites: Vec<NetBoxSiteSummary>,
    pub racks: Vec<NetBoxRackSummary>,
    pub devices: Vec<NetBoxDeviceSummary>,
    pub ip_addresses: Vec<NetBoxIpAddressSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentitySummary {
    pub realm: String,
    pub group_count: u16,
    pub role_count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KeycloakRealmSummary {
    pub realm: String,
    pub enabled: bool,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KeycloakGroupSummary {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KeycloakRoleSummary {
    pub id: String,
    pub name: String,
    pub composite: bool,
    pub client_role: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KeycloakIdentitySnapshot {
    pub realms: Vec<KeycloakRealmSummary>,
    pub groups: Vec<KeycloakGroupSummary>,
    pub roles: Vec<KeycloakRoleSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretMountSummary {
    pub mount_path: String,
    pub policy_count: u16,
    pub transit_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OpenBaoMountSummary {
    pub path: String,
    pub engine_type: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OpenBaoPolicySummary {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OpenBaoSecretSnapshot {
    pub mounts: Vec<OpenBaoMountSummary>,
    pub policies: Vec<OpenBaoPolicySummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitOpsChangePreview {
    pub change_id: String,
    pub target_branch: String,
    pub files_changed: u16,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GitOpsApplicationSummary {
    pub name: String,
    pub namespace: String,
    pub sync_status: String,
    pub health_status: String,
    pub revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GitOpsSyncSnapshot {
    pub applications: Vec<GitOpsApplicationSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualizationClusterSummary {
    pub cluster_name: String,
    pub vm_count: u16,
    pub storage_pool_count: u16,
    pub backup_job_count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenStackProjectSummary {
    pub project_id: String,
    pub instance_count: u16,
    pub network_count: u16,
    pub volume_count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceMigrationPlan {
    pub schema_name: String,
    pub migration_count: u16,
    pub table_count: u16,
    pub destructive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MfaPolicySummary {
    pub realm: String,
    pub policy_id: String,
    pub enrolled_subjects: u16,
    pub unenrolled_subjects: u16,
    pub recovery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MfaEnrollmentRequest {
    pub tenant_id: String,
    pub subject: String,
    pub policy_id: String,
    pub factors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerSiteProvisionRequest {
    pub customer_id: String,
    pub site_id: String,
    pub deployment_profile: String,
    pub residency_zone: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerSiteSummary {
    pub customer_id: String,
    pub site_count: u16,
    pub active_sites: u16,
    pub residency_zones: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageMeterSummary {
    pub meter_id: String,
    pub customer_id: String,
    pub metric_name: String,
    pub quantity: u64,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageRatingRequest {
    pub customer_id: String,
    pub meter_id: String,
    pub quantity: u64,
    pub billing_period: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvoicePreviewSummary {
    pub invoice_id: String,
    pub customer_id: String,
    pub billing_period: String,
    pub amount_usd: f64,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvoiceGenerationRequest {
    pub customer_id: String,
    pub billing_period: String,
    pub plan_id: String,
    pub approval_ref: String,
}

pub trait DnsReadAdapter {
    fn list_zones(&self) -> AdapterResult<Vec<DnsZoneSummary>>;
    fn list_records(&self, zone_name: &str) -> AdapterResult<Vec<DnsRecordSummary>>;
}

pub trait InventoryReadAdapter {
    fn inventory_summary(&self) -> AdapterResult<InventorySummary>;
}

pub trait NetBoxReadAdapter {
    fn inventory_snapshot(&self) -> AdapterResult<NetBoxInventorySnapshot>;
}

pub trait IdentityReadAdapter {
    fn identity_summary(&self) -> AdapterResult<IdentitySummary>;
}

pub trait KeycloakReadAdapter {
    fn identity_snapshot(&self, realm: &str) -> AdapterResult<KeycloakIdentitySnapshot>;
}

pub trait SecretsReadAdapter {
    fn secret_mounts(&self) -> AdapterResult<Vec<SecretMountSummary>>;
}

pub trait OpenBaoReadAdapter {
    fn secret_snapshot(&self) -> AdapterResult<OpenBaoSecretSnapshot>;
}

pub trait GitOpsReadAdapter {
    fn preview_change(&self, request: &ChangeRequest) -> AdapterResult<GitOpsChangePreview>;
}

pub trait ArgoCdReadAdapter {
    fn sync_snapshot(&self) -> AdapterResult<GitOpsSyncSnapshot>;
}

pub trait VirtualizationReadAdapter {
    fn cluster_summary(&self) -> AdapterResult<VirtualizationClusterSummary>;
}

pub trait OpenStackReadAdapter {
    fn project_summary(&self, project_id: &str) -> AdapterResult<OpenStackProjectSummary>;
}

pub trait PortalPersistenceAdapter {
    fn migration_plan(&self) -> AdapterResult<PersistenceMigrationPlan>;
}

pub trait IdentityMfaAdapter {
    fn mfa_summary(&self, tenant_id: &str) -> AdapterResult<MfaPolicySummary>;
    fn enforce_mfa_policy(&self, request: &MfaEnrollmentRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait CustomerOperationsAdapter {
    fn customer_site_summary(&self, customer_id: &str) -> AdapterResult<CustomerSiteSummary>;
    fn provision_customer_site(
        &self,
        request: &CustomerSiteProvisionRequest,
    ) -> AdapterResult<AdapterReceipt>;
}

pub trait UsageMeteringAdapter {
    fn meter_summary(&self, customer_id: &str) -> AdapterResult<Vec<UsageMeterSummary>>;
    fn rate_usage(&self, request: &UsageRatingRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait BillingAdapter {
    fn preview_invoice(
        &self,
        request: &InvoiceGenerationRequest,
    ) -> AdapterResult<InvoicePreviewSummary>;
    fn generate_invoice(&self, request: &InvoiceGenerationRequest)
        -> AdapterResult<AdapterReceipt>;
}

pub trait IdentityAdapter {
    fn create_tenant(&self, request: &TenantRequest) -> AdapterResult<AdapterReceipt>;
    fn assign_role(&self, request: &RoleAssignmentRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait DnsAdapter {
    fn create_zone(&self, request: &DnsZoneRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait InventoryAdapter {
    fn register_device(&self, request: &InventoryDeviceRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait SecretsAdapter {
    fn apply_policy(&self, request: &SecretPolicyRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait GitOpsAdapter {
    fn submit_change(&self, request: &ChangeRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait HealthProbeAdapter {
    fn probe(&self, endpoint: &str) -> AdapterResult<ConnectorHealth>;
}

pub trait BareMetalProvisionerAdapter {
    fn enroll_node(&self, request: &BareMetalNodeRequest) -> AdapterResult<AdapterReceipt>;
    fn deploy_node(&self, request: &BareMetalNodeRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait RedfishAdapter {
    fn power_action(&self, request: &RedfishPowerRequest) -> AdapterResult<AdapterReceipt>;
    fn apply_firmware_baseline(
        &self,
        request: &FirmwareBaselineRequest,
    ) -> AdapterResult<AdapterReceipt>;
}

pub trait OpenStackAdapter {
    fn provision_instance(
        &self,
        request: &OpenStackProvisionRequest,
    ) -> AdapterResult<AdapterReceipt>;
}

pub trait VirtualizationAdapter {
    fn provision_virtual_machine(
        &self,
        request: &VirtualMachineProvisionRequest,
    ) -> AdapterResult<AdapterReceipt>;
}

pub trait KubernetesAdapter {
    fn submit_workload(&self, request: &KubernetesWorkloadRequest)
        -> AdapterResult<AdapterReceipt>;
}

pub trait CephAdapter {
    fn create_storage_allocation(
        &self,
        request: &CephStorageRequest,
    ) -> AdapterResult<AdapterReceipt>;
}

pub trait RegistryAdapter {
    fn create_project(&self, request: &RegistryProjectRequest) -> AdapterResult<AdapterReceipt>;
}

pub trait SecurityFindingAdapter {
    fn ingest_findings(&self, request: &FindingIngestRequest) -> AdapterResult<AdapterReceipt>;
}

#[derive(Debug, Clone)]
pub struct PowerDnsHttpAdapter {
    base_url: String,
    api_key: String,
    server_id: String,
    timeout: Duration,
}

impl PowerDnsHttpAdapter {
    pub fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        server_id: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            server_id: server_id.into(),
            timeout: Duration::from_secs(10),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn api_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    fn get_json<T>(&self, path: &str) -> AdapterResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.api_url(path);
        let response = ureq::AgentBuilder::new()
            .timeout(self.timeout)
            .build()
            .get(&url)
            .set("X-API-Key", &self.api_key)
            .set("Accept", "application/json")
            .call()
            .map_err(|err| AdapterError {
                target: AdapterTarget::PowerDns,
                message: format!("GET {url} failed: {err}"),
            })?;

        response.into_json::<T>().map_err(|err| AdapterError {
            target: AdapterTarget::PowerDns,
            message: format!("GET {url} returned invalid JSON: {err}"),
        })
    }
}

#[derive(Debug, Deserialize)]
struct PowerDnsZone {
    name: String,
    #[serde(default)]
    account: Option<String>,
    #[serde(default)]
    dnssec: bool,
}

#[derive(Debug, Deserialize)]
struct PowerDnsZoneDetail {
    #[serde(default)]
    rrsets: Vec<PowerDnsRrset>,
}

#[derive(Debug, Deserialize)]
struct PowerDnsRrset {
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    ttl: u32,
    #[serde(default)]
    records: Vec<PowerDnsRecord>,
}

#[derive(Debug, Deserialize)]
struct PowerDnsRecord {
    content: String,
    #[serde(default)]
    disabled: bool,
}

impl DnsReadAdapter for PowerDnsHttpAdapter {
    fn list_zones(&self) -> AdapterResult<Vec<DnsZoneSummary>> {
        let path = format!("/api/v1/servers/{}/zones", self.server_id);
        self.get_json::<Vec<PowerDnsZone>>(&path).map(|zones| {
            zones
                .into_iter()
                .map(|zone| DnsZoneSummary {
                    owner_tenant: zone.account.unwrap_or_else(|| "unassigned".to_string()),
                    zone_name: zone.name,
                    dnssec_enabled: zone.dnssec,
                })
                .collect()
        })
    }

    fn list_records(&self, zone_name: &str) -> AdapterResult<Vec<DnsRecordSummary>> {
        let zone_id = powerdns_zone_id(zone_name);
        let path = format!("/api/v1/servers/{}/zones/{zone_id}", self.server_id);
        self.get_json::<PowerDnsZoneDetail>(&path).map(|detail| {
            detail
                .rrsets
                .into_iter()
                .map(|rrset| DnsRecordSummary {
                    zone_name: zone_name.to_string(),
                    record_name: rrset.name,
                    record_type: rrset.record_type,
                    ttl: rrset.ttl,
                    records: rrset
                        .records
                        .into_iter()
                        .filter(|record| !record.disabled)
                        .map(|record| record.content)
                        .collect(),
                })
                .collect()
        })
    }
}

fn powerdns_zone_id(zone_name: &str) -> String {
    zone_name.replace('/', "%2F")
}

#[derive(Debug, Clone)]
pub struct NetBoxHttpAdapter {
    base_url: String,
    token: String,
    timeout: Duration,
}

impl NetBoxHttpAdapter {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.into(),
            timeout: Duration::from_secs(10),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn api_url(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}/{}", self.base_url, path.trim_start_matches('/'))
        }
    }

    fn get_json_url<T>(&self, url: &str) -> AdapterResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let authorization = format!("Token {}", self.token);
        let response = ureq::AgentBuilder::new()
            .timeout(self.timeout)
            .build()
            .get(url)
            .set("Authorization", &authorization)
            .set("Accept", "application/json")
            .call()
            .map_err(|err| AdapterError {
                target: AdapterTarget::NetBox,
                message: format!("GET {url} failed: {err}"),
            })?;

        response.into_json::<T>().map_err(|err| AdapterError {
            target: AdapterTarget::NetBox,
            message: format!("GET {url} returned invalid JSON: {err}"),
        })
    }

    fn get_paginated<T>(&self, path: &str) -> AdapterResult<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut url = self.api_url(path);
        let mut results = Vec::new();

        for _ in 0..1000 {
            let page = self.get_json_url::<NetBoxPage<T>>(&url)?;
            results.extend(page.results);

            match page.next.filter(|next| !next.trim().is_empty()) {
                Some(next) => url = self.api_url(&next),
                None => return Ok(results),
            }
        }

        Err(AdapterError {
            target: AdapterTarget::NetBox,
            message: format!("GET {path} exceeded pagination safety limit"),
        })
    }
}

#[derive(Debug, Deserialize)]
struct NetBoxPage<T> {
    #[serde(default)]
    next: Option<String>,
    results: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum NetBoxBrief {
    Object(NetBoxBriefObject),
    Text(String),
    Number(u64),
}

impl NetBoxBrief {
    fn name(&self) -> String {
        match self {
            NetBoxBrief::Object(object) => object
                .name
                .as_ref()
                .or(object.display.as_ref())
                .or(object.label.as_ref())
                .or(object.value.as_ref())
                .or(object.slug.as_ref())
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),
            NetBoxBrief::Text(value) => value.clone(),
            NetBoxBrief::Number(value) => value.to_string(),
        }
    }

    fn status(&self) -> String {
        match self {
            NetBoxBrief::Object(object) => object
                .value
                .as_ref()
                .or(object.label.as_ref())
                .or(object.name.as_ref())
                .or(object.display.as_ref())
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),
            NetBoxBrief::Text(value) => value.clone(),
            NetBoxBrief::Number(value) => value.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct NetBoxBriefObject {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    slug: Option<String>,
    #[serde(default)]
    value: Option<String>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    display: Option<String>,
}

#[derive(Debug, Deserialize)]
struct NetBoxSite {
    id: u64,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    slug: Option<String>,
    #[serde(default)]
    display: Option<String>,
    #[serde(default)]
    status: Option<NetBoxBrief>,
}

#[derive(Debug, Deserialize)]
struct NetBoxRack {
    id: u64,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    display: Option<String>,
    #[serde(default)]
    site: Option<NetBoxBrief>,
    #[serde(default)]
    status: Option<NetBoxBrief>,
}

#[derive(Debug, Deserialize)]
struct NetBoxDevice {
    id: u64,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    display: Option<String>,
    #[serde(default)]
    role: Option<NetBoxBrief>,
    #[serde(default)]
    device_role: Option<NetBoxBrief>,
    #[serde(default)]
    site: Option<NetBoxBrief>,
    #[serde(default)]
    rack: Option<NetBoxBrief>,
    #[serde(default)]
    status: Option<NetBoxBrief>,
}

#[derive(Debug, Deserialize)]
struct NetBoxIpAddress {
    id: u64,
    address: String,
    #[serde(default)]
    status: Option<NetBoxBrief>,
    #[serde(default)]
    assigned_object: Option<NetBoxAssignedObject>,
}

#[derive(Debug, Deserialize)]
struct NetBoxAssignedObject {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    display: Option<String>,
    #[serde(default)]
    device: Option<NetBoxBrief>,
    #[serde(default)]
    virtual_machine: Option<NetBoxBrief>,
}

impl NetBoxSite {
    fn into_summary(self) -> NetBoxSiteSummary {
        let name = self
            .name
            .or(self.display)
            .unwrap_or_else(|| format!("site-{}", self.id));
        let slug = self
            .slug
            .unwrap_or_else(|| name.to_ascii_lowercase().replace(' ', "-"));

        NetBoxSiteSummary {
            id: self.id,
            name,
            slug,
            status: brief_status(self.status, "unknown"),
        }
    }
}

impl NetBoxRack {
    fn into_summary(self) -> NetBoxRackSummary {
        NetBoxRackSummary {
            id: self.id,
            name: self
                .name
                .or(self.display)
                .unwrap_or_else(|| format!("rack-{}", self.id)),
            site: brief_name(self.site, "unknown-site"),
            status: brief_status(self.status, "unknown"),
        }
    }
}

impl NetBoxDevice {
    fn into_summary(self) -> NetBoxDeviceSummary {
        let role = self.role.or(self.device_role);

        NetBoxDeviceSummary {
            id: self.id,
            name: self
                .name
                .or(self.display)
                .unwrap_or_else(|| format!("device-{}", self.id)),
            role: brief_name(role, "unknown-role"),
            site: brief_name(self.site, "unknown-site"),
            rack: brief_name(self.rack, "unracked"),
            status: brief_status(self.status, "unknown"),
        }
    }
}

impl NetBoxIpAddress {
    fn into_summary(self) -> NetBoxIpAddressSummary {
        NetBoxIpAddressSummary {
            id: self.id,
            address: self.address,
            status: brief_status(self.status, "unknown"),
            assigned_object: assigned_object_name(self.assigned_object),
        }
    }
}

fn brief_name(brief: Option<NetBoxBrief>, fallback: &str) -> String {
    brief
        .map(|value| value.name())
        .unwrap_or_else(|| fallback.to_string())
}

fn brief_status(brief: Option<NetBoxBrief>, fallback: &str) -> String {
    brief
        .map(|value| value.status())
        .unwrap_or_else(|| fallback.to_string())
}

fn assigned_object_name(assigned_object: Option<NetBoxAssignedObject>) -> String {
    assigned_object
        .and_then(|object| {
            let NetBoxAssignedObject {
                name,
                display,
                device,
                virtual_machine,
            } = object;

            name.or(display)
                .or_else(|| device.map(|brief| brief.name()))
                .or_else(|| virtual_machine.map(|brief| brief.name()))
        })
        .unwrap_or_else(|| "unassigned".to_string())
}

fn len_as_u16(len: usize) -> u16 {
    u16::try_from(len).unwrap_or(u16::MAX)
}

impl NetBoxReadAdapter for NetBoxHttpAdapter {
    fn inventory_snapshot(&self) -> AdapterResult<NetBoxInventorySnapshot> {
        let sites = self
            .get_paginated::<NetBoxSite>("/api/dcim/sites/")?
            .into_iter()
            .map(NetBoxSite::into_summary)
            .collect();
        let racks = self
            .get_paginated::<NetBoxRack>("/api/dcim/racks/")?
            .into_iter()
            .map(NetBoxRack::into_summary)
            .collect();
        let devices = self
            .get_paginated::<NetBoxDevice>("/api/dcim/devices/")?
            .into_iter()
            .map(NetBoxDevice::into_summary)
            .collect();
        let ip_addresses = self
            .get_paginated::<NetBoxIpAddress>("/api/ipam/ip-addresses/")?
            .into_iter()
            .map(NetBoxIpAddress::into_summary)
            .collect();

        Ok(NetBoxInventorySnapshot {
            sites,
            racks,
            devices,
            ip_addresses,
        })
    }
}

impl InventoryReadAdapter for NetBoxHttpAdapter {
    fn inventory_summary(&self) -> AdapterResult<InventorySummary> {
        let snapshot = self.inventory_snapshot()?;

        Ok(InventorySummary {
            site_count: len_as_u16(snapshot.sites.len()),
            rack_count: len_as_u16(snapshot.racks.len()),
            device_count: len_as_u16(snapshot.devices.len()),
            circuit_count: 0,
            ip_address_count: len_as_u16(snapshot.ip_addresses.len()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeycloakHttpAdapter {
    base_url: String,
    token: String,
    realm: String,
    timeout: Duration,
}

impl KeycloakHttpAdapter {
    pub fn new(
        base_url: impl Into<String>,
        token: impl Into<String>,
        realm: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.into(),
            realm: realm.into(),
            timeout: Duration::from_secs(10),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn api_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    fn get_json<T>(&self, path: &str) -> AdapterResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.api_url(path);
        let authorization = format!("Bearer {}", self.token);
        let response = ureq::AgentBuilder::new()
            .timeout(self.timeout)
            .build()
            .get(&url)
            .set("Authorization", &authorization)
            .set("Accept", "application/json")
            .call()
            .map_err(|err| AdapterError {
                target: AdapterTarget::Keycloak,
                message: format!("GET {url} failed: {err}"),
            })?;

        response.into_json::<T>().map_err(|err| AdapterError {
            target: AdapterTarget::Keycloak,
            message: format!("GET {url} returned invalid JSON: {err}"),
        })
    }
}

#[derive(Debug, Deserialize)]
struct KeycloakRealm {
    #[serde(default)]
    realm: Option<String>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    enabled: bool,
    #[serde(default, rename = "displayName")]
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct KeycloakGroup {
    id: String,
    name: String,
    #[serde(default)]
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct KeycloakRole {
    #[serde(default)]
    id: Option<String>,
    name: String,
    #[serde(default)]
    composite: bool,
    #[serde(default, rename = "clientRole")]
    client_role: bool,
}

impl KeycloakRealm {
    fn into_summary(self) -> KeycloakRealmSummary {
        let realm = self
            .realm
            .or(self.id)
            .unwrap_or_else(|| "unknown-realm".to_string());
        let display_name = self.display_name.clone().unwrap_or_else(|| realm.clone());

        KeycloakRealmSummary {
            realm,
            enabled: self.enabled,
            display_name,
        }
    }
}

impl KeycloakGroup {
    fn into_summary(self) -> KeycloakGroupSummary {
        let path = self.path.unwrap_or_else(|| format!("/{}", self.name));

        KeycloakGroupSummary {
            id: self.id,
            name: self.name,
            path,
        }
    }
}

impl KeycloakRole {
    fn into_summary(self) -> KeycloakRoleSummary {
        let id = self.id.unwrap_or_else(|| self.name.clone());

        KeycloakRoleSummary {
            id,
            name: self.name,
            composite: self.composite,
            client_role: self.client_role,
        }
    }
}

impl KeycloakReadAdapter for KeycloakHttpAdapter {
    fn identity_snapshot(&self, realm: &str) -> AdapterResult<KeycloakIdentitySnapshot> {
        let realm_segment = keycloak_path_segment(realm);
        let realms = self
            .get_json::<Vec<KeycloakRealm>>("/admin/realms")?
            .into_iter()
            .map(KeycloakRealm::into_summary)
            .collect();
        let groups = self
            .get_json::<Vec<KeycloakGroup>>(&format!("/admin/realms/{realm_segment}/groups"))?
            .into_iter()
            .map(KeycloakGroup::into_summary)
            .collect();
        let roles = self
            .get_json::<Vec<KeycloakRole>>(&format!("/admin/realms/{realm_segment}/roles"))?
            .into_iter()
            .map(KeycloakRole::into_summary)
            .collect();

        Ok(KeycloakIdentitySnapshot {
            realms,
            groups,
            roles,
        })
    }
}

impl IdentityReadAdapter for KeycloakHttpAdapter {
    fn identity_summary(&self) -> AdapterResult<IdentitySummary> {
        let snapshot = self.identity_snapshot(&self.realm)?;

        Ok(IdentitySummary {
            realm: self.realm.clone(),
            group_count: len_as_u16(snapshot.groups.len()),
            role_count: len_as_u16(snapshot.roles.len()),
        })
    }
}

fn keycloak_path_segment(segment: &str) -> String {
    segment.replace(' ', "%20").replace('/', "%2F")
}

#[derive(Debug, Clone)]
pub struct OpenBaoHttpAdapter {
    base_url: String,
    token: String,
    timeout: Duration,
}

impl OpenBaoHttpAdapter {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.into(),
            timeout: Duration::from_secs(10),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn api_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    fn request_json<T>(&self, method: &str, path: &str) -> AdapterResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.api_url(path);
        let response = ureq::AgentBuilder::new()
            .timeout(self.timeout)
            .build()
            .request(method, &url)
            .set("X-Vault-Token", &self.token)
            .set("Accept", "application/json")
            .call()
            .map_err(|err| AdapterError {
                target: AdapterTarget::OpenBao,
                message: format!("{method} {url} failed: {err}"),
            })?;

        response.into_json::<T>().map_err(|err| AdapterError {
            target: AdapterTarget::OpenBao,
            message: format!("{method} {url} returned invalid JSON: {err}"),
        })
    }
}

#[derive(Debug, Deserialize)]
struct OpenBaoMount {
    #[serde(rename = "type")]
    engine_type: String,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenBaoPolicyList {
    #[serde(default)]
    keys: Vec<String>,
    #[serde(default)]
    policies: Vec<String>,
}

impl OpenBaoPolicyList {
    fn names(self) -> Vec<String> {
        if self.keys.is_empty() {
            self.policies
        } else {
            self.keys
        }
    }
}

impl OpenBaoReadAdapter for OpenBaoHttpAdapter {
    fn secret_snapshot(&self) -> AdapterResult<OpenBaoSecretSnapshot> {
        let mounts = self
            .request_json::<BTreeMap<String, OpenBaoMount>>("GET", "/v1/sys/mounts")?
            .into_iter()
            .map(|(path, mount)| OpenBaoMountSummary {
                path,
                engine_type: mount.engine_type,
                description: mount.description.unwrap_or_default(),
            })
            .collect();
        let policies = self
            .request_json::<OpenBaoPolicyList>("LIST", "/v1/sys/policies/acl")?
            .names()
            .into_iter()
            .map(|name| OpenBaoPolicySummary { name })
            .collect();

        Ok(OpenBaoSecretSnapshot { mounts, policies })
    }
}

impl SecretsReadAdapter for OpenBaoHttpAdapter {
    fn secret_mounts(&self) -> AdapterResult<Vec<SecretMountSummary>> {
        let snapshot = self.secret_snapshot()?;
        let policy_count = len_as_u16(snapshot.policies.len());

        Ok(snapshot
            .mounts
            .into_iter()
            .map(|mount| SecretMountSummary {
                transit_enabled: mount.engine_type == "transit",
                mount_path: mount.path,
                policy_count,
            })
            .collect())
    }
}

#[derive(Debug, Clone)]
pub struct ArgoCdHttpAdapter {
    base_url: String,
    token: String,
    timeout: Duration,
}

impl ArgoCdHttpAdapter {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.into(),
            timeout: Duration::from_secs(10),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn api_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    fn get_json<T>(&self, path: &str) -> AdapterResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.api_url(path);
        let authorization = format!("Bearer {}", self.token);
        let response = ureq::AgentBuilder::new()
            .timeout(self.timeout)
            .build()
            .get(&url)
            .set("Authorization", &authorization)
            .set("Accept", "application/json")
            .call()
            .map_err(|err| AdapterError {
                target: AdapterTarget::ArgoCd,
                message: format!("GET {url} failed: {err}"),
            })?;

        response.into_json::<T>().map_err(|err| AdapterError {
            target: AdapterTarget::ArgoCd,
            message: format!("GET {url} returned invalid JSON: {err}"),
        })
    }
}

#[derive(Debug, Deserialize)]
struct ArgoCdApplications {
    #[serde(default)]
    items: Vec<ArgoCdApplication>,
}

#[derive(Debug, Deserialize)]
struct ArgoCdApplication {
    #[serde(default)]
    metadata: Option<ArgoCdMetadata>,
    #[serde(default)]
    status: Option<ArgoCdStatus>,
}

#[derive(Debug, Deserialize)]
struct ArgoCdMetadata {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    namespace: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ArgoCdStatus {
    #[serde(default)]
    sync: Option<ArgoCdSyncStatus>,
    #[serde(default)]
    health: Option<ArgoCdHealthStatus>,
}

#[derive(Debug, Deserialize)]
struct ArgoCdSyncStatus {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    revision: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ArgoCdHealthStatus {
    #[serde(default)]
    status: Option<String>,
}

impl ArgoCdApplication {
    fn into_summary(self) -> GitOpsApplicationSummary {
        let metadata = self.metadata.unwrap_or(ArgoCdMetadata {
            name: None,
            namespace: None,
        });
        let status = self.status.unwrap_or(ArgoCdStatus {
            sync: None,
            health: None,
        });
        let sync = status.sync.unwrap_or(ArgoCdSyncStatus {
            status: None,
            revision: None,
        });
        let health = status.health.unwrap_or(ArgoCdHealthStatus { status: None });

        GitOpsApplicationSummary {
            name: metadata.name.unwrap_or_else(|| "unknown-app".to_string()),
            namespace: metadata.namespace.unwrap_or_else(|| "argocd".to_string()),
            sync_status: sync.status.unwrap_or_else(|| "Unknown".to_string()),
            health_status: health.status.unwrap_or_else(|| "Unknown".to_string()),
            revision: sync.revision.unwrap_or_else(|| "unknown".to_string()),
        }
    }
}

impl ArgoCdReadAdapter for ArgoCdHttpAdapter {
    fn sync_snapshot(&self) -> AdapterResult<GitOpsSyncSnapshot> {
        let applications = self
            .get_json::<ArgoCdApplications>("/api/v1/applications")?
            .items
            .into_iter()
            .map(ArgoCdApplication::into_summary)
            .collect();

        Ok(GitOpsSyncSnapshot { applications })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanningInfrastructureAdapter {
    pub target: AdapterTarget,
    pub mode: AdapterMode,
}

impl PlanningInfrastructureAdapter {
    pub fn new(target: AdapterTarget, mode: AdapterMode) -> Self {
        Self { target, mode }
    }

    fn planned(&self, prefix: &str, external_id: &str) -> AdapterReceipt {
        AdapterReceipt {
            target: self.target,
            external_id: format!("{prefix}:{external_id}"),
            status: AdapterStatus::Planned,
        }
    }

    fn target_name(&self) -> &'static str {
        match self.target {
            AdapterTarget::Keycloak => "keycloak",
            AdapterTarget::PowerDns => "powerdns",
            AdapterTarget::NetBox => "netbox",
            AdapterTarget::OpenBao => "openbao",
            AdapterTarget::ArgoCd => "argocd",
            AdapterTarget::Flux => "flux",
            AdapterTarget::CloudStack => "cloudstack",
            AdapterTarget::Proxmox => "proxmox",
            AdapterTarget::OpenNebula => "opennebula",
            AdapterTarget::OpenStack => "openstack",
            AdapterTarget::PostgreSql => "postgresql",
            AdapterTarget::PrivacyIdea => "privacyidea",
            AdapterTarget::Authentik => "authentik",
            AdapterTarget::CloudKitty => "cloudkitty",
            AdapterTarget::OpenMeter => "openmeter",
            AdapterTarget::KillBill => "killbill",
            AdapterTarget::Lago => "lago",
            AdapterTarget::OpenCost => "opencost",
            AdapterTarget::Kubernetes => "kubernetes",
            AdapterTarget::Ceph => "ceph",
            AdapterTarget::Redfish => "redfish",
            AdapterTarget::Maas => "maas",
            AdapterTarget::Foreman => "foreman",
            AdapterTarget::Ironic => "ironic",
            AdapterTarget::Metal3 => "metal3",
            AdapterTarget::Tinkerbell => "tinkerbell",
            AdapterTarget::Harbor => "harbor",
            AdapterTarget::DefectDojo => "defectdojo",
            AdapterTarget::DependencyTrack => "dependency-track",
            AdapterTarget::Wazuh => "wazuh",
        }
    }
}

impl HealthProbeAdapter for PlanningInfrastructureAdapter {
    fn probe(&self, endpoint: &str) -> AdapterResult<ConnectorHealth> {
        Ok(ConnectorHealth {
            target: self.target,
            endpoint: endpoint.to_string(),
            reachable: false,
            mode: self.mode,
        })
    }
}

impl DnsReadAdapter for PlanningInfrastructureAdapter {
    fn list_zones(&self) -> AdapterResult<Vec<DnsZoneSummary>> {
        Ok(vec![DnsZoneSummary {
            zone_name: format!("{}.example.gov", self.target_name()),
            owner_tenant: "ministry-health".to_string(),
            dnssec_enabled: true,
        }])
    }

    fn list_records(&self, zone_name: &str) -> AdapterResult<Vec<DnsRecordSummary>> {
        Ok(vec![DnsRecordSummary {
            zone_name: zone_name.to_string(),
            record_name: format!("www.{zone_name}"),
            record_type: "A".to_string(),
            ttl: 300,
            records: vec!["192.0.2.10".to_string()],
        }])
    }
}

impl InventoryReadAdapter for PlanningInfrastructureAdapter {
    fn inventory_summary(&self) -> AdapterResult<InventorySummary> {
        Ok(InventorySummary {
            site_count: 1,
            rack_count: 10,
            device_count: 64,
            circuit_count: 4,
            ip_address_count: 512,
        })
    }
}

impl NetBoxReadAdapter for PlanningInfrastructureAdapter {
    fn inventory_snapshot(&self) -> AdapterResult<NetBoxInventorySnapshot> {
        Ok(NetBoxInventorySnapshot {
            sites: vec![NetBoxSiteSummary {
                id: 1,
                name: "National Region 1".to_string(),
                slug: "national-region-1".to_string(),
                status: "active".to_string(),
            }],
            racks: vec![NetBoxRackSummary {
                id: 10,
                name: "RACK-A01".to_string(),
                site: "National Region 1".to_string(),
                status: "active".to_string(),
            }],
            devices: vec![NetBoxDeviceSummary {
                id: 100,
                name: "rack-a-node-01".to_string(),
                role: "compute".to_string(),
                site: "National Region 1".to_string(),
                rack: "RACK-A01".to_string(),
                status: "active".to_string(),
            }],
            ip_addresses: vec![NetBoxIpAddressSummary {
                id: 1000,
                address: "192.0.2.10/32".to_string(),
                status: "active".to_string(),
                assigned_object: "rack-a-node-01".to_string(),
            }],
        })
    }
}

impl IdentityReadAdapter for PlanningInfrastructureAdapter {
    fn identity_summary(&self) -> AdapterResult<IdentitySummary> {
        Ok(IdentitySummary {
            realm: "osdc".to_string(),
            group_count: 6,
            role_count: 12,
        })
    }
}

impl KeycloakReadAdapter for PlanningInfrastructureAdapter {
    fn identity_snapshot(&self, realm: &str) -> AdapterResult<KeycloakIdentitySnapshot> {
        Ok(KeycloakIdentitySnapshot {
            realms: vec![KeycloakRealmSummary {
                realm: realm.to_string(),
                enabled: true,
                display_name: "OSDC Sovereign Cloud".to_string(),
            }],
            groups: vec![
                KeycloakGroupSummary {
                    id: "grp-platform-operators".to_string(),
                    name: "platform-operators".to_string(),
                    path: "/platform-operators".to_string(),
                },
                KeycloakGroupSummary {
                    id: "grp-tenant-admins".to_string(),
                    name: "tenant-admins".to_string(),
                    path: "/tenant-admins".to_string(),
                },
            ],
            roles: vec![
                KeycloakRoleSummary {
                    id: "role-tenant-admin".to_string(),
                    name: "tenant-admin".to_string(),
                    composite: false,
                    client_role: false,
                },
                KeycloakRoleSummary {
                    id: "role-security-reviewer".to_string(),
                    name: "security-reviewer".to_string(),
                    composite: false,
                    client_role: false,
                },
            ],
        })
    }
}

impl SecretsReadAdapter for PlanningInfrastructureAdapter {
    fn secret_mounts(&self) -> AdapterResult<Vec<SecretMountSummary>> {
        Ok(vec![SecretMountSummary {
            mount_path: "tenants/ministry-health".to_string(),
            policy_count: 3,
            transit_enabled: true,
        }])
    }
}

impl OpenBaoReadAdapter for PlanningInfrastructureAdapter {
    fn secret_snapshot(&self) -> AdapterResult<OpenBaoSecretSnapshot> {
        Ok(OpenBaoSecretSnapshot {
            mounts: vec![
                OpenBaoMountSummary {
                    path: "secret/".to_string(),
                    engine_type: "kv".to_string(),
                    description: "tenant secret storage".to_string(),
                },
                OpenBaoMountSummary {
                    path: "transit/".to_string(),
                    engine_type: "transit".to_string(),
                    description: "tenant encryption keys".to_string(),
                },
            ],
            policies: vec![
                OpenBaoPolicySummary {
                    name: "default".to_string(),
                },
                OpenBaoPolicySummary {
                    name: "tenant-ministry-health".to_string(),
                },
                OpenBaoPolicySummary {
                    name: "platform-secrets-admin".to_string(),
                },
            ],
        })
    }
}

impl GitOpsReadAdapter for PlanningInfrastructureAdapter {
    fn preview_change(&self, request: &ChangeRequest) -> AdapterResult<GitOpsChangePreview> {
        Ok(GitOpsChangePreview {
            change_id: request.id.clone(),
            target_branch: format!("osdc/{}", request.target_environment),
            files_changed: request.files.len() as u16,
            requires_approval: !request.rollout_plan.required_approvers.is_empty(),
        })
    }
}

impl ArgoCdReadAdapter for PlanningInfrastructureAdapter {
    fn sync_snapshot(&self) -> AdapterResult<GitOpsSyncSnapshot> {
        Ok(GitOpsSyncSnapshot {
            applications: vec![
                GitOpsApplicationSummary {
                    name: "tenant-api".to_string(),
                    namespace: "argocd".to_string(),
                    sync_status: "Synced".to_string(),
                    health_status: "Healthy".to_string(),
                    revision: "git-sha-tenant-api".to_string(),
                },
                GitOpsApplicationSummary {
                    name: "edge-shield".to_string(),
                    namespace: "argocd".to_string(),
                    sync_status: "OutOfSync".to_string(),
                    health_status: "Progressing".to_string(),
                    revision: "git-sha-edge-shield".to_string(),
                },
            ],
        })
    }
}

impl VirtualizationReadAdapter for PlanningInfrastructureAdapter {
    fn cluster_summary(&self) -> AdapterResult<VirtualizationClusterSummary> {
        Ok(VirtualizationClusterSummary {
            cluster_name: format!("{}-regional-pilot", self.target_name()),
            vm_count: 28,
            storage_pool_count: 3,
            backup_job_count: 4,
        })
    }
}

impl OpenStackReadAdapter for PlanningInfrastructureAdapter {
    fn project_summary(&self, project_id: &str) -> AdapterResult<OpenStackProjectSummary> {
        Ok(OpenStackProjectSummary {
            project_id: project_id.to_string(),
            instance_count: 18,
            network_count: 5,
            volume_count: 24,
        })
    }
}

impl PortalPersistenceAdapter for PlanningInfrastructureAdapter {
    fn migration_plan(&self) -> AdapterResult<PersistenceMigrationPlan> {
        Ok(PersistenceMigrationPlan {
            schema_name: "osdc_portal".to_string(),
            migration_count: 1,
            table_count: 11,
            destructive: false,
        })
    }
}

impl IdentityMfaAdapter for PlanningInfrastructureAdapter {
    fn mfa_summary(&self, tenant_id: &str) -> AdapterResult<MfaPolicySummary> {
        Ok(MfaPolicySummary {
            realm: tenant_id.to_string(),
            policy_id: "MFA_TENANT_ADMIN".to_string(),
            enrolled_subjects: 24,
            unenrolled_subjects: 2,
            recovery_enabled: true,
        })
    }

    fn enforce_mfa_policy(&self, request: &MfaEnrollmentRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned(
            "mfa-policy",
            &format!(
                "{}:{}:{}",
                request.tenant_id, request.subject, request.policy_id
            ),
        ))
    }
}

impl CustomerOperationsAdapter for PlanningInfrastructureAdapter {
    fn customer_site_summary(&self, customer_id: &str) -> AdapterResult<CustomerSiteSummary> {
        Ok(CustomerSiteSummary {
            customer_id: customer_id.to_string(),
            site_count: 2,
            active_sites: 1,
            residency_zones: vec!["national-region-1".to_string()],
        })
    }

    fn provision_customer_site(
        &self,
        request: &CustomerSiteProvisionRequest,
    ) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned(
            "customer-site",
            &format!("{}:{}", request.customer_id, request.site_id),
        ))
    }
}

impl UsageMeteringAdapter for PlanningInfrastructureAdapter {
    fn meter_summary(&self, customer_id: &str) -> AdapterResult<Vec<UsageMeterSummary>> {
        Ok(vec![
            UsageMeterSummary {
                meter_id: "METER_VM_HOURS".to_string(),
                customer_id: customer_id.to_string(),
                metric_name: "instance_hours".to_string(),
                quantity: 18_800,
                unit: "hour".to_string(),
            },
            UsageMeterSummary {
                meter_id: "METER_OBJECT_GB".to_string(),
                customer_id: customer_id.to_string(),
                metric_name: "object_storage_gb_month".to_string(),
                quantity: 2_100,
                unit: "gb-month".to_string(),
            },
        ])
    }

    fn rate_usage(&self, request: &UsageRatingRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned(
            "rated-usage",
            &format!(
                "{}:{}:{}",
                request.customer_id, request.meter_id, request.billing_period
            ),
        ))
    }
}

impl BillingAdapter for PlanningInfrastructureAdapter {
    fn preview_invoice(
        &self,
        request: &InvoiceGenerationRequest,
    ) -> AdapterResult<InvoicePreviewSummary> {
        Ok(InvoicePreviewSummary {
            invoice_id: format!("INV_{}_{}", request.customer_id, request.billing_period),
            customer_id: request.customer_id.clone(),
            billing_period: request.billing_period.clone(),
            amount_usd: 27_640.0,
            status: "draft".to_string(),
        })
    }

    fn generate_invoice(
        &self,
        request: &InvoiceGenerationRequest,
    ) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned(
            "invoice",
            &format!(
                "{}:{}:{}",
                request.customer_id, request.plan_id, request.billing_period
            ),
        ))
    }
}

impl IdentityAdapter for PlanningInfrastructureAdapter {
    fn create_tenant(&self, request: &TenantRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("tenant", &request.tenant_id))
    }

    fn assign_role(&self, request: &RoleAssignmentRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned(
            "role-assignment",
            &format!("{}:{}:{}", request.tenant_id, request.subject, request.role),
        ))
    }
}

impl DnsAdapter for PlanningInfrastructureAdapter {
    fn create_zone(&self, request: &DnsZoneRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("dns-zone", &request.zone_name))
    }
}

impl InventoryAdapter for PlanningInfrastructureAdapter {
    fn register_device(&self, request: &InventoryDeviceRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("inventory-device", &request.asset_tag))
    }
}

impl SecretsAdapter for PlanningInfrastructureAdapter {
    fn apply_policy(&self, request: &SecretPolicyRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("secret-policy", &request.path))
    }
}

impl GitOpsAdapter for PlanningInfrastructureAdapter {
    fn submit_change(&self, request: &ChangeRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("gitops-change", &request.id))
    }
}

impl BareMetalProvisionerAdapter for PlanningInfrastructureAdapter {
    fn enroll_node(&self, request: &BareMetalNodeRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("enroll-node", &request.hostname))
    }

    fn deploy_node(&self, request: &BareMetalNodeRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("deploy-node", &request.request_id))
    }
}

impl RedfishAdapter for PlanningInfrastructureAdapter {
    fn power_action(&self, request: &RedfishPowerRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("power-action", &request.node_id))
    }

    fn apply_firmware_baseline(
        &self,
        request: &FirmwareBaselineRequest,
    ) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("firmware-baseline", &request.baseline_id))
    }
}

impl OpenStackAdapter for PlanningInfrastructureAdapter {
    fn provision_instance(
        &self,
        request: &OpenStackProvisionRequest,
    ) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("openstack-instance", &request.project_id))
    }
}

impl VirtualizationAdapter for PlanningInfrastructureAdapter {
    fn provision_virtual_machine(
        &self,
        request: &VirtualMachineProvisionRequest,
    ) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned(
            "virtual-machine",
            &format!("{}:{}", request.project_id, request.instance_name),
        ))
    }
}

impl KubernetesAdapter for PlanningInfrastructureAdapter {
    fn submit_workload(
        &self,
        request: &KubernetesWorkloadRequest,
    ) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("kubernetes-workload", &request.workload_name))
    }
}

impl CephAdapter for PlanningInfrastructureAdapter {
    fn create_storage_allocation(
        &self,
        request: &CephStorageRequest,
    ) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("ceph-allocation", &request.tenant_id))
    }
}

impl RegistryAdapter for PlanningInfrastructureAdapter {
    fn create_project(&self, request: &RegistryProjectRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("registry-project", &request.project))
    }
}

impl SecurityFindingAdapter for PlanningInfrastructureAdapter {
    fn ingest_findings(&self, request: &FindingIngestRequest) -> AdapterResult<AdapterReceipt> {
        Ok(self.planned("finding-ingest", &request.engagement))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
        time::Duration,
    };

    struct PlanningGitOpsAdapter;

    fn spawn_powerdns_fixture() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                let mut buffer = [0_u8; 4096];
                let read = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..read]);
                assert!(request.contains("X-API-Key: test-key"));
                let first_line = request.lines().next().unwrap_or_default();
                let body = if first_line.starts_with("GET /api/v1/servers/localhost/zones ") {
                    r#"[{"name":"health.example.gov.","account":"ministry-health","dnssec":true}]"#
                } else if first_line
                    .starts_with("GET /api/v1/servers/localhost/zones/health.example.gov. ")
                {
                    r#"{
                        "rrsets": [
                            {
                                "name": "www.health.example.gov.",
                                "type": "A",
                                "ttl": 300,
                                "records": [
                                    {"content": "192.0.2.20", "disabled": false},
                                    {"content": "192.0.2.21", "disabled": true}
                                ]
                            },
                            {
                                "name": "health.example.gov.",
                                "type": "MX",
                                "ttl": 3600,
                                "records": [
                                    {"content": "10 mail.health.example.gov.", "disabled": false}
                                ]
                            }
                        ]
                    }"#
                } else {
                    r#"{"error":"unexpected path"}"#
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
        });

        format!("http://{addr}")
    }

    fn spawn_netbox_fixture() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let next_sites_url = format!("http://{addr}/api/dcim/sites/?offset=1");

        thread::spawn(move || {
            for _ in 0..5 {
                let (mut stream, _) = listener.accept().unwrap();
                let mut buffer = [0_u8; 4096];
                let read = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..read]);
                assert!(request
                    .to_ascii_lowercase()
                    .contains("authorization: token test-token"));
                let first_line = request.lines().next().unwrap_or_default();
                let body = if first_line.starts_with("GET /api/dcim/sites/ ") {
                    format!(
                        r#"{{
                            "count": 2,
                            "next": "{}",
                            "previous": null,
                            "results": [
                                {{
                                    "id": 1,
                                    "name": "National Region 1",
                                    "slug": "national-region-1",
                                    "status": {{"value": "active", "label": "Active"}}
                                }}
                            ]
                        }}"#,
                        next_sites_url
                    )
                } else if first_line.starts_with("GET /api/dcim/sites/?offset=1 ") {
                    r#"{
                        "count": 2,
                        "next": null,
                        "previous": "http://example.invalid/api/dcim/sites/",
                        "results": [
                            {
                                "id": 2,
                                "name": "Disaster Recovery",
                                "slug": "disaster-recovery",
                                "status": {"value": "planned", "label": "Planned"}
                            }
                        ]
                    }"#
                    .to_string()
                } else if first_line.starts_with("GET /api/dcim/racks/ ") {
                    r#"{
                        "count": 1,
                        "next": null,
                        "previous": null,
                        "results": [
                            {
                                "id": 10,
                                "name": "RACK-A01",
                                "site": {"name": "National Region 1", "slug": "national-region-1"},
                                "status": {"value": "active", "label": "Active"}
                            }
                        ]
                    }"#
                    .to_string()
                } else if first_line.starts_with("GET /api/dcim/devices/ ") {
                    r#"{
                        "count": 1,
                        "next": null,
                        "previous": null,
                        "results": [
                            {
                                "id": 100,
                                "name": "rack-a-node-01",
                                "role": {"name": "compute", "slug": "compute"},
                                "site": {"name": "National Region 1", "slug": "national-region-1"},
                                "rack": {"name": "RACK-A01"},
                                "status": {"value": "active", "label": "Active"}
                            }
                        ]
                    }"#
                    .to_string()
                } else if first_line.starts_with("GET /api/ipam/ip-addresses/ ") {
                    r#"{
                        "count": 1,
                        "next": null,
                        "previous": null,
                        "results": [
                            {
                                "id": 1000,
                                "address": "192.0.2.10/32",
                                "status": {"value": "active", "label": "Active"},
                                "assigned_object": {
                                    "display": "rack-a-node-01 eth0",
                                    "device": {"name": "rack-a-node-01"}
                                }
                            }
                        ]
                    }"#
                    .to_string()
                } else {
                    r#"{"error":"unexpected path"}"#.to_string()
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
        });

        format!("http://{addr}")
    }

    fn spawn_keycloak_fixture() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            for _ in 0..3 {
                let (mut stream, _) = listener.accept().unwrap();
                let mut buffer = [0_u8; 4096];
                let read = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..read]);
                assert!(request
                    .to_ascii_lowercase()
                    .contains("authorization: bearer test-token"));
                let first_line = request.lines().next().unwrap_or_default();
                let body = if first_line.starts_with("GET /admin/realms ") {
                    r#"[
                        {
                            "realm": "osdc",
                            "enabled": true,
                            "displayName": "OSDC Sovereign Cloud"
                        }
                    ]"#
                } else if first_line.starts_with("GET /admin/realms/osdc/groups ") {
                    r#"[
                        {
                            "id": "grp-platform-operators",
                            "name": "platform-operators",
                            "path": "/platform-operators"
                        },
                        {
                            "id": "grp-tenant-admins",
                            "name": "tenant-admins",
                            "path": "/tenant-admins"
                        }
                    ]"#
                } else if first_line.starts_with("GET /admin/realms/osdc/roles ") {
                    r#"[
                        {
                            "id": "role-tenant-admin",
                            "name": "tenant-admin",
                            "composite": false,
                            "clientRole": false
                        },
                        {
                            "id": "role-platform-admin",
                            "name": "platform-admin",
                            "composite": true,
                            "clientRole": false
                        }
                    ]"#
                } else {
                    r#"{"error":"unexpected path"}"#
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
        });

        format!("http://{addr}")
    }

    fn spawn_openbao_fixture() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                let mut buffer = [0_u8; 4096];
                let read = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..read]);
                assert!(request.contains("X-Vault-Token: test-token"));
                let first_line = request.lines().next().unwrap_or_default();
                let body = if first_line.starts_with("GET /v1/sys/mounts ") {
                    r#"{
                        "secret/": {
                            "type": "kv",
                            "description": "tenant secrets"
                        },
                        "transit/": {
                            "type": "transit",
                            "description": "tenant encryption keys"
                        }
                    }"#
                } else if first_line.starts_with("LIST /v1/sys/policies/acl ") {
                    r#"{
                        "keys": [
                            "default",
                            "tenant-ministry-health",
                            "platform-secrets-admin"
                        ]
                    }"#
                } else {
                    r#"{"error":"unexpected path"}"#
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
        });

        format!("http://{addr}")
    }

    fn spawn_argocd_fixture() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0_u8; 4096];
            let read = stream.read(&mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer[..read]);
            assert!(request
                .to_ascii_lowercase()
                .contains("authorization: bearer test-token"));
            let first_line = request.lines().next().unwrap_or_default();
            let body = if first_line.starts_with("GET /api/v1/applications ") {
                r#"{
                    "items": [
                        {
                            "metadata": {
                                "name": "tenant-api",
                                "namespace": "argocd"
                            },
                            "status": {
                                "sync": {
                                    "status": "Synced",
                                    "revision": "abc123"
                                },
                                "health": {
                                    "status": "Healthy"
                                }
                            }
                        },
                        {
                            "metadata": {
                                "name": "edge-shield",
                                "namespace": "argocd"
                            },
                            "status": {
                                "sync": {
                                    "status": "OutOfSync",
                                    "revision": "def456"
                                },
                                "health": {
                                    "status": "Progressing"
                                }
                            }
                        }
                    ]
                }"#
            } else {
                r#"{"error":"unexpected path"}"#
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).unwrap();
        });

        format!("http://{addr}")
    }

    impl GitOpsAdapter for PlanningGitOpsAdapter {
        fn submit_change(&self, request: &ChangeRequest) -> AdapterResult<AdapterReceipt> {
            Ok(AdapterReceipt {
                target: AdapterTarget::ArgoCd,
                external_id: format!("planned:{}", request.id),
                status: AdapterStatus::Planned,
            })
        }
    }

    #[test]
    fn adapter_contracts_can_plan_gitops_change_submission() {
        let request = ChangeRequest {
            id: "cr-0001".to_string(),
            title: "stage edge config".to_string(),
            requester: "operator".to_string(),
            target_system: "edge-shield".to_string(),
            target_environment: "staging".to_string(),
            change_type: osdc_models::ChangeType::ConfigScript,
            risk: osdc_models::ChangeRisk::Medium,
            files: Vec::new(),
            validations: Vec::new(),
            rollout_plan: osdc_models::RolloutPlan {
                strategy: osdc_models::RolloutStrategy::GitOpsPullRequest,
                stages: Vec::new(),
                required_approvers: vec!["edge-owner".to_string()],
            },
            rollback_plan: osdc_models::RollbackPlan {
                trigger_conditions: Vec::new(),
                restore_actions: Vec::new(),
                evidence_required: Vec::new(),
            },
            audit_events: Vec::new(),
        };

        let receipt = PlanningGitOpsAdapter.submit_change(&request).unwrap();

        assert_eq!(receipt.target, AdapterTarget::ArgoCd);
        assert_eq!(receipt.status, AdapterStatus::Planned);
        assert_eq!(receipt.external_id, "planned:cr-0001");
    }

    #[test]
    fn planning_infrastructure_adapter_plans_bare_metal_and_redfish_actions() {
        let bare_metal =
            PlanningInfrastructureAdapter::new(AdapterTarget::Metal3, AdapterMode::GitOps);
        let request = BareMetalNodeRequest {
            request_id: "hreq-001".to_string(),
            hostname: "gpu-node-01".to_string(),
            profile_id: "HP_GPU_AI".to_string(),
            bmc_address: "https://bmc-gpu-node-01.example".to_string(),
            target_pool: "ai-gpu".to_string(),
        };

        let enroll_receipt = bare_metal.enroll_node(&request).unwrap();
        let deploy_receipt = bare_metal.deploy_node(&request).unwrap();

        assert_eq!(enroll_receipt.target, AdapterTarget::Metal3);
        assert_eq!(enroll_receipt.status, AdapterStatus::Planned);
        assert_eq!(enroll_receipt.external_id, "enroll-node:gpu-node-01");
        assert_eq!(deploy_receipt.external_id, "deploy-node:hreq-001");

        let redfish =
            PlanningInfrastructureAdapter::new(AdapterTarget::Redfish, AdapterMode::GuardedApi);
        let power_receipt = redfish
            .power_action(&RedfishPowerRequest {
                node_id: "gpu-node-01".to_string(),
                action: "power-cycle".to_string(),
                approval_ref: "CAB-42".to_string(),
            })
            .unwrap();
        let firmware_receipt = redfish
            .apply_firmware_baseline(&FirmwareBaselineRequest {
                node_id: "gpu-node-01".to_string(),
                baseline_id: "fw-bmc-2026q2".to_string(),
                rollback_ref: "fw-bmc-2026q1".to_string(),
            })
            .unwrap();

        assert_eq!(power_receipt.target, AdapterTarget::Redfish);
        assert_eq!(power_receipt.external_id, "power-action:gpu-node-01");
        assert_eq!(
            firmware_receipt.external_id,
            "firmware-baseline:fw-bmc-2026q2"
        );
    }

    #[test]
    fn planning_infrastructure_adapter_plans_cloud_storage_registry_and_security_actions() {
        let openstack =
            PlanningInfrastructureAdapter::new(AdapterTarget::OpenStack, AdapterMode::GuardedApi);
        let openstack_receipt = openstack
            .provision_instance(&OpenStackProvisionRequest {
                project_id: "research-a".to_string(),
                flavor: "gpu-open.1x16g".to_string(),
                image: "debian-13".to_string(),
                network_id: "research-private".to_string(),
            })
            .unwrap();
        assert_eq!(openstack_receipt.target, AdapterTarget::OpenStack);
        assert_eq!(
            openstack_receipt.external_id,
            "openstack-instance:research-a"
        );

        let kubernetes =
            PlanningInfrastructureAdapter::new(AdapterTarget::Kubernetes, AdapterMode::GitOps);
        let workload_receipt = kubernetes
            .submit_workload(&KubernetesWorkloadRequest {
                namespace: "ai-serving".to_string(),
                workload_name: "vllm-gpu-endpoint".to_string(),
                manifest_path: "clusters/prod/ai/vllm.yaml".to_string(),
                git_ref: "refs/heads/platform/prod".to_string(),
            })
            .unwrap();
        assert_eq!(
            workload_receipt.external_id,
            "kubernetes-workload:vllm-gpu-endpoint"
        );

        let ceph = PlanningInfrastructureAdapter::new(AdapterTarget::Ceph, AdapterMode::GuardedApi);
        let ceph_receipt = ceph
            .create_storage_allocation(&CephStorageRequest {
                tenant_id: "research-a".to_string(),
                storage_class: "ceph-rbd-replicated".to_string(),
                capacity_gib: 2048,
            })
            .unwrap();
        assert_eq!(ceph_receipt.external_id, "ceph-allocation:research-a");

        let harbor =
            PlanningInfrastructureAdapter::new(AdapterTarget::Harbor, AdapterMode::GuardedApi);
        let harbor_receipt = harbor
            .create_project(&RegistryProjectRequest {
                project: "research-a".to_string(),
                owner: "research-platform".to_string(),
                require_signed_images: true,
            })
            .unwrap();
        assert_eq!(harbor_receipt.external_id, "registry-project:research-a");

        let defect_dojo = PlanningInfrastructureAdapter::new(
            AdapterTarget::DefectDojo,
            AdapterMode::EvidenceIngest,
        );
        let finding_receipt = defect_dojo
            .ingest_findings(&FindingIngestRequest {
                engagement: "release-2026-06".to_string(),
                scanner: "trivy".to_string(),
                artifact_path: "evidence/security/trivy.json".to_string(),
                owner: "platform-security".to_string(),
            })
            .unwrap();
        assert_eq!(
            finding_receipt.external_id,
            "finding-ingest:release-2026-06"
        );
    }

    #[test]
    fn planning_adapter_covers_first_live_integration_milestones() {
        let identity =
            PlanningInfrastructureAdapter::new(AdapterTarget::Keycloak, AdapterMode::GuardedApi);
        let tenant = identity
            .create_tenant(&TenantRequest {
                tenant_id: "ministry-health".to_string(),
                display_name: "Ministry of Health".to_string(),
                data_residency_zone: "national-region-1".to_string(),
            })
            .unwrap();
        let role = identity
            .assign_role(&RoleAssignmentRequest {
                tenant_id: "ministry-health".to_string(),
                subject: "ops-team".to_string(),
                role: "tenant-admin".to_string(),
            })
            .unwrap();
        assert_eq!(tenant.external_id, "tenant:ministry-health");
        assert_eq!(
            role.external_id,
            "role-assignment:ministry-health:ops-team:tenant-admin"
        );

        let dns = PlanningInfrastructureAdapter::new(AdapterTarget::PowerDns, AdapterMode::GitOps);
        let zone = dns
            .create_zone(&DnsZoneRequest {
                zone_name: "health.example.gov".to_string(),
                owner_tenant: "ministry-health".to_string(),
                dnssec_required: true,
            })
            .unwrap();
        assert_eq!(zone.target, AdapterTarget::PowerDns);
        assert_eq!(zone.external_id, "dns-zone:health.example.gov");

        let inventory =
            PlanningInfrastructureAdapter::new(AdapterTarget::NetBox, AdapterMode::ReadOnly);
        let device = inventory
            .register_device(&InventoryDeviceRequest {
                name: "rack-a-node-01".to_string(),
                role: "compute".to_string(),
                rack: "rack-a".to_string(),
                asset_tag: "ASSET-001".to_string(),
            })
            .unwrap();
        assert_eq!(device.external_id, "inventory-device:ASSET-001");

        let secrets =
            PlanningInfrastructureAdapter::new(AdapterTarget::OpenBao, AdapterMode::GuardedApi);
        let policy = secrets
            .apply_policy(&SecretPolicyRequest {
                path: "tenants/ministry-health".to_string(),
                owner: "security".to_string(),
                rotation_days: 90,
            })
            .unwrap();
        assert_eq!(policy.external_id, "secret-policy:tenants/ministry-health");
    }

    #[test]
    fn powerdns_http_adapter_reads_zones_and_records() {
        let base_url = spawn_powerdns_fixture();
        let adapter = PowerDnsHttpAdapter::new(base_url, "test-key", "localhost")
            .with_timeout(Duration::from_secs(2));

        let zones = adapter.list_zones().unwrap();
        assert_eq!(
            zones,
            vec![DnsZoneSummary {
                zone_name: "health.example.gov.".to_string(),
                owner_tenant: "ministry-health".to_string(),
                dnssec_enabled: true,
            }]
        );

        let records = adapter.list_records("health.example.gov.").unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].record_name, "www.health.example.gov.");
        assert_eq!(records[0].record_type, "A");
        assert_eq!(records[0].ttl, 300);
        assert_eq!(records[0].records, vec!["192.0.2.20"]);
        assert_eq!(records[1].record_type, "MX");
        assert_eq!(records[1].records, vec!["10 mail.health.example.gov."]);
    }

    #[test]
    fn netbox_http_adapter_reads_inventory_snapshot() {
        let base_url = spawn_netbox_fixture();
        let adapter =
            NetBoxHttpAdapter::new(base_url, "test-token").with_timeout(Duration::from_secs(2));

        let snapshot = adapter.inventory_snapshot().unwrap();

        assert_eq!(snapshot.sites.len(), 2);
        assert_eq!(snapshot.sites[0].name, "National Region 1");
        assert_eq!(snapshot.sites[0].slug, "national-region-1");
        assert_eq!(snapshot.sites[0].status, "active");
        assert_eq!(snapshot.sites[1].status, "planned");
        assert_eq!(snapshot.racks.len(), 1);
        assert_eq!(snapshot.racks[0].site, "National Region 1");
        assert_eq!(snapshot.devices.len(), 1);
        assert_eq!(snapshot.devices[0].name, "rack-a-node-01");
        assert_eq!(snapshot.devices[0].role, "compute");
        assert_eq!(snapshot.devices[0].rack, "RACK-A01");
        assert_eq!(snapshot.ip_addresses.len(), 1);
        assert_eq!(snapshot.ip_addresses[0].address, "192.0.2.10/32");
        assert_eq!(
            snapshot.ip_addresses[0].assigned_object,
            "rack-a-node-01 eth0"
        );
    }

    #[test]
    fn keycloak_http_adapter_reads_identity_snapshot() {
        let base_url = spawn_keycloak_fixture();
        let adapter = KeycloakHttpAdapter::new(base_url, "test-token", "osdc")
            .with_timeout(Duration::from_secs(2));

        let snapshot = adapter.identity_snapshot("osdc").unwrap();

        assert_eq!(snapshot.realms.len(), 1);
        assert_eq!(snapshot.realms[0].realm, "osdc");
        assert!(snapshot.realms[0].enabled);
        assert_eq!(snapshot.groups.len(), 2);
        assert_eq!(snapshot.groups[0].path, "/platform-operators");
        assert_eq!(snapshot.roles.len(), 2);
        assert_eq!(snapshot.roles[0].name, "tenant-admin");
        assert!(!snapshot.roles[0].client_role);
        assert!(snapshot.roles[1].composite);
    }

    #[test]
    fn openbao_http_adapter_reads_secret_snapshot() {
        let base_url = spawn_openbao_fixture();
        let adapter =
            OpenBaoHttpAdapter::new(base_url, "test-token").with_timeout(Duration::from_secs(2));

        let snapshot = adapter.secret_snapshot().unwrap();

        assert_eq!(snapshot.mounts.len(), 2);
        assert_eq!(snapshot.mounts[0].path, "secret/");
        assert_eq!(snapshot.mounts[0].engine_type, "kv");
        assert_eq!(snapshot.mounts[1].engine_type, "transit");
        assert_eq!(snapshot.policies.len(), 3);
        assert!(snapshot
            .policies
            .iter()
            .any(|policy| policy.name == "tenant-ministry-health"));
    }

    #[test]
    fn argocd_http_adapter_reads_sync_snapshot() {
        let base_url = spawn_argocd_fixture();
        let adapter =
            ArgoCdHttpAdapter::new(base_url, "test-token").with_timeout(Duration::from_secs(2));

        let snapshot = adapter.sync_snapshot().unwrap();

        assert_eq!(snapshot.applications.len(), 2);
        assert_eq!(snapshot.applications[0].name, "tenant-api");
        assert_eq!(snapshot.applications[0].sync_status, "Synced");
        assert_eq!(snapshot.applications[0].health_status, "Healthy");
        assert_eq!(snapshot.applications[0].revision, "abc123");
        assert_eq!(snapshot.applications[1].name, "edge-shield");
        assert_eq!(snapshot.applications[1].sync_status, "OutOfSync");
        assert_eq!(snapshot.applications[1].health_status, "Progressing");
    }

    #[test]
    fn planning_adapter_models_customer_ops_mfa_metering_and_billing() {
        let mfa =
            PlanningInfrastructureAdapter::new(AdapterTarget::PrivacyIdea, AdapterMode::GuardedApi);
        let summary = mfa.mfa_summary("health.gov").unwrap();
        assert_eq!(summary.realm, "health.gov");
        assert!(summary.recovery_enabled);

        let mfa_receipt = mfa
            .enforce_mfa_policy(&MfaEnrollmentRequest {
                tenant_id: "CUST_HEALTH".to_string(),
                subject: "tenant-admins".to_string(),
                policy_id: "MFA_TENANT_ADMIN".to_string(),
                factors: vec!["webauthn".to_string(), "totp".to_string()],
            })
            .unwrap();
        assert_eq!(
            mfa_receipt.external_id,
            "mfa-policy:CUST_HEALTH:tenant-admins:MFA_TENANT_ADMIN"
        );

        let customer_ops =
            PlanningInfrastructureAdapter::new(AdapterTarget::NetBox, AdapterMode::GuardedApi);
        let site_summary = customer_ops.customer_site_summary("CUST_HEALTH").unwrap();
        assert_eq!(site_summary.active_sites, 1);
        let site_receipt = customer_ops
            .provision_customer_site(&CustomerSiteProvisionRequest {
                customer_id: "CUST_HEALTH".to_string(),
                site_id: "SITE_HEALTH_REGIONAL".to_string(),
                deployment_profile: "DSP_250KW_REGIONAL".to_string(),
                residency_zone: "national-region-1".to_string(),
            })
            .unwrap();
        assert_eq!(
            site_receipt.external_id,
            "customer-site:CUST_HEALTH:SITE_HEALTH_REGIONAL"
        );

        let metering =
            PlanningInfrastructureAdapter::new(AdapterTarget::OpenMeter, AdapterMode::GuardedApi);
        let meters = metering.meter_summary("CUST_HEALTH").unwrap();
        assert!(meters
            .iter()
            .any(|meter| meter.meter_id == "METER_VM_HOURS"));
        let rating = metering
            .rate_usage(&UsageRatingRequest {
                customer_id: "CUST_HEALTH".to_string(),
                meter_id: "METER_VM_HOURS".to_string(),
                quantity: 18_800,
                billing_period: "2026-06".to_string(),
            })
            .unwrap();
        assert_eq!(
            rating.external_id,
            "rated-usage:CUST_HEALTH:METER_VM_HOURS:2026-06"
        );

        let billing =
            PlanningInfrastructureAdapter::new(AdapterTarget::KillBill, AdapterMode::GuardedApi);
        let invoice_request = InvoiceGenerationRequest {
            customer_id: "CUST_HEALTH".to_string(),
            billing_period: "2026-06".to_string(),
            plan_id: "BILL_PUBLIC_CRITICAL".to_string(),
            approval_ref: "APPROVAL-001".to_string(),
        };
        let preview = billing.preview_invoice(&invoice_request).unwrap();
        assert_eq!(preview.invoice_id, "INV_CUST_HEALTH_2026-06");
        let invoice = billing.generate_invoice(&invoice_request).unwrap();
        assert_eq!(
            invoice.external_id,
            "invoice:CUST_HEALTH:BILL_PUBLIC_CRITICAL:2026-06"
        );
    }

    #[test]
    fn planning_adapter_models_small_site_virtualization_profiles() {
        let proxmox =
            PlanningInfrastructureAdapter::new(AdapterTarget::Proxmox, AdapterMode::GuardedApi);
        let proxmox_receipt = proxmox
            .provision_virtual_machine(&VirtualMachineProvisionRequest {
                project_id: "edge-micro".to_string(),
                instance_name: "dns-a".to_string(),
                template: "debian-13-cloudinit".to_string(),
                network_id: "edge-services".to_string(),
                storage_profile: "zfs-replicated".to_string(),
            })
            .unwrap();
        assert_eq!(proxmox_receipt.target, AdapterTarget::Proxmox);
        assert_eq!(
            proxmox_receipt.external_id,
            "virtual-machine:edge-micro:dns-a"
        );

        let cloudstack =
            PlanningInfrastructureAdapter::new(AdapterTarget::CloudStack, AdapterMode::GuardedApi);
        let cloudstack_receipt = cloudstack
            .provision_virtual_machine(&VirtualMachineProvisionRequest {
                project_id: "regional-pilot".to_string(),
                instance_name: "tenant-api-01".to_string(),
                template: "ubuntu-lts".to_string(),
                network_id: "tenant-private".to_string(),
                storage_profile: "ceph-backed".to_string(),
            })
            .unwrap();
        assert_eq!(cloudstack_receipt.target, AdapterTarget::CloudStack);
        assert_eq!(
            cloudstack_receipt.external_id,
            "virtual-machine:regional-pilot:tenant-api-01"
        );
    }

    #[test]
    fn planning_probe_reports_unreachable_plan_only_endpoint() {
        let adapter =
            PlanningInfrastructureAdapter::new(AdapterTarget::NetBox, AdapterMode::ReadOnly);

        let health = adapter.probe("https://netbox.internal.example").unwrap();

        assert_eq!(health.target, AdapterTarget::NetBox);
        assert_eq!(health.endpoint, "https://netbox.internal.example");
        assert!(!health.reachable);
        assert_eq!(health.mode, AdapterMode::ReadOnly);
    }

    #[test]
    fn read_first_contracts_cover_all_live_adapter_roadmap_targets() {
        let powerdns =
            PlanningInfrastructureAdapter::new(AdapterTarget::PowerDns, AdapterMode::ReadOnly);
        let zones = powerdns.list_zones().unwrap();
        assert_eq!(zones[0].owner_tenant, "ministry-health");
        assert!(zones[0].dnssec_enabled);
        let records = powerdns.list_records(&zones[0].zone_name).unwrap();
        assert_eq!(records[0].record_type, "A");
        assert_eq!(records[0].records, vec!["192.0.2.10"]);

        let netbox =
            PlanningInfrastructureAdapter::new(AdapterTarget::NetBox, AdapterMode::ReadOnly);
        let inventory = netbox.inventory_summary().unwrap();
        assert_eq!(inventory.site_count, 1);
        assert!(inventory.device_count >= 64);
        let snapshot = netbox.inventory_snapshot().unwrap();
        assert_eq!(snapshot.sites[0].name, "National Region 1");
        assert_eq!(snapshot.devices[0].rack, "RACK-A01");
        assert_eq!(snapshot.ip_addresses[0].assigned_object, "rack-a-node-01");

        let keycloak =
            PlanningInfrastructureAdapter::new(AdapterTarget::Keycloak, AdapterMode::ReadOnly);
        let identity = keycloak.identity_summary().unwrap();
        assert_eq!(identity.realm, "osdc");
        assert!(identity.role_count >= 12);
        let identity_snapshot = keycloak.identity_snapshot("osdc").unwrap();
        assert_eq!(identity_snapshot.realms[0].realm, "osdc");
        assert!(identity_snapshot
            .groups
            .iter()
            .any(|group| group.name == "tenant-admins"));
        assert!(identity_snapshot
            .roles
            .iter()
            .any(|role| role.name == "tenant-admin"));

        let openbao =
            PlanningInfrastructureAdapter::new(AdapterTarget::OpenBao, AdapterMode::ReadOnly);
        let mounts = openbao.secret_mounts().unwrap();
        assert_eq!(mounts[0].mount_path, "tenants/ministry-health");
        assert!(mounts[0].transit_enabled);
        let secret_snapshot = openbao.secret_snapshot().unwrap();
        assert!(secret_snapshot
            .mounts
            .iter()
            .any(|mount| mount.engine_type == "transit"));
        assert!(secret_snapshot
            .policies
            .iter()
            .any(|policy| policy.name == "tenant-ministry-health"));

        let gitops = PlanningInfrastructureAdapter::new(AdapterTarget::ArgoCd, AdapterMode::GitOps);
        let change = ChangeRequest {
            id: "cr-live-001".to_string(),
            title: "stage regional pilot VM".to_string(),
            requester: "platform-owner".to_string(),
            target_system: "cloudstack".to_string(),
            target_environment: "staging".to_string(),
            change_type: osdc_models::ChangeType::InfrastructurePlan,
            risk: osdc_models::ChangeRisk::Medium,
            files: vec![osdc_models::ConfigArtifact {
                path: "clusters/staging/apps/tenant-api.yaml".to_string(),
                owner: "platform-owner".to_string(),
                language: "yaml".to_string(),
                secret_policy: osdc_models::SecretPolicy::ReferencesOnly,
            }],
            validations: Vec::new(),
            rollout_plan: osdc_models::RolloutPlan {
                strategy: osdc_models::RolloutStrategy::GitOpsPullRequest,
                stages: Vec::new(),
                required_approvers: vec!["platform-owner".to_string()],
            },
            rollback_plan: osdc_models::RollbackPlan {
                trigger_conditions: Vec::new(),
                restore_actions: Vec::new(),
                evidence_required: Vec::new(),
            },
            audit_events: Vec::new(),
        };
        let preview = gitops.preview_change(&change).unwrap();
        assert_eq!(preview.change_id, "cr-live-001");
        assert_eq!(preview.target_branch, "osdc/staging");
        assert_eq!(preview.files_changed, 1);
        assert!(preview.requires_approval);
        let sync = gitops.sync_snapshot().unwrap();
        assert!(sync
            .applications
            .iter()
            .any(|application| application.sync_status == "OutOfSync"));

        let proxmox =
            PlanningInfrastructureAdapter::new(AdapterTarget::Proxmox, AdapterMode::ReadOnly);
        let proxmox_cluster = proxmox.cluster_summary().unwrap();
        assert!(proxmox_cluster.cluster_name.contains("proxmox"));
        assert!(proxmox_cluster.backup_job_count > 0);

        let cloudstack =
            PlanningInfrastructureAdapter::new(AdapterTarget::CloudStack, AdapterMode::ReadOnly);
        let cloudstack_cluster = cloudstack.cluster_summary().unwrap();
        assert!(cloudstack_cluster.cluster_name.contains("cloudstack"));
        assert!(cloudstack_cluster.storage_pool_count > 0);

        let openstack =
            PlanningInfrastructureAdapter::new(AdapterTarget::OpenStack, AdapterMode::ReadOnly);
        let project = openstack.project_summary("ministry-health").unwrap();
        assert_eq!(project.project_id, "ministry-health");
        assert!(project.instance_count > 0);

        let postgres =
            PlanningInfrastructureAdapter::new(AdapterTarget::PostgreSql, AdapterMode::PlanOnly);
        let migrations = postgres.migration_plan().unwrap();
        assert_eq!(migrations.schema_name, "osdc_portal");
        assert!(migrations.table_count >= 9);
        assert!(!migrations.destructive);
    }
}
