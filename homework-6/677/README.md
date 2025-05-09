# homework 6

# 紀錄：
1. 環境安裝
* install substrate contracrt node
* git clone https://github.com/paritytech/substrate-contracts-node.git
![alt text](https://github.com/MartinYeung5/substrate-advance-8-homework/blob/main/homework-6/677/0.png?raw=true)

2. install cargo-contract
* cargo install cargo-contract
![alt text](https://github.com/MartinYeung5/substrate-advance-8-homework/blob/main/homework-6/677/1.png?raw=true)

3. 創建新的 ink! 項目
* cargo contract new ink_project_20241015
![alt text](https://github.com/MartinYeung5/substrate-advance-8-homework/blob/main/homework-6/677/2.png?raw=true)

4. 檢查 rust 版本，修改成以下版本就可以
* 重點，不要降低 rust 版本，用 1.81.0 就可以
* rustup show
![alt text](https://github.com/MartinYeung5/substrate-advance-8-homework/blob/main/homework-6/677/3.png?raw=true)

5. 快速修改 rust 版本
* rustup update
![alt text](https://github.com/MartinYeung5/substrate-advance-8-homework/blob/main/homework-6/677/4.png?raw=true)

6. 當完成修改ink! 項目後，可以build
* cargo contract build
![alt text](https://github.com/MartinYeung5/substrate-advance-8-homework/blob/main/homework-6/677/5.png?raw=true)

7. 進行項目 test
* cargo test
![alt text](https://github.com/MartinYeung5/substrate-advance-8-homework/blob/main/homework-6/677/6.png?raw=true)

