#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        BalanceTooLow,
        AllowanceTooLow,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }
    #[ink(event)]
    pub struct Approve {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }
    type ResultTransfer<T> = core::result::Result<T, Error>;
    type ResultApproval<T> = core::result::Result<T, Error>;

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            balances.insert(Self::env().caller(), &total_supply);
            Self {
                total_supply,
                balances,
                ..Default::default()
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> ResultTransfer<()> {
            let sender = self.env().caller();

            self.transfer_helper(&sender, &to, value)
        }
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> ResultTransfer<()> {
            let sender = self.env().caller();
            let allowance = self.allowances.get(&(from, sender)).unwrap_or_default();
            if allowance < value {
                return Err(Error::AllowanceTooLow);
            }

            let new_allowance = allowance.checked_sub(value).ok_or(Error::AllowanceTooLow)?;
            self.allowances.insert(&(from, sender), &new_allowance);
            self.transfer_helper(&from, &to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, value: Balance) -> ResultApproval<()> {
            let sender = self.env().caller();
            self.allowances.insert(&(sender, to), &value);
            self.env().emit_event(Approve {
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
        ) -> ResultTransfer<()> {
            let balance_from = self.balance_of(*from);
            let balance_to = self.balance_of(*to);

            if value > balance_from {
                return Err(Error::BalanceTooLow);
            }
            let new_balance_from = balance_from
                .checked_sub(value)
                .ok_or(Error::BalanceTooLow)?;
            let new_balance_to = balance_to.checked_add(value).ok_or(Error::BalanceTooLow)?;

            self.balances.insert(from, &new_balance_from);
            self.balances.insert(to, &new_balance_to);

            self.env().emit_event(Transfer {
                from: *from,
                to: *to,
                value,
            });

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        // type Event = <Erc20 as ::ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn constructor_works() {
            let erc20 = Erc20::new(100);
            assert_eq!(erc20.total_supply(), 100);

            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.balance_of(accounts.alice), 100);
        }

        #[ink::test]
        fn transfer_works_and_emits_event() {
            let mut erc20 = Erc20::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Perform the transfer
            erc20.transfer(accounts.bob, 40).unwrap();

            // Assert balances
            assert_eq!(erc20.balance_of(accounts.alice), 60);
            assert_eq!(erc20.balance_of(accounts.bob), 40);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use super::*;
        use ink_e2e::{subxt::dynamic::Value, ChainBackend, ContractsBackend};

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        /// CONTRACTS_NODE=/home/xiao/Workspaces/substrate-contracts-node/target/release/substrate-contracts-node cargo test --features e2e-tests
        /// ink_e2e = { version = "5.0.0" }
        #[ink_e2e::test]
        async fn instantiate_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let mut constructor = Erc20Ref::new(10);

            let contract = client
                .instantiate("erc20", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let call_builder = contract.call_builder::<Erc20>();

            let total_supply = call_builder.total_supply();
            let total_supply_res = client
                .call(&ink_e2e::alice(), &total_supply)
                .dry_run()
                .await?;

            assert_eq!(total_supply_res.return_value(), 10);

            Ok(())
        }
    }
}
