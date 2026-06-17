use std::{env, error::Error, time::Duration};

use osdc_adapters::{KeycloakHttpAdapter, KeycloakReadAdapter};

fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OSDC_KEYCLOAK_URL")?;
    let token = env::var("OSDC_KEYCLOAK_TOKEN")?;
    let realm = env::var("OSDC_KEYCLOAK_REALM").unwrap_or_else(|_| "osdc".to_string());

    let adapter = KeycloakHttpAdapter::new(base_url, token, realm.clone())
        .with_timeout(Duration::from_secs(10));
    let snapshot = adapter.identity_snapshot(&realm)?;

    println!("{}", serde_json::to_string_pretty(&snapshot)?);

    Ok(())
}
