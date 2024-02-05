use lazy_static::lazy_static;
use log::info;
use reqwest::header;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Asset {
    symbol: String,
}

#[derive(Debug)]
pub struct Alpaca {
    client: reqwest::Client,
    base_endpoint: String,
    watchlist: Vec<Asset>,
}

lazy_static! {
    static ref ACCESS_KEY: String = std::env::var("ACCESS_KEY").expect("ACCESS_KEY is not set");
    static ref SECRET_KEY: String = std::env::var("SECRET_KEY").expect("SECRET_KEY is not set");
}

impl Alpaca {
    pub async fn new() -> Self {
        info!("Creating new alpaca connection",);

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
            watchlist: Vec::new(),
        }
    }

    pub async fn get_account_details(&self) -> serde_json::Value {
        info!("Get account details");

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
        info!("Get open positions");

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
        info!("Get open orders");

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

    pub async fn get_watchlist(&self) -> &Vec<Asset> {
        info!("Get watchlist");

        &self.watchlist
    }
}
