# Data Model

This is the first-pass domain model for Rust crates and API schemas. The model should remain small until real integrations force more detail.

## Site

Represents a datacentre campus, building, room, or pilot container.

- `id`
- `name`
- `country`
- `climate_zone`
- `currency`
- `grid_carbon_kg_per_kwh`
- `electricity_price_per_kwh`
- `water_price_per_m3`
- `latitude`
- `longitude`

## Energy Profile

Used by cost and sustainability calculators.

- `it_load_kw`
- `pue`
- `annual_hours`
- `onsite_renewable_kwh`
- `battery_round_trip_efficiency`
- `water_liters_per_facility_kwh`
- `grid_carbon_kg_per_kwh`
- `electricity_price_per_kwh`

## Rack

Models flexible racks without assuming one vendor.

- `standard`: `Eia19`, `Open19`, `OcpOpenRackV3`, `OcpOpenRackWide`, or `Custom`
- `height_units`
- `usable_width_mm`
- `depth_mm`
- `rated_static_load_kg`
- `rated_dynamic_load_kg`
- `max_power_kw`
- `cooling_class`: air, rear-door heat exchanger, direct-to-chip liquid, immersion, hybrid

## Cooling Zone

Represents a facility cooling boundary.

- `id`
- `name`
- `target_supply_temp_c`
- `max_return_temp_c`
- `cooling_capacity_kw`
- `redundancy_model`
- `economizer_enabled`
- `ground_loop_enabled`
- `liquid_cooling_enabled`

## Workload

Represents a scheduled compute or AI job.

- `id`
- `tenant`
- `kind`: service, batch, training, inference, simulation
- `cpu_cores`
- `memory_gb`
- `accelerators`
- `storage_gb`
- `expected_runtime_hours`
- `priority`
- `carbon_policy`
- `scheduler_backend`

## Model Artifact

Represents an AI model or derived model.

- `id`
- `name`
- `family`
- `version`
- `license`
- `classification`: fully open-source AI, open-weight, research-only, non-commercial, restricted
- `parameter_count`
- `quantization`
- `source_url`
- `safety_card_url`
- `approved_for_shared_use`

## Cost Summary

Output model for calculators.

- `it_energy_kwh`
- `facility_energy_kwh`
- `non_it_energy_kwh`
- `grid_import_kwh`
- `renewable_fraction`
- `energy_cost`
- `carbon_kg`
- `water_liters`
- `pue`
- `wue_l_per_it_kwh`
- `cue_kg_per_it_kwh`

## Integration Rule

Any model field that originates in an external system should carry provenance:

- `source_system`
- `source_id`
- `last_seen_at`
- `confidence`
- `manual_override`

This keeps the unified interface honest when NetBox, meters, BMS gateways, and manual planning data disagree.
