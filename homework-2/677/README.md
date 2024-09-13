# homework-2
修改/編寫/創建文檔:
* poe/src/benchmarking.rs
* poe/src/lib.rs
* poe/src/Cargo.toml
* runtime/src/lib.rs
* runtime/src/Cargo.toml

# 測試方法：
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

# 學號和錢包地址
* 學號：677
* 錢包地址：1vEmj4HgPbUcNi2YR7NpciQ5MdbMw12uEY1oAT8bpQcxbsv