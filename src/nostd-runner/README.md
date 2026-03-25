# smt-nostd-runner

This is a Sparse Merkle Tree (SMT) verification runner on ckb-vm, with test and benchmark purpose.
It is compiled with Rust no-std support targeting `riscv64imac-unknown-none-elf`.

## Usage

The binary accepts command-line arguments in hex format:

```
<root_hash> <proof> [<key_1> <value_1> <key_2> <value_2> ... <key_N> <value_N>]
```

- `root_hash` — 32-byte expected SMT root, hex-encoded (64 hex chars)
- `proof` — compiled Merkle proof, hex-encoded (variable length)
- `key_N` / `value_N` — 32-byte key-value pairs to verify membership, hex-encoded (64 hex chars each)

All key-value pairs must be provided together; an odd number of remaining arguments is an error.

## Build

```bash
# Build the release binary
make build
```

## Features

| Feature      | Description                          |
|--------------|--------------------------------------|
| `enable_log` | Enable logging via `ckb-std` logger  |
