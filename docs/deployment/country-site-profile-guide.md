# Country Site Profile Guide

Country profiles turn the repository from a generic design into a planning tool for ministries, universities, operators, investors, and development partners.

Example profiles live in `data/country-profiles/`. They are planning examples, not official national statistics.

## Profile Shape

```json
{
  "country": "Example",
  "currency": "USD",
  "grid_reliability": {
    "average_outage_hours_per_month": 0,
    "voltage_stability": "unknown",
    "generator_required": true,
    "grid_outage_risk": "unknown"
  },
  "climate": {
    "design_dry_bulb_c": 45,
    "water_stress": "high",
    "dust_filtration_required": true
  },
  "energy": {
    "electricity_price_per_kwh": 0.12,
    "diesel_price_per_liter": 1.0,
    "solar_capacity_factor": 0.2
  },
  "procurement": {
    "import_duty_percent": 15,
    "shipping_multiplier": 1.25,
    "local_labour_multiplier": 0.55
  },
  "sovereignty": {
    "data_residency_required": true,
    "national_key_management": true,
    "offline_backup_required": true
  }
}
```

## Planning Metrics

The Rust model can carry optional planning fields for:

- autonomy hours;
- grid outage risk;
- diesel and fallback cost;
- import duty;
- local labour factor;
- spare-parts locality score;
- vendor lock-in score;
- maintainability score;
- sovereign-control score;
- backup and restore maturity;
- operator skill requirement.

## Use in Costing

Country-profile values should feed landed-cost and operating-cost calculators:

```text
landed_cost = base_cost * shipping_multiplier
landed_cost_with_duty = landed_cost * (1 + import_duty_percent / 100)
local_labour_cost = baseline_labour_cost * local_labour_multiplier
fallback_fuel_cost = generator_liters_per_hour * outage_hours * diesel_price_per_liter
```

Each value should be labeled as measured, quoted, estimated, or placeholder before being used in procurement decisions.
