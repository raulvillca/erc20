#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::string::String;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc777::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
        NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME, TOTAL_SUPPLY_RUNTIME_ARG_NAME,
        OPERATOR_RUNTIME_ARG_NAME, GRANULARITY_RUNTIME_ARG_NAME, TOKEN_HOLDER_RUNTIME_ARG_NAME
    },
    Address, ERC777,
};
use casper_types::{CLValue, U256};

#[no_mangle]
pub extern "C" fn name() {
    let name = ERC777::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let symbol = ERC777::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    let decimals = ERC777::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn granularity() {
    let granularity = ERC777::default().granularity();
    runtime::ret(CLValue::from_t(granularity).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let total_supply = ERC777::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC777::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn send() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC777::default()
        .send(recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn is_operator_for() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let token: Address = runtime::get_named_arg(TOKEN_HOLDER_RUNTIME_ARG_NAME);

    runtime::ret(CLValue::from_t(default_operators, token).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn authorize_operator() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);

    ERC777::default().authorize_operator(operator).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn revoke_operator() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);

    ERC777::default().approve(operator).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn default_operators() {
    let default_operators = ERC777::default().default_operators();
    runtime::ret(CLValue::from_t(default_operators).unwrap_or_revert());
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
    let granularity = runtime::get_named_arg(GRANULARITY_RUNTIME_ARG_NAME);
    let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);

    let _token = ERC777::install(name, symbol, decimals, granularity, total_supply).unwrap_or_revert();
}
