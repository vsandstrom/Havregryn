# Havregryn

Havregryn is a granular delay and texture synthesizer. The name is swedish for grains of oats, from which you can make havregrynsgröt, oatmeal.
<img width="612" alt="Skärmavbild 2024-07-17 kl  07 27 56" src="https://github.com/user-attachments/assets/63c42358-c391-445d-9a58-0d5086c75d45">

## Building

After installing [Rust](https://rustup.rs/), you can compile Havregryn as follows:

```shell
git submodule update --init --recursive
cargo xtask bundle havregryn --release
```

This will compile a VST3 for your platform. 
