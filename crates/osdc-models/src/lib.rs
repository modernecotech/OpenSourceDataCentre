use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteProfile {
    pub site: SiteIdentity,
    pub energy: EnergyProfile,
    #[serde(default)]
    pub cooling: Option<CoolingRecoveryProfile>,
    #[serde(default)]
    pub resilience: Option<ResilienceProfile>,
    #[serde(default)]
    pub procurement: Option<ProcurementProfile>,
    #[serde(default)]
    pub sovereignty: Option<SovereigntyProfile>,
    #[serde(default)]
    pub operations: Option<OperationsProfile>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteIdentity {
    pub name: String,
    pub country: String,
    pub currency: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EnergyProfile {
    pub it_load_kw: f64,
    pub pue: f64,
    pub annual_hours: f64,
    pub electricity_price_per_kwh: f64,
    pub grid_carbon_kg_per_kwh: f64,
    pub water_liters_per_facility_kwh: f64,
    #[serde(default)]
    pub onsite_renewable_kwh: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryProfile {
    pub country: String,
    pub currency: String,
    pub grid_reliability: GridReliabilityProfile,
    pub climate: ClimateProfile,
    pub energy: CountryEnergyProfile,
    pub procurement: ProcurementProfile,
    pub sovereignty: SovereigntyProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridReliabilityProfile {
    pub average_outage_hours_per_month: f64,
    pub voltage_stability: String,
    pub generator_required: bool,
    pub grid_outage_risk: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClimateProfile {
    pub design_dry_bulb_c: f64,
    pub water_stress: String,
    pub dust_filtration_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CountryEnergyProfile {
    pub electricity_price_per_kwh: f64,
    pub diesel_price_per_liter: f64,
    pub solar_capacity_factor: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResilienceProfile {
    pub required_autonomy_hours: f64,
    #[serde(default)]
    pub battery_autonomy_hours: Option<f64>,
    #[serde(default)]
    pub generator_autonomy_hours: Option<f64>,
    #[serde(default)]
    pub grid_outage_risk: Option<String>,
    #[serde(default)]
    pub fallback_generator_required: Option<bool>,
    #[serde(default)]
    pub diesel_price_per_liter: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcurementProfile {
    pub import_duty_percent: f64,
    pub shipping_multiplier: f64,
    pub local_labour_multiplier: f64,
    #[serde(default)]
    pub spare_parts_locality_score: Option<f64>,
    #[serde(default)]
    pub vendor_lock_in_score: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SovereigntyProfile {
    pub data_residency_required: bool,
    pub national_key_management: bool,
    pub offline_backup_required: bool,
    #[serde(default)]
    pub sovereign_control_score: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationsProfile {
    #[serde(default)]
    pub maintainability_score: Option<f64>,
    #[serde(default)]
    pub backup_restore_maturity: Option<String>,
    #[serde(default)]
    pub operator_skill_requirement: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceCatalogueSelection {
    pub profile_id: String,
    pub deployment_stage: String,
    pub bundles: Vec<String>,
    pub services: Vec<String>,
    pub ui_workflows: Vec<String>,
    pub upgrade_policy: ServiceUpgradePolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceUpgradePolicy {
    pub default_update_class: String,
    pub gitops_required: bool,
    pub blind_upgrades_allowed: bool,
    pub required_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub id: String,
    pub title: String,
    pub requester: String,
    pub target_system: String,
    pub target_environment: String,
    pub change_type: ChangeType,
    pub risk: ChangeRisk,
    pub files: Vec<ConfigArtifact>,
    pub validations: Vec<ValidationResult>,
    pub rollout_plan: RolloutPlan,
    pub rollback_plan: RollbackPlan,
    pub audit_events: Vec<AuditEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    ConfigScript,
    ServiceUpgrade,
    InfrastructurePlan,
    AccessPolicy,
    EmergencyPatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeRisk {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigArtifact {
    pub path: String,
    pub owner: String,
    pub language: String,
    pub secret_policy: SecretPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretPolicy {
    NoSecretsAllowed,
    ReferencesOnly,
    EncryptedValuesAllowed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationResult {
    pub check_id: String,
    pub command: String,
    pub status: ValidationStatus,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationStatus {
    Pending,
    Passed,
    Failed,
    Waived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RolloutPlan {
    pub strategy: RolloutStrategy,
    pub stages: Vec<RolloutStage>,
    pub required_approvers: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RolloutStrategy {
    GitOpsPullRequest,
    StagedCanary,
    RackByRack,
    MaintenanceWindow,
    EmergencyFastTrack,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RolloutStage {
    pub name: String,
    pub target: String,
    pub health_checks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RollbackPlan {
    pub trigger_conditions: Vec<String>,
    pub restore_actions: Vec<String>,
    pub evidence_required: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub actor: String,
    pub action: String,
    pub timestamp_utc: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortalChangeStatus {
    Draft,
    Submitted,
    Approved,
    Running,
    Blocked,
    Complete,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalDecision {
    Pending,
    Approved,
    Rejected,
    Waived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub approval_id: String,
    pub change_id: String,
    pub owner: String,
    pub decision: ApprovalDecision,
    #[serde(default)]
    pub decided_by: Option<String>,
    #[serde(default)]
    pub decided_at_utc: Option<String>,
    #[serde(default)]
    pub evidence_ref: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceBundleStatus {
    Pending,
    Passed,
    Failed,
    Waived,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub bundle_id: String,
    #[serde(default)]
    pub change_id: Option<String>,
    pub workflow_id: String,
    pub bundle_path: String,
    pub status: EvidenceBundleStatus,
    pub produced_by: String,
    #[serde(default)]
    pub summary: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfrastructureRequestStatus {
    Draft,
    Submitted,
    Approved,
    Running,
    Blocked,
    Complete,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfrastructureRequest {
    pub request_id: String,
    pub workflow_id: String,
    pub resource_name: String,
    pub owner: String,
    pub environment: String,
    #[serde(default)]
    pub change_id: Option<String>,
    #[serde(default)]
    pub evidence_bundle_id: Option<String>,
    pub status: InfrastructureRequestStatus,
    #[serde(default)]
    pub payload_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdapterProofRun {
    pub run_id: String,
    pub proof_id: String,
    pub milestone_id: String,
    pub adapter_target: String,
    pub mode: String,
    pub status: EvidenceBundleStatus,
    pub evidence_path: String,
    #[serde(default)]
    pub summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedAuditEvent {
    pub event_id: String,
    #[serde(default)]
    pub change_id: Option<String>,
    pub actor: String,
    pub action: String,
    pub timestamp_utc: String,
    #[serde(default)]
    pub evidence_ref: Option<String>,
    #[serde(default)]
    pub payload_summary: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomerAccountStatus {
    Prospect,
    Onboarding,
    Active,
    Pilot,
    Suspended,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerAccount {
    pub customer_id: String,
    pub display_name: String,
    pub customer_type: String,
    pub residency_zone: String,
    pub primary_region: String,
    pub identity_realm: String,
    pub billing_account: String,
    pub support_tier: String,
    pub service_owner: String,
    pub status: CustomerAccountStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomerSiteStatus {
    Planned,
    Onboarding,
    Pilot,
    Active,
    Suspended,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerSiteInstance {
    pub site_id: String,
    pub customer_id: String,
    pub country: String,
    pub city: String,
    pub deployment_stage: String,
    pub it_load_kw: u32,
    pub substrate: String,
    pub provisioner: String,
    pub data_residency_zone: String,
    pub source_of_truth: String,
    pub ops_owner: String,
    pub status: CustomerSiteStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MfaPolicyStatus {
    Template,
    Pilot,
    Enforced,
    Retired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MfaPolicy {
    pub policy_id: String,
    pub scope: String,
    pub provider_stack: String,
    pub factors: Vec<String>,
    pub enrollment_flow: String,
    pub recovery_method: String,
    pub enforcement_point: String,
    pub evidence_path: String,
    pub owner: String,
    pub status: MfaPolicyStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingRecordStatus {
    Draft,
    Review,
    Approved,
    Released,
    Disputed,
    Paid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingPlan {
    pub plan_id: String,
    pub customer_segment: String,
    pub services_included: Vec<String>,
    pub rating_engine: String,
    pub invoice_engine: String,
    pub currency: String,
    pub minimum_commit_usd: u32,
    pub tax_policy: String,
    pub approval_owner: String,
    pub status: BillingRecordStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageMeter {
    pub meter_id: String,
    pub service_domain: String,
    pub source_system: String,
    pub metric_name: String,
    pub unit: String,
    pub collection_cadence: String,
    pub rating_plan: String,
    pub evidence_path: String,
    pub owner: String,
    pub status: BillingRecordStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoicePreview {
    pub invoice_id: String,
    pub customer_id: String,
    pub billing_period: String,
    pub plan_id: String,
    pub usage_summary: Vec<String>,
    pub amount_usd: f64,
    pub credits_usd: f64,
    pub tax_usd: f64,
    pub status: BillingRecordStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CostSummary {
    pub it_energy_kwh: f64,
    pub facility_energy_kwh: f64,
    pub non_it_energy_kwh: f64,
    pub grid_import_kwh: f64,
    pub renewable_fraction: f64,
    pub energy_cost: f64,
    pub carbon_kg: f64,
    pub water_liters: f64,
    pub pue: f64,
    pub wue_liters_per_it_kwh: f64,
    pub cue_kg_per_it_kwh: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CoolingRecoveryProfile {
    pub rack_heat_kw: f64,
    pub capture_fraction: f64,
    pub drive_temp_c: f64,
    pub sink_temp_c: f64,
    pub thermal_cop: f64,
    pub auxiliary_cooling_load_kw: f64,
    pub displaced_electric_chiller_cop: f64,
    pub pump_and_controls_kw: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CoolingRecoverySummary {
    pub captured_heat_kw: f64,
    pub recovered_cooling_kw: f64,
    pub cooling_offset_kw: f64,
    pub unmet_auxiliary_cooling_kw: f64,
    pub surplus_recovered_cooling_kw: f64,
    pub equivalent_compressor_power_avoided_kw: f64,
    pub net_electric_power_savings_kw: f64,
    pub heat_rejection_kw: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RackStandard {
    Eia19,
    Open19,
    OcpOpenRackV3,
    OcpOpenRackWide,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RackProfile {
    pub standard: RackStandard,
    pub height_units: u16,
    pub max_power_kw: f64,
    pub rated_static_load_kg: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelLicenseClass {
    FullyOpenSourceAi,
    PermissiveOpenWeight,
    RestrictedOpenWeight,
    NonCommercial,
    InternalOnly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_extended_site_profile() {
        let profile: SiteProfile =
            serde_json::from_str(include_str!("../../../examples/site-profile.json")).unwrap();

        assert_eq!(profile.site.country, "KE");
        assert!(profile.resilience.is_some());
        assert!(profile.procurement.is_some());
        assert!(profile.sovereignty.is_some());
        assert!(profile.operations.is_some());
    }

    #[test]
    fn deserializes_country_profile_examples() {
        let profiles = [
            include_str!("../../../data/country-profiles/iraq-example.json"),
            include_str!("../../../data/country-profiles/jordan-example.json"),
            include_str!("../../../data/country-profiles/egypt-example.json"),
            include_str!("../../../data/country-profiles/kenya-example.json"),
            include_str!("../../../data/country-profiles/pakistan-example.json"),
        ];

        for raw in profiles {
            let profile: CountryProfile = serde_json::from_str(raw).unwrap();

            assert!(!profile.country.is_empty());
            assert!(profile.procurement.shipping_multiplier >= 1.0);
            assert!((0.0..=1.0).contains(&profile.energy.solar_capacity_factor));
        }
    }

    #[test]
    fn deserializes_service_catalogue_examples() {
        let profiles = [
            include_str!("../../../examples/service-catalogue/50kw-edge-services.json"),
            include_str!("../../../examples/service-catalogue/250kw-regional-pilot-services.json"),
            include_str!("../../../examples/service-catalogue/1mw-sovereign-cloud-services.json"),
            include_str!("../../../examples/service-catalogue/5mw-ai-ready-services.json"),
        ];

        for raw in profiles {
            let profile: ServiceCatalogueSelection = serde_json::from_str(raw).unwrap();

            assert!(!profile.profile_id.is_empty());
            assert!(!profile.bundles.is_empty());
            assert!(profile.services.iter().any(|service| service == "identity"));
            assert!(profile.upgrade_policy.gitops_required);
            assert!(!profile.upgrade_policy.blind_upgrades_allowed);
            assert!(!profile.upgrade_policy.required_gates.is_empty());
        }
    }

    #[test]
    fn serializes_gitops_change_request_model() {
        let request = ChangeRequest {
            id: "cr-edge-waf-0001".to_string(),
            title: "Enable Coraza blocking mode for public API".to_string(),
            requester: "security-admin".to_string(),
            target_system: "edge-shield".to_string(),
            target_environment: "staging".to_string(),
            change_type: ChangeType::ConfigScript,
            risk: ChangeRisk::High,
            files: vec![ConfigArtifact {
                path: "/etc/coraza/osdc-crs.conf".to_string(),
                owner: "root".to_string(),
                language: "modsecurity".to_string(),
                secret_policy: SecretPolicy::NoSecretsAllowed,
            }],
            validations: vec![ValidationResult {
                check_id: "coraza-config".to_string(),
                command: "coraza --validate /etc/coraza/osdc-crs.conf".to_string(),
                status: ValidationStatus::Pending,
                summary: "queued".to_string(),
            }],
            rollout_plan: RolloutPlan {
                strategy: RolloutStrategy::StagedCanary,
                stages: vec![RolloutStage {
                    name: "edge-a staging".to_string(),
                    target: "edge-a".to_string(),
                    health_checks: vec!["waf detection logs clean".to_string()],
                }],
                required_approvers: vec!["security-owner".to_string()],
            },
            rollback_plan: RollbackPlan {
                trigger_conditions: vec!["5xx rate exceeds baseline".to_string()],
                restore_actions: vec!["restore previous Git commit".to_string()],
                evidence_required: vec!["rollback hash recorded".to_string()],
            },
            audit_events: vec![AuditEvent {
                event_id: "audit-0001".to_string(),
                actor: "security-admin".to_string(),
                action: "created".to_string(),
                timestamp_utc: "2026-06-17T00:00:00Z".to_string(),
                evidence_ref: "git:cr-edge-waf-0001".to_string(),
            }],
        };

        let raw = serde_json::to_string(&request).unwrap();
        let decoded: ChangeRequest = serde_json::from_str(&raw).unwrap();

        assert_eq!(decoded.id, request.id);
        assert_eq!(
            decoded.files[0].secret_policy,
            SecretPolicy::NoSecretsAllowed
        );
        assert_eq!(decoded.rollout_plan.strategy, RolloutStrategy::StagedCanary);
        assert_eq!(decoded.validations[0].status, ValidationStatus::Pending);
    }

    #[test]
    fn serializes_portal_persistence_records() {
        let approval = ApprovalRecord {
            approval_id: "approval-0001".to_string(),
            change_id: "cr-0001".to_string(),
            owner: "platform-owner".to_string(),
            decision: ApprovalDecision::Approved,
            decided_by: Some("lead-operator".to_string()),
            decided_at_utc: Some("2026-06-17T10:00:00Z".to_string()),
            evidence_ref: Some("target/assurance/change/cr-0001".to_string()),
            notes: Some("staging checks passed".to_string()),
        };
        let evidence = EvidenceBundle {
            bundle_id: "evidence-0001".to_string(),
            change_id: Some("cr-0001".to_string()),
            workflow_id: "WF_VM_PROVISION".to_string(),
            bundle_path: "target/assurance/change/cr-0001".to_string(),
            status: EvidenceBundleStatus::Passed,
            produced_by: "scripts/assurance-run.sh".to_string(),
            summary: vec![
                "cargo test passed".to_string(),
                "GitOps diff attached".to_string(),
            ],
        };
        let request = InfrastructureRequest {
            request_id: "infra-0001".to_string(),
            workflow_id: "WF_VM_PROVISION".to_string(),
            resource_name: "health-api-staging".to_string(),
            owner: "tenant-health".to_string(),
            environment: "staging".to_string(),
            change_id: Some("cr-0001".to_string()),
            evidence_bundle_id: Some("evidence-0001".to_string()),
            status: InfrastructureRequestStatus::Running,
            payload_summary: vec!["2 vCPU".to_string(), "8 GiB RAM".to_string()],
        };
        let proof_run = AdapterProofRun {
            run_id: "proof-run-0001".to_string(),
            proof_id: "PROOF_POSTGRES_MIGRATION".to_string(),
            milestone_id: "ADAPT_009".to_string(),
            adapter_target: "PostgreSQL".to_string(),
            mode: "plan".to_string(),
            status: EvidenceBundleStatus::Passed,
            evidence_path: "target/assurance/adapter-proofs/latest/postgresql.md".to_string(),
            summary: vec!["six portal-state tables found".to_string()],
        };
        let audit = PersistedAuditEvent {
            event_id: "audit-0001".to_string(),
            change_id: Some("cr-0001".to_string()),
            actor: "lead-operator".to_string(),
            action: "approved".to_string(),
            timestamp_utc: "2026-06-17T10:00:00Z".to_string(),
            evidence_ref: Some("target/assurance/change/cr-0001".to_string()),
            payload_summary: vec!["approval recorded".to_string()],
        };

        let raw = serde_json::to_string(&(
            approval.clone(),
            evidence.clone(),
            request.clone(),
            proof_run.clone(),
            audit.clone(),
        ))
        .unwrap();
        let decoded: (
            ApprovalRecord,
            EvidenceBundle,
            InfrastructureRequest,
            AdapterProofRun,
            PersistedAuditEvent,
        ) = serde_json::from_str(&raw).unwrap();

        assert_eq!(decoded.0, approval);
        assert_eq!(decoded.1.status, EvidenceBundleStatus::Passed);
        assert_eq!(decoded.2.status, InfrastructureRequestStatus::Running);
        assert_eq!(decoded.3.proof_id, "PROOF_POSTGRES_MIGRATION");
        assert_eq!(decoded.4.action, "approved");
    }

    #[test]
    fn serializes_customer_operations_records() {
        let account = CustomerAccount {
            customer_id: "CUST_HEALTH".to_string(),
            display_name: "Ministry of Health".to_string(),
            customer_type: "public-sector".to_string(),
            residency_zone: "national-region-1".to_string(),
            primary_region: "regional-pilot-1".to_string(),
            identity_realm: "health.gov".to_string(),
            billing_account: "BA_HEALTH".to_string(),
            support_tier: "mission-critical".to_string(),
            service_owner: "public-cloud-owner".to_string(),
            status: CustomerAccountStatus::Active,
        };
        let site = CustomerSiteInstance {
            site_id: "SITE_HEALTH_REGIONAL".to_string(),
            customer_id: account.customer_id.clone(),
            country: "Kenya".to_string(),
            city: "Nairobi".to_string(),
            deployment_stage: "250kw-regional-pilot".to_string(),
            it_load_kw: 250,
            substrate: "CloudStack+Kubernetes+Ceph".to_string(),
            provisioner: "MAAS+Metal3".to_string(),
            data_residency_zone: account.residency_zone.clone(),
            source_of_truth: "NetBox".to_string(),
            ops_owner: account.service_owner.clone(),
            status: CustomerSiteStatus::Active,
        };
        let mfa = MfaPolicy {
            policy_id: "MFA_TENANT_ADMIN".to_string(),
            scope: "tenant admins".to_string(),
            provider_stack: "Keycloak+privacyIDEA".to_string(),
            factors: vec!["webauthn".to_string(), "totp".to_string()],
            enrollment_flow: "admin-invitation".to_string(),
            recovery_method: "recovery-codes+break-glass-approval".to_string(),
            enforcement_point: "portal-command-queue".to_string(),
            evidence_path: "docs/security/open-source-mfa.md".to_string(),
            owner: "identity-owner".to_string(),
            status: MfaPolicyStatus::Pilot,
        };
        let plan = BillingPlan {
            plan_id: "BILL_PUBLIC_CRITICAL".to_string(),
            customer_segment: "public-sector".to_string(),
            services_included: vec!["vm".to_string(), "database".to_string()],
            rating_engine: "CloudKitty+OpenMeter".to_string(),
            invoice_engine: "Kill-Bill".to_string(),
            currency: "USD".to_string(),
            minimum_commit_usd: 25_000,
            tax_policy: "local-tax-profile".to_string(),
            approval_owner: "commercial-owner".to_string(),
            status: BillingRecordStatus::Draft,
        };
        let meter = UsageMeter {
            meter_id: "METER_VM_HOURS".to_string(),
            service_domain: "compute".to_string(),
            source_system: "OpenStack+CloudStack".to_string(),
            metric_name: "instance_hours".to_string(),
            unit: "hour".to_string(),
            collection_cadence: "hourly".to_string(),
            rating_plan: plan.plan_id.clone(),
            evidence_path: "docs/commercial/billing-and-metering.md".to_string(),
            owner: "cloud-owner".to_string(),
            status: BillingRecordStatus::Draft,
        };
        let invoice = InvoicePreview {
            invoice_id: "INV_HEALTH_2026_06".to_string(),
            customer_id: account.customer_id.clone(),
            billing_period: "2026-06".to_string(),
            plan_id: plan.plan_id.clone(),
            usage_summary: vec!["18800 vm-hours".to_string()],
            amount_usd: 27_640.0,
            credits_usd: 0.0,
            tax_usd: 4_146.0,
            status: BillingRecordStatus::Draft,
        };

        let raw = serde_json::to_string(&(
            account.clone(),
            site.clone(),
            mfa.clone(),
            plan.clone(),
            meter.clone(),
            invoice.clone(),
        ))
        .unwrap();
        let decoded: (
            CustomerAccount,
            CustomerSiteInstance,
            MfaPolicy,
            BillingPlan,
            UsageMeter,
            InvoicePreview,
        ) = serde_json::from_str(&raw).unwrap();

        assert_eq!(decoded.0, account);
        assert_eq!(decoded.1.status, CustomerSiteStatus::Active);
        assert_eq!(decoded.2.status, MfaPolicyStatus::Pilot);
        assert_eq!(decoded.3.minimum_commit_usd, 25_000);
        assert_eq!(decoded.4.rating_plan, "BILL_PUBLIC_CRITICAL");
        assert_eq!(decoded.5.status, BillingRecordStatus::Draft);
    }
}
