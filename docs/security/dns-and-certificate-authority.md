# DNS and Certificate Authority

DNS and certificate issuance are sovereignty controls. A country should know who can change public service records, issue certificates, revoke certificates, and audit changes.

## DNS Baseline

```text
PowerDNS Authoritative
+ PostgreSQL backend
+ dnsdist in front
+ DNSSEC
+ Rust API adapter
+ tenant/project policy via OPA
```

## Certificate Baseline

- Caddy automatic HTTPS for public routes where ACME is appropriate.
- cert-manager for Kubernetes services.
- Smallstep or internal CA for private services.
- OpenBao or HSM-backed storage for CA-sensitive material where required.

## Rules

- Zone changes must be reviewed and logged.
- DNSSEC status must be visible in the operator console.
- Certificate issuance must be tied to route ownership.
- Private CA roots must have documented custodians and recovery procedures.
- Expiry alerts must be tested.

## Evidence

- Zone inventory.
- DNSSEC check.
- Certificate inventory.
- Issuance log.
- Revocation test.
- Expiry alert test.
