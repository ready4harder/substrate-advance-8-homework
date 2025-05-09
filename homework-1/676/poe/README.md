存证是一种在线服务，可用于在某一时间点验证计算机文件的存在性。
存证服务有如下三个功能：  
1 创建存证；
2 撤销存证；
3 转移存证，接收两个参数，一个是内容的哈希值，另一个是存证的接收账户地址。 

本次作业要求，编写存证模块的单元测试代码，包括：
● 创建存证的测试用例
● 撤销存证的测试用例
● 转移存证的测试用例

关于如何实现pallet:

Substrate  FRAME 使用宏来封装复杂的代码块。一个自定义的 pallet 包括如下几类最重要的宏：

必须提供的一组必备属性宏：

#[pallet::pallet]： 强制性的托盘属性，为托盘定义一个结构（struct），以便它可以存储易于检索的数据，必须声明为pub struct Pallet<T>(_);。
#[pallet::config]：强制性的托盘属性，定义托盘的配置特征。
 
一般都提供的核心属性宏：

#[pallet::call]   ： 托盘实现的可调用功能函数。 
#[pallet::error]：  托盘实现的自定义错误消息。
#[pallet::event]： 托盘实现的可分发事件。 
#[pallet::storage]: 托盘实现的需要存储的数据类型。
#[pallet::hooks]:   托盘实现的可以定制的HOOK函数。
  

学号： 676


## 关于作业的说明：

## Task 1
# 编译代码
cargo build   --release
# 执行测试命令
cargo test 


## Task 2
要编译启用基准测试功能的节点，请运行以下命令：

# solochain  命令
cargo build --package solochain-template-node --release
# compile with runtime-benchmarks feature
cargo build --package solochain-template-node --release --features runtime-benchmarks

# 计算权重

##  列出可用的基准测试：
./target/release/solochain-template-node benchmark pallet --list

##  运行时的所有基准测试
./target/release/solochain-template-node benchmark pallet \
    --chain dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet "*" \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output pallets/all-weight.rs

 
# 非solochain  
cargo build --release 
# compile with runtime-benchmarks feature
cargo build --package solochain-template-node --release  --features runtime-benchmarks

