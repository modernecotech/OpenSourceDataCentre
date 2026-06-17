# Import Duty and Landed Costs

First price is not landed cost. Country profiles should model the full cost of getting a component installed, tested, and maintainable.

## Cost Formula

```text
base_cost = quantity * unit_price
shipping_cost = base_cost * (shipping_multiplier - 1)
import_duty = (base_cost + shipping_cost) * import_duty_percent / 100
local_labour_cost = baseline_labour_cost * local_labour_multiplier
landed_cost = base_cost + shipping_cost + import_duty + local_labour_cost + commissioning_cost
```

## Adders

- Customs broker, inspection, warehousing, and inland freight.
- Installation labour.
- Commissioning labour.
- Spares and tools.
- Training.
- Warranty exclusions.
- Replacement lead time and downtime risk.

## Records

Each quote should include date, country, currency, supplier, warranty, taxes, shipping, import assumptions, lead time, and substitution notes.
