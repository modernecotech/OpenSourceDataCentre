# SIEM and SOC Open Source Stack

Sovereign datacentres need security monitoring that can operate locally and preserve data residency.

## Baseline Stack

- Wazuh for host and endpoint security monitoring.
- OpenSearch for search and indexing where Loki is not enough.
- Zeek for network security telemetry.
- Suricata for IDS/IPS signatures.
- MISP for threat-intelligence sharing where policy allows.
- Loki, Tempo, Prometheus, VictoriaMetrics, Grafana, and OpenTelemetry for platform telemetry.

## Modes

| Mode | Description |
| --- | --- |
| Local/private intelligence | No external community-sharing path; use local detections and national feeds. |
| Community intelligence enabled | Share and receive indicators where data-residency and privacy policy allow. |

## Required Alerts

- Privileged login.
- Failed login spikes.
- DNS zone change.
- Certificate issuance or revocation.
- WAF block surge.
- CrowdSec remediation surge.
- Tunnel peer change.
- Secrets access or rotation.
- Backup failure.
- Edge config rollout failure.

## Evidence

- Alert catalogue.
- Retention matrix.
- Incident drill.
- Log export policy.
- Analyst access review.
