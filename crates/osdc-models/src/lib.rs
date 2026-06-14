use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteProfile {
    pub site: SiteIdentity,
    pub energy: EnergyProfile,
    #[serde(default)]
    pub cooling: Option<CoolingRecoveryProfile>,
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
