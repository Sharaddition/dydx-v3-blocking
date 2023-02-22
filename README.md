Rust(Non-Async) client for dYdX (v3 API) built from official source code.

[dYdX API Docs](https://docs.dydx.exchange/)

# Installation

Install [dydx-v3-blocking](https://crates.io/crates/dydx-v3-blocking) from crates.io. Add the following line to your `Cargo.toml` file's dependencies section:

```rust
[dependencies]
dydx-v3-blocking = { git = "https://github.com/Sharaddition/dydx-v3-blocking" }
tokio = { version = "1.18.2", features = ["full"] }
```

# Usage

Sample code to call Get Markets API

```rust
use dydx_v3_blocking::{types::*, ClientOptions, DydxClient};

fn main() {
    let options: ClientOptions = ClientOptions {
        network_id: None,
        api_timeout: None,
        api_key_credentials: None,
        stark_private_key: None,
        eth_private_key: None,
    };
    let client = DydxClient::new("https://api.dydx.exchange", options);
    let response = client
        .public
        .get_markets(Some(DydxMarket::BTC_USD))
        .unwrap();
    dbg!(response);
}
```

## Run tests

```sh
cargo test
```

### Disclaimer

Please use it at your own risk, do your own due-dilligence, I'm not responsible for anything that happens due to this library.