#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[ink::contract]
mod erc20 {

    use ink::storage::Mapping;

    type  Result<T> = core::result::Result<T,Error>;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }


    #[ink(event)]
    pub struct Approval{
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq,Eq)]
    #[ink::scale_derive(Encode, Decode,TypeInfo)]
    pub enum Error{
        BalanceTooLow,
        AllowanceTooLow,
        Overflow,
    }



    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total: Balance) -> Self {
            let mut balances = Mapping::new();
            balances.insert(Self::env().caller(), &total);
            Self {
                total_supply: total,
                balances,
                allowances: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or_default()
        }

        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from,&to,value)?;
            Ok(())
        }

        fn allowances_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowances_of(caller, from);
            if allowance < value {
                return Err(Error::AllowanceTooLow);
            }
            self.transfer_from_to(&from,&to,value)?;
            #[allow(clippy::arithmetic_side_effects)]
            self.allowances.insert((&caller,&from),&(allowance - value));
            Ok(())
        }

     
        fn transfer_from_to(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<()> {
            let from_balance = self.balance_of(from.clone());
            if from_balance < value {
                return Err(Error::BalanceTooLow);
            }
            #[allow(clippy::arithmetic_side_effects)]
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_impl(to);
            self.balances.insert(to, &(to_balance.checked_add(value).ok_or(Error::Overflow)?));  
            self.env().emit_event(Transfer{from:*from,to:*to,value});
            Ok(())
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();       
            #[allow(clippy::arithmetic_side_effects)]
            self.allowances.insert((&caller,&spender),&value);
            self.env().emit_event(Approval{owner:caller,spender,value});
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
        use ink::{scale, scale_info::Type};


        
        #[ink::test]
        fn total_supply_works() {
            let erc20 = Erc20::new(100);
            assert_eq!(erc20.total_supply(), 100);
        }

        #[ink::test]
        fn balance_of_works() {
            let erc20 = Erc20::new(100);
            let accounts =
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.balance_of(accounts.alice), 100);
        }

        #[ink::test]
        fn transfer_works() {
            let mut erc20 = Erc20::new(2000);
            let accounts =
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
         
            let alice = accounts.alice;
            let bob = accounts.bob;

            assert_eq!(erc20.balance_of(bob), 0);
            // 测试普通转账
            assert_eq!(erc20.transfer(bob, 500), Ok(()));
    
            assert_eq!(erc20.balance_of(bob), 500);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);

            let event = &emitted_events[0];
            
            let decoded = <Transfer as scale::Decode>::decode(&mut &event.data[..]).expect("Failed to decode event");
            let Transfer{from,to,value} = decoded;
        
            assert_eq!(from, alice);
            assert_eq!(to, bob);
            assert_eq!(value, 500);

         
            
        }


        #[ink::test]
        fn transfer_overflow() {
            let mut erc20 = Erc20::new(100);
            let accounts =
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let alice = accounts.alice;
            let bob = accounts.bob;

            
            // 转账前金额
            assert_eq!(erc20.balance_of(bob), 0);
            assert_eq!(erc20.balance_of(alice), 100);

             // 测试转账超过余额
             assert_eq!(erc20.transfer(bob, 2000), Err(Error::BalanceTooLow));
             assert_eq!(erc20.balance_of(alice), 100);
             assert_eq!(erc20.balance_of(bob), 0);
        }



       

        #[ink::test]
        fn approve_works() {
            let mut erc20 = Erc20::new(2000);
            let accounts =
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let owner = accounts.alice;
            let spender = accounts.bob;

            assert_eq!(erc20.approve(spender, 1000), Ok(()));
            assert_eq!(erc20.allowances_of(owner, spender), 1000);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);

            let event = &emitted_events[0];
            let decoded = <Approval as scale::Decode>::decode(&mut &event.data[..]).expect("Failed to decode event");
            let Approval{owner,spender,value} = decoded;
            assert_eq!(owner, owner);
            assert_eq!(spender, spender);
            assert_eq!(value, 1000);
        }

        #[ink::test]
        fn approve_failed() {
            let mut erc20 = Erc20::new(2000);
            let accounts =
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let alice = accounts.alice;
            let bob = accounts.bob;

            // 测试approve失败  
            assert_eq!(erc20.approve(bob, 100), Ok(()));

            assert_eq!(erc20.transfer_from(alice, bob, 500), Err(Error::AllowanceTooLow));
        }

    }


}
