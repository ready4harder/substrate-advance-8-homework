1. 基于第三课的提交，完成Offchain worker的开发，取到最近的Dot价格，并在拍卖成功Event事件中，显示Kitty美元价格。
2. 完成第一种runtime upgrade，在runtime/lib中增加任意状态修改的逻辑，并使用try runtime验证，使用set code升级链
3. 完成第二种runtime upgrade，在pallet里面设置storage 版本。修改Kitty数据结构，验证升级和数据迁移结果
4. 升级的时候正确的计算和返回weight