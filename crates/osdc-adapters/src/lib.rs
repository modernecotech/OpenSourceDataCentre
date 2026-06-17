use std::{error::Error, fmt};

use osdc_models::ChangeRequest;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsZoneSummary {
    pub zone_name: String,
    pub owner_tenant: String,
    pub dnssec_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventorySummary {
    pub site_count: u16,
    pub rack_count: u16,
    pub device_count: u16,
    pub circuit_count: u16,
    pub ip_address_count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentitySummary {
    pub realm: String,
    pub group_count: u16,
    pub role_count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretMountSummary {
    pub mount_path: String,
    pub policy_count: u16,
    pub transit_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitOpsChangePreview {
    pub change_id: String,
    pub target_branch: String,
    pub files_changed: u16,
    pub requires_approval: bool,
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
}

pub trait InventoryReadAdapter {
    fn inventory_summary(&self) -> AdapterResult<InventorySummary>;
}

pub trait IdentityReadAdapter {
    fn identity_summary(&self) -> AdapterResult<IdentitySummary>;
}

pub trait SecretsReadAdapter {
    fn secret_mounts(&self) -> AdapterResult<Vec<SecretMountSummary>>;
}

pub trait GitOpsReadAdapter {
    fn preview_change(&self, request: &ChangeRequest) -> AdapterResult<GitOpsChangePreview>;
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

impl IdentityReadAdapter for PlanningInfrastructureAdapter {
    fn identity_summary(&self) -> AdapterResult<IdentitySummary> {
        Ok(IdentitySummary {
            realm: "osdc".to_string(),
            group_count: 6,
            role_count: 12,
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

    struct PlanningGitOpsAdapter;

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

        let netbox =
            PlanningInfrastructureAdapter::new(AdapterTarget::NetBox, AdapterMode::ReadOnly);
        let inventory = netbox.inventory_summary().unwrap();
        assert_eq!(inventory.site_count, 1);
        assert!(inventory.device_count >= 64);

        let keycloak =
            PlanningInfrastructureAdapter::new(AdapterTarget::Keycloak, AdapterMode::ReadOnly);
        let identity = keycloak.identity_summary().unwrap();
        assert_eq!(identity.realm, "osdc");
        assert!(identity.role_count >= 12);

        let openbao =
            PlanningInfrastructureAdapter::new(AdapterTarget::OpenBao, AdapterMode::ReadOnly);
        let mounts = openbao.secret_mounts().unwrap();
        assert_eq!(mounts[0].mount_path, "tenants/ministry-health");
        assert!(mounts[0].transit_enabled);

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
