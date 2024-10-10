#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
///
/// Used as `ensure!(expression_to_ensure, expression_to_return_on_false)`.
macro_rules! ensure {
    ( $condition:expr, $error:expr $(,)? ) => {{
        if !$condition {
            return ::core::result::Result::Err(::core::convert::Into::into($error));
        }
    }};
}
#[ink::contract]
mod erc20 {

    use ink::{
        prelude::{borrow::ToOwned, string::String},
        storage::Mapping,
    };

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, String>;

    /// A simple ERC-20 contract.
    #[ink(storage)]
    pub struct ERC20 {
        owner: AccountId,
        /// Token name.
        name: String,
        /// Token symbol.
        symbol: String,
        /// Total token supply.
        total_supply: Balance,
        /// Mapping from owner to number of owned token.
        balances: Mapping<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
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
        #[ink(topic)]
        value: Balance,
    }

    impl ERC20 {
        /// Creates a new ERC-20 contract with the specified name and symbol.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Result<Self> {
            ensure!(!name.is_empty(), "NameCannotBeEmpty");
            ensure!(
                name.chars().all(|c| c.is_ascii_lowercase()),
                "TheCharactersOfTheNameMustBeLowerCase"
            );
            ensure!(name.len() <= 8, "NameMustBeLessThanOrEqualTo8");
            ensure!(!symbol.is_empty(), "SymbolCannotBeEmpty");
            ensure!(
                symbol.chars().all(|c| c.is_ascii_uppercase()),
                "TheCharactersOfTheSymbolMustBeUpperCase"
            );
            ensure!(symbol.len() <= 8, "SymbolMustBeLessThanOrEqualTo8");

