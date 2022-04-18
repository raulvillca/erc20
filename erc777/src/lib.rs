//! A library for developing ERC20 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC20`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.
//!
//! To create an example ERC20 contract which uses this library, use the cargo-casper tool:
//!
//! ```bash
//! cargo install cargo-casper
//! cargo casper --erc20 <PATH TO NEW PROJECT>
//! ```

#![warn(missing_docs)]
#![no_std]

extern crate alloc;

mod address;
mod default_operators;
mod balances;
pub mod constants;
mod detail;
pub mod entry_points;
mod error;
mod total_supply;

use alloc::vec::Vec;
use crate::address::Address;
use alloc::string::{String, ToString};

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, EntryPoints, Key, URef, U256};

use constants::{
    BALANCES_KEY_NAME, DECIMALS_KEY_NAME, ERC777_TOKEN_CONTRACT_KEY_NAME,
    NAME_KEY_NAME, SYMBOL_KEY_NAME, TOTAL_SUPPLY_KEY_NAME, GRANULARITY_KEY_NAME, DEFAULT_OPERATORS_KEY_NAME
};
pub use error::Error;

/// Implementation of ERC20 standard functionality.
#[derive(Default)]
pub struct ERC777 {
    name_uref: OnceCell<URef>,
    symbol_uref: OnceCell<URef>,
    balances_uref: OnceCell<URef>,
    total_supply_uref: OnceCell<URef>,
    default_operators_uref: OnceCell<URef>,
}

impl ERC777 {
    fn new(
        name_uref: URef,
        symbol_uref: URef, 
        balances_uref: URef,
        total_supply_uref: URef,  
        default_operators_uref: URef
    ) -> Self {
        Self {
            name_uref: name_uref.into(),
            symbol_uref: symbol_uref.into(),
            balances_uref: balances_uref.into(),
            total_supply_uref: total_supply_uref.into(),
            default_operators_uref: default_operators_uref.into()
        }
    }

    fn balances_uref(&self) -> URef {
        *self.balances_uref.get_or_init(balances::get_balances_uref)
    }

    fn read_balance(&self, owner: Address) -> U256 {
        balances::read_balance_from(self.balances_uref(), owner)
    }

    fn write_balance(&mut self, owner: Address, amount: U256) {
        balances::write_balance_to(self.balances_uref(), owner, amount)
    }

    fn total_supply_uref(&self) -> URef {
        *self
            .total_supply_uref
            .get_or_init(total_supply::total_supply_uref)
    }

    fn read_total_supply(&self) -> U256 {
        total_supply::read_total_supply_from(self.total_supply_uref())
    }

    fn write_total_supply(&self, total_supply: U256) {
        total_supply::write_total_supply_to(self.total_supply_uref(), total_supply)
    }

    fn transfer_balance(
        &mut self,
        sender: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        balances::transfer_balance(self.balances_uref(), sender, recipient, amount)
    }

    fn default_operators_uref(&self) -> URef {
        *self.default_operators_uref.get_or_init(default_operators::default_operators_uref)
    }

