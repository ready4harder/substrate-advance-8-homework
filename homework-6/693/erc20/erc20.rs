use ink::storage::Mapping;


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
    owner: AccountId,
    #[ink(topic)]
    spender: AccountId,
    value: Balance,
}

#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {
    InsufficientBalance,
    InsufficientAllowance,
}

pub type Result<T> = core::result::Result<T, Error>;

impl Erc20 {
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

    #[ink(message)]
    pub fn total_supply(&self) -> Balance {
        self.total_supply
    }
    #[ink(message)]
    pub fn balance_of(&self, owner: AccountId) -> Balance {
        self.balance_of_impl(&owner)
    }

    #[inline]
    fn balance_of_impl(&self, owner: &AccountId) -> Balance {
        self.balances.get(owner).unwrap_or_default()
    }

    #[ink(message)]
    pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self.allowance_impl(&owner, &spender)
    }
    #[inline]
    fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
        self.allowances.get((owner, spender)).unwrap_or_default()
    }

    #[ink(message)]
    pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
        let from = self.env().caller();
        self.transfer_from_to(&from, &to, value)
    }

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
