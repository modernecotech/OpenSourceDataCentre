use std::{env, error::Error, time::Duration};

use osdc_adapters::{ArgoCdHttpAdapter, ArgoCdReadAdapter};

fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OSDC_ARGOCD_URL")?;
    let token = env::var("OSDC_ARGOCD_TOKEN")?;

    let adapter = ArgoCdHttpAdapter::new(base_url, token).with_timeout(Duration::from_secs(10));
    let snapshot = adapter.sync_snapshot()?;

    println!("{}", serde_json::to_string_pretty(&snapshot)?);

    Ok(())
}
