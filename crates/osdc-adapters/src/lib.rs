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
}
