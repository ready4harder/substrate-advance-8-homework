# Proof of Existence

## Run benchmark

```shell
# compile with runtime-benchmarks feature
cargo build --release --features runtime-benchmarks

# benchmark dispatchables in poe pallet
# download frame-weight-template.hbs from [polkadot-sdk repo](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/.maintain/frame-weight-template.hbs).
./target/production/solochain-template-node benchmark pallet \
--wasm-execution=compiled \
--pallet pallet_poe \
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--output pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs
```
