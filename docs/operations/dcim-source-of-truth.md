# DCIM Source of Truth

DCIM should track the physical reality of the datacentre.

Source systems:

- NetBox or openDCIM for racks, devices, circuits, cables, IPAM, and tenants;
- EPMS for measured power;
- BMS for environmental state;
- inventory system for spares and assets;
- ticketing system for changes.

Rules:

- every rack and circuit has an owner;
- every customer demarcation is documented;
- every cable has endpoints;
- power capacity is based on measured and contracted values;
- manual overrides are explicit and time-limited.
