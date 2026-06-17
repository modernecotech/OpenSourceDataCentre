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
}
