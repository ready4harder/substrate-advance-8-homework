# homework 4

# 測試紀錄：
0. 準備 
* 升級前必須清除所有數據 (rm -rf /tmp/blockchain)

1. cargo build --release --features try-runtime
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_1.png?raw=true)

2. try-runtime --runtime ./target/release/wbuild/solochain-template-runtime/solochain_template_runtime.wasm on-runtime-upgrade --checks pre-and-post --disable-idempotency-checks --no-weight-warnings live --uri ws://127.0.0.1:9944
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_2.png?raw=true)

3. 檢查版本:當前是101
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_4.png?raw=true)

4. 創建kitties的頁面
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_5.png?raw=true)

5. 創建kitties - 提交交易
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_6.png?raw=true)

6. 創建kitties - 創建成功
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_7.png?raw=true)

7. 創建kitties - 檢查是否有新kitty
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_8.png?raw=true)

8. 更新wasm的頁面
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_9.png?raw=true)

9. 更新wasm - 準備上傳
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_10.png?raw=true)

10. 更新wasm - 確認上傳
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_11.png?raw=true)

11. 更新wasm - 交易
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_12.png?raw=true)

12. 更新wasm - 成功交易
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_13.png?raw=true)

13. 檢查最新kittt狀態
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_14.png?raw=true)

* 升級成功後，鏈狀態會清零

14. 成功進行migration
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_15.png?raw=true)

15. 取到最近的Dot價格，並在拍賣成功Event事件中，顯示Kitty美元價格
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241004_16.png?raw=true)
