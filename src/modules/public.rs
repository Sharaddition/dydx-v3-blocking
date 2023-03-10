use super::super::{ResponseError, Result};
use http::StatusCode;
use serde::Deserialize;
use std::time::Duration;

pub use super::super::types::*;

#[readonly::make]
#[derive(Debug, Clone)]
pub struct Public<'a> {
    client: reqwest::blocking::Client,
    #[readonly]
    pub host: &'a str,
}

impl Public<'_> {
    pub fn new(host: &str, api_timeout: u64) -> Public {
        Public {
            client: reqwest::blocking::ClientBuilder::new()
                .timeout(Duration::from_secs(api_timeout))
                .build()
                .expect("Client::new()"),
            host,
        }
    }

    pub fn get_markets(&self, market: Option<&str>) -> Result<MarketsResponse> {
        let mut parameter = Vec::new();
        if let Some(local_var) = market {
            parameter.push(("market", local_var));
        }
        let response = self.get("markets", parameter);
        response
    }

    pub fn get_orderbook(&self, market: &str) -> Result<OrderbookResponse> {
        let path = format!("orderbook/{}", market);
        let response = self.get(path.as_str(), Vec::new());
        response
    }

    pub fn get_trades(
        &self,
        market: &str,
        starting_before_or_at: Option<&str>,
    ) -> Result<TradesResponse> {
        let path = format!("trades/{}", market);
        let mut parameter = Vec::new();
        if let Some(local_var) = starting_before_or_at {
            parameter.push(("startingBeforeOrAt", local_var));
        }

        let response = self.get(path.as_str(), parameter);
        response
    }

    pub fn get_fast_withdrawal(
        &self,
        credit_asset: Option<&str>,
        credit_amount: Option<&str>,
        debit_amount: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut parameter = Vec::new();
        if let Some(local_var) = credit_asset {
            parameter.push(("creditAsset", local_var));
        }
        if let Some(local_var) = credit_amount {
            parameter.push(("creditAmount", local_var));
        }
        if let Some(local_var) = debit_amount {
            parameter.push(("debitAmount", local_var));
        }
        let response: serde_json::Value = self.get("fast-withdrawals", parameter)?;
        Ok(response)
    }

    pub fn get_stats(&self, market: &str, days: Option<&str>) -> Result<MarketStatsResponse> {
        let path = format!("stats/{}", market);
        let mut parameter = Vec::new();
        if let Some(local_var) = days {
            parameter.push(("days", local_var));
        }
        let response = self.get(path.as_str(), parameter);
        response
    }

    pub fn get_historical_funding(
        &self,
        market: &str,
        effective_before_or_at: Option<&str>,
    ) -> Result<HistoricalFundingResponse> {
        let path = format!("historical-funding/{}", market);
        let mut parameter = Vec::new();
        if let Some(local_var) = effective_before_or_at {
            parameter.push(("effectiveBeforeOrAt", local_var));
        }
        let response = self.get(path.as_str(), parameter);
        response
    }

    pub fn get_candles(
        &self,
        market: &str,
        resolution: Option<&str>,
        from_iso: Option<&str>,
        to_iso: Option<&str>,
        limit: Option<&str>,
    ) -> Result<CandlesResponse> {
        let path = format!("candles/{}", market);
        let mut parameters = Vec::new();
        if let Some(local_var) = resolution {
            parameters.push(("resolution", local_var));
        }
        if let Some(local_var) = from_iso {
            parameters.push(("fromISO", local_var));
        }
        if let Some(local_var) = to_iso {
            parameters.push(("toISO", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }

        let response = self.get(path.as_str(), parameters);
        response
    }

    pub fn get_config(&self) -> Result<ConfigResponse> {
        let response = self.get("config", Vec::new());
        response
    }

    pub fn check_if_user_exists(&self, ethereum_address: &str) -> Result<UserExistsResponse> {
        let parameters = vec![("ethereumAddress", ethereum_address)];
        let response = self.get("users/exists", parameters);
        response
    }

    pub fn check_if_username_exists(&self, username: &str) -> Result<UsernameExistsResponse> {
        let parameters = vec![("username", username)];
        let response = self.get("usernames", parameters);
        response
    }

    pub fn get_time(&self) -> Result<GetTimeResponse> {
        let response = self.get("time", Vec::new());
        response
    }

    pub fn get_leaderboard_pnls(
        &self,
        period: &str,
        starting_before_or_at: &str,
        sort_by: &str,
        limit: Option<&str>,
    ) -> Result<LeaderboardPnlResponse> {
        let mut parameters = vec![
            ("period", period),
            ("startingBeforeOrAt", starting_before_or_at),
            ("sortBy", sort_by),
        ];
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        let response = self.get("leaderboard-pnl", parameters);
        response
    }

    pub fn get_public_retroactive_mining_rewards(
        &self,
        ethereum_address: &str,
    ) -> Result<PublicRetroactiveMiningRewardsResponse> {
        let parameters = vec![("ethereumAddress", ethereum_address)];
        let response = self
            .get("rewards/public-retroactive-mining", parameters)
            ;
        response
    }

    pub fn get_currently_revealed_hedgies(&self) -> Result<CurrentlyRevealedHedgies> {
        let response = self.get("hedgies/current", Vec::new());
        response
    }

    pub fn get_historically_revealed_hedgies(
        &self,
        nft_reveal_type: &str,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<HedgiePeriodResponse> {
        let mut parameters = Vec::new();
        parameters.push(("nftRevealType", nft_reveal_type));
        if let Some(local_var) = start {
            parameters.push(("start", local_var));
        }
        if let Some(local_var) = end {
            parameters.push(("end", local_var));
        }
        let response = self.get("hedgies/history", parameters);
        response
    }

    pub fn get_insurance_fund_balance(&self) -> Result<InsuranceFundBalanceResponse> {
        let response = self.get("insurance-fund/balance", Vec::new());
        response
    }

    pub fn get_profile(&self, public_id: &str) -> Result<ProfilePublicResponse> {
        let path = format!("profile/{}", public_id);
        let response = self.get(path.as_str(), Vec::new());
        response
    }

    pub fn verify_email(&self, token: &str) -> Result<StatusCode> {
        let param = vec![("token", token)];
        let response = self.put("emails/verify-email", &param);
        response
    }

 fn get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        parameters: Vec<(&str, &str)>,
    ) -> Result<T> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = self.client.get(url).query(&parameters);
        let response = req_builder.send();

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    return Ok(response.json::<T>().unwrap())
                }
                _ => {
                    let error = ResponseError {
                        code: response.status().to_string(),
                        message: response.text().unwrap(),
                    };
                    return Err(Box::new(error));
                }
            },
            Err(err) => {
                return Err(Box::new(err));
            }
        };
    }

 fn put(&self, path: &str, parameters: &[(&str, &str)]) -> Result<StatusCode> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = self.client.put(url).query(parameters);
        let result = req_builder.send()?;
        Ok(result.status())
    }
}
