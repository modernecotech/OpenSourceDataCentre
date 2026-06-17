# TIA-942 Gap Analysis

This file tracks gaps against a commercial datacentre infrastructure review frame: site, architecture, electrical, mechanical, telecom, security, and operations.

| Domain | Expected evidence | Current OSDC state | Next file |
| --- | --- | --- | --- |
| Site | Flood, seismic, geotechnical, utility, fibre, road, noise, permitting, and authority review. | Country profiles exist, site due diligence is not yet deep. | `docs/site-selection/site-selection-scorecard.md` |
| Electrical | Utility, generators, UPS/DC bus, earthing, protection, EPO, metering, and maintenance procedures. | DC-first concept exists. | [Electrical single-line](../engineering/electrical-single-line-250kw.md) |
| Mechanical | Cooling topology, redundancy, water treatment, leak detection, controls, and thermal acceptance. | Thermal spine concept exists. | [Cooling P&ID](../engineering/cooling-piping-and-instrumentation.md) |
| Telecom | Meet-me room, cross-connects, carrier onboarding, route diversity, and customer demarcation. | Software networking is strong; commercial interconnection is early. | [Meet-me room design](../network-commercial/meet-me-room-design.md) |
| Security | Perimeter, visitor flow, CCTV retention, access reviews, cage/rack locks, chain of custody. | Basic perimeter/security concepts. | [Customer responsibility matrix](customer-responsibility-matrix.md) |
| Operations | NOC/SOC, MOP/SOP/EOP, ticketing, shift handover, access approvals, drills, and post-incident review. | Operator training direction exists. | [MOP template](../operations/mop-template.md) |
