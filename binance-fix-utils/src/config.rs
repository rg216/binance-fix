use std::env;

use quickfix::SessionId;

#[derive(Debug)]
pub struct BinanceConfig {
    pub api_key: String,
    pub api_secret: String,
    pub sign_pemkey: String,
}

impl BinanceConfig {
    pub fn from_env() -> Self {
        macro_rules! read {
            ($key:expr) => {
                env::var($key).expect(concat!("Missing env variable: ", $key))
            };
        }

        Self {
            api_key: read!("BINANCE_API_KEY"),
            api_secret: read!("BINANCE_API_SECRET"),
            sign_pemkey: read!("BINANCE_SIGN_PEMKEY"),
        }
    }

    pub fn order_entry_session_id(&self) -> SessionId {
        SessionId::try_new(
            binance_fix44_order_entry::FIX_BEGIN_STRING,
            &self.api_key,
            "SPOT",
            "order-entry",
        )
        .expect("Fail to build session ID")
    }

    pub fn market_data_session_id(&self) -> SessionId {
        SessionId::try_new(
            binance_fix44_order_entry::FIX_BEGIN_STRING,
            &self.api_key,
            "SPOT",
            "market-data",
        )
        .expect("Fail to build session ID")
    }
}
