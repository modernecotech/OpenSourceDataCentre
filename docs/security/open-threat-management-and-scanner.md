# Open Threat Management and Scanner Platform

OSDC should provide an open-source alternative to a commercial CNAPP/Wiz-style control plane. The goal is broad, evidence-driven risk visibility across code, containers, Kubernetes, hosts, networks, identities, secrets, and runtime behavior.

No single open-source project fully replaces a commercial CNAPP. OSDC should compose several mature tools behind one portal workflow.

## Capability Map

Machine-readable catalogues:

- [threat-management-stack.csv](../../data/security/threat-management-stack.csv)
- [scanner-coverage.csv](../../data/security/scanner-coverage.csv)

| Capability | Open-source stack | Portal role |
| --- | --- | --- |
| Application security posture | DefectDojo | Aggregate findings, deduplicate, assign owners, track remediation. |
| SBOM and component risk | Dependency-Track, Syft, CycloneDX | Track component inventory and supply-chain risk by service/version. |
| Image and filesystem scanning | Trivy, Grype, Harbor scanner | Block promotion on critical findings unless a waiver exists. |
| Source and dependency scanning | OSV-Scanner, cargo-audit, Semgrep, gitleaks | Run on PRs and scheduled scans. |
| IaC and policy scanning | Checkov, KICS, Conftest, OPA, Kyverno | Prevent unsafe OpenTofu, Kubernetes, and policy changes. |
| Kubernetes posture | Kubescape, kube-bench, Trivy Kubernetes | Continuously check workloads, RBAC, admission, and cluster posture. |
| Endpoint vulnerability and compliance | Wazuh, OpenSCAP, osquery | Track packages, host state, compliance baselines, and agent health. |
| Network vulnerability and exposure | Greenbone OpenVAS, Nuclei, Naabu | Scan approved zones and feed exposure findings into the risk queue. |
| Runtime threat detection | Falco, Wazuh, Suricata, Zeek | Detect anomalous host, container, Kubernetes, and network behavior. |
| Threat intelligence | MISP, OpenCTI, Sigma rules | Enrich detections and route findings into the SOC workflow. |
| Asset and relationship graph | NetBox, OpenSearch, optional Neo4j | Approximate attack paths by joining assets, ownership, exposure, identity, and findings. |

## Wiz-Style Functions and OSDC Equivalents

| Wiz-style function | OSDC open implementation |
| --- | --- |
| Inventory of workloads and services | NetBox/openDCIM, Kubernetes API, Harbor, OpenSearch, Wazuh inventory. |
| Vulnerability view | DefectDojo, Dependency-Track, Trivy, Grype, OSV-Scanner, Wazuh. |
| Kubernetes posture | Kubescape, kube-bench, Kyverno, OPA, Trivy Kubernetes. |
| Image and dependency risk | Harbor scanner, Syft SBOMs, Dependency-Track, Trivy, Grype. |
| Runtime detection | Falco, Wazuh, Suricata, Zeek. |
| Secret exposure | gitleaks, trufflehog, OpenBao audit logs. |
| Attack path approximation | Join asset graph, exposure scans, identity data, workload labels, and findings in OpenSearch or Neo4j. |
| Remediation workflow | DefectDojo, Forgejo issues, Zammad/GLPI, portal action queue. |

## Scanner Pipeline

```text
PR or scheduled job
  -> source and dependency scan
  -> SBOM generation
  -> image and IaC scan
  -> Kubernetes and host posture scan
  -> runtime and network signals
  -> DefectDojo and Dependency-Track ingestion
  -> Wazuh/OpenSearch correlation
  -> portal risk summary
  -> ticket or accepted-risk record
```

## Risk Rules

- Critical exploitable finding on internet-facing, privileged, or identity-plane asset blocks promotion.
- Critical finding on an internal asset requires owner review, mitigation date, and compensating control.
- No waiver is permanent. Every waiver needs an expiry, owner, and evidence link.
- Runtime detections on production workloads create incident records, not only vulnerability tickets.
- OT and facility scans must respect maintenance windows, segmentation, and passive-first collection.

## What the Portal Should Show

- asset or service;
- owner;
- environment;
- scanner source;
- severity;
- exploitability or exposure;
- internet or tenant reachability;
- affected version or policy;
- remediation action;
- waiver state;
- upgrade ring;
- evidence path;
- ticket or incident ID.

## Initial Integration Path

1. Run `scripts/assurance-run.sh --ring RING_STAGING --strict-security` in CI or on the operator workstation.
2. Generate SBOMs with Syft and ingest them into Dependency-Track.
3. Run Trivy/Grype/OSV-Scanner in CI and upload results to DefectDojo.
4. Run Kubescape and Wazuh/OpenSCAP on staging clusters and nodes.
5. Add OpenVAS/Nuclei network scans for approved non-OT zones.
6. Stream Falco/Wazuh/Suricata/Zeek alerts into Wazuh/OpenSearch.
7. Show the summarized risk queue in the OSDC assurance console.
8. Block upgrades through the assurance gates until findings are fixed, waived, or accepted by the owner.

## Source Notes

- Trivy documentation: https://trivy.dev/docs/latest/
- Wazuh vulnerability detection: https://documentation.wazuh.com/current/user-manual/capabilities/vulnerability-detection/
- DefectDojo documentation: https://docs.defectdojo.com/
- Dependency-Track documentation: https://docs.dependencytrack.org/
- Falco documentation: https://falco.org/
- Kubescape documentation: https://kubescape.io/docs/
- OpenSCAP project: https://www.open-scap.org/
- OpenVAS by Greenbone: https://www.openvas.org/