            Ok(Self {
                owner: Self::env().caller(),
                name,
                symbol,
                total_supply: Default::default(),
                balances: Mapping::default(),
                allowances: Default::default(),
            })
        }
    }

    impl ERC20 {
        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.update_impl(Some(from), Some(to), value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with
        /// `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            ensure!(spender != owner, "SpenderIsSelf");
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the account balance of `from`.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            if from == caller {
                return self.transfer(to, value);
            }
            let allowance = self.allowance_impl(&from, &caller);
            ensure!(allowance >= value, "InsufficientAllowance");
            self.update_impl(Some(from), Some(to), value)?;
            let allowance = self.allowance_impl(&from, &caller);
            self.allowances.insert(
                (&from, &caller),
                &allowance
                    .checked_sub(value)
                    .ok_or("InsufficientAllowance".to_owned())?,
            );
            Ok(())
        }
    }

    impl ERC20 {
        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Returns the symbol of the token, usually a shorter version of the name.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Returns the number of decimals used to get its user representation.
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            12
        }
    }
    /// Trait implemented by Contract module which provides a basic access control mechanism, where
    //  * there is an account (an owner) that can be granted exclusive access to
    //  * specific functions.
    //  *
    //  * The initial owner is set to the address provided by the deployer.

    impl ERC20 {
        /// Returns the address of the current owner.
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }
    }

    impl ERC20 {
        /// Destroys a `value` amount of tokens from the caller.
        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<()> {
            self.update_impl(Some(self.env().caller()), None, value)
        }
    }

    impl ERC20 {
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            self.update_impl(None, Some(to), value)
        }
    }

    #[ink(impl)]
    impl ERC20 {
        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        ///
        /// # Note
        ///
        /// Prefer to call this method over `balance_of` since this
        /// works using references which are more efficient in Wasm.
        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set.
        ///
        /// # Note
        ///
        /// Prefer to call this method over `allowance` since this
        /// works using references which are more efficient in Wasm.
        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        fn update_impl(
            &mut self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            value: Balance,
        ) -> Result<()> {
            if let Some(from) = &from {
                let from_balance = self.balance_of_impl(from);
                ensure!(from_balance >= value, "InsufficientBalance");

                self.balances.insert(
                    from,
                    &from_balance
                        .checked_sub(value)
                        .ok_or("InsufficientBalance".to_owned())?,
                );
            } else {
                self.total_supply = self
                    .total_supply
                    .checked_add(value)
                    .ok_or("OverflowBalance".to_owned())?;
            }

            if let Some(to) = &to {
                let to_balance = self.balance_of_impl(to);
                self.balances
                    .insert(to, &(to_balance.checked_add(value).unwrap()));
            } else {
                self.total_supply = self
                    .total_supply
                    .checked_sub(value)
                    .ok_or("InsufficientTotalSupply".to_owned())?;
            }

            self.env().emit_event(Transfer { from, to, value });
            Ok(())
        }
    }

    /// Unit tests.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink::{
            env::hash::{Blake2x256, CryptoHash, HashOutput},
            primitives::Clear,
        };

        fn assert_transfer_event(
            event: &ink::env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Transfer as ink::scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            let Transfer { from, to, value } = decoded_event;
            assert_eq!(from, expected_from, "encountered invalid Transfer.from");
            assert_eq!(to, expected_to, "encountered invalid Transfer.to");
            assert_eq!(value, expected_value, "encountered invalid Trasfer.value");

            fn encoded_into_hash<T>(entity: T) -> Hash
            where
                T: ink::scale::Encode,
            {
                let mut result = Hash::CLEAR_HASH;
                let len_result = result.as_ref().len();
                let encoded = entity.encode();
                let len_encoded = encoded.len();
                if len_encoded <= len_result {
                    result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                    return result;
                }
                let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
                <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
                let copy_len = core::cmp::min(hash_output.len(), len_result);
                result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
                result
            }

            let mut expected_topics = Vec::new();
            expected_topics.push(
                ink::blake2x256!("Transfer(Option<AccountId>,Option<AccountId>,Balance)").into(),
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

            for (n, (actual_topic, expected_topic)) in
                event.topics.iter().zip(expected_topics).enumerate()
            {
                let topic = <Hash as ink::scale::Decode>::decode(&mut &actual_topic[..])
                    .expect("encountered invalid topic encoding");
                assert_eq!(topic, expected_topic, "encountered invalid topic at {n}");
            }
        }

        /// The default constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let (name, symbol) = ("test".to_owned(), "TEST".to_owned());
            let erc20 = ERC20::new(name.clone(), symbol.clone()).unwrap();

            assert_eq!(ERC20::name(&erc20), name);
            assert_eq!(ERC20::symbol(&erc20), symbol);
        }

        #[ink::test]
        fn new_fail_when_name_is_invalid() {
            // Constructor works.
            assert!(ERC20::new(String::new(), "TEST".to_owned())
                .is_err_and(|e| e == "NameCannotBeEmpty".to_owned()));
            assert!(ERC20::new("Test".to_owned(), "TEST".to_owned())
                .is_err_and(|e| e == "TheCharactersOfTheNameMustBeLowerCase".to_owned()));
            assert!(ERC20::new("testtestt".to_owned(), "TEST".to_owned())
                .is_err_and(|e| e == "NameMustBeLessThanOrEqualTo8".to_owned()));
        }

        #[ink::test]
        fn new_fail_when_symbol_is_invalid() {
            // Constructor works.
            assert!(ERC20::new("test".to_owned(), String::new())
                .is_err_and(|e| e == "SymbolCannotBeEmpty".to_owned()));
            assert!(ERC20::new("test".to_owned(), "Test".to_owned())
                .is_err_and(|e| e == "TheCharactersOfTheSymbolMustBeUpperCase".to_owned()));
            assert!(ERC20::new("test".to_owned(), "TESTTESTT".to_owned())
                .is_err_and(|e| e == "SymbolMustBeLessThanOrEqualTo8".to_owned()));
        }

        /// The total supply was applied.
        #[ink::test]
        fn total_supply_works() {
            // Constructor works.
            let erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            // Get the token total supply.
            assert_eq!(erc20.total_supply(), 0);
        }

        /// Get the actual balance of an account.
        #[ink::test]
        fn balance_of_works() {
            // Constructor works

            let erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();

            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Alice owns all the tokens on contract instantiation
            assert_eq!(erc20.balance_of(accounts.alice), 0);
            // Bob does not owns tokens
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn mint_works() {
            // Constructor works.

            let mut erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            // Transfer event triggered during initial construction.
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.mint(accounts.alice, 100), Ok(()));

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            // Check first transfer event related to ERC-20 instantiation.
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        #[ink::test]
        fn burn_works() {
            // Constructor works.

            let mut erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            // Transfer event triggered during initial construction.
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let value = 100;
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.mint(accounts.alice, value), Ok(()));
            assert_eq!(erc20.total_supply(), value);
            assert_eq!(erc20.burn(value), Ok(()));
            assert_eq!(erc20.total_supply(), 0);
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                value,
            );
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                None,
                value,
            );
        }

        #[ink::test]
        fn transfer_works() {
            // Constructor works.

            let mut erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            // Transfer event triggered during initial construction.
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.mint(accounts.alice, 100), Ok(()));
            // Alice transfers 10 tokens to Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            // Bob owns 10 tokens.
            assert_eq!(erc20.balance_of(accounts.bob), 10);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // Check first transfer event related to the mint.
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

            let mut erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Set Bob as caller
            set_caller(accounts.bob);

            // Bob fails to transfers 10 tokens to Eve.
            assert_eq!(
                erc20.transfer(accounts.eve, 10),
                Err("InsufficientBalance".to_owned())
            );
            // Alice owns all the tokens.
            assert_eq!(erc20.balance_of(accounts.alice), 0);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);
        }

        #[ink::test]
        fn transfer_from_caller_is_from_works() {
            // Constructor works.

            let mut erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            // Transfer event triggered during initial construction.
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.mint(accounts.alice, 100), Ok(()));

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
            // Eve owns tokens.
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.

            let mut erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            // Transfer event triggered during initial construction.
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.mint(accounts.alice, 100), Ok(()));

            // Set Bob as caller.
            set_caller(accounts.bob);

            // Bob fails to transfer tokens owned by Alice.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Err("InsufficientAllowance".to_owned())
            );
            // Set Bob as caller.
            set_caller(accounts.alice);
            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // Set Bob as caller.
            set_caller(accounts.bob);
            // The approve event takes place.
            assert_eq!(ink::env::test::recorded_events().count(), 2);

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
            // Eve owns tokens.
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // The second event `emitted_events[1]` is an Approve event that we skip
            // checking.
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        #[ink::test]
        fn allowance_failed_when_spender_is_self() {
            let mut erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice approves Bob for token transfers on her behalf.
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert!(erc20
                .approve(accounts.alice, initial_allowance)
                .is_err_and(|e| e == "SpenderIsSelf".to_owned()));
        }

        #[ink::test]
        fn allowance_must_not_change_on_failed_transfer() {
            let mut erc20 = ERC20::new("test".to_owned(), "TEST".to_owned()).unwrap();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice approves Bob for token transfers on her behalf.
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

            // Set Bob as caller.
            set_caller(accounts.bob);

            // Bob tries to transfer tokens from Alice to Eve.
            let emitted_events_before = ink::env::test::recorded_events();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
                Err("InsufficientBalance".to_owned())
            );
            // Allowance must have stayed the same
            assert_eq!(
                erc20.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // No more events must have been emitted
            let emitted_events_after = ink::env::test::recorded_events();
            assert_eq!(emitted_events_before.count(), emitted_events_after.count());
        }

        fn set_caller(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;
