# KBS Client Tool

This is a simple client for the KBS that facilitates testing of the KBS
and other basic attestation flows.

You can run this tool inside of a TEE to make a request with real attestation evidence.
You can also provide pre-existing evidence or use the sample attester as a fallback.

The client tool can also be used to provision the KBS/AS with resources and policies.

For more sophisticated attestation clients, please refer to [guest components](https://github.com/confidential-containers/guest-components)

For help:

```shell
kbs-client -h
```

We have a community version of kbs-client on [Github ORAS](https://github.com/confidential-containers/trustee/pkgs/container/staged-images%2Fkbs-client).

## Building and installing the client

Build the client binary with support to the default features as:

```shell
make -C ../../kbs cli
```

By default the client is built with support to the sample attester, apart from the
TEE specific ones. If you want to build it with that sample attester only (this will
require fewer dependencies and so usually handy for CI) then you can pass the
`sample_only` feature as:

```shell
make -C ../../kbs cli CLI_FEATURES=sample_only
```

Find the built binary at `../../target/release/kbs-client`. You can get it
installed into the system as:
```shell
sudo make -C ../../kbs install-cli
```

## Examples

All these commands assume you are in the project root (`trustee`) folder.

Get a resource from the KBS (after attesting)

```shell
kbs-client --url http://127.0.0.1:8080 get-resource --path my_repo/resource_type/123abc
```

Add a resource to the KBS

```shell
./target/release/kbs-client --url http://127.0.0.1:8080 config --auth-private-key ./kbs/config/private.key  set-resource --path my_repo/resource_type/123abc --resource-file test_resource
```

Set a resource policy
```shell
./target/release/kbs-client --url http://127.0.0.1:8080 config --auth-private-key ./kbs/config/private.key  set-resource-policy --policy-file ./kbs/sample_policies/allow_all.rego
```

## Using TPM evidence

Run KBS server in one terminal
```sh
sudo ./target/release/kbs --config-file ./kbs/config/kbs-config.toml 
```

Run kbs-client in another terminal

Assuming you are still using the relaxed (allow_all..rego policy), run the following
command to get resource

```sh
export ENABLE_SAMPLE_DEVICE="TPM"
export TCTI="device:/dev/tpm1"
./target/release/kbs-client --url http://127.0.0.1:8080  get-resource --path default/test/dummy
```

If you want to set custom policy, take a look at `sampledevice_pcrs.rego`.
This uses PCRs in the additional-evidence sent for sample device.
Adapt it for your use.

Set the policy

```sh
./target/release/kbs-client --url http://127.0.0.1:8080 config --auth-private-key ./kbs/config/private.key set-resource-policy --policy-file  ./kbs/sample_policies/sampledevice_pcrs.rego
```
