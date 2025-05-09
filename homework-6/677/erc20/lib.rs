#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// 結果類型
    pub type Result<T> = core::result::Result<T, Error>;

    /// ERC-20 合約
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// 總 token 的供應.
        total_supply: Balance,
        /// Mapping from owner to number of owned token.
        balances: Mapping<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw from another account.
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    /// Event for Transfer action
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event for an approval is allowed to withdraw
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// 報錯類型
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// 當 balance 不足的時候
        InsufficientBalance,
        /// 當 allowance 不足的時候
        InsufficientAllowance,
    }

    impl Erc20 {
        /// 設定初始值
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
            }
        }

        /// 獲得總 token 的供應數量
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// 獲得 owner 戶口的 balance 情況
        /// 當戶口不存在就會返回0值
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }

        /// 利用 references 比 Wasm 更有效率
        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        /// 獲得 spender 的 allowance 情況
        /// 當 allowance 未設置就會返回0值
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        /// 利用 references 比 Wasm 更有效率
        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// 將數量為 value 的 token 由 caller 轉到 'to' 的戶口 
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// 讓 spender 可以從 caller 提取多次上限為 vlaue 的 token 
        /// 會執行 Approval 事件
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        /// 由 "from" 轉移數量為 vlaue 的 token 到 "to"
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to(&from, &to, value)?;
            // 檢查 allowance >= value
            #[allow(clippy::arithmetic_side_effects)]
            self.allowances
                .insert((&from, &caller), &(allowance - value));
            Ok(())
        }

        /// 由 "caller" 轉移數量為 vlaue 的 token 到 "to"
        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of_impl(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }
            // We checked that from_balance >= value
            #[allow(clippy::arithmetic_side_effects)]
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_impl(to);
            self.balances
                .insert(to, &(to_balance.checked_add(value).unwrap()));
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });
            Ok(())
        }
    }

    /// 以下是測試
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink::primitives::{
            Clear,
            Hash,
        };

        fn assert_transfer_event(
            event: &ink::env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event =
                <Transfer as ink::scale::Decode>::decode(&mut &event.data[..])
                    .expect("encountered invalid contract event data buffer");
            let Transfer { from, to, value } = decoded_event;
            assert_eq!(from, expected_from, "encountered invalid Transfer.from");
            assert_eq!(to, expected_to, "encountered invalid Transfer.to");
            assert_eq!(value, expected_value, "encountered invalid Trasfer.value");

            let mut expected_topics = Vec::new();
            expected_topics.push(
                ink::blake2x256!("Transfer(Option<AccountId>,Option<AccountId>,Balance)")
                    .into(),
            );
            if let Some(from) = expected_from {
                expected_topics.push(encoded_into_hash(from));
            } else {
                expected_topics.push(Hash::CLEAR_HASH);
            }
            if let Some(to) = expected_to {
                expected_topics.push(encoded_into_hash(to));
            } else {
                expected_topics.push(Hash::CLEAR_HASH);
            }
            expected_topics.push(encoded_into_hash(value));

            let topics = event.topics.clone();
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::CLEAR_HASH;
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);

                assert_eq!(
                    topic_hash, expected_topic,
                    "encountered invalid topic at {n}"
                );
            }
        }

        /// 新創 Constructor 的測試
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let _erc20 = Erc20::new(100);

            // Transfer 事件執行當 Constructor 進行初始化
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        /// 總 token 供應量的測試
        #[ink::test]
        fn total_supply_works() {
            // Constructor works.
            let erc20 = Erc20::new(100);
            // Transfer 事件執行當 Constructor 進行初始化
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // 獲得 token 總供應量
            assert_eq!(erc20.total_supply(), 100);
        }

        /// 獲得戶口 balance 的測試
        #[ink::test]
        fn balance_of_works() {
            // Constructor works
            let erc20 = Erc20::new(100);
            // Transfer 事件執行當 Constructor 進行初始化
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // 在合約初始時，Alice 擁有所有 token (100)
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            // 在合約初始時，Bob 是沒有任何 token (0)
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        /// transfer 的測試
        #[ink::test]
        fn transfer_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // Transfer 事件執行當 Constructor 進行初始化
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Alice 轉移 10 個 token 給 Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            // Bob 擁有 10 個 token.
            assert_eq!(erc20.balance_of(accounts.bob), 10);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // 檢查第一個轉移事件
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // 檢查第二個轉移事件
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        /// 無效的 transfer 的測試
        #[ink::test]
        fn invalid_transfer_should_fail() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);

            // 把合約設置為被召喚角色， Bob 設為召喚角色
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob 向 eve 轉移 10 個 tokens 失敗
            assert_eq!(
                erc20.transfer(accounts.eve, 10),
                Err(Error::InsufficientBalance)
            );
            // Alice 擁有所有 token
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);

            // 轉移事件被觸發
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        /// transfer_from 的測試
        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // Transfer 事件執行當 Constructor 進行初始化
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Bob 向 Alice 轉移 token 失敗
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Err(Error::InsufficientAllowance)
            );
            // Alice 批准 Bob 轉移自己的 token
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // approve 事件發生
            assert_eq!(ink::env::test::recorded_events().count(), 2);

            // 把合約設置為被召喚角色， Bob 設為召喚角色
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob 把 token 由 Alice 轉移給 Eve
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
            // Eve 當下擁有的 token 情況
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // 檢查所有在較早前的 call 中所發生的 transfer 事件
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );

            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        #[ink::test]
        fn allowance_must_not_change_on_failed_transfer() {
            let mut erc20 = Erc20::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice 批准 Bob 轉移自己的 token
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

            // 獲得合約地址
            let callee = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(callee);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob 嘗試由 Alice 轉移 token 給 Eve.
            let emitted_events_before = ink::env::test::recorded_events().count();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
                Err(Error::InsufficientBalance)
            );
            // 檢查 Allowance 是不是一樣
            assert_eq!(
                erc20.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // 檢查是否已經沒有事件
            assert_eq!(
                emitted_events_before,
                ink::env::test::recorded_events().count()
            )
        }

        fn encoded_into_hash<T>(entity: T) -> Hash
        where
            T: ink::scale::Encode,
        {
            use ink::{
                env::hash::{
                    Blake2x256,
                    CryptoHash,
                    HashOutput,
                },
                primitives::Clear,
            };

            let mut result = Hash::CLEAR_HASH;
            let len_result = result.as_ref().len();
            let encoded = entity.encode();
            let len_encoded = encoded.len();
            if len_encoded <= len_result {
                result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                return result
            }
            let mut hash_output =
                <<Blake2x256 as HashOutput>::Type as Default>::default();
            <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
            let copy_len = core::cmp::min(hash_output.len(), len_result);
            result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
            result
        }
    }

    //e2e-tests
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_transfer<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // 初始設置
            let total_supply = 1_000_000_000;
            let mut constructor = Erc20Ref::new(total_supply);
            let erc20 = client
                .instantiate("erc20", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = erc20.call_builder::<Erc20>();

            // 轉移動作
            let total_supply_msg = call_builder.total_supply();
            let total_supply_res = client
                .call(&ink_e2e::bob(), &total_supply_msg)
                .dry_run()
                .await?;

            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let transfer_to_bob = 500_000_000u128;
            let transfer = call_builder.transfer(bob_account, transfer_to_bob);
            let _transfer_res = client
                .call(&ink_e2e::alice(), &transfer)
                .submit()
                .await
                .expect("transfer failed");

            let balance_of = call_builder.balance_of(bob_account);
            let balance_of_res = client
                .call(&ink_e2e::alice(), &balance_of)
                .dry_run()
                .await?;

            // 檢查
            assert_eq!(
                total_supply,
                total_supply_res.return_value(),
                "total_supply"
            );
            assert_eq!(transfer_to_bob, balance_of_res.return_value(), "balance_of");

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_allowances<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // 初始設置
            let total_supply = 1_000_000_000;
            let mut constructor = Erc20Ref::new(total_supply);
            let erc20 = client
                .instantiate("erc20", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = erc20.call_builder::<Erc20>();

            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let charlie_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

            let amount = 500_000_000u128;
            // 進行交易
            let transfer_from =
                call_builder.transfer_from(bob_account, charlie_account, amount);
            let transfer_from_result = client
                .call(&ink_e2e::charlie(), &transfer_from)
                .submit()
                .await;

            assert!(
                transfer_from_result.is_err(),
                "unapproved transfer_from should fail"
            );

            // Bob 批准 Charlie 轉移最多"amount"數量的 token
            let approved_value = 1_000u128;
            let approve_call = call_builder.approve(charlie_account, approved_value);
            client
                .call(&ink_e2e::bob(), &approve_call)
                .submit()
                .await
                .expect("approve failed");

            // `transfer_from` 的"amount"批准數量
            let transfer_from =
                call_builder.transfer_from(bob_account, charlie_account, approved_value);
            let transfer_from_result = client
                .call(&ink_e2e::charlie(), &transfer_from)
                .submit()
                .await;
            assert!(
                transfer_from_result.is_ok(),
                "approved transfer_from should succeed"
            );

            let balance_of = call_builder.balance_of(bob_account);
            let balance_of_res = client
                .call(&ink_e2e::alice(), &balance_of)
                .dry_run()
                .await?;

            // `transfer_from` 超出批准的"amount"數量
            let transfer_from =
                call_builder.transfer_from(bob_account, charlie_account, 1);
            let transfer_from_result = client
                .call(&ink_e2e::charlie(), &transfer_from)
                .submit()
                .await;
            assert!(
                transfer_from_result.is_err(),
                "transfer_from exceeding the approved amount should fail"
            );

            assert_eq!(
                total_supply - approved_value,
                balance_of_res.return_value(),
                "balance_of"
            );

            Ok(())
        }
    }
}