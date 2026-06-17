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
}
