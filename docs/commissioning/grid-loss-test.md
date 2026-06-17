# Grid-Loss Test

## Purpose

Prove the site can handle loss of utility input without unsafe conditions, undocumented alarms, or uncontrolled IT shutdown.

## Preconditions

- Electrical engineer approval.
- Load bank or staged IT load available.
- BESS state of charge within test range.
- Generator or fallback path ready if included in the scenario.
- Operators, safety observer, and rollback plan present.

## Procedure

1. Record baseline power, DC bus voltage, rack bus voltage, BESS state of charge, cooling status, and alarms.
2. Simulate or perform approved utility input loss.
3. Confirm DC bus and 48 V rack or row buses remain within design limits.
4. Confirm telemetry, portal, logs, and alarms record the event.
5. Confirm cooling systems continue in the expected mode.
6. Restore utility input or transfer to fallback source.
7. Record recovery time, alarms, operator actions, and defects.

## Pass Criteria

- No unsafe condition.
- Critical IT load remains powered for the required autonomy window.
- Operators can explain all alarms and actions.
- Event record is complete.
