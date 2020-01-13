pushd ../bls12_377
make libfpc_native
popd
RUSTFLAGS="-L ../bls12_377" cargo +nightly build --manifest-path ./native/Cargo.toml --release --features gen_header
