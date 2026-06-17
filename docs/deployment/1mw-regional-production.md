# 1 MW Regional Production

The 1 MW regional production pattern turns the pilot into dependable sovereign cloud capacity for national health, education, government apps, research, and regional public services.

## Primary Goals

- Provide a production home for priority public-sector workloads.
- Formalize operations, change control, maintenance, backup, incident response, and security review.
- Expand rack, power, cooling, network, storage, and tenant capacity without changing the platform model.

## Baseline Shape

- Approximately 40 racks at 25 kW average.
- Multiple power and cooling blocks with maintainable isolation boundaries.
- Stronger fire zoning, security zoning, fibre paths, and service corridors.
- Larger BESS and generator strategy based on measured outage and fuel data.
- OpenStack/Ceph/Kubernetes baseline with tenant isolation, quota, chargeback, identity, policy, and audit controls.
- MAAS, Ironic, and Metal3 for disciplined bare-metal lifecycle, with NetBox as source of truth.
- Wazuh, Falco, Suricata, Zeek, OpenBao, Keycloak, OPA, and Edge Shield integrated into the assurance and audit fabric.
- Dedicated staging environment for patch and restore tests.

## Readiness Requirements

- A staffed 24x7 or on-call operations model.
- Documented maintenance windows and change approval.
- Tested disaster-recovery objectives for each service class.
- Updated country profile with measured energy, water, grid, and incident data.
- Local spares kits sized for production mean-time-to-repair targets.
