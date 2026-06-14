use osdc_models::{CoolingRecoveryProfile, CoolingRecoverySummary, CostSummary, EnergyProfile};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CalculatorError {
    #[error("IT load must be non-negative")]
    NegativeItLoad,
    #[error("PUE must be at least 1.0")]
    InvalidPue,
    #[error("annual hours must be in the range 1..=8760")]
    InvalidAnnualHours,
    #[error("{field} must be non-negative")]
    NegativeInput { field: &'static str },
    #[error("{field} must be in the range 0.0..=1.0")]
    InvalidFraction { field: &'static str },
    #[error("{field} must be greater than zero")]
    NonPositiveInput { field: &'static str },
    #[error("heat-driven cooling requires drive temperature above sink temperature")]
    InvalidCoolingTemperatures,
}

pub fn annual_site_summary(input: EnergyProfile) -> Result<CostSummary, CalculatorError> {
    validate_energy_profile(input)?;

    let it_energy_kwh = input.it_load_kw * input.annual_hours;
    let facility_energy_kwh = it_energy_kwh * input.pue;
    let non_it_energy_kwh = facility_energy_kwh - it_energy_kwh;
    let grid_import_kwh = (facility_energy_kwh - input.onsite_renewable_kwh).max(0.0);
    let renewable_fraction = if facility_energy_kwh == 0.0 {
        0.0
    } else {
        (input.onsite_renewable_kwh / facility_energy_kwh).clamp(0.0, 1.0)
    };
    let energy_cost = grid_import_kwh * input.electricity_price_per_kwh;
    let carbon_kg = grid_import_kwh * input.grid_carbon_kg_per_kwh;
    let water_liters = facility_energy_kwh * input.water_liters_per_facility_kwh;
    let wue_liters_per_it_kwh = ratio_or_zero(water_liters, it_energy_kwh);
    let cue_kg_per_it_kwh = ratio_or_zero(carbon_kg, it_energy_kwh);

    Ok(CostSummary {
        it_energy_kwh,
        facility_energy_kwh,
        non_it_energy_kwh,
        grid_import_kwh,
        renewable_fraction,
        energy_cost,
        carbon_kg,
        water_liters,
        pue: input.pue,
        wue_liters_per_it_kwh,
        cue_kg_per_it_kwh,
    })
}

pub fn cooling_recovery_summary(
    input: CoolingRecoveryProfile,
) -> Result<CoolingRecoverySummary, CalculatorError> {
    validate_cooling_recovery_profile(input)?;

    let captured_heat_kw = input.rack_heat_kw * input.capture_fraction;
    let recovered_cooling_kw = captured_heat_kw * input.thermal_cop;
    let cooling_offset_kw = recovered_cooling_kw.min(input.auxiliary_cooling_load_kw);
    let unmet_auxiliary_cooling_kw =
        (input.auxiliary_cooling_load_kw - recovered_cooling_kw).max(0.0);
    let surplus_recovered_cooling_kw =
        (recovered_cooling_kw - input.auxiliary_cooling_load_kw).max(0.0);
    let equivalent_compressor_power_avoided_kw =
        cooling_offset_kw / input.displaced_electric_chiller_cop;
    let net_electric_power_savings_kw =
        equivalent_compressor_power_avoided_kw - input.pump_and_controls_kw;

    // Heat-driven chillers still reject the drive heat plus the heat lifted from the chilled loop.
    let heat_rejection_kw = captured_heat_kw + recovered_cooling_kw;

    Ok(CoolingRecoverySummary {
        captured_heat_kw,
        recovered_cooling_kw,
        cooling_offset_kw,
        unmet_auxiliary_cooling_kw,
        surplus_recovered_cooling_kw,
        equivalent_compressor_power_avoided_kw,
        net_electric_power_savings_kw,
        heat_rejection_kw,
    })
}

fn validate_energy_profile(input: EnergyProfile) -> Result<(), CalculatorError> {
    if input.it_load_kw < 0.0 {
        return Err(CalculatorError::NegativeItLoad);
    }

    if input.pue < 1.0 {
        return Err(CalculatorError::InvalidPue);
    }

    if !(1.0..=8760.0).contains(&input.annual_hours) {
        return Err(CalculatorError::InvalidAnnualHours);
    }

    non_negative("electricity_price_per_kwh", input.electricity_price_per_kwh)?;
    non_negative("grid_carbon_kg_per_kwh", input.grid_carbon_kg_per_kwh)?;
    non_negative(
        "water_liters_per_facility_kwh",
        input.water_liters_per_facility_kwh,
    )?;
    non_negative("onsite_renewable_kwh", input.onsite_renewable_kwh)?;

    Ok(())
}

fn validate_cooling_recovery_profile(input: CoolingRecoveryProfile) -> Result<(), CalculatorError> {
    non_negative("rack_heat_kw", input.rack_heat_kw)?;
    fraction("capture_fraction", input.capture_fraction)?;
    non_negative("thermal_cop", input.thermal_cop)?;
    non_negative("auxiliary_cooling_load_kw", input.auxiliary_cooling_load_kw)?;
    positive(
        "displaced_electric_chiller_cop",
        input.displaced_electric_chiller_cop,
    )?;
    non_negative("pump_and_controls_kw", input.pump_and_controls_kw)?;

    if input.drive_temp_c <= input.sink_temp_c {
        return Err(CalculatorError::InvalidCoolingTemperatures);
    }

    Ok(())
}

fn non_negative(field: &'static str, value: f64) -> Result<(), CalculatorError> {
    if value < 0.0 {
        Err(CalculatorError::NegativeInput { field })
    } else {
        Ok(())
    }
}

fn positive(field: &'static str, value: f64) -> Result<(), CalculatorError> {
    if value <= 0.0 {
        Err(CalculatorError::NonPositiveInput { field })
    } else {
        Ok(())
    }
}

fn fraction(field: &'static str, value: f64) -> Result<(), CalculatorError> {
    if !(0.0..=1.0).contains(&value) {
        Err(CalculatorError::InvalidFraction { field })
    } else {
        Ok(())
    }
}

fn ratio_or_zero(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_profile() -> EnergyProfile {
        EnergyProfile {
            it_load_kw: 250.0,
            pue: 1.25,
            annual_hours: 8760.0,
            electricity_price_per_kwh: 0.14,
            grid_carbon_kg_per_kwh: 0.38,
            water_liters_per_facility_kwh: 0.25,
            onsite_renewable_kwh: 650_000.0,
        }
    }

    fn sample_cooling_profile() -> CoolingRecoveryProfile {
        CoolingRecoveryProfile {
            rack_heat_kw: 250.0,
            capture_fraction: 0.85,
            drive_temp_c: 68.0,
            sink_temp_c: 32.0,
            thermal_cop: 0.45,
            auxiliary_cooling_load_kw: 80.0,
            displaced_electric_chiller_cop: 4.0,
            pump_and_controls_kw: 4.0,
        }
    }

    #[test]
    fn calculates_annual_site_summary() {
        let summary = annual_site_summary(sample_profile()).unwrap();

        assert_close(summary.it_energy_kwh, 2_190_000.0);
        assert_close(summary.facility_energy_kwh, 2_737_500.0);
        assert_close(summary.non_it_energy_kwh, 547_500.0);
        assert_close(summary.grid_import_kwh, 2_087_500.0);
        assert_close(summary.energy_cost, 292_250.0);
        assert_close(summary.carbon_kg, 793_250.0);
        assert_close(summary.water_liters, 684_375.0);
        assert_close(summary.renewable_fraction, 0.2374429223744292);
        assert_close(summary.wue_liters_per_it_kwh, 0.3125);
        assert_close(summary.cue_kg_per_it_kwh, 0.3622146118721461);
    }

    #[test]
    fn rejects_pue_below_one() {
        let mut profile = sample_profile();
        profile.pue = 0.99;

        assert_eq!(
            annual_site_summary(profile).unwrap_err(),
            CalculatorError::InvalidPue
        );
    }

    #[test]
    fn onsite_renewables_do_not_create_negative_grid_import() {
        let mut profile = sample_profile();
        profile.onsite_renewable_kwh = 9_999_999.0;

        let summary = annual_site_summary(profile).unwrap();

        assert_eq!(summary.grid_import_kwh, 0.0);
        assert_eq!(summary.carbon_kg, 0.0);
        assert_eq!(summary.renewable_fraction, 1.0);
    }

    #[test]
    fn calculates_cooling_recovery_summary() {
        let summary = cooling_recovery_summary(sample_cooling_profile()).unwrap();

        assert_close(summary.captured_heat_kw, 212.5);
        assert_close(summary.recovered_cooling_kw, 95.625);
        assert_close(summary.cooling_offset_kw, 80.0);
        assert_close(summary.unmet_auxiliary_cooling_kw, 0.0);
        assert_close(summary.surplus_recovered_cooling_kw, 15.625);
        assert_close(summary.equivalent_compressor_power_avoided_kw, 20.0);
        assert_close(summary.net_electric_power_savings_kw, 16.0);
        assert_close(summary.heat_rejection_kw, 308.125);
    }

    #[test]
    fn rejects_invalid_cooling_temperatures() {
        let mut profile = sample_cooling_profile();
        profile.drive_temp_c = profile.sink_temp_c;

        assert_eq!(
            cooling_recovery_summary(profile).unwrap_err(),
            CalculatorError::InvalidCoolingTemperatures
        );
    }

    #[test]
    fn rejects_capture_fraction_above_one() {
        let mut profile = sample_cooling_profile();
        profile.capture_fraction = 1.01;

        assert_eq!(
            cooling_recovery_summary(profile).unwrap_err(),
            CalculatorError::InvalidFraction {
                field: "capture_fraction"
            }
        );
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.000_001,
            "actual {actual} != expected {expected}"
        );
    }
}
