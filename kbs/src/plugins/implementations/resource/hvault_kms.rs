// Copyright (c) 2025
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use super::backend::{ResourceDesc, StorageBackend};
use anyhow::{Context, Result};
use derivative::Derivative;
use log::info;
use serde::Deserialize;
use std::collections::HashMap;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv1;

#[derive(Derivative, Deserialize, Clone, PartialEq)]
#[derivative(Debug)]
pub struct VaultKmsBackendConfig {
    vault_addr: String,
    #[derivative(Debug = "ignore")]
    token_path: Option<String>, // Optional path for token
}

pub struct VaultKmsBackend {
    client: VaultClient,
}

#[async_trait::async_trait]

impl StorageBackend for VaultKmsBackend {
    // ResourceDesc is of the form repository_name/resource_type/resource_tag
    // eg. secret/kbsres1/key1
    // For vault KV it becomes mount/path/secret_name
    // where mount is the mount path of the KV store, path is the path to the secret, and secret_name is the name of the secret.
    // So repository_name == mount
    // resource_type == path
    // resource_tag == secret_name
    // For hashicorp vault the mount path is named "secret"

    async fn read_secret_resource(&self, resource_desc: ResourceDesc) -> Result<Vec<u8>> {
        info!(
            "Use Vault KMS backend. Ignore {}/{}/{}",
            resource_desc.repository_name, resource_desc.resource_type, resource_desc.resource_tag
        );

        // Get secret from Vault KV1 store
        let secret: HashMap<String, String> = kv1::get(
            &self.client,
            &resource_desc.repository_name,
            &resource_desc.resource_type,
        )
        .await
        .context("failed to get resource from Vault KMS")?;

        // Get the secret vault for key resource_desc.resource_tag
        // Extract the specific resource tag value
        let value = secret.get(&resource_desc.resource_tag).ok_or_else(|| {
            anyhow::anyhow!(
                "failed to get resource from Vault KMS, resource tag: {}",
                resource_desc.resource_tag
            )
        })?;

        Ok(value.as_bytes().to_vec())
    }

    async fn write_secret_resource(&self, resource_desc: ResourceDesc, data: &[u8]) -> Result<()> {
        info!(
            "Writing to Vault KMS backend: {}/{}",
            resource_desc.repository_name, resource_desc.resource_type
        );

        // Convert data to a UTF-8 string
        let value = String::from_utf8(data.to_vec()).context("Secret data is not valid UTF-8")?;

        // Prepare the secret map with &str key
        let mut secret = HashMap::new();
        secret.insert(resource_desc.resource_tag.as_str(), value);

        // Write to Vault KV1 store
        kv1::set(
            &self.client,
            &resource_desc.repository_name,
            &resource_desc.resource_type,
            &secret,
        )
        .await
        .context("Failed to write resource to Vault KMS")?;

        Ok(())
    }
}

impl VaultKmsBackend {
    const DEFAULT_TOKEN_PATH: &'static str = "/run/vault/token";

    pub fn new(config: &VaultKmsBackendConfig) -> Result<Self> {
        use std::fs;

        let token_path = config
            .token_path
            .as_deref()
            .unwrap_or(Self::DEFAULT_TOKEN_PATH);

        let token = fs::read_to_string(token_path)
            .with_context(|| format!("reading Vault token from {}", token_path))?
            .trim()
            .to_string();

        let client_settings = VaultClientSettingsBuilder::default()
            .address(&config.vault_addr)
            .token(&token)
            .build()
            .context("building Vault client settings")?;

        let client = VaultClient::new(client_settings).context("creating Vault client")?;

        Ok(Self { client })
    }
}
