# 问题
1. mock.rs 中 不加这个无法通过编译，少了一些trait的实现
   `#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]`
2. 为什么不能测试触发的事件，比较奇怪
# 测试
![img.png](img.png)