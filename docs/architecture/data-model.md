# Data Model

This is the first-pass domain model for Rust crates and API schemas. The model should remain small until real integrations force more detail.

## Site

Represents a datacentre campus, building, room, or pilot container.

- `id`
- `name`
- `country`
- `climate_zone`
- `currency`
- `grid_carbon_kg_per_kwh`
- `electricity_price_per_kwh`
- `water_price_per_m3`
- `latitude`
- `longitude`
- optional `resilience`
- optional `procurement`
- optional `sovereignty`
- optional `operations`

## Country Profile

Represents country-specific planning assumptions used by cost, resilience, and sovereignty calculators. These are planning inputs, not official datasets unless a deployment team replaces placeholders with measured or cited values.

- `country`
- `currency`
- `grid_reliability`
- `climate`
- `energy`
- `procurement`
- `sovereignty`

## Grid Reliability

- `average_outage_hours_per_month`
- `voltage_stability`
- `generator_required`
- `grid_outage_risk`

## Climate

- `design_dry_bulb_c`
- `water_stress`
- `dust_filtration_required`

## Energy Profile

Used by cost and sustainability calculators.

- `it_load_kw`
- `pue`
- `annual_hours`
- `onsite_renewable_kwh`
- `battery_round_trip_efficiency`
- `water_liters_per_facility_kwh`
- `grid_carbon_kg_per_kwh`
- `electricity_price_per_kwh`

## Resilience Profile

Used to model autonomy and fallback-power assumptions for country-specific deployments.

- `required_autonomy_hours`
- `battery_autonomy_hours`
- `generator_autonomy_hours`
- `grid_outage_risk`
- `fallback_generator_required`
- `diesel_price_per_liter`

## Procurement Profile

Used to model landed cost, local maintainability, and vendor dependency.

- `import_duty_percent`
- `shipping_multiplier`
- `local_labour_multiplier`
- `spare_parts_locality_score`
- `vendor_lock_in_score`

## Sovereignty Profile

Used to describe whether a deployment can keep data, keys, backups, and administrative control local.

- `data_residency_required`
- `national_key_management`
- `offline_backup_required`
- `sovereign_control_score`

## Operations Profile

Used to capture local operating maturity.

- `maintainability_score`
- `backup_restore_maturity`
- `operator_skill_requirement`

## Rack

Models flexible racks without assuming one vendor.

- `standard`: `Eia19`, `Open19`, `OcpOpenRackV3`, `OcpOpenRackWide`, or `Custom`
- `height_units`
- `usable_width_mm`
- `depth_mm`
- `rated_static_load_kg`
- `rated_dynamic_load_kg`
- `max_power_kw`
- `cooling_class`: air, rear-door heat exchanger, direct-to-chip liquid, immersion, hybrid

## Cooling Zone

Represents a facility cooling boundary.

- `id`
- `name`
- `target_supply_temp_c`
- `max_return_temp_c`
- `cooling_capacity_kw`
- `redundancy_model`
- `economizer_enabled`
- `ground_loop_enabled`
- `liquid_cooling_enabled`

## Rack Heat Recovery

Represents the cooling concept where rack heat is captured, collated in an underfloor or service-trench thermal spine, then reused or rejected.

- `rack_heat_kw`
- `capture_fraction`
- `drive_temp_c`
- `sink_temp_c`
- `thermal_cop`
- `auxiliary_cooling_load_kw`
- `displaced_electric_chiller_cop`
- `pump_and_controls_kw`

Outputs:

- `captured_heat_kw`
- `recovered_cooling_kw`
- `cooling_offset_kw`
- `unmet_auxiliary_cooling_kw`
- `equivalent_compressor_power_avoided_kw`
- `net_electric_power_savings_kw`
- `heat_rejection_kw`

The model must not imply perpetual cooling. Heat-driven cooling offsets compressor work, but drive heat plus lifted heat still need final rejection.

## Workload

Represents a scheduled compute or AI job.

- `id`
- `tenant`
- `kind`: service, batch, training, inference, simulation
- `cpu_cores`
- `memory_gb`
- `accelerators`
- `storage_gb`
- `expected_runtime_hours`
- `priority`
- `carbon_policy`
- `scheduler_backend`

## GitOps Change Workflow

Represents browser or API changes that must be validated, reviewed, rolled out, and audited before touching production infrastructure.

