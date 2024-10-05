# Migrations
## install try runtime
cargo install --git https://github.com/paritytech/try-runtime-cli --locked


## build node
cargo build
./target/debug/solochain-template-node --dev --base-path /tmp/blockchain

## use try-runtime
cargo build --features try-runtime

 try-runtime --runtime target/debug/wbuild/solochain-template-runtime/solochain_template_runtime.wasm on-runtime-upgrade live --uri ws://127.0.0.1:9944
