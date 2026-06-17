# PowerDNS Read Adapter

Last reviewed: 2026-06-17.

`osdc-adapters` now includes the first concrete read-first adapter: `PowerDnsHttpAdapter`.

It is intentionally narrow. It can:

- call the PowerDNS Authoritative HTTP API;
- send the `X-API-Key` header;
- read zones from `/api/v1/servers/{server_id}/zones`;
- read record sets from `/api/v1/servers/{server_id}/zones/{zone}`;
- parse DNSSEC and account metadata where PowerDNS returns it;
- ignore disabled records when returning active record summaries.

It does not yet:

- create or modify zones;
- open GitOps pull requests for DNS changes;
- persist read evidence into PostgreSQL;
- expose live PowerDNS reads through the portal UI;
- manage dnsdist, recursive DNS, certificates, or WAF routing.

## Run Against A Live PowerDNS API

```bash
OSDC_POWERDNS_URL=http://127.0.0.1:8081 \
OSDC_POWERDNS_API_KEY=change-me \
OSDC_POWERDNS_SERVER_ID=localhost \
OSDC_POWERDNS_ZONE=example.gov. \
cargo run -p osdc-adapters --example powerdns-read
```

`OSDC_POWERDNS_ZONE` is optional. Without it, the example prints only zones. With it, the example prints the zone record sets as well.

## Offline Test

The adapter crate has a local fake PowerDNS fixture, so CI does not need a live DNS server:

```bash
cargo test -p osdc-adapters powerdns_http_adapter_reads_zones_and_records
```

## Next Step

The next depth milestone should persist one read evidence object and show it in the infrastructure workbench. That should happen before broadening into NetBox, Keycloak, OpenBao, billing, or cloud write paths.
