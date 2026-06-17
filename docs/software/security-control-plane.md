# Security Control Plane

The sovereign cloud service catalogue should make security controls visible as managed services, not hidden implementation details.

## Control Areas

| Control | Open-source stack | UI workflow |
| --- | --- | --- |
| Identity and access | Keycloak, OPA, Kyverno, SPIFFE/SPIRE | Users, groups, roles, MFA, SSO, workload identity. |
| Secrets and keys | OpenBao, Barbican, HSM where available | Secrets, transit encryption, rotation, audit, recovery. |
| Registry and supply chain | Harbor, cosign, Syft, Grype, Trivy | Image signing, SBOM, vulnerability state, promotion gates. |
| Policy as code | OPA, Kyverno, Conftest, Checkov | Misconfiguration checks and admission policy. |
| Runtime security | Falco, Wazuh agent, osquery | Runtime alerts and host inventory. |
| Network detection | Zeek, Suricata | IDS/NDR detections and packet-derived evidence. |
| SOC/SIEM | Wazuh, OpenSearch, Sigma rules, MISP/OpenCTI | Security findings, tenant risk, incident workflow. |
| Threat management and scanners | DefectDojo, Dependency-Track, Trivy, Grype, OSV-Scanner, Kubescape, OpenVAS, OpenSCAP | Wiz-like risk queue, scan coverage, finding ownership, waivers, and remediation workflow. |

## Evidence

Each control should show:

- owner;
- enabled services;
- policy version;
- findings;
- exceptions;
- last test date;
- upgrade state;
- audit trail.

## Related Docs

- [SIEM and SOC Open Source Stack](../security/siem-soc-open-source-stack.md)
- [Supply Chain Security](../security/supply-chain-security.md)
- [Open Threat Management and Scanner Platform](../security/open-threat-management-and-scanner.md)
- [Secrets and Key Management](../security/secrets-and-key-management.md)
- [Zero Trust Access](../security/zero-trust-access.md)
