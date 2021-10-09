#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_storage::{
        lazy::Lazy,
        collections::HashMap
    };
   // use ink_env::AccountId;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: HashMap<AccountId, Balance>,
        allowances: HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,

        #[ink(topic)]
        to: Option<AccountId>,

        #[ink(topic)]
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
   
    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut balances = ink_storage::collections::HashMap::new();
            let caller = Self::env().caller();
            balances.insert(caller, initial_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply
            });
            Self {
                total_supply: initial_supply,
                balances,
                allowances: HashMap::new()
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            // ACTION: Get the `self.env().caller()` and store it as the `owner`
            // ACTION: Insert the new allowance into the `allowances` HashMap
            //   HINT: The key tuple is `(owner, spender)`
            // ACTION: `emit` the `Approval` event you created using these values
            // ACTION: Return true if everything was successful
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner: owner,
                spender: spender,
                value: value
            });
            true
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            // ACTION: Create a getter for the `allowances` HashMap
            //   HINT: Take a look at the getters above if you forget the details
            // ACTION: Return the `allowance` value
            *(self.allowances.get(&(owner, spender)).unwrap_or(&0))
        }

        
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // ACTION: Get the allowance for `(from, self.env().caller())` using `allowance_of_or_zero`
            // ACTION: `if` the `allowance` is less than the `value`, exit early and return `false`
            // ACTION: `insert` the new allowance into the map for `(from, self.env().caller())`
            // ACTION: Finally, call the `transfer_from_to` for `from` and `to`
            // ACTION: Return true if everything was successful
            let allowance = *(self.allowances.get(&(from, self.env().caller())).unwrap_or(&0));
            if allowance < value {
                return false;
            }

            let transfer_result = self.transfer_from_to(from, to, value);
            // Check `transfer_result` because `from` account may not have enough balance
            //   and return false.
            if !transfer_result {
                return false
            }

            // Decrease the value of the allowance and transfer the tokens.
            self.allowances.insert((from, self.env().caller()), allowance - value);
            true
        }


        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            self.transfer_from_to(self.env().caller(), to, value)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let balance_of_from = self.balance_of_or_zero(&from);
            let balance_of_to = self.balance_of_or_zero(&to);
            
            if balance_of_from < value {
                return false;
            }
            self.balances.insert(from, balance_of_from - value);
            self.balances.insert(to, balance_of_to + value);
            Self::env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value: value
            });
            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *(self.balances.get(owner).unwrap_or(&0))
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;



        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut erc20 = Erc20::new(11);
            assert_eq!(erc20.total_supply(), 11);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            contract.approve(AccountId::from([0x1; 32]), 20);
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
        }

        #[ink::test]
        fn allowances_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            contract.approve(AccountId::from([0x1; 32]), 200);
            assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 200);

            assert!(contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 50));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
            assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 150);

            assert!(!contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 100));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 50);
            assert_eq!(contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x1; 32])), 150);
        }
    }
}