- `ChangeRequest`
- `ConfigArtifact`
- `ValidationResult`
- `RolloutPlan`
- `RolloutStage`
- `RollbackPlan`
- `AuditEvent`

Core enums:

- `ChangeType`: config script, service upgrade, infrastructure plan, access policy, emergency patch.
- `ChangeRisk`: low, medium, high, critical.
- `ValidationStatus`: pending, passed, failed, waived.
- `RolloutStrategy`: GitOps pull request, staged canary, rack-by-rack, maintenance window, emergency fast track.
- `SecretPolicy`: no secrets allowed, references only, encrypted values allowed.

## Portal Persistence

Represents OSDC-owned workflow state that should be stored in PostgreSQL once the portal moves beyond in-process prototype state. External systems remain authoritative for their own data; the portal persists requests, approvals, evidence, proof runs, and audit records around those systems.

The initial migration is `crates/osdc-portal/migrations/0001_osdc_portal_state.sql`. The architecture note is [Portal Persistence](portal-persistence.md).

- `PortalChangeStatus`: draft, submitted, approved, running, blocked, complete, rejected.
- `ApprovalRecord`: approval ID, change ID, owner, decision, approver, decision time, evidence reference, notes.
- `EvidenceBundle`: evidence bundle ID, change ID, workflow ID, bundle path, status, producer, summary.
- `InfrastructureRequest`: request ID, workflow ID, resource name, owner, environment, change ID, evidence bundle ID, status, payload summary.
- `AdapterProofRun`: run ID, proof ID, milestone ID, adapter target, mode, status, evidence path, summary.
- `PersistedAuditEvent`: event ID, change ID, actor, action, timestamp, evidence reference, payload summary.
- `CustomerAccount`: customer ID, display name, type, residency zone, primary region, identity realm, billing account, support tier, owner, status.
- `CustomerSiteInstance`: site ID, customer ID, country, city, deployment stage, IT load, substrate, provisioner, residency zone, source of truth, owner, status.
- `MfaPolicy`: policy ID, scope, provider stack, factors, enrolment flow, recovery method, enforcement point, evidence path, owner, status.
- `BillingPlan`: plan ID, customer segment, included services, rating engine, invoice engine, currency, minimum commit, tax policy, approval owner, status.
- `UsageMeter`: meter ID, service domain, source system, metric name, unit, cadence, rating plan, evidence path, owner, status.
- `InvoicePreview`: invoice ID, customer ID, billing period, plan ID, usage summary, amount, credits, tax, status.

## Customer Operations

Represents the multi-customer operating layer for countries, universities, public agencies, regional datacentre companies, and telecom operators that run their own sites on OSDC.

The customer model is intentionally split:

- Customer account state belongs to the portal workflow and PostgreSQL.
- Identity users, groups, roles, and sessions remain in Keycloak, privacyIDEA, or authentik.
- Sites, racks, devices, circuits, and IPAM remain in NetBox or the selected source-of-truth tool.
- Live cloud resources remain in Proxmox, CloudStack, OpenStack, Kubernetes, Ceph, and bare-metal lifecycle tools.
- Usage rating and invoice details remain in CloudKitty, OpenMeter, Lago, Kill Bill, or a local finance system.

The portal stores the command, approval, evidence, and summary records that let an operator manage onboarding, MFA, site provisioning, usage metering, billing, invoice preview, and customer support from one browser UI.

## Model Artifact

Represents an AI model or derived model.

- `id`
- `name`
- `family`
- `version`
- `license`
- `classification`: fully open-source AI, open-weight, research-only, non-commercial, restricted
- `parameter_count`
- `quantization`
- `source_url`
- `safety_card_url`
- `approved_for_shared_use`

## Cost Summary

Output model for calculators.

- `it_energy_kwh`
- `facility_energy_kwh`
- `non_it_energy_kwh`
- `grid_import_kwh`
- `renewable_fraction`
- `energy_cost`
- `carbon_kg`
- `water_liters`
- `pue`
- `wue_l_per_it_kwh`
- `cue_kg_per_it_kwh`

## Integration Rule

Any model field that originates in an external system should carry provenance:

- `source_system`
- `source_id`
- `last_seen_at`
- `confidence`
- `manual_override`

This keeps the unified interface honest when NetBox, meters, BMS gateways, and manual planning data disagree.
