use std::env;

#[tokio::test]
async fn functional_lws_daemon_test() {
    let (address, view_key, monero_lws) = setup_monero().await;

    monero_lws
        .login(address, view_key, true, true)
        .await
        .unwrap();

    monero_lws
        .import_request(address, view_key, Some(50))
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    monero_lws
        .get_address_info(address, view_key)
        .await
        .unwrap();

    monero_lws.get_address_txs(address, view_key).await.unwrap();

    monero_lws
        .get_random_outs(11, vec![monero::Amount::from_pico(1000000)])
        .await
        .unwrap();

    let outs = monero_lws
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
            assert_eq!("HTTP status client error (403 Forbidden) for url (http://localhost:8443/get_unspent_outs)", format!("{}", err));
        }
    };
}

async fn setup_monero() -> (
    monero::Address,
    monero::PrivateKey,
    monero_lws::LwsRpcClient,
) {
    let dhost = env::var("MONERO_DAEMON_HOST").unwrap_or_else(|_| "localhost".into());
    let daemon_client = monero_rpc::RpcClient::new(format!("http://{}:18081", dhost));
    let daemon = daemon_client.daemon();
    let regtest = daemon.regtest();
    let whost = env::var("MONERO_WALLET_HOST_1").unwrap_or_else(|_| "localhost".into());
    let wallet_client = monero_rpc::RpcClient::new(format!("http://{}:18083", whost));
    let wallet = wallet_client.wallet();
    wallet
        .create_wallet("wallet_name".to_string(), None, "English".to_string())
        .await
        .ok();
    wallet
        .open_wallet("wallet_name".to_string(), None)
        .await
        .ok();
    let address = wallet.get_address(0, None).await.unwrap().address;
    let viewkey = wallet
        .query_key(monero_rpc::PrivateKeyType::View)
        .await
        .unwrap();

    regtest.generate_blocks(100, address).await.unwrap();
    let dhost = env::var("MONERO_DAEMON_HOST").unwrap_or_else(|_| "localhost".into());
    let lws_client = monero_lws::LwsRpcClient::new(format!("http://{}:8443", dhost));
    (address, viewkey, lws_client)
}
