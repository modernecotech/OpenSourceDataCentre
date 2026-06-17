# Sequence of Operations

The sequence of operations describes how facility systems behave in normal, maintenance, degraded, and emergency states.

Required sequences:

- normal grid-connected operation;
- solar/BESS support and battery state-of-charge limits;
- grid loss and DC-bus ride-through;
- fallback generator start and transfer through boundary rectifier;
- cooling pump failover;
- high return-temperature alarm;
- leak detection response;
- fire alarm response;
- emergency power-off;
- controlled tenant workload shed;
- recovery after outage or emergency stop.

Each sequence should define:

- trigger;
- controller of record;
- setpoints;
- alarm levels;
- operator action;
- automatic action;
- inhibited actions;
- rollback or recovery path;
- evidence record.
