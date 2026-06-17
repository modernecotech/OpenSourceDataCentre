# OT Security Zones and Conduits

Facility technology must be segmented from tenant and office networks.

Candidate zones:

- BMS controllers;
- EPMS meters and breakers;
- BESS and inverter controllers;
- generator controls;
- cooling plant controllers;
- fire and life-safety interfaces;
- access control and CCTV/NVR;
- DCIM and NetBox/openDCIM;
- operator workstations;
- OSDC portal and telemetry collectors;
- tenant cloud networks.

Conduit rules:

- default deny between OT and IT zones;
- jump host or brokered remote access only;
- MFA and session logging for privileged OT access;
- no direct tenant access to OT systems;
- time synchronization and immutable logs;
- tested backup and restore for controllers and configs;
- patch windows aligned with facility risk;
- emergency break-glass path with audit.

The OSDC portal should read OT state through controlled gateways. It should not directly control safety-critical plant.
