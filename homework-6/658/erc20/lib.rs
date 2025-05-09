#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// ERC-20 合约。
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// 代币总供应量。
        total_supply: Balance,
        /// 从所有者到拥有的token数量的映射。
        balances: Mapping<AccountId, Balance>,
        /// 允许账户提取的代币金额映射
        /// 来自另一个帐户。
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    /// tonken 转移时发出的事件。
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// 错误类型
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// 如果没有足够的余额来满足请求，则返回。
        InsufficientBalance,
        /// 如果没有足够的配额来满足请求，则返回。
        InsufficientAllowance,
    }

    /// ERC-20 结果类型。
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// 创建具有指定初始供应量的新 ERC-20 合约。
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

       /// 返回代币总供应量。
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// 返回指定“所有者”的帐户余额。
        ///
        /// 如果帐户不存在，则返回“0”。
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }

        /// 返回指定“所有者”的帐户余额。
        ///
        /// 如果帐户不存在，则返回“0”。
        ///
        /// ＃ note
        ///
        /// 更喜欢调用此方法而不是`balance_of`，因为
        /// 使用在 Wasm 中更有效的引用来工作。
        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        /// 返回仍允许“spender”从“owner”提取的金额。
        ///
        /// 如果未设置限额，则返回“0”。
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        /// 返回仍允许“spender”从“owner”提取的金额。
        ///
        /// 如果未设置限额，则返回“0”。
        ///
        /// ＃ note
        ///
        /// 更喜欢调用此方法而不是`allowance`，因为
        /// 使用在 Wasm 中更有效的引用来工作。
        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// 将“value”数量的代币从调用者的账户转移到账户“to”。
        ///
        /// 成功时会发出 `Transfer` 事件。
        ///
        /// # Error
        ///
        /// 如果没有足够的令牌，则返回 `InsufficientBalance` 错误
        /// 调用者的账户余额。
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// 允许`spender`多次从调用者的帐户中取款，最多可达
        /// `value` 金额。
        ///
        /// 如果再次调用此函数，它将覆盖当前的限额
        /// `值`。
        ///
        /// 发出“Approval”事件。
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

        /// 代表“from”将“value”代币转移到账户“to”。
        ///
        /// 这可用于允许合约代表某人转移代币和/或
        /// 例如，以子货币收取费用。
        ///
        /// 成功时会发出 `Transfer` 事件。
        ///
        /// # Errors
        ///
        /// 如果没有足够的token，则返回 `InsufficientAllowance` 错误
        /// 调用者从 `from` 中退出。
        ///
        /// 如果没有足够的令牌，则返回 `InsufficientBalance` 错误
        /// `from` 的账户余额。
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
            // We checked that allowance >= value
            #[allow(clippy::arithmetic_side_effects)]
            self.allowances
                .insert((&from, &caller), &(allowance - value));
            Ok(())
        }

        /// 将“value”数量的代币从调用者的账户转移到账户“to”。
        ///
        /// 成功时会发出 `Transfer` 事件。
        ///
        /// # Errors
        ///
        /// 如果没有足够的token，则返回 `InsufficientBalance` 错误
        /// 调用者的账户余额。
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

        /// 默认构造函数完成其工作。
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let _erc20 = Erc20::new(100);

            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

       /// 总供应量测试。
        #[ink::test]
        fn total_supply_works() {
            // Constructor works.
            let erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Get the token total supply.
            assert_eq!(erc20.total_supply(), 100);
        }

       /// 获取账户实际余额。
        #[ink::test]
        fn balance_of_works() {
            // Constructor works
            let erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Alice owns all the tokens on contract instantiation
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            // Bob does not owns tokens
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Alice transfers 10 tokens to Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            // Bob owns 10 tokens.
            assert_eq!(erc20.balance_of(accounts.bob), 10);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // Check first transfer event related to ERC-20 instantiation.
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Check the second transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);

           // 将合约设置为被调用者，将 Bob 设置为调用者。
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob 未能将 10 个代币转移给 Eve。
            assert_eq!(
                erc20.transfer(accounts.eve, 10),
                Err(Error::InsufficientBalance)
            );
            // Alice 拥有所有代币。
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);

            // 初始构造期间触发的传输事件。
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // 初始构造期间触发的传输事件。
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Bob 无法转移 Alice 拥有的代币。
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Err(Error::InsufficientAllowance)
            );
           // Alice 代表她批准 Bob 进行代币转账。
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // 批准事件发生。
            assert_eq!(ink::env::test::recorded_events().count(), 2);

            // 将合约设置为被调用者，将 Bob 设置为调用者。
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

           // Bob 将代币从 Alice 转移到 Eve。
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
           // Eve 拥有代币。
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // 检查之前调用期间发生的所有传输事件：
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // 第二个事件 `emissed events[1]` 是一个已批准的事件，我们跳过检查。
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

            // Alice 代表她批准 Bob 进行代币转账。
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

            // 获取合约地址。
            let callee = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(callee);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob 尝试将代币从 Alice 转移到 Eve。
            let emitted_events_before = ink::env::test::recorded_events().count();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
                Err(Error::InsufficientBalance)
            );
            // Allowance 保持不变
            assert_eq!(
                erc20.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // 不得再发出任何事件
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

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_transfer<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {

            let total_supply = 1_000_000_000;
            let mut constructor = Erc20Ref::new(total_supply);
            let erc20 = client
                .instantiate("erc20", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = erc20.call_builder::<Erc20>();

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
            // tx
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

            // 鲍勃批准查理代表他转账最多金额
            let approved_value = 1_000u128;
            let approve_call = call_builder.approve(charlie_account, approved_value);
            client
                .call(&ink_e2e::bob(), &approve_call)
                .submit()
                .await
                .expect("approve failed");

           // `transfer_from` 批准的金额
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

            // `transfer_from` again, this time exceeding the approved amount
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