# Billing And Metering

Last reviewed: 2026-06-17.

OSDC billing should be transparent, exportable, and based on open-source systems. The platform should not invent a closed billing engine inside the portal. The portal coordinates plans, approvals, invoice previews, and evidence while metering and billing tools own their specialist records.

## Baseline Stack

| Function | Open-source options | Role |
| --- | --- | --- |
| Cloud rating | CloudKitty | Rated usage from OpenStack-style services. |
| Event metering | OpenMeter | Usage events and metering for Kubernetes, AI jobs, data products, and portal actions. |
| Subscription billing | Kill Bill | Accounts, subscriptions, invoices, payments, credits, and tax integration. |
| Product billing | Lago | Product-led usage billing, plans, and invoice workflows. |
| Cost visibility | OpenCost | Kubernetes cost allocation and capacity showback. |

## Catalogue Sources

- `data/commercial/billing-plans.csv`
- `data/commercial/usage-meters.csv`
- `data/commercial/invoice-preview.csv`

## Billing Flow

1. Assign a billing plan to the customer account.
2. Map each service domain to one or more meters.
3. Collect usage from cloud, storage, Kubernetes, data platform, network, and work-order systems.
4. Rate usage in CloudKitty, OpenMeter, or Lago.
5. Generate an invoice preview in Kill Bill or Lago.
6. Apply SLA credits, tax policy, and manual adjustments.
7. Require finance approval before customer release.
8. Persist the evidence bundle in the portal workflow state.

## Guardrails

- Operators should be able to preview invoices before release.
- Usage data must be exportable as CSV or JSON.
- Every credit or write-off must carry an approval record.
- Customer-visible bills must link to the service catalogue, SLA class, meter source, and evidence bundle.
- The billing system must support local currency and tax policy even when planning catalogues use USD.

