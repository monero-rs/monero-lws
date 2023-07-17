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

use rand::{distributions::Alphanumeric, Rng};
use std::env; // 0.8

#[tokio::test]
async fn functional_lws_daemon_test() {
    let (address, view_key, monero_lws_client, regtest) = setup_monero().await;

    let blocks = regtest.generate_blocks(100, address).await.unwrap().height;

    monero_lws_client
        .login(address, view_key, true, true)
        .await
        .unwrap();

    monero_lws_client
        .import_request(address, view_key, Some(blocks - 1))
        .await
        .unwrap();

    regtest.generate_blocks(1, address).await.unwrap();

    monero_lws_client
        .get_address_info(address, view_key)
        .await
        .unwrap();

    monero_lws_client
        .get_address_txs(address, view_key)
        .await
        .unwrap();

    monero_lws_client
        .get_random_outs(11, vec![monero::Amount::from_pico(1000000)])
        .await
        .unwrap();

    let outs = monero_lws_client
        .get_unspent_outs(
            address,
            view_key,
            monero::Amount::from_pico(1000),
            10,
            true,
            monero::Amount::from_pico(100),
        )
        .await;
    match outs {
        Ok(_) => {}
        Err(err) => {
            assert_eq!("HTTP status client error (403 Forbidden) for url (http://localhost:38884/get_unspent_outs)", format!("{}", err));
        }
    };
}

async fn setup_monero() -> (
    monero::Address,
    monero::PrivateKey,
    monero_lws::LwsRpcClient,
    monero_rpc::RegtestDaemonJsonRpcClient,
) {
    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let dhost = env::var("MONERO_DAEMON_HOST").unwrap_or_else(|_| "localhost".into());
    let daemon_client = monero_rpc::RpcClient::new(format!("http://{}:18081", dhost));
    let daemon = daemon_client.daemon();
    let regtest = daemon.regtest();
    let whost = env::var("MONERO_WALLET_HOST_1").unwrap_or_else(|_| "localhost".into());
    let wallet_client = monero_rpc::RpcClient::new(format!("http://{}:18083", whost));
    let wallet = wallet_client.wallet();
    wallet
        .create_wallet(wallet_name.clone(), None, "English".to_string())
        .await
        .ok();
    wallet.open_wallet(wallet_name, None).await.ok();
    let address = wallet.get_address(0, None).await.unwrap().address;
    let viewkey = wallet
        .query_key(monero_rpc::PrivateKeyType::View)
        .await
        .unwrap();

    regtest.generate_blocks(100, address).await.unwrap();
    let dhost = env::var("MONERO_DAEMON_HOST").unwrap_or_else(|_| "localhost".into());
    let lws_client = monero_lws::LwsRpcClient::new(format!("http://{}:38884", dhost), None);
    (address, viewkey, lws_client, regtest)
}
