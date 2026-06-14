# Costing Data

This directory contains planning-grade cost data for the scale scenarios used by the portal.

- `scenario-costs-2026.csv` defines the four build scales from edge micro to national AI-ready.
- `scenario-category-costs-2026.csv` breaks each scenario into building, DC microgrid, cooling, fire/security, racks/network, and commissioning categories.
- `marketplace-price-basis-2026.csv` records the Alibaba/AliExpress-derived planning ranges and derived installed-cost assumptions.

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
