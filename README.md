# bls-embedded

Install Rust:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Run with:
```
cd bls12_377
cargo build
cargo test
cargo bench
cargo bench -- Fp_m
cargo bench -- G2Projective_s
```
