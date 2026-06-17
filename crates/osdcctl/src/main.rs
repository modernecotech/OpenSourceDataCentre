use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use osdc_calc::{annual_site_summary, cooling_recovery_summary};
use osdc_models::SiteProfile;

#[derive(Debug, Parser)]
#[command(
    name = "osdcctl",
    about = "Open Source Data Centre planning and calculator CLI"
)]
struct Cli {
    /// Path to a site profile JSON file.
    profile: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let profile = read_profile(&cli.profile)?;
    let summary = annual_site_summary(profile.energy)?;

    println!("site: {}", profile.site.name);
    println!("country: {}", profile.site.country);
    println!("currency: {}", profile.site.currency);
    println!("it_energy_kwh: {:.2}", summary.it_energy_kwh);
    println!("facility_energy_kwh: {:.2}", summary.facility_energy_kwh);
    println!("grid_import_kwh: {:.2}", summary.grid_import_kwh);
    println!(
        "renewable_fraction: {:.2}%",
        summary.renewable_fraction * 100.0
    );
    println!("energy_cost: {:.2}", summary.energy_cost);
    println!("carbon_kg: {:.2}", summary.carbon_kg);
    println!("water_liters: {:.2}", summary.water_liters);
    println!("pue: {:.3}", summary.pue);
    println!(
        "wue_liters_per_it_kwh: {:.3}",
        summary.wue_liters_per_it_kwh
    );
    println!("cue_kg_per_it_kwh: {:.3}", summary.cue_kg_per_it_kwh);

    if let Some(cooling) = profile.cooling {
        let cooling = cooling_recovery_summary(cooling)?;

        println!("cooling_captured_heat_kw: {:.2}", cooling.captured_heat_kw);
        println!(
            "cooling_recovered_cooling_kw: {:.2}",
            cooling.recovered_cooling_kw
        );
        println!("cooling_offset_kw: {:.2}", cooling.cooling_offset_kw);
        println!(
            "cooling_unmet_auxiliary_kw: {:.2}",
            cooling.unmet_auxiliary_cooling_kw
        );
        println!(
            "cooling_net_electric_savings_kw: {:.2}",
            cooling.net_electric_power_savings_kw
        );
        println!(
            "cooling_heat_rejection_kw: {:.2}",
            cooling.heat_rejection_kw
        );
    }

    if let Some(resilience) = &profile.resilience {
        println!(
            "required_autonomy_hours: {:.2}",
            resilience.required_autonomy_hours
        );
        print_optional_f64("battery_autonomy_hours", resilience.battery_autonomy_hours);
        print_optional_f64(
            "generator_autonomy_hours",
            resilience.generator_autonomy_hours,
        );
        print_optional_text("grid_outage_risk", resilience.grid_outage_risk.as_deref());
        print_optional_bool(
            "fallback_generator_required",
            resilience.fallback_generator_required,
        );
        print_optional_f64("diesel_price_per_liter", resilience.diesel_price_per_liter);
    }

    if let Some(procurement) = &profile.procurement {
        println!(
            "import_duty_percent: {:.2}",
            procurement.import_duty_percent
        );
        println!(
            "shipping_multiplier: {:.3}",
            procurement.shipping_multiplier
        );
        println!(
            "local_labour_multiplier: {:.3}",
            procurement.local_labour_multiplier
        );
        print_optional_f64(
            "spare_parts_locality_score",
            procurement.spare_parts_locality_score,
        );
        print_optional_f64("vendor_lock_in_score", procurement.vendor_lock_in_score);
    }

    if let Some(sovereignty) = &profile.sovereignty {
        println!(
            "data_residency_required: {}",
            sovereignty.data_residency_required
        );
        println!(
            "national_key_management: {}",
            sovereignty.national_key_management
        );
        println!(
            "offline_backup_required: {}",
            sovereignty.offline_backup_required
        );
        print_optional_f64(
            "sovereign_control_score",
            sovereignty.sovereign_control_score,
        );
    }

    if let Some(operations) = &profile.operations {
        print_optional_f64("maintainability_score", operations.maintainability_score);
        print_optional_text(
            "backup_restore_maturity",
            operations.backup_restore_maturity.as_deref(),
        );
        print_optional_text(
            "operator_skill_requirement",
            operations.operator_skill_requirement.as_deref(),
        );
    }

    Ok(())
}

fn read_profile(path: &PathBuf) -> Result<SiteProfile> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read profile {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse profile {}", path.display()))
}

fn print_optional_f64(label: &str, value: Option<f64>) {
    if let Some(value) = value {
        println!("{label}: {value:.2}");
    }
}

fn print_optional_bool(label: &str, value: Option<bool>) {
    if let Some(value) = value {
        println!("{label}: {value}");
    }
}

fn print_optional_text(label: &str, value: Option<&str>) {
    if let Some(value) = value {
        println!("{label}: {value}");
    }
}
