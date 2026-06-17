# Costing Data

This directory contains planning-grade cost data for the scale scenarios used by the portal.

- `scenario-costs-2026.csv` defines the four build scales from edge micro to national AI-ready.
- `scenario-category-costs-2026.csv` breaks each scenario into building, DC microgrid, cooling, fire/security, racks/network, and commissioning categories.
- `marketplace-price-basis-2026.csv` records the Alibaba/AliExpress-derived planning ranges and derived installed-cost assumptions.
- `../country-profiles/` contains example country-planning packs for grid reliability, climate, energy, procurement, and sovereignty assumptions.

The scenario ladder is documented in:

- [50 kW edge micro](../../docs/deployment/50kw-edge-micro.md)
- [250 kW regional pilot](../../docs/deployment/250kw-regional-pilot.md)
- [1 MW regional production](../../docs/deployment/1mw-regional-production.md)
- [5 MW national AI-ready](../../docs/deployment/5mw-national-ai-ready.md)

The portal exposes these through:

- `/planner`
- `/api/cost/planning`
- `/api/cost/scenarios`
- `/api/cost/categories`
- `/api/cost/price-basis`

Validation:

```bash
scripts/verify.sh
```
