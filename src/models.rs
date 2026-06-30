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

use crate::util::*;
use monero::{cryptonote::hash::Hash as CryptoNoteHash, util::address::PaymentId};
use serde::{Deserialize, Serialize};

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
    pub spent_outputs: Vec<SpendObject>,
    pub payment_id: Option<HashString<PaymentId>>,
    pub coinbase: u8,
    pub mempool: u8,
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
    pub new_request: u8,
    pub request_fulfilled: u8,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub new_address: u8,
    pub generated_locally: u8,
    pub start_height: Option<u64>,
}
