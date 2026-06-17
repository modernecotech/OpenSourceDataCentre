# DC-Bus Ride-Through Test

## Purpose

Prove that the DC-first power system provides no-break ride-through at the rack or row bus during input disturbances.

## Measurements

- 380-400 VDC facility bus voltage.
- 48 VDC rack or row bus voltage.
- Converter state.
- BESS current and state of charge.
- Rack load.
- Alarms and telemetry timestamps.

## Procedure

1. Establish stable staged load.
2. Record baseline bus values.
3. Introduce the approved input disturbance or transfer scenario.
4. Confirm rack or row bus voltage stays inside design limits.
5. Confirm converters, protection, telemetry, and alarms behave as expected.
6. Return to normal input and record recovery.

## Pass Criteria

- No rack load interruption.
- No unexplained protection trip.
- Telemetry proves the ride-through event and recovery.
