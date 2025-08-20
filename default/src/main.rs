// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::env;

use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&env::var("AZURE_KEYVAULT_URL")?, credential, None)?;
    let secret = client
        .get_secret("my-secret", "", None)
        .await?
        .into_body()
        .await?;
    println!("{}", secret.value.unwrap_or_default());

    Ok(())
}
