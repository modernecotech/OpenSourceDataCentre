# 50 kW Edge Micro

The 50 kW edge micro pattern is the first proof point for local operation. It is small enough for a university, hospital group, ministry edge site, ISP, or regional public-service node.

## Primary Goals

- Prove local operators can install, monitor, patch, back up, and restore the platform.
- Host low-risk sovereign services such as identity pilots, DNS, monitoring, small Kubernetes clusters, object storage, and backup targets.
- Validate local power quality, solar contribution, dust filtration, cooling serviceability, and incident response.

## Baseline Shape

- 4 racks, approximately 50 kW IT load.
- Standard 19-inch equipment first, with adapter space for Open19 or OCP patterns later.
- Compact DC-first power path with metering and documented isolation points.
- Small BESS and generator or rental-generator input where grid reliability requires it.
- Air or rear-door heat exchanger cooling depending on climate and density.
- Proxmox VE or Apache CloudStack as the default small-site compute substrate; OpenStack should be avoided unless the operator already has the skills to run it.
- Ceph small cluster, ZFS, or Proxmox Backup for storage and recovery depending on operator maturity.
- NetBox/openDCIM, Keycloak, OPA, PowerDNS, Edge Shield, Prometheus/VictoriaMetrics, Grafana, lightweight Kubernetes, and Rust API adapters.

## Exit Criteria

- Operators complete backup and restore without vendor intervention.
- Grid-loss and generator-start procedures are tested.
- Core dashboards and logs remain accessible during expected failures.
- BOM substitutions, local spares, and maintenance intervals are documented.
- Cost assumptions are updated with local import duty, shipping, labour, fuel, tariff, and water data.
