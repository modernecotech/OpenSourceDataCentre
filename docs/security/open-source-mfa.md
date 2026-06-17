# Open-Source MFA And 2FA

Last reviewed: 2026-06-17.

OSDC should require open-source multi-factor authentication for portal users, operators, customer admins, billing admins, GitOps approvers, and break-glass accounts.

The baseline stack is:

- Keycloak for OIDC/SAML realms, groups, roles, policies, and required actions.
- privacyIDEA for token lifecycle, TOTP, WebAuthn/U2F, recovery, and central token administration.
- authentik where a simpler identity-provider and flow-builder model fits the operator better.

## Policy Classes

The policy catalogue is `data/software/identity-mfa-policies.csv`.

| Policy | Scope | Intent |
| --- | --- | --- |
| `MFA_ALL_USERS` | all tenant users | baseline TOTP/WebAuthn enrolment. |
| `MFA_TENANT_ADMIN` | customer administrators | stronger device-bound enrolment and recovery controls. |
| `MFA_OPERATOR_PRIVILEGED` | operators and remote hands | privileged admin, VPN, GitOps, and guarded action access. |
| `MFA_BILLING_ADMIN` | finance users | invoice, credit, and plan-change approval. |

## Operational Requirements

- MFA must be enforced at the identity provider and at privileged portal actions.
- Recovery codes must be generated, protected, and audited.
- Break-glass accounts must be sealed, time-bound, reviewed after use, and excluded from normal daily administration.
- Customer admins should be able to enrol users through the browser portal without receiving direct infrastructure credentials.
- Operator access to BMC, GitOps, billing, and identity administration must require privileged MFA.

## Evidence

Evidence should include:

- realm policy export;
- token enrolment summary;
- privileged-user review;
- recovery-code configuration;
- break-glass use records;
- failed enrolment or bypass attempts;
- approval records for emergency token use.

