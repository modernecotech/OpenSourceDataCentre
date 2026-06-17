use std::{env, error::Error, time::Duration};

use osdc_adapters::{DnsReadAdapter, PowerDnsHttpAdapter};

fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OSDC_POWERDNS_URL")?;
    let api_key = env::var("OSDC_POWERDNS_API_KEY")?;
    let server_id = env::var("OSDC_POWERDNS_SERVER_ID").unwrap_or_else(|_| "localhost".to_string());
    let zone = env::var("OSDC_POWERDNS_ZONE").ok();

    let adapter = PowerDnsHttpAdapter::new(base_url, api_key, server_id)
        .with_timeout(Duration::from_secs(10));
    let zones = adapter.list_zones()?;
    println!("{}", serde_json::to_string_pretty(&zones)?);

    if let Some(zone) = zone {
        let records = adapter.list_records(&zone)?;
        println!("{}", serde_json::to_string_pretty(&records)?);
    }

    Ok(())
}
