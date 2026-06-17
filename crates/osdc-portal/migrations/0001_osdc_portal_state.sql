CREATE SCHEMA IF NOT EXISTS osdc_portal;

CREATE TABLE IF NOT EXISTS osdc_portal.change_requests (
    change_id text PRIMARY KEY,
    title text NOT NULL,
    requester text NOT NULL,
    target_system text NOT NULL,
    target_environment text NOT NULL,
    change_type text NOT NULL,
    risk text NOT NULL,
    status text NOT NULL DEFAULT 'draft',
    files jsonb NOT NULL DEFAULT '[]'::jsonb,
    validations jsonb NOT NULL DEFAULT '[]'::jsonb,
    rollout_plan jsonb NOT NULL DEFAULT '{}'::jsonb,
    rollback_plan jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS osdc_portal.approval_records (
    approval_id text PRIMARY KEY,
    change_id text NOT NULL REFERENCES osdc_portal.change_requests(change_id) ON DELETE CASCADE,
    owner text NOT NULL,
    decision text NOT NULL DEFAULT 'pending',
    decided_by text,
    decided_at timestamptz,
    evidence_ref text,
    notes text,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS osdc_portal.evidence_bundles (
    bundle_id text PRIMARY KEY,
    change_id text REFERENCES osdc_portal.change_requests(change_id) ON DELETE SET NULL,
    workflow_id text NOT NULL,
    bundle_path text NOT NULL,
    status text NOT NULL DEFAULT 'pending',
    produced_by text NOT NULL,
    summary jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS osdc_portal.audit_events (
    event_id text PRIMARY KEY,
    change_id text REFERENCES osdc_portal.change_requests(change_id) ON DELETE SET NULL,
    actor text NOT NULL,
    action text NOT NULL,
    timestamp_utc timestamptz NOT NULL DEFAULT now(),
    evidence_ref text,
    payload jsonb NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS osdc_portal.infrastructure_requests (
    request_id text PRIMARY KEY,
    workflow_id text NOT NULL,
    resource_name text NOT NULL,
    owner text NOT NULL,
    environment text NOT NULL,
    change_id text REFERENCES osdc_portal.change_requests(change_id) ON DELETE SET NULL,
    evidence_bundle_id text REFERENCES osdc_portal.evidence_bundles(bundle_id) ON DELETE SET NULL,
    status text NOT NULL DEFAULT 'draft',
    requested_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    payload jsonb NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS osdc_portal.adapter_proof_runs (
    run_id text PRIMARY KEY,
    proof_id text NOT NULL,
    milestone_id text NOT NULL,
    adapter_target text NOT NULL,
    mode text NOT NULL,
    status text NOT NULL,
    evidence_path text NOT NULL,
    started_at timestamptz NOT NULL DEFAULT now(),
    completed_at timestamptz,
    summary jsonb NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE IF NOT EXISTS osdc_portal.customer_accounts (
    customer_id text PRIMARY KEY,
    display_name text NOT NULL,
    customer_type text NOT NULL,
    residency_zone text NOT NULL,
    primary_region text NOT NULL,
    identity_realm text NOT NULL,
    billing_account text NOT NULL,
    support_tier text NOT NULL,
    service_owner text NOT NULL,
    status text NOT NULL DEFAULT 'onboarding',
    payload jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS osdc_portal.customer_site_instances (
    site_id text PRIMARY KEY,
    customer_id text NOT NULL REFERENCES osdc_portal.customer_accounts(customer_id) ON DELETE CASCADE,
    deployment_stage text NOT NULL,
    it_load_kw numeric NOT NULL,
    substrate text NOT NULL,
    provisioner text NOT NULL,
    data_residency_zone text NOT NULL,
    source_of_truth text NOT NULL,
    ops_owner text NOT NULL,
    status text NOT NULL DEFAULT 'planned',
    payload jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS osdc_portal.identity_mfa_policies (
    policy_id text PRIMARY KEY,
    scope text NOT NULL,
    provider_stack text NOT NULL,
    factors jsonb NOT NULL DEFAULT '[]'::jsonb,
    enrollment_flow text NOT NULL,
    recovery_method text NOT NULL,
    enforcement_point text NOT NULL,
    evidence_path text NOT NULL,
    owner text NOT NULL,
    status text NOT NULL DEFAULT 'template',
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS osdc_portal.usage_meter_snapshots (
    snapshot_id text PRIMARY KEY,
    customer_id text NOT NULL REFERENCES osdc_portal.customer_accounts(customer_id) ON DELETE CASCADE,
    meter_id text NOT NULL,
    billing_period text NOT NULL,
    quantity numeric NOT NULL,
    unit text NOT NULL,
    source_system text NOT NULL,
    status text NOT NULL DEFAULT 'draft',
    evidence_ref text,
    payload jsonb NOT NULL DEFAULT '{}'::jsonb,
    recorded_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS osdc_portal.invoice_previews (
    invoice_id text PRIMARY KEY,
    customer_id text NOT NULL REFERENCES osdc_portal.customer_accounts(customer_id) ON DELETE CASCADE,
    billing_period text NOT NULL,
    plan_id text NOT NULL,
    amount_usd numeric NOT NULL,
    credits_usd numeric NOT NULL DEFAULT 0,
    tax_usd numeric NOT NULL DEFAULT 0,
    status text NOT NULL DEFAULT 'draft',
    approval_ref text,
    payload jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_change_requests_status
    ON osdc_portal.change_requests(status);

CREATE INDEX IF NOT EXISTS idx_change_requests_target
    ON osdc_portal.change_requests(target_system, target_environment);

CREATE INDEX IF NOT EXISTS idx_approval_records_change
    ON osdc_portal.approval_records(change_id);

CREATE INDEX IF NOT EXISTS idx_evidence_bundles_workflow
    ON osdc_portal.evidence_bundles(workflow_id, status);

CREATE INDEX IF NOT EXISTS idx_audit_events_change_time
    ON osdc_portal.audit_events(change_id, timestamp_utc);

CREATE INDEX IF NOT EXISTS idx_infrastructure_requests_workflow_status
    ON osdc_portal.infrastructure_requests(workflow_id, status);

CREATE INDEX IF NOT EXISTS idx_infrastructure_requests_owner
    ON osdc_portal.infrastructure_requests(owner);

CREATE INDEX IF NOT EXISTS idx_adapter_proof_runs_milestone
    ON osdc_portal.adapter_proof_runs(milestone_id, status);

CREATE INDEX IF NOT EXISTS idx_customer_accounts_status
    ON osdc_portal.customer_accounts(status);

CREATE INDEX IF NOT EXISTS idx_customer_site_instances_customer_status
    ON osdc_portal.customer_site_instances(customer_id, status);

CREATE INDEX IF NOT EXISTS idx_identity_mfa_policies_status
    ON osdc_portal.identity_mfa_policies(status);

CREATE INDEX IF NOT EXISTS idx_usage_meter_snapshots_customer_period
    ON osdc_portal.usage_meter_snapshots(customer_id, billing_period);

CREATE INDEX IF NOT EXISTS idx_invoice_previews_customer_status
    ON osdc_portal.invoice_previews(customer_id, status);
