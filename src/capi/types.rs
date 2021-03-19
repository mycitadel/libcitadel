// Citadel C bindings library (libcitadel)
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.

use libc::{c_char};
use std::ptr;

use bitcoin::consensus::serialize;
use citadel::client::InvoiceType;
use citadel::rpc::message;
use lnpbp::bech32::ToBech32String;
use rgb::{Consignment, validation::{self, Validity}};
use wallet::descriptor;

use crate::TryIntoRaw;
use rgb::validation::{Info, Warning, Failure};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum descriptor_type {
    BARE,
    HASHED,
    SEGWIT,
    TAPROOT,
}

impl From<descriptor_type> for descriptor::ContentType {
    fn from(t: descriptor_type) -> Self {
        match t {
            descriptor_type::BARE => descriptor::ContentType::Bare,
            descriptor_type::HASHED => descriptor::ContentType::Hashed,
            descriptor_type::SEGWIT => descriptor::ContentType::SegWit,
            descriptor_type::TAPROOT => descriptor::ContentType::Taproot,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum invoice_type {
    ADDRESS_UTXO,
    DESCRIPTOR,
    PSBT,
}

impl From<invoice_type> for InvoiceType {
    fn from(t: invoice_type) -> Self {
        match t {
            invoice_type::ADDRESS_UTXO => InvoiceType::AddressUtxo,
            invoice_type::DESCRIPTOR => InvoiceType::Descriptor,
            invoice_type::PSBT => InvoiceType::Psbt,
        }
    }
}

// TODO: Provide memory release function for `prepared_transfer_t`
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct prepared_transfer_t {
    pub success: bool,
    pub consignment_bech32: *const c_char,
    pub psbt_base64: *const c_char,
}

impl prepared_transfer_t {
    pub fn failure() -> Self {
        prepared_transfer_t {
            success: false,
            consignment_bech32: ptr::null(),
            psbt_base64: ptr::null(),
        }
    }
}

impl From<message::PreparedTransfer> for prepared_transfer_t {
    fn from(p: message::PreparedTransfer) -> Self {
        prepared_transfer_t {
            success: true,
            consignment_bech32: p
                .consignment
                .as_ref()
                .map(Consignment::to_bech32_string)
                .and_then(String::try_into_raw)
                .unwrap_or(ptr::null()),
            psbt_base64: base64::encode(&serialize(&p.psbt))
                .try_into_raw()
                .expect("base64 PSBT representation contains zero byte"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum validity_t {
    UNABLE_TO_VALIDATE,
    VALID,
    UNRESOLVED_TX,
    INVALID,
}

impl From<Validity> for validity_t {
    fn from(validity: Validity) -> Self {
        match validity {
            Validity::Valid => Self::VALID,
            Validity::UnresolvedTransactions => Self::UNRESOLVED_TX,
            Validity::Invalid => Self::INVALID,
        }
    }
}

// TODO: Provide memory release function for `prepared_transfer_t`
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct validation_status_t {
    pub validity: validity_t,
    pub info_len: u32,
    pub info: *const *const c_char,
    pub warn_len: u32,
    pub warn: *const *const c_char,
    pub failures_len: u32,
    pub failures: *const *const c_char,
}

impl validation_status_t {
    pub fn failure() -> Self {
        validation_status_t {
            validity: validity_t::UNABLE_TO_VALIDATE,
            info_len: 0,
            info: ptr::null(),
            warn_len: 0,
            warn: ptr::null(),
            failures_len: 0,
            failures: ptr::null()
        }
    }
}

impl From<validation::Status> for validation_status_t {
    fn from(status: validation::Status) -> Self {

        let mut info = status.info
            .iter()
            .map(Info::to_string)
            .map(String::try_into_raw)
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        info.shrink_to_fit();
        let (info, info_len, _) = info.into_raw_parts();

        let mut warn = status.warnings
            .iter()
            .map(Warning::to_string)
            .map(String::try_into_raw)
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        warn.shrink_to_fit();
        let (warn, warn_len, _) = warn.into_raw_parts();

        let mut failures = status.failures
            .iter()
            .map(Failure::to_string)
            .map(String::try_into_raw)
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        failures.shrink_to_fit();
        let (failures, failures_len, _) = failures.into_raw_parts();

        validation_status_t {
            validity: status.validity().into(),
            info_len: info_len as u32,
            info,
            warn_len: warn_len as u32,
            warn,
            failures_len: failures_len as u32,
            failures
        }
    }
}
