# BOM Data

This directory contains planning BOM data for Open Source Data Centre.

- `component-catalog.csv` maps state-of-the-art datacentre systems to simplified, vendor-neutral choices.
- `bom-250kw-open-regional.csv` is a starter bill of materials for a 250 kW regional pilot built around rack thermal spine cooling.

The BOM files use pricing placeholders. Local teams should fill in unit costs, shipping, import duty, local labour, and lead time before using them for procurement.

Every critical BOM line should eventually carry:

- a documented function;
- at least one second-source option;
- local spare-part classification;
- maintenance interval;
- failure symptoms;
- safe replacement procedure;
- commissioning or post-replacement test.

Procurement guidance lives in `docs/procurement/`.
