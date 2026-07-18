// Rust Monero Light Wallet Server RPC Client
// Written in 2021-2022 by
//   Sebastian Kung <seb.kung@gmail.com>
//   Monero Rust Contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
#![allow(unexpected_cfgs)]

use std::fmt;

use crate::util::*;
use monero::{cryptonote::hash::Hash as CryptoNoteHash, util::address::PaymentId};
use serde::{
    de::{Error as DeserializerError, Visitor},
    Deserialize, Deserializer, Serialize,
};

macro_rules! hash_type {
    ($name:ident, $len:expr) => {
        ::fixed_hash::construct_fixed_hash! {
            #[derive(::serde::Serialize, ::serde::Deserialize)]
            pub struct $name($len);
        }
        hash_type_impl!($name);
    };
}

hash_type!(BlockHash, 32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    OK,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum MoneroResult<T> {
    OK(T),
}

impl<T> MoneroResult<T> {
    pub fn into_inner(self) -> T {
        match self {
            MoneroResult::OK(v) => v,
        }
    }
}

// Compatibility with version 0.1
fn number_or_boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("integer or boolean")
        }

        fn visit_bool<E: DeserializerError>(self, value: bool) -> Result<bool, E> {
            Ok(value)
        }

        fn visit_u64<E: DeserializerError>(self, value: u64) -> Result<bool, E> {
            let boolean = match value {
                0 => false,
                1 => true,
                _ => {
                    return Err(E::invalid_value(
                        serde::de::Unexpected::Unsigned(value),
                        &self,
                    ))
                }
            };
            Ok(boolean)
        }
    }

    deserializer.deserialize_any(BoolVisitor)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressInfo {
    pub locked_funds: String,
    pub total_received: String,
    pub total_sent: String,
    pub scanned_height: u64,
    pub scanned_block_height: u64,
    pub start_height: u64,
    pub transaction_height: u64,
    pub blockchain_height: u64,
    pub spent_outputs: Vec<SpendObject>,
    pub rates: Option<Rates>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Rates {
    pub AUD: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpendObject {
    pub amount: String,
    pub key_image: HashString<CryptoNoteHash>,
    pub tx_pub_key: HashString<CryptoNoteHash>,
    pub out_index: u16,
    pub mixin: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressTxs {
    pub total_received: String,
    pub scanned_height: u64,
    pub scanned_block_height: u64,
    pub start_height: u64,
    pub blockchain_height: u64,
    // May not be present in version 0.3
    #[serde(default)]
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub hash: HashString<CryptoNoteHash>,
    pub timestamp: String,
    pub total_received: String,
    pub total_sent: String,
    pub unlock_time: u64,
    pub height: Option<u64>,
    // May not be present in version 0.3
    #[serde(default)]
    pub spent_outputs: Vec<SpendObject>,
    pub payment_id: Option<HashString<PaymentId>>,
    #[serde(deserialize_with = "number_or_boolean")]
    pub coinbase: bool,
    #[serde(deserialize_with = "number_or_boolean")]
    pub mempool: bool,
    pub mixin: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmountOuts {
    pub amount_outs: Vec<RandomOutput>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RandomOutputs {
    pub amount: String,
    pub outputs: Vec<RandomOutput>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RandomOutput {
    pub global_index: u64,
    pub public_key: HashString<CryptoNoteHash>,
    pub rct: HashString<CryptoNoteHash>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnspentOuts {
    pub per_kb_fee: u64,
    pub fee_mask: u64,
    pub amount: String,
    pub outputs: Vec<Output>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub tx_id: u64,
    pub amount: String,
    pub index: u16,
    pub global_index: u64,
    pub rct: String,
    pub tx_hash: HashString<CryptoNoteHash>,
    pub tx_prefix_hash: String,
    pub public_key: HashString<CryptoNoteHash>,
    pub tx_pub_key: HashString<CryptoNoteHash>,
    pub spend_key_images: Vec<HashString<CryptoNoteHash>>,
    pub timestamp: String,
    pub height: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportResponse {
    pub payment_address: Option<monero::Address>,
    pub payment_id: Option<HashString<PaymentId>>,
    pub import_fee: Option<String>,
    #[serde(deserialize_with = "number_or_boolean")]
    pub new_request: bool,
    #[serde(deserialize_with = "number_or_boolean")]
    pub request_fulfilled: bool,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(deserialize_with = "number_or_boolean")]
    pub new_address: bool,
    #[serde(deserialize_with = "number_or_boolean")]
    pub generated_locally: bool,
    pub start_height: Option<u64>,
}
