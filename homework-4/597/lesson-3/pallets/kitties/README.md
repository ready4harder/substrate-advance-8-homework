License: MIT-0
# Install try-runtime
cargo install --git https://github.com/paritytech/try-runtime-cli --locked

cargo build --release --features try-runtime

# Runtime live
try-runtime --runtime ./target/release/wbuild/solochain-template-runtime/solochain_template_runtime.wasm on-runtime-upgrade --checks pre-and-post --disable-idempotency-checks --no-weight-warnings live --uri ws://127.0.0.1:9944
