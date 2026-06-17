# AI Power Ramp Policy

AI workloads can ramp power quickly.

Policy controls:

- per-rack power cap;
- queue admission based on thermal and power headroom;
- staged job start;
- solar surplus preference where policy allows;
- emergency workload shed;
- tenant notification;
- rack PDU telemetry;
- BMS/EPMS correlation.

The scheduler should respect facility constraints rather than treating power as infinite.
