use std::{env, error::Error, time::Duration};

use osdc_adapters::{NetBoxHttpAdapter, NetBoxReadAdapter};

fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OSDC_NETBOX_URL")?;
    let token = env::var("OSDC_NETBOX_TOKEN")?;

    let adapter = NetBoxHttpAdapter::new(base_url, token).with_timeout(Duration::from_secs(10));
    let snapshot = adapter.inventory_snapshot()?;

    println!("{}", serde_json::to_string_pretty(&snapshot)?);

    Ok(())
}
