# EPMS and BMS Architecture

The Rust portal should summarize and audit facility state, not replace certified local controllers.

System boundaries:

| Layer | Role |
| --- | --- |
| Local controllers | Safety-critical control for power, cooling, fire interfaces, and plant. |
| EPMS | Electrical metering, breaker state, generator, BESS, power quality, and alarms. |
| BMS | Cooling plant, pumps, valves, leak detection, temperature, humidity, and alarms. |
| DCIM | Racks, devices, circuits, cabling, power capacity, and asset state. |
| OSDC portal | Read-only summary by default, change workflow, evidence, correlation, and audit. |

Required design outputs:

- point list;
- alarm philosophy;
- retention policy;
- network segmentation;
- time synchronization;
- read/write permissions;
- manual override rules;
- escalation matrix;
- capacity planning interface.
