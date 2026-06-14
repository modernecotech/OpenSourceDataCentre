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

    Ok(())
}

fn read_profile(path: &PathBuf) -> Result<SiteProfile> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read profile {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse profile {}", path.display()))
}
