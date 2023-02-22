macro_rules! b {
        ($e:expr) => {
                // tokio_test::block_on($e)
                $e
        };
}

use dydx_v3_blocking::constants::*;
use dydx_v3_blocking::modules::eth_sign::*;
use dydx_v3_blocking::ClientOptions;
use dydx_v3_blocking::DydxClient;
use speculate::speculate;

#[cfg(test)]
speculate! {
        describe "onboardingTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let options = ClientOptions {
                                network_id: Some(TESTNET_NETWORK_ID),
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                eth_private_key: Some(TEST_PRIVATE_KEY),
                        };
                        DydxClient::new(TESTNET_API_URL, options)

                }

                it "createUser" {
                        b!({
                                let user_data = CreateUserParams {
                                        stark_key: TEST_STARK_PRIVATE_KEY,
                                        stark_key_y_coordinate: TEST_STARK_Y_COORDINATE,
                                        referred_by_affiliate_link: None,
                                        country: None

                                };

                                let _response = DydxClient().onboarding.unwrap().create_user(user_data ,TEST_ADDRESS);
                                // dbg!(&_response);

                        });
                }
                it "deriveStarkKey" {
                        b!({
                                let _response = DydxClient().onboarding.unwrap().derive_stark_key(TEST_ADDRESS);
                                // dbg!(&_response);

                        });
                }
                it "recoverDefaultApiCredentials" {
                        b!({
                                let onboarding = DydxClient().onboarding.unwrap();
                                let _response = onboarding.recover_default_api_key_credentials(TEST_ADDRESS);
                                // dbg!(&_response);

                        });
                }

        }

}
