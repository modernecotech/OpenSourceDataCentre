# Test Harnesses

The project needs test harnesses for both code and real-world systems. A datacentre reference design is only useful if operators can prove that it behaves as expected.

## Rust Software Tests

- Unit tests for formulas, policy decisions, parsers, and adapters.
- Golden JSON fixtures for calculator inputs/outputs.
- Property tests for bounds such as non-negative energy, rack loads, and queue shares.
- Integration tests against containerized PostgreSQL, mock NetBox, mock Prometheus, and mock scheduler APIs.
- Contract tests for OpenAPI/gRPC endpoints.

## Repository Verification

`scripts/verify.sh` is the first repository-wide gate. It currently checks:

- Rust formatting and tests.
- JSON fixture parsing.
- CSV rectangularity and primary-ID uniqueness for key catalogues.
- service-catalogue maturity values and service example cross-references.
- commercial-readiness gap priorities, statuses, primary IDs, and evidence-file references.
- site-selection, physical-security, sustainability, and AI-ready catalogue IDs and evidence references.
- engineering evidence and operations procedure catalogue IDs, statuses, priorities, and document references.
- delivery and commissioning catalogue IDs, statuses, criticalities, and evidence/document references.
- developer-platform catalogue IDs, template IDs, environment IDs, promotion-gate IDs, and VS Code workflow IDs.
- data-platform service, product, pipeline, ontology, access-policy, and template IDs.
- system UI connector IDs plus hardware provisioning pipeline, profile, and request IDs.
- local Markdown link validity.
- portal route documentation against the Rust route table.
- config-script catalogue risk, validation command, and sample artifact presence.
- licence metadata.

`scripts/verify.sh --security` additionally requires `cargo-audit` and `syft`, then runs a dependency vulnerability audit and writes an SPDX SBOM to `target/osdc-sbom.spdx.json`.

GitHub Actions should run the default verifier on every push and pull request.

For a broader operator evidence bundle, use [Assurance Automation Runbook](assurance-automation-runbook.md). The first runnable commands are:

```bash
scripts/assurance-run.sh --ring RING_DEV
scripts/upgrade-with-assurance.sh --ring RING_STAGING --service osdc-platform --change-ref PR-123
```

The broader assurance catalogue lives in:

- [assurance-automation-jobs.csv](../../data/software/assurance-automation-jobs.csv)
- [test-harness-catalogue.csv](../../data/software/test-harness-catalogue.csv)
- [upgrade-rings.csv](../../data/software/upgrade-rings.csv)
- [upgrade-test-gates.csv](../../data/software/upgrade-test-gates.csv)
- [scanner-coverage.csv](../../data/security/scanner-coverage.csv)

The control model is described in [Assurance Test and Upgrade Fabric](../software/assurance-test-and-upgrade-fabric.md).

## Cost Calculator Tests

- Golden examples for small edge site, regional datacentre, AI training cluster, and islanded solar-plus-storage site.
- Currency and tax handling tests.
- Sensitivity sweeps for electricity price, PUE, WUE, utilization, carbon intensity, and battery efficiency.
- Validation against measured telemetry once deployments exist.

## Facilities Tests

- Commissioning checklists for HVAC, lighting, access control, solar, sodium-ion batteries, metering, DC converters, DC protection, and generator boundary interfaces.
- Sensor calibration records for temperature, humidity, flow, pressure, power, water, and door events.
- Failover tests for cooling pumps, valves, controllers, network links, 380-400 VDC backbone, 48 VDC rack bus, and telemetry gateways.
- Solar sodium-ion DC microgrid tests for grid loss, no-break rack-bus ride-through, fallback generator start through boundary rectifier, black start, load shedding, battery-low protection, polarity, insulation resistance, isolation monitoring, ripple, and DC breaker coordination.
- Earth-cooling tests for loop pressure, thermal response, antifreeze/corrosion chemistry, and seasonal ground recovery.
- Thermal-load tests using staged IT load banks before production deployment.
- Rack thermal-spine tests for capture fraction, hot-loop temperature, two-phase stability where used, sorption chiller COP, bypass behavior, and final heat-rejection capacity.

The required commissioning pack is in `docs/commissioning/`:

- [Commissioning overview](../commissioning/commissioning-overview.md)
- [Grid-loss test](../commissioning/grid-loss-test.md)
- [DC-bus ride-through test](../commissioning/dc-bus-ride-through-test.md)
- [Cooling-failover test](../commissioning/cooling-failover-test.md)
- [Generator-start test](../commissioning/generator-start-test.md)
- [Backup-restore test](../commissioning/backup-restore-test.md)

## Rack and Mechanical Tests

- Static load and tip-risk tests for racks and adapters.
- Cable-management and service-removal tests.
- Airflow obstruction checks.
- Liquid pressure/leak tests for cooling manifolds.
- Thermal-spine module tests for isolation-valve access, leak-channel drainage, sensor replacement, insulation integrity, and safe walking/floor loading.
- Seismic or transport tests where relevant.

## AI and Queue Tests

- Scheduler fairness tests for Kueue and Slurm profiles.
- GPU isolation and device-plugin tests.
- Model-serving load tests for vLLM/SGLang.
- Tenant data-retention tests.
- Safety and license gate tests for model onboarding.
- Carbon-aware scheduling tests that defer low-priority jobs when policy allows.

## Security Tests

- Identity and role tests through Keycloak.
- OPA policy tests for admin/operator/tenant actions.
- Network-policy tests for OT/IT segmentation.
- Break-glass access tests with auditable logs.
- Dependency and container image scanning.
- Source, dependency, container image, IaC, Kubernetes, endpoint, network, runtime, compliance, and OT scan coverage through the open threat-management pipeline.
- Connector contract tests for NetBox, Redfish/OpenBMC, MAAS, Ironic, Metal3, Tinkerbell, DefectDojo, Dependency-Track, Wazuh, and other UI-backed systems.

The open-source threat-management model is described in [Open Threat Management and Scanner Platform](../security/open-threat-management-and-scanner.md).

## Acceptance Gates

No release should be considered production-ready unless it can answer:

- What changed?
- Which tests were run?
- Which risks remain?
- Which systems are simulated versus measured?
- What rollback path exists?
- Which scanner findings remain open or waived?
- Which upgrade ring is approved next?
