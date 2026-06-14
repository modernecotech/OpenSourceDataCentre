# Cost Calculators

The calculators should help communities decide whether a design is affordable, operable, and sovereign. The goal is not only lowest first cost; it is long-term maintainability without licensing traps.

## Calculator Families

1. Building and site CAPEX
   - Land, civil works, building shell, grounding, drainage, fire zones, security zones, and permits.

2. Mechanical and cooling CAPEX/OPEX
   - Rack heat capture modules, underfloor/service-trench thermal spine, HVAC plant, pumps, chillers, sorption chillers, heat exchangers, water-side economizers, earth loops, Cold UTES, liquid-cooling loops, sensors, maintenance.

3. Electrical CAPEX/OPEX
   - Grid interconnect, switchgear, UPS, PDUs, busways, batteries, solar PV, inverters, metering, spares.

4. Rack and compute CAPEX/OPEX
   - Rack standard, adapters, shelves, cabling, switches, servers, accelerators, storage, spare parts, replacement cycles.

5. Software and operations
   - Open-source support subscriptions where chosen, staffing, training, connectivity, backup media, security reviews, hardware lifecycle.

6. AI workload cost
   - GPU hours, model cache storage, queue wait, energy, cooling overhead, carbon intensity, amortized hardware, and tenant chargeback.

7. Sustainability
   - PUE, WUE, CUE, renewable fraction, grid import, water stress, embodied carbon, heat reuse, and autonomy during outage.

## Baseline Formulas

```text
IT energy kWh = IT load kW * operating hours
Facility energy kWh = IT energy kWh * PUE
Non-IT energy kWh = Facility energy kWh - IT energy kWh
Grid import kWh = max(Facility energy kWh - onsite renewable kWh, 0)
Renewable fraction = min(onsite renewable kWh / Facility energy kWh, 1)
Energy cost = Grid import kWh * electricity price per kWh
Carbon kg = Grid import kWh * grid carbon kg per kWh
Water liters = Facility energy kWh * water liters per facility kWh
WUE = Water liters / IT energy kWh
CUE = Carbon kg / IT energy kWh
Captured rack heat kW = Rack heat kW * capture fraction
Recovered cooling kW = Captured rack heat kW * thermal COP
Avoided compressor power kW = Cooling offset kW / displaced electric chiller COP
Net electric savings kW = Avoided compressor power kW - pump and controls kW
Sorption heat rejection kW ~= Captured rack heat kW + recovered cooling kW
```

## Validation Rules

- PUE must be greater than or equal to 1.0.
- Annual hours must be between 1 and 8760 for a one-year estimate.
- IT load must be non-negative.
- Electricity price, water rate, and carbon intensity must be non-negative.
- Onsite renewable energy must not reduce grid import below zero.
- Heat-driven cooling must include final heat rejection; recovered cooling cannot be counted as eliminating the captured rack heat.
- Thermal COP and displaced electric chiller COP must be stated separately.
- Calculator outputs must state which assumptions are measured, estimated, or defaulted.

## Data Inputs

Prefer open and auditable inputs:

- Utility tariffs and time-of-use periods.
- Local solar irradiance datasets.
- Local water price and water-stress factors.
- Local grid carbon factors.
- Equipment BOMs from FreeCAD/CSV exports.
- Telemetry from meters and monitoring systems.
- Procurement quotes with date, vendor, country, warranty, and currency.

## First Rust Implementation

The initial `osdc-calc` crate implements the baseline annual site calculator. It should grow next into:

- Rack power/cooling estimates.
- Rack thermal-spine heat capture and sorption cooling offset estimates.
- Solar and battery sizing helper.
- AI job cost estimator.
- CAPEX and replacement-cycle model.
- Sensitivity-analysis output as CSV.

The code should remain boring and auditable. Operators should be able to check every formula.
