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

//! Monero daemon and wallet RPC.

// Coding conventions
#![forbid(unsafe_code)]

#[macro_use]
mod util;
mod models;

pub use self::{models::*, util::*};

use jsonrpc_core::types::*;

use serde::Deserialize;
use serde_json::Value;
use std::{
    fmt::Debug,
    iter::{empty, once},
    sync::Arc,
};

enum RpcParams {
    Map(Box<dyn Iterator<Item = (String, Value)> + Send + 'static>),
}

impl RpcParams {
    fn map<M>(v: M) -> Self
    where
        M: Iterator<Item = (&'static str, Value)> + Send + 'static,
    {
        RpcParams::Map(Box::new(v.map(|(k, v)| (k.to_string(), v))))
    }
}

impl From<RpcParams> for Params {
    fn from(value: RpcParams) -> Self {
        match value {
            RpcParams::Map(v) => Params::Map(v.collect()),
        }
    }
}

#[derive(Clone, Debug)]
struct RemoteCaller {
    http_client: reqwest::Client,
    addr: String,
}

impl RemoteCaller {
    async fn daemon_rpc_call<T>(&self, method: &'static str, params: RpcParams) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de> + Send + 'static + Debug,
    {
        let client = self.http_client.clone();
        let uri = format!("{}/{}", &self.addr, method);
        let json_params: jsonrpc_core::types::params::Params = params.into();
        let rsp = client.post(uri).json(&json_params).send().await?;
        if rsp.status() != 200 {
            rsp.error_for_status()?;
            panic!("should never reach here");
        }
        let rsp = rsp.json::<T>().await?;
        Ok(rsp)
    }
}

#[derive(Clone, Debug)]
struct CallerWrapper(Arc<RemoteCaller>);

impl CallerWrapper {
    async fn request<T>(&self, method: &'static str, params: RpcParams) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de> + Send + 'static + Debug,
    {
        let c = self.0.daemon_rpc_call(method, params).await?;
        Ok(serde_json::from_value(c)?)
    }
}

/// Base RPC client. It is useless on its own, please see the attached methods instead.
#[derive(Clone, Debug)]
pub struct LwsRpcClient {
    inner: CallerWrapper,
}

impl LwsRpcClient {
    pub fn new(addr: String, proxy: Option<String>) -> Self {
        if let Some(proxy_address) = proxy {
            Self {
                inner: CallerWrapper(Arc::new(RemoteCaller {
                    http_client: reqwest::Client::builder()
                        .proxy(reqwest::Proxy::all(proxy_address).unwrap())
                        .build()
                        .unwrap(),
                    addr,
                })),
            }
        } else {
            Self {
                inner: CallerWrapper(Arc::new(RemoteCaller {
                    http_client: reqwest::ClientBuilder::new().build().unwrap(),
                    addr,
                })),
            }
        }
    }

    pub async fn get_address_info(
        &self,
        address: monero::Address,
        view_key: monero::PrivateKey,
    ) -> anyhow::Result<AddressInfo> {
        let params = empty()
            .chain(once(("address", address.to_string().into())))
            .chain(once(("view_key", view_key.to_string().into())));
        self.inner
            .request("get_address_info", RpcParams::map(params))
            .await
    }

    pub async fn get_address_txs(
        &self,
        address: monero::Address,
        view_key: monero::PrivateKey,
    ) -> anyhow::Result<AddressTxs> {
        let params = empty()
            .chain(once(("address", address.to_string().into())))
            .chain(once(("view_key", view_key.to_string().into())));
        self.inner
            .request("get_address_txs", RpcParams::map(params))
            .await
    }

    pub async fn get_random_outs(
        &self,
        count: u32,
        amounts: Vec<monero::Amount>,
    ) -> anyhow::Result<AmountOuts> {
        let params = empty().chain(once(("count", count.into()))).chain(once((
            "amounts",
            amounts
                .into_iter()
                .map(|s| s.as_pico().to_string())
                .collect::<Vec<_>>()
                .into(),
        )));

        self.inner
            .request("get_random_outs", RpcParams::map(params))
            .await
    }

    pub async fn get_unspent_outs(
        &self,
        address: monero::Address,
        view_key: monero::PrivateKey,
        amount: monero::Amount,
        mixin: u32,
        use_dust: bool,
        dust_threshold: monero::Amount,
    ) -> anyhow::Result<UnspentOuts> {
        let params = empty()
            .chain(once(("address", address.to_string().into())))
            .chain(once(("view_key", view_key.to_string().into())))
            .chain(once(("amount", amount.as_pico().to_string().into())))
            .chain(once(("mixin", mixin.into())))
            .chain(once(("use_dust", use_dust.into())))
            .chain(once((
                "dust_threshold",
                dust_threshold.as_pico().to_string().into(),
            )));
        self.inner
            .request("get_unspent_outs", RpcParams::map(params))
            .await
    }

    pub async fn import_request(
        &self,
        address: monero::Address,
        view_key: monero::PrivateKey,
        from_height: Option<u64>,
    ) -> anyhow::Result<ImportResponse> {
        let params = empty()
            .chain(once(("address", address.to_string().into())))
            .chain(once(("view_key", view_key.to_string().into())))
            .chain(from_height.map(|v| ("from_height", v.into())));

        self.inner
            .request("import_wallet_request", RpcParams::map(params))
            .await
    }

    pub async fn login(
        &self,
        address: monero::Address,
        view_key: monero::PrivateKey,
        create_account: bool,
        generated_locally: bool,
    ) -> anyhow::Result<LoginResponse> {
        let params = empty()
            .chain(once(("address", address.to_string().into())))
            .chain(once(("view_key", view_key.to_string().into())))
            .chain(once(("create_account", create_account.into())))
            .chain(once(("generated_locally", generated_locally.into())));
        self.inner.request("login", RpcParams::map(params)).await
    }
}
