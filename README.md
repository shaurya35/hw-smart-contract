# hello-world

A small Solana program written in Rust that keeps a counter on chain.

This is a simple example to learn how a Solana program reads from an account,
changes some data, and writes it back.

## What it does

The program stores a single number (the count) inside an account. Each time
you call the program, you send an instruction that tells it what to do:

- `Increase` adds 1 to the count
- `Decrease` takes 1 away from the count

The count never goes below 0. If you try to decrease it below 0, it stays at 0.

## How it works

1. The program reads the first account passed to it. This account holds the
   counter.
2. The account must sign the transaction. If it does not, the program returns
   an error.
3. The program reads the current count from the account data.
4. It reads the instruction (`Increase` or `Decrease`) from the instruction
   data.
5. It changes the count and writes the new value back to the account.

## Data format

The counter is stored as a `u32` value. The program uses Borsh to turn the
data into bytes and back again.

The instruction is also a Borsh value. It is one of two choices: `Increase` or
`Decrease`.

## Project layout

```
hello-world/
  src/lib.rs     the program code
  Cargo.toml     project settings and dependencies
```

## Build

You need Rust and the Solana tool suite installed.

```bash
cargo build-sbf
```

This builds the program into a `.so` file that you can deploy to a Solana
cluster.

## Deploy

```bash
solana program deploy ./target/deploy/hello_world.so
```

Run this against your chosen cluster, for example a local validator or devnet.
