//! Implementation of total supply.


use crate::Address;
use alloc::vec::Vec;
use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef, U256};

use crate::{constants::DEFAULT_OPERATORS_KEY_NAME, detail};

#[inline]
pub(crate) fn default_operators_uref() -> URef {
    detail::get_uref(DEFAULT_OPERATORS_KEY_NAME)
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn read_default_operators_from(uref: URef) -> Vec<u8> {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn validate_token_for_operator(uref: URef, operator: Address, token_holder: Address) -> bool {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_default_operators_to(uref: URef, operator: Address) {
    storage::write(uref, operator);
}
