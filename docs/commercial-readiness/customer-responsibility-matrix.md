# Customer Responsibility Matrix

Commercial datacentre services need clear responsibility boundaries.

| Area | OSDC/operator responsibility | Customer/tenant responsibility |
| --- | --- | --- |
| Facility shell | Maintain building, security perimeter, fire systems, cooling, power, and access controls. | Stay within contracted area, access rules, and permitted use. |
| Rack/cage | Provide assigned space, locks where contracted, power demarcation, and environmental monitoring. | Install labelled equipment, keep airflow clear, maintain asset records. |
| Power | Provide contracted feed and breaker/PDU demarcation. | Stay within kW commitment and approved redundancy model. |
| Network | Provide meet-me room, patch records, and approved cross-connect workflow. | Manage customer routers, carrier contracts, BGP policy, and port security unless managed service is purchased. |
| Cloud platform | Operate OSDC-managed platform services and evidence. | Maintain tenant workloads, data classification, credentials, and application backups unless managed service is purchased. |
| Access | Operate identity proofing, badge issue, escort, visitor log, and review process. | Approve named users, remove leavers, and follow escort rules. |
| Incident response | Notify, triage, preserve evidence, and coordinate service restoration. | Provide customer technical contact, participate in diagnosis, and approve emergency access where required. |
| Data and keys | Provide residency controls and key-management service where contracted. | Classify data, approve key custodians, and define exit/return requirements. |

This matrix should be attached to onboarding packs, SLAs, and service orders.
