macro_rules! b {
        ($e:expr) => {
                // tokio_test::block_on($e)
                $e
        };
}

use dydx_v3_blocking::constants::*;
use dydx_v3_blocking::types::*;
use dydx_v3_blocking::ClientOptions;
use dydx_v3_blocking::DydxClient;
use speculate::speculate;

#[cfg(test)]
speculate! {
        describe "publicTest" {
                fn DydxClient() -> DydxClient<'static> {

                        let options: ClientOptions = ClientOptions {
                                network_id: None,
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                eth_private_key: None
                        };
                        DydxClient::new(MAINNET_API_URL, options)
                    }

                it "getMarket" {
                        b!({
                                let _response = DydxClient().public.get_markets(Some(DydxMarket::BTC_USD)).unwrap();
                                // dbg!(&_response.markets["BTC-USD"].oracle_price);
                        });
                }

                it "getMarketWithNoParameter" {
                        b!({
                                let _response = DydxClient().public.get_markets(None).unwrap();
                                // dbg!(&_response.markets["1INCH-USD"]);
                                // dbg!(_response);
                        });
                }


                it "getOrderbook" {
                        b!({
                                let _response = DydxClient().public.get_orderbook(DydxMarket::ETH_USD).unwrap();
                                // println!("{:?}", _response.asks[0].size);
                                // dbg!(_response);
                        });
                }

                it "getTrades" {
                        b!({
                                let _response = DydxClient().public.get_trades(DydxMarket::ETH_USD, None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getFastWithdrawal" {
                        b!({
                                let _response = DydxClient().public.get_fast_withdrawal(None, None, None).unwrap();
                                // dbg!(&_response["liquidityProviders"]);
                        });
                }

                it "getMarketStats" {
                        b!({
                                let _response = DydxClient().public.get_stats(DydxMarket::UNI_USD, Some(MarketStatisticDay::SEVEN)).unwrap();
                                // dbg!(&_response.markets["UNI-USD"]);
                        });
                }

                it "getHistoricalFunding" {
                        b!({
                                let _response = DydxClient().public.get_historical_funding(DydxMarket::SUSHI_USD, None).unwrap();
                                // dbg!(&_response.historical_funding.into_iter().nth(0));
                        });
                }


                it "getCandles" {
                        b!({
                                let _response = DydxClient().public.get_candles(DydxMarket::ETH_USD, Some(CandleResolution::FIVE_MINS), Some("2022-01-05T17:33:43.163Z"),Some("2022-01-06T17:33:43.163Z"), Some("4")).unwrap();
                                // dbg!(_response);
                        });
                }

                // it "getCandlesWithNoParameter" {
                //         b!({
                //                 let _response = DydxClient().public.get_candles(DydxMarket::ETH_USD, None, None, None, None).unwrap();
                //                 // dbg!(_response);
                //         });
                // }

                it "getConfig" {
                        b!({
                                let _response = DydxClient().public.get_config().unwrap();
                                // dbg!(_response);
                        });
                }

                it "checkIfUserExists" {
                        b!({
                                let _response = DydxClient().public.check_if_user_exists(TEST_ADDRESS).unwrap();
                                // dbg!(_response);
                        });
                }

                it "checkIfUsernameExists" {
                        b!({
                                let _response = DydxClient().public.check_if_username_exists("faeravca").unwrap();
                                // dbg!(_response);
                        });
                }

                it "getTime" {
                        b!({
                                let _response = DydxClient().public.get_time().unwrap();
                                // dbg!(_response);
                        });
                }

                it "getLeaderboardPnls" {
                        b!({
                                let _response = DydxClient().public.get_leaderboard_pnls("SILVER", "2022-04-05T17:33:43.163Z", "PERCENT", None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getPublicRetroactiveMiningRewards" {
                        b!({
                                let _response = DydxClient().public.get_public_retroactive_mining_rewards(TEST_ADDRESS).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getCurrentlyRevealedHedgies" {
                        b!({
                                let _response = DydxClient().public.get_currently_revealed_hedgies().unwrap();
                                // dbg!(_response);
                        });
                }

                it "getHistoricallyRevealedHedgies" {
                        b!({
                                let _response = DydxClient().public.get_historically_revealed_hedgies(NftRevealType::DAY, None, None).unwrap();
                                // dbg!(_response);
                        });
                }

                it "getInsuranceFundBalance" {
                        b!({
                                let _response = DydxClient().public.get_insurance_fund_balance().unwrap();
                                // dbg!(_response);
                        });
                }

                it "getPublicProfile" {
                        b!({
                                let _response = DydxClient().public.get_profile("SIFTBRXH").unwrap();
                                // dbg!(_response);
                        });
                }

                it "verifyEmail" {
                        b!({
                                let _response = DydxClient().public.verify_email("aaa").unwrap();
                                // dbg!(_response);
                        });
                }
        }
}
