// Copyright (C) Copyright Confidential Containers Project Authors.
//
// SPDX-License-Identifier: Apache-2.0
//
use serde::Deserialize;
use std::path::PathBuf;

const DEFAULT_MAX_TRUSTED_AK_KEYS: usize = 100;
const DEFAULT_TRUSTED_AK_KEYS_DIR: &str = "/opt/confidential-containers/trusted_tpm_ak_keys";

#[derive(Deserialize, Debug)]
pub struct TpmVerifierConfig {
    pub trusted_ak_keys_dir: Option<PathBuf>,
    /// Maximum number of trusted AK keys to load. Defaults to 100.
    #[serde(default = "default_max_trusted_ak_keys")]
    pub max_trusted_ak_keys: usize,
}

fn default_max_trusted_ak_keys() -> usize {
    DEFAULT_MAX_TRUSTED_AK_KEYS
}

fn default_trusted_ak_keys_dir() -> PathBuf {
    PathBuf::from(DEFAULT_TRUSTED_AK_KEYS_DIR)
}

impl Default for TpmVerifierConfig {
    fn default() -> Self {
        Self {
            trusted_ak_keys_dir: Some(default_trusted_ak_keys_dir()),
            max_trusted_ak_keys: default_max_trusted_ak_keys(),
        }
    }
}