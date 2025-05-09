# Homework-2 作业
为 proof of existence (poe) 模块的可调用函数 create_claim, revoke_claim, transfer_claim 添加 benchmark 用例，并且将 benchmark 运行的结果应用在可调用函数上

## 我的实现：

代码在 benchmarking.rs 文件中。

编译命令：
```shell
cargo build --profile=production --features runtime-benchmarks
```
编译过程相对比较长，主要在 wasm 相关模块以及最终的 node 节点耗时比较长。

对于结果运用的部分，在执行编译结束后，注意，对于 runtime benchmark 的配置需要在编译项目前完成，完成项目编译后执行以下命令以应用运行结果：
```shell
./target/production/solochain-template-node benchmark pallet \
    --chain dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet pallet_poe \
    --extrinsic "*" \
    --steps 20 \
    --repeat 10 \
    --output pallets/poe/src/weight.rs \
    --template .maintain/frame-weight-template.hbs
```
依此我们可生成 weight.rs 文件。