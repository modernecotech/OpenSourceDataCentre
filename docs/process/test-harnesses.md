# Test Harnesses

The project needs test harnesses for both code and real-world systems. A datacentre reference design is only useful if operators can prove that it behaves as expected.

## Rust Software Tests

- Unit tests for formulas, policy decisions, parsers, and adapters.
- Golden JSON fixtures for calculator inputs/outputs.
- Property tests for bounds such as non-negative energy, rack loads, and queue shares.
- Integration tests against containerized PostgreSQL, mock NetBox, mock Prometheus, and mock scheduler APIs.
- Contract tests for OpenAPI/gRPC endpoints.

## Cost Calculator Tests

- Golden examples for small edge site, regional datacentre, AI training cluster, and islanded solar-plus-storage site.
- Currency and tax handling tests.
- Sensitivity sweeps for electricity price, PUE, WUE, utilization, carbon intensity, and battery efficiency.
- Validation against measured telemetry once deployments exist.

## Facilities Tests

- Commissioning checklists for HVAC, lighting, access control, solar, batteries, metering, UPS, and generator interfaces.
- Sensor calibration records for temperature, humidity, flow, pressure, power, water, and door events.
- Failover tests for cooling pumps, valves, controllers, network links, UPS, and telemetry gateways.
- Earth-cooling tests for loop pressure, thermal response, antifreeze/corrosion chemistry, and seasonal ground recovery.
- Thermal-load tests using staged IT load banks before production deployment.

## Rack and Mechanical Tests

- Static load and tip-risk tests for racks and adapters.
- Cable-management and service-removal tests.
- Airflow obstruction checks.
- Liquid pressure/leak tests for cooling manifolds.
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

## Acceptance Gates

No release should be considered production-ready unless it can answer:

- What changed?
- Which tests were run?
- Which risks remain?
- Which systems are simulated versus measured?
- What rollback path exists?
