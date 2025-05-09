# 基础信息

学号：654

钱包地址：1481PpgfoBGxTsfHEFDrRg6ECt8yrYMPEZE1UEj2FdfaZvyU

# 测试

```
cargo build --profile=production --features runtime-benchmarks
```

```
./target/production/solochain-template-node benchmark pallet \
 --chain dev \ 
 --execution=wasm \
 --wasm-execution=compiled \ 
 --pallet pallet_poe \ 
 --extrinsic "*" \ 
 --steps 20 \
 --repeat 10 \
 --output pallets/poe/src/weights.rs \
 --template .maintain/frame-weight-template.hbs
```
