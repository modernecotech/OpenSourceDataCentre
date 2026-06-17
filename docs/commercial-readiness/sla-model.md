# SLA Model

The SLA model describes what a commercial customer can expect and what is excluded.

Source data:

- [sla-classes.csv](../../data/commercial/sla-classes.csv)

Minimum SLA fields:

- service scope;
- target;
- measurement window;
- customer responsibilities;
- provider responsibilities;
- maintenance exclusions;
- force-majeure exclusions;
- credit method;
- escalation path;
- evidence source.

Initial classes:

| Class | Scope | Evidence |
| --- | --- | --- |
| `SLA_POWER_A` | Contracted A/B rack power. | EPMS records, breaker state, incident record. |
| `SLA_COOLING_A` | Approved operating envelope. | BMS trend logs, sensor calibration, incident record. |
| `SLA_NETWORK_A` | Cross-connect or managed network service. | Interface telemetry, carrier demarcation record, ticket log. |
| `SLA_REMOTE_A` | Remote-hands response. | Ticket timestamps, approval record, closeout evidence. |
| `SLA_CLOUD_A` | Sovereign cloud platform service. | Portal health, service telemetry, incident record. |

This is not a final contract. Local counsel and the operator must convert it into project-specific terms.
