# DDoS Realistic Threat Model

OSDC Edge Shield provides local and regional edge protection. It does not provide magical unlimited DDoS absorption.

## What It Can Help With

- Basic L7 rate limiting.
- WAF blocking for common web/API attacks.
- Local abuse blocking using logs and request patterns.
- DNS steering to healthy local nodes.
- Cache absorption for public static content.
- Private origin tunnels that hide origin addresses.
- Emergency bypass and static fallback pages.

## What It Cannot Replace Alone

- Cloudflare-scale global anycast capacity.
- Global volumetric DDoS absorption.
- Commercial bot intelligence from massive global traffic visibility.
- Browser isolation at global scale.
- A global private backbone.
- Managed compliance certifications.
- 24/7 commercial SOC response.

## Required External Coordination

- Upstream ISP filtering.
- IXP coordination.
- Scrubbing provider contracts where risk requires them.
- BGP blackhole and FlowSpec procedures.
- National telecom or regulator incident contacts.
- Public-communications plan for critical service degradation.

## Evidence

- Upstream contact list.
- Emergency route and blackhole procedure.
- Bypass static site test.
- Cache capacity test.
- DDoS exercise report.
