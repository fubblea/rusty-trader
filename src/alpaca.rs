use lazy_static::lazy_static;
use reqwest::header;

pub struct Alpaca {
    client: reqwest::Client,
    base_endpoint: String,
}

lazy_static! {
    static ref ACCESS_KEY: String = std::env::var("ACCESS_KEY").expect("ACCESS_KEY is not set");
    static ref SECRET_KEY: String = std::env::var("SECRET_KEY").expect("SECRET_KEY is not set");
}

impl Alpaca {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "APCA-API-KEY-ID",
            header::HeaderValue::from_static(&ACCESS_KEY),
        );
        headers.insert(
            "APCA-API-SECRET-KEY",
            header::HeaderValue::from_static(&SECRET_KEY),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Unable to build headers");

        Self {
            client,
            base_endpoint: "https://paper-api.alpaca.markets/v2".to_string(),
        }
    }

    pub async fn get_account_details(&self) -> serde_json::Value {
        let endpoint = format!("{}/{}", &self.base_endpoint, "account");

        self.client
            .get(endpoint)
            .send()
            .await
            .expect("Unable to get account")
            .json()
            .await
            .expect("Unable to convert to json")
    }

    pub async fn get_open_positions(&self) -> serde_json::Value {
        let endpoint = format!("{}/{}", &self.base_endpoint, "positions");

        self.client
            .get(endpoint)
            .send()
            .await
            .expect("Unable to get open positions")
            .json()
            .await
            .expect("Unable to convert to json")
    }

    pub async fn get_open_orders(&self) -> serde_json::Value {
        let endpoint = format!("{}/{}", &self.base_endpoint, "orders?status=open");

        self.client
            .get(endpoint)
            .send()
            .await
            .expect("Unable to get open positions")
            .json()
            .await
            .expect("Unable to convert to json")
    }
}
