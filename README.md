benchmark commond: cargo build --profile=production --features runtime-benchmarks

run:
./target/production/solochain-template-node benchmark pallet \
 --wasm-execution=compiled \
 --pallet pallet_kitties \
 --extrinsic "*" \
 --steps 20 \
 --repeat 10 \
 --output pallets/kitties/src/weights.rs \
 --template .maintain/frame-weight-template.hbs

test:
cargo test --package pallet-kitties --features runtime-benchmarks

# substrate-advance-8-homework

## 课程名称

Substrate 开发进阶与项目实战第 8 期

### 课程简介

本课程由 OneBlock+ 和 Polkadot 联合推出，旨在通过对 Pallet 开发的技巧和测试、Runtime 升级和数据迁移、XCM 消息的应用、Ink 智能合约的学习，结合课后的 Task 任务、参加 Polkadot Hackathon 2024 Bangkok 实践，让开发者熟练掌握项目开发技巧，快速进行项目落地！

快来运用 Substrate 技术开启项目开发之旅，参加 2024 年波卡黑客松大赛，一起瓜分百万奖金吧！

### 课程设置

6 次视频课+6 个 Task+6 次 office hour

### 批改标准：

10 分/个，共 60 分

1）是否完成简答基本内容

2）是否结合题目进行深度思考

3）是否完成 Demo 中的基本功能

4）符合开源社区标准（包括但不限于代码注释完整并且提交到上述 github 地址里）

### 奖励明细

1. 无抄袭，且得分 70%及以上

2. 完成全部 6 个 task 任务

3. 满足以上条件者，瓜分 500U 奖学金

### 奖励发放

1.奖金将以 DOT 或现金红包的形式、在结业后 7 个工作日发放

2.DOT 发放需满足 1.3DOT 最低转账要求，否则予以现金红包发放

### 注意事项

1）复制 homework 目录下的 000 并使用自己的学号新建一个目录；

2）.gitignore 不要提交编译出来的文件；

3）不可复制粘贴他人作业内容，发现即 0 分，不予发放奖励
