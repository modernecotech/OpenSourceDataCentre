# Uptime/Tier Gap Analysis

OSDC may describe Tier-like design intent, but must not claim Uptime certification unless formally audited.

Review areas:

| Area | OSDC current position | Evidence needed |
| --- | --- | --- |
| Power topology | DC-first microgrid concept with fallback generator boundary. | Electrical single-line diagrams, protection study, maintenance bypass method, fault-current model. |
| Cooling topology | Rack thermal spine, rear-door capture, dry cooler, backup cooling concepts. | Cooling P&ID, sequence of operations, failure-mode tests, thermal ride-through model. |
| Concurrent maintainability | N+1 critical pumps and controls are described for pilots. | MOPs proving components can be maintained without tenant outage. |
| Fault tolerance | Not claimed. | Fault-mode analysis and external review before any higher resilience claim. |
| Operations | Operator training and commissioning direction exists. | MOP/SOP/EOP pack, staffing model, incident command, evidence retention. |

Boundary statement:

The reference design is a planning baseline. A project-specific engineer of record must define the actual certification boundary, accepted topology, drawings, calculations, commissioning scripts, and operational controls.
