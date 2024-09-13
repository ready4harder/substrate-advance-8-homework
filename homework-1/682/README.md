# Task1 完成
测试方法：
```Bash
# 进到 polkadot-sdk-solo-template-dev-courses
cd polkadot-sdk-solo-template-dev-courses
#配置好根目录的配置
在根目录的 Cargo.toml 文件中，内容：
[workspace]
   members = [
       "node",
       "runtime",
       "pallets/poe",
   ]

# 执行测试命令
cargo test -p pallet-poe
```

# 学号和钱包地址
1. 学号：682
2. 钱包地址：15XjAHj24yrx6cNGE2vVPwwjXXL1wEbKENT3HmUgBYrVECss
