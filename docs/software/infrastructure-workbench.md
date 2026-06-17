# Unified Infrastructure Workbench

Last reviewed: 2026-06-17.

The infrastructure workbench is the front door for day-to-day OSDC infrastructure management. It gives operators and end users one browser surface for creating, changing, testing, upgrading, and scanning infrastructure without hiding the mature open-source systems underneath.

The workbench does not try to turn every upstream option into a bespoke form. It exposes guided workflows for common actions, then links each action to the relevant connector contracts, config/GitOps path, required tests, upgrade gates, automation job, and evidence target.

## Current Prototype

The Rust portal serves the workbench at `/infrastructure` and redirects `/` there. The page is backed by `/api/infrastructure/workbench`, which composes:

- `data/software/infrastructure-workflows.csv` for user-facing workflows.
- `data/software/deployment-stack-profiles.csv` for Proxmox, CloudStack, OpenStack, Ceph, Kubernetes, bare-metal, Edge Shield, and GitOps pairings by scale.
- `data/software/system-ui-connectors.csv` for backend system adapters, auth patterns, write modes, owners, and evidence paths.
- `data/software/test-harness-catalogue.csv` for required functional, security, GitOps, Kubernetes, endpoint, runtime, data, and facility tests.
- `data/software/upgrade-test-gates.csv` for blocking promotion and upgrade gates.
- `data/software/assurance-automation-jobs.csv` for runnable local/GitOps automation hooks.

The UI includes a guided request form, workflow catalogue, selected connector table, required test harness table, blocking gate table, and preview output before a change is opened.

## User Workflow

1. Select a workflow such as creating a tenant, provisioning a VM, provisioning Kubernetes, provisioning storage, requesting GPU capacity, provisioning hardware, exposing an edge endpoint, creating a data product, running an upgrade, or running a security scan.
2. Select a deployment profile such as 50 kW edge, 250 kW regional pilot, 1 MW regional production, or 5 MW national/AI-ready.
3. Choose environment, owner, resource name, and change mode.
4. Review the generated connector path, required tests, blocking gates, automation command, and evidence location.
5. Stage a GitOps pull request or guarded API action.
6. Run required tests and attach evidence before promotion.

## Adapter Direction

Prototype buttons currently stage UI feedback. Production adapters should turn the selected workflow into typed records:

- `ChangeRequest`
- `ValidationResult`
- `RolloutPlan`
- `RollbackPlan`
- `EvidenceBundle`
- `AuditEvent`

Read-only adapters should come first for NetBox, PowerDNS, Keycloak, OpenBao, Proxmox, CloudStack, OpenStack, Kubernetes, Ceph, GitOps, DefectDojo, Dependency-Track, Wazuh, Kubescape, OpenVAS, Redfish/OpenBMC, MAAS, Ironic, Metal3, and Tinkerbell. Write paths should remain GitOps-first or guarded by approvals, validators, rollback checks, and audit.

## Testing Principle

Every workflow row must declare required tests and gates. The UI should never let an operator treat infrastructure creation, hardware commissioning, security exposure, or upgrade promotion as a naked button click. The visible workflow must show what will be tested, which gates are blocking, where evidence lands, and which owner is accountable.
