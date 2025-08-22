// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use azure_core::http::{ClientOptions, TransportOptions};
use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
use setup::setup;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;

    let client = reqwest::ClientBuilder::new()
        .gzip(false)
        .deflate(false)
        .build()?;
    let options = ClientOptions {
        transport: Some(TransportOptions::new(Arc::new(client))),
        ..Default::default()
    };

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(
        &env::var("AZURE_KEYVAULT_URL")?,
        credential,
        Some(SecretClientOptions {
            client_options: options.clone(),
            ..Default::default()
        }),
    )?;
    let secret = client
        .get_secret("my-secret", "", None)
        .await?
        .into_body()
        .await?;
    println!("{}", secret.value.unwrap_or_default());

    Ok(())
}
