use std::{env, error::Error, time::Duration};

use osdc_adapters::{OpenBaoHttpAdapter, OpenBaoReadAdapter};

fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OSDC_OPENBAO_ADDR")?;
    let token = env::var("OSDC_OPENBAO_TOKEN")?;

    let adapter = OpenBaoHttpAdapter::new(base_url, token).with_timeout(Duration::from_secs(10));
    let snapshot = adapter.secret_snapshot()?;

    println!("{}", serde_json::to_string_pretty(&snapshot)?);

    Ok(())
}
