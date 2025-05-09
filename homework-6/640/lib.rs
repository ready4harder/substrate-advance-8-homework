#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {

    use ink::storage::Mapping;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BalanceTooLow,
        AllowanceTooLow,
    }

    type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            balances.insert(Self::env().caller(), &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(Self::env().caller()),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                ..Default::default()
            }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            self.transfer_helper(&sender, &to, value)
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let sender = self.env().caller();
            let mut allowance = self.allowances.get(&(from, sender)).unwrap_or_default();

            if allowance < value {
                return Err(Error::AllowanceTooLow);
            }

            self.allowances
                .insert(&(from, sender), &(&allowance - value));

            return self.transfer_helper(&from, &to, value);
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            self.allowances.insert(&(sender, to), &value);

            self.env().emit_event(Approval {
                from: sender,
                to,
                value,
            });

            Ok(())
        }

        pub fn transfer_helper(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let balance_from = self.balance_of(*from);
            let balance_to = self.balance_of(*to);

            if value > balance_from {
                return Err(Error::BalanceTooLow);
            }

            self.balances.insert(from, &(balance_from - value));
            self.balances.insert(to, &(balance_to + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        type Event = <Erc20 as ::ink::reflect::ContractEventBase>::Type;
        /// We test if the default constructor does its job.
        #[ink::test]
        fn constructor_works() {
            let erc20 = Erc20::new(10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.total_supply(), 10000);
            assert_eq!(erc20.balance_of(accounts.alice), 10000);

            let emitted_event = ink::env::test::recorded_events().collect::<Vec<_>>();
            let event = &emitted_event[0];
            let decoded =
                <Event as scale::Decode>::decode(&mut &event.data[..]).expect("decoded error");

            match decoded {
                Event::Transfer(Transfer { from, to, value }) => {
                    assert!(from.is_none(), "nint from error");
                    assert_eq!(to, Some(accounts.alice));
                    assert_eq!(value, 10000, "nint value error");
                }
                _ => panic!("match error"),
            }
        }

        #[ink::test]
        fn transfer_should_work() {
            let mut erc20 = Erc20::new(10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let res = erc20.transfer(accounts.bob, 12);
            assert!(res.is_ok());
            assert_eq!(erc20.balance_of(accounts.alice), 10000 - 12);
            assert_eq!(erc20.balance_of(accounts.bob), 12);
        }

        #[ink::test]
        fn transfer_should_fault() {
            let mut erc20 = Erc20::new(10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let res = erc20.transfer(accounts.alice, 12);
            assert!(res.is_err());
            assert_eq!(res, Err(Error::BalanceTooLow));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
    
        use super::*;
        use ink_e2e::build_message;
    
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    
        #[ink_e2e::test]
        async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let total_supply = 123;
            let constructor = Erc20Ref::new(total_supply);
    
            // 增加 endowment 确保合约可以正确部署
            let constract_acc_id = client
                .instantiate("erc20", &ink_e2e::alice(), constructor, 100_000_000_000_000_000, None)
                .await
                .expect("instantiate failed")
                .account_id;
    
            let alice_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            
            // 构建 transfer 消息并增加 gas 限制
            let transfer_msg = build_message::<Erc20Ref>(constract_acc_id.clone())
                .call(|erc20| erc20.transfer(bob_acc, 2));
    
            let res = client.call(&ink_e2e::alice(), transfer_msg, 0, Some(5_000_000_000_000)).await;
            assert!(res.is_ok(), "Transfer failed");
    
            // 检查 Alice 的余额，增加 gas 限制
            let balance_of_msg = build_message::<Erc20Ref>(constract_acc_id.clone()).call(|erc20| erc20.balance_of(alice_acc));
            let balance_of_alice_result = client.call_dry_run(&ink_e2e::alice(), &balance_of_msg, 0, Some(5_000_000_000_000)).await;
            let balance_of_alice: Balance = balance_of_alice_result.return_value();
            assert_eq!(balance_of_alice, 121, "Expected balance of Alice to be 121, got {}", balance_of_alice);    
            Ok(())
        }
    }
}
