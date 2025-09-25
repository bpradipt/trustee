# TPM Verifier Configuration

## Configuration File

The TPM verifier reads its configuration from the attestation-service config file. It expects a `verifiers` section as shown below: 

```toml
[attestation_service.verifiers.tpm]
max_trusted_ak_keys = 100
trusted_ak_keys_dir = "./work/trusted_tpm_ak_keys"
```

### Configuration Fields

- `trusted_ak_keys_dir` (optional): Directory containing trusted AK public keys. The public keys must be PEM formated (default: /opt/confidential-containers/trusted_tmp_ak_keys, defined by `DEFAULT_TRUSTED_AK_KEYS_DIR` constant)
- `max_trusted_ak_keys` (optional): Maximum number of trusted AK keys to load (default: 100, defined by `DEFAULT_MAX_TRUSTED_AK_KEYS` constant)

