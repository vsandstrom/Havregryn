# Havregryn

## Building

After installing [Rust](https://rustup.rs/), you can compile Havregryn as follows:

```shell
git submodule update --init --recursive
cargo xtask bundle havregryn --release
```

This will compile a VST3 for your platform. 
