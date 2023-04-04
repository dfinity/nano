# quill

Minimalistic ledger and governance toolkit for cold wallets.

`quill` is a toolkit for interacting with the Network Nervous System's (NNS) canisters using self-custody keys. These
keys
can be held in an air-gapped computer (a computer
that has never connected to the internet) known as a cold wallet. To support cold wallets, `quill` takes a two-phase
approach to sending query/update calls to the IC. In the first phase, `quill` is used with the various subcommands to
generate and sign messages based on user input, without needing access to the internet. In the second phase, the signed
message(s) are sent to the IC. Since this requires connection to boundary nodes via the internet, cold-wallet users will
transport the signed message(s) from the air-gapped computer (i.e. with a USB stick, or via QR code) to a computer connected to the
internet.

## Disclaimer

YOU EXPRESSLY ACKNOWLEDGE AND AGREE THAT USE OF THIS SOFTWARE IS AT YOUR SOLE RISK.
AUTHORS OF THIS SOFTWARE SHALL NOT BE LIABLE FOR DAMAGES OF ANY TYPE, WHETHER DIRECT OR INDIRECT.

## Usage

This will sign a transfer transaction and print to STDOUT:

    quill transfer <account-id> --amount <amount> --pem-file <path>

To display the signed message in human-readable form:

    quill send --dry-run <path-to-file>

`quill` could be used on an online computer to send any signed transaction:

    quill send <path-to-file>

To get the principal and the account id:

    quill public-ids --pem-file <path>

### Governance

This is how you’d stake/top-up a neuron:

    quill neuron-stake --amount 2.5 --name 1 --pem-file <path>

Managing the neuron:

    quill neuron-manage <neuron-id> [OPERATIONS] --pem-file <path>

All the commands above will generate signed messages, which can be sent on the online machine using the `send` command
from above.

## Download & Install

Use pre-built binaries from the latest [release](https://github.com/dfinity/quill/releases).

### MacOS (Intel Chip & Apple Silicon)

#### Install quill
1. Download the file named `quill-macos-x86_64`
2. Move the file to your `/usr/local/bin` directory to make it available system-wide

```shell
sudo mv quill-macos-x86_64 /usr/local/bin/quill
```

3. Make the file executable

```shell
chmod +x /usr/local/bin/quill
```

4. Run quill

```shell
quill -h
```

### Linux

1. Download the file specific to your system architecture
    1. for x86 download `quill-linux-x86_64`
    2. for arm32 download `quill-arm_32`
    3. for Alpine download `quill-linux-x86_64-musl`

2. Move the file to your `/usr/local/bin` directory to make it available system-wide

```shell
sudo mv quill-linux-x86_64 /usr/local/bin/quill
```

3. Make the file executable

```shell
chmod +x /usr/local/bin/quill 
```

4. Run quill

```shell
quill -h
```

### Windows

1. Download the file named `quill-windows-x86_64.exe`

2. Move it and a shell to a convenient location, e.g.

```ps1
move-item quill-windows-x86_64.exe ~\quill.exe
set-location ~
```

3. Run quill

```ps1
.\quill.exe -h
```

## Build

To compile `quill` run:

```sh
cargo build --release --locked
```

After this, find the binary at `target/release/quill`.

Quill has two optional features, both activated by default:

- `static-ssl`, to build OpenSSL from source instead of dynamically linking a preinstalled version (requires a C compiler)
- `hsm`, to enable PKCS#11 HSM support (requires runtime dynamic linking)

To build a version of Quill that links OpenSSL dynamically, but retains HSM support, run:

```sh
cargo build --release --locked --no-default-features --feature hsm
```

To build a version of Quill compatible with statically-linked-only environments, such as Alpine, run:

```sh
cargo build --release --locked --no-default-features --feature static-ssl
```

### Building with Nix

If you have Nix installed, you can use it to provide an environment for
running `cargo`. Just replace the above build steps with the following:

To compile `quill` run:

1. `nix-shell`
4. `cargo build --release --locked`

After this, find the binary at `target/release/quill`.

## Testnets

If you have access to an Internet Computer testnet (for example, a version the
replica binary and NNS running locally), you can target quill at this test
network by setting the `IC_URL` environment variable to the full URL. In addition
to that, it is required to use the `--insecure-local-dev-mode` flag. For
example:

    IC_URL=https://nnsdapp.dfinity.network quill --insecure-local-dev-mode --pem-file <path> list-neurons

## Contribution

`quill` is a very critical link in the workflow of the management of valuable assets.
`quill`'s code must stay clean, simple, readable and leave no room for ambiguities, so that it can be reviewed and
audited by anyone.
Hence, if you would like to propose a change, please adhere to the following principles:

1. Be concise and only add functional code.
2. Optimize for correctness, then for readability.
3. Avoid adding dependencies at all costs unless it's completely unreasonable to do so.
4. Every new feature (+ a test) is proposed only after it was tested on real wallets.
5. Increment the last digit of the crate version whenever the functionality scope changes.

## Credit

Originally forked from the [SDK](https://github.com/dfinity/sdk).
