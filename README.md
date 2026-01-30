# X-Wing HPKE demo

This is a standalone Rust project that uses `hpke-rs` v0.5.0 from crates.io
with the `hpke-rs-libcrux` provider to demonstrate the X-Wing (draft-06) KEM.

## Run the example

From this directory:

```
cargo run
```

You should see:

```
X-Wing HPKE round-trip ok
```

## Notes

- Uses `hpke-rs = "0.5.0"` and `hpke-rs-libcrux = "0.5.0"` from crates.io.