    /// Installs the ERC20 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install(
        name: String,
        symbol: String,
        decimals: u8,
        granularity: u8,
        initial_supply: U256,
    ) -> Result<ERC777, Error> {
        let default_entry_points = entry_points::default();
        ERC777::install_custom(
            name,
            symbol,
            decimals,
            granularity,
            initial_supply,
            ERC777_TOKEN_CONTRACT_KEY_NAME,
            default_entry_points,
        )
    }

    /// Returns the name of the token.
    pub fn name(&self) -> String {
        detail::read_from(NAME_KEY_NAME)
    }

    /// Returns the symbol of the token.
    pub fn symbol(&self) -> String {
        detail::read_from(SYMBOL_KEY_NAME)
    }

    /// Returns the decimals of the token.
    pub fn decimals(&self) -> u8 {
        detail::read_from(DECIMALS_KEY_NAME)
    }

    /// Returns the total supply of the token.
    pub fn total_supply(&self) -> U256 {
        self.read_total_supply()
    }

    pub fn granularity(&self) -> u8 {
        detail::read_from(GRANULARITY_KEY_NAME)
    }

    /// Returns the balance of `owner`.
    pub fn balance_of(&self, owner: Address) -> U256 {
        balances::read_balance_from(self.balances_uref(), owner)
    }

    pub fn is_operator_for(&self, operator: Address, token_holder: Address) -> bool {
        default_operators::validate_token_for_operator(self.default_operators_uref(), operator, token_holder)
    }

    pub fn authorize_operator(&self, operator: Address) -> Result<(), Error> {
        //default_operators::write_default_operators_to(operator);
        Ok(())
    }

    pub fn revoke_operator(&self, operator: Address) -> Result<(), Error> {
        //default_operators::write_default_operators_to(operator);
        Ok(())
    }

    pub fn default_operators(&self) -> Vec<u8> {
        default_operators::read_default_operators_from(self.default_operators_uref())
    }

    pub fn operator_send(&self, sender: Address, recepient: Address, amount: U256, data: &str, operator_data: &str) -> Result<(), Error> {
        balances::write_balance_to(self.balances_uref(), recepient, amount);
        Ok(())
    }

    pub fn operator_burn(&self, account: Address, amount: U256, data: &str, operator_data: &str) -> Result<(), Error> {
        balances::write_balance_to(self.balances_uref(), account, amount);
        Ok(())
    }

    /// Transfers `amount` of tokens from the direct caller to `recipient`.
    pub fn send(&mut self, recipient: Address, amount: U256) -> Result<(), Error> {
        let sender = detail::get_immediate_caller_address()?;
        self.transfer_balance(sender, recipient, amount)
    }

    /// Mints `amount` new tokens and adds them to `owner`'s balance and to the token total supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn mint(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
        let new_balance = {
            let balance = self.read_balance(owner);
            balance.checked_add(amount).ok_or(Error::Overflow)?
        };
        let new_total_supply = {
            let total_supply: U256 = self.read_total_supply();
            total_supply.checked_add(amount).ok_or(Error::Overflow)?
        };
        self.write_balance(owner, new_balance);
        self.write_total_supply(new_total_supply);
        Ok(())
    }

    /// Burns (i.e. subtracts) `amount` of tokens from `owner`'s balance and from the token total
    /// supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn burn(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
        let new_balance = {
            let balance = self.read_balance(owner);
            balance
                .checked_sub(amount)
                .ok_or(Error::InsufficientBalance)?
        };
        let new_total_supply = {
            let total_supply = self.read_total_supply();
            total_supply.checked_sub(amount).ok_or(Error::Overflow)?
        };
        self.write_balance(owner, new_balance);
        self.write_total_supply(new_total_supply);
        Ok(())
    }

    /// Installs the ERC20 contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC20::install`] instead, as it will create the default set
    /// of ERC20 entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        name: String,
        symbol: String,
        decimals: u8,
        granularity: u8,
        initial_supply: U256,
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC777, Error> {
        let balances_uref = storage::new_dictionary(BALANCES_KEY_NAME).unwrap_or_revert();
        let default_operators_uref = storage::new_dictionary(DEFAULT_OPERATORS_KEY_NAME).unwrap_or_revert();
        // We need to hold on a RW access rights because tokens can be minted or burned.
        let total_supply_uref = storage::new_uref(initial_supply).into_read_write();

        let mut named_keys = NamedKeys::new();

        let name_uref = storage::new_uref(name).into_read();
        let name_key = {
            Key::from(name_uref)
        };

        let symbol_uref = storage::new_uref(symbol).into_read();
        let symbol_key = {
            Key::from(symbol_uref)
        };    

        let decimals_key = {
            let decimals_uref = storage::new_uref(decimals).into_read();
            Key::from(decimals_uref)
        };

        let granularity_key = {
            let granularity_uref = storage::new_uref(granularity).into_read();
            Key::from(granularity_uref)
        };

        let total_supply_key = Key::from(total_supply_uref);

        let balances_dictionary_key = {
            // Sets up initial balance for the caller - either an account, or a contract.
            let caller = detail::get_caller_address()?;
            balances::write_balance_to(balances_uref, caller, initial_supply);

            runtime::remove_key(BALANCES_KEY_NAME);

            Key::from(balances_uref)
        };

        let default_operators_key = {
            // Sets up initial operators for the caller - either an account, or a contract.
            let caller = detail::get_caller_address()?;
            default_operators::write_default_operators_to(default_operators_uref, caller);

            runtime::remove_key(DEFAULT_OPERATORS_KEY_NAME);

            Key::from(default_operators_uref)
        };

        named_keys.insert(NAME_KEY_NAME.to_string(), name_key);
        named_keys.insert(SYMBOL_KEY_NAME.to_string(), symbol_key);
        named_keys.insert(DECIMALS_KEY_NAME.to_string(), decimals_key);
        named_keys.insert(GRANULARITY_KEY_NAME.to_string(), granularity_key);
        named_keys.insert(BALANCES_KEY_NAME.to_string(), balances_dictionary_key);
        named_keys.insert(TOTAL_SUPPLY_KEY_NAME.to_string(), total_supply_key);
        named_keys.insert(DEFAULT_OPERATORS_KEY_NAME.to_string(), default_operators_key);

        let (contract_hash, _version) =
            storage::new_locked_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC777::new(
            name_uref,
            symbol_uref,
            balances_uref,
            total_supply_uref,
            default_operators_uref
        ))
    }
}
