macro_rules! b {
    ($e:expr) => {
        // tokio_test::block_on($e)
        $e
    };
}

use chrono::{DateTime, Duration, Utc};
use dydx_v3_blocking::constants::*;
use dydx_v3_blocking::helper::*;
use dydx_v3_blocking::types::*;
use dydx_v3_blocking::ClientOptions;
use dydx_v3_blocking::DydxClient;
use speculate::speculate;

#[cfg(test)]
speculate! {
        describe "privateTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let api_key = ApiKeyCredentials {
                                // test account
                                key: "6761e340-7c01-065e-d3e4-8338bfa4f0b7",
                                secret: "kaWlSJiFfIyIa0kPkGepTwVhtWzVxmvvXMezzRw2",
                                passphrase: "-VlJxCva5OhyhQEXWtFy"
                        };
                        let options = ClientOptions {
                                network_id: Some(TESTNET_NETWORK_ID),
                                api_timeout: None,
                                api_key_credentials: Some(api_key),
                                stark_private_key: Some(TEST_STARK_PRIVATE_KEY),
                                eth_private_key: None
                        };
                        DydxClient::new(TESTNET_API_URL, options)
                    }

                fn create_test_order() -> OrderResponse {
                        let datetime_now: DateTime<Utc> = Utc::now();
                        let expiration = datetime_now + Duration::minutes(3);
                        let expiration_unix = expiration.timestamp();

                        let order_params = ApiOrderParams {
                                position_id: POSITION_ID,
                                market: DydxMarket::BTC_USD,
                                side: OrderSide::BUY,
                                type_field: OrderType::MARKET,
                                time_in_force: TimeInForce::FOK,
                                post_only: false,
                                size: "0.01",
                                price: "100000",
                                limit_fee: "0.1",
                                client_id: None,
                                cancel_id: None,
                                trigger_price: None,
                                trailing_percent: None,
                                expiration: expiration_unix,
                        };
                        let order = DydxClient().private.unwrap().create_order(order_params).unwrap();
                        order
                }

                it "getRegistration" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_registration();
                                // match _response {
                                //         Ok(v) => dbg!(v.to_string()),
                                //         Err(e) => dbg!(e.to_string())
                                // }
                        });
                }

                it "getAccountId" {
                        b!({
                                let _uuid = get_account_id(TEST_ADDRESS);
                                // dbg!(_uuid);
                        });
                }

                it "getApiKeys" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_api_keys().unwrap();
                                // dbg!(_response);
                        });
                }

                it "getUser" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_user().unwrap();
                                // dbg!(_response);
                        });
                }

                it "getAccount" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_account(TEST_ADDRESS).unwrap();
                                dbg!(_response);
                        });
                }

                it "getAccountUnauthorized" {
                        b!({
                                fn DydxClientNonAuth() -> DydxClient<'static> {
                                        let api_key = ApiKeyCredentials {
                                                // account2 testnet
                                                key: "ed85a071-c6b4-b4f1-c965-efb238d16c5e",
                                                secret: "1iDz27dyq4RspTkP-rfTcFN6ouxTgHmTT_sKJogU",
                                                passphrase: "CfbXaq6O-Yd3jKOqh10a"
                                        };
                                        let options = ClientOptions {
                                                network_id: Some(3),
                                                api_timeout: None,
                                                api_key_credentials: Some(api_key),
                                                stark_private_key: Some("0657eaa201ba872f72c0e6e2db278d8cda1b60de4313f02213aaf2b3421bff56"),
                                                eth_private_key: None
                                        };
                                        // DydxClient::new("https://api.dydx.exchange", Some(options))
                                        DydxClient::new("https://api.stage.dydx.exchange", options)
                                    }

                                let _response = DydxClientNonAuth().private.unwrap().get_accounts();
                                // match _response {
                                //         Ok(v) => println!("{:?}", v),
                                //         Err(e) => println!("{:?}", e),
                                //     }
                                // dbg!(_response);
                        });
                }

                it "getAccounts" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_accounts().unwrap();
                                // dbg!(&_response);
                        });
                }

                it "getPositionsWithNoParameters" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_positions(None, None, None, None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getPositions" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_positions(Some(DydxMarket::BTC_USD), None, None, Some("2022-04-01T02:43:02.946Z")).unwrap();
                                // dbg!(_response);
                        });
                }

                it "createOrder" {
                        b!({
                                let _order = create_test_order();
                                // dbg!(_order);
                        });
                }

                it "getTransfers" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_transfers("WITHDRAWAL", None, None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "createWithdraw" {
                        b!({
                                let datetime_now: DateTime<Utc> = Utc::now();
                                let expiration = datetime_now + Duration::days(8);
                                let expiration_unix = expiration.timestamp();

                                let withdraw_params = ApiWithdrawParams {
                                        position_id: POSITION_ID,
                                        amount: "3",
                                        asset: "USDC",
                                        expiration: expiration_unix,
                                };
                                let _response = DydxClient().private.unwrap().create_withdraw(withdraw_params).unwrap();
                                // dbg!(_response);
                        });
                }

                it "createFastWithdraw" {
                        b!({
                                let datetime_now: DateTime<Utc> = Utc::now();
                                let expiration = datetime_now + Duration::days(8);
                                let expiration_unix = expiration.timestamp();

                                let withdraw_params = ApiFastWithdrawalParams {
                                        position_id: POSITION_ID,
                                        credit_asset: "USDC",
                                        credit_amount: "10",
                                        debit_amount: "11",
                                        to_address: TEST_ADDRESS,
                                        lp_position_id: "2",
                                        lp_stark_key: "04a9ecd28a67407c3cff8937f329ca24fd631b1d9ca2b9f2df47c7ebf72bf0b0",
                                        expiration: expiration_unix,
                                };
                                let _response = DydxClient().private.unwrap().create_fast_withdraw(withdraw_params);
                                // dbg!(_response);
                        });
                }


                it "createTransfer" {
                        b!({
                                let datetime_now: DateTime<Utc> = Utc::now();
                                let expiration = datetime_now + Duration::days(10);
                                let expiration_unix = expiration.timestamp();

                                let transfer_params = TransferParams {
                                        amount: "1",
                                        position_id: POSITION_ID,
                                        // send to test account 1
                                        receiver_account_id: "0192dc2d-a344-5dc1-ba37-1c4af5259a61",
                                        receiver_public_key: "05d319b5e52ad99ff8dd8bbc45bf1c71abf66d9d6d5ea7dd886d4bfcba7fe61f",
                                        receiver_position_id: "227",
                                        expiration: expiration_unix,
                                };
                                let _response = DydxClient().private.unwrap().create_transfer(transfer_params).unwrap();
                                // dbg!(_response);
                        });
                }

                it "updateUser" {
                        b!({
                                let userData = UserParams {
                                        email: Some("eaese@example.com"),
                                        user_data: "{}",
                                        username: None,
                                        is_sharing_username: None,
                                        is_sharing_address: Some(true),
                                        country: None

                                };
                                let _user = DydxClient().private.unwrap().update_user(userData);
                                // dbg!(_user);
                        });
                }

                it "getAccountLeaderboardPnl" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_account_leaderboard_pnl("DAILY", None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getAccountHistoricalLeaderboardPnl" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_historical_leaderboard_pnls("DAILY", None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "createAccount" {
                        b!({
                                let test_stark_public_key = "0084868a931891833df41ff83980f79889a045a46bbd273ec60ff402d7a1293f";
                                let test_stark_public_key_y_coordinate = "024b4de622d8acc9198800f5607c4a25968425fe268afe0272dbd3d5e119407d";
                                let _response = DydxClient().private.unwrap().create_account(test_stark_public_key, test_stark_public_key_y_coordinate).unwrap();
                                // dbg!(_response);
                        });
                }

                it "cancelOrder" {
                        b!({
                                let datetime_now: DateTime<Utc> = Utc::now();
                                let expiration = datetime_now + Duration::minutes(3);
                                let expiration_unix = expiration.timestamp();

                                let order_params = ApiOrderParams {
                                        position_id: POSITION_ID,
                                        market: DydxMarket::BTC_USD,
                                        side: OrderSide::SELL,
                                        type_field: OrderType::LIMIT,
                                        time_in_force: TimeInForce::GTT,
                                        post_only: false,
                                        size: "0.01",
                                        price: "100000",
                                        limit_fee: "0.1",
                                        client_id: None,
                                        cancel_id: None,
                                        trigger_price: None,
                                        trailing_percent: None,
                                        expiration: expiration_unix,
                                };
                                let order = DydxClient().private.unwrap().create_order(order_params).unwrap();

                                let _response = DydxClient().private.unwrap().cancel_order(order.order.id.as_str()).unwrap();
                                // dbg!(_response);
                        });
                }

                it "cancelOrders" {
                        b!({
                                let _response = DydxClient().private.unwrap().cancel_all_orders(Some(DydxMarket::BTC_USD)).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getOrders" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_orders(Some(DydxMarket::BTC_USD), None, None, None, None, None,None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getActiveOrders" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_active_orders(DydxMarket::BTC_USD, Some("SELL"), None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getOrderById" {
                        b!({
                                let order = create_test_order();
                                let _response = DydxClient().private.unwrap().get_order_by_id(&order.order.id).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getOrderByClientId" {
                        b!({
                                let order = create_test_order();
                                let _response = DydxClient().private.unwrap().get_order_by_client_id(&order.order.client_id).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getFills" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_fills(None, None, Some("3"), None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getFundingPayments" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_funding_payments(None, Some("2"), None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getHistoricalPnl" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_historical_pnl(None, None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getTradingRewards" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_trading_rewards(None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getRetroactiveMiningRewards" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_retroactive_mining_rewards().unwrap();
                                // dbg!(_response);
                        });
                }

                it "getLiquidityProviderRewards" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_liquidity_provider_rewards(None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "requestTestnetTokens" {
                        b!({
                                let _response = DydxClient().private.unwrap().request_testnet_tokens();
                                // dbg!(_response);
                        });
                }


                it "sendVerificationEmail" {
                        b!({
                                let _response = DydxClient().private.unwrap().send_verification_email().unwrap();
                                // assert_eq!(_response, 400);
                        });
                }

                it "getPrivateProfile" {
                        b!({
                                let _response = DydxClient().private.unwrap().get_profile().unwrap();
                                // dbg!(_response);
                        });
                }
        }
}
