# Cooling-Failover Test

## Purpose

Prove that loss of a pump, fan, sensor, valve, controller, or cooling branch does not create an uncontrolled thermal event under expected load.

## Preconditions

- Thermal load or representative IT load available.
- Temperature and flow telemetry verified.
- Safe abort thresholds defined.
- Manual bypass or rollback plan documented.

## Procedure

1. Record baseline supply and return temperatures, flow, pump state, fan state, rack inlet temperature, and alarms.
2. Disable the approved component or cooling branch.
3. Confirm redundant path starts or operator procedure stabilizes the system.
4. Track temperatures until stable or abort threshold.
5. Restore normal configuration.
6. Record recovery time, alarms, manual steps, and defects.

## Pass Criteria

- No rack inlet temperature exceeds the approved limit.
- Operators can identify the failed component and correct response.
- Telemetry and alarms match the physical event.
