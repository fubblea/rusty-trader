use log::info;
use std::env;

use mongodb::{bson::doc, Client, Collection};

use crate::{Trader, TraderConfig};

#[derive(Debug)]
pub struct Mongo {
    traders: Collection<Trader>,
    configs: Collection<TraderConfig>,
}

impl Mongo {
    pub async fn new(uri: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::with_uri_str(uri.unwrap_or(env::var("MONGO_URI")?.as_str())).await?;
        info!("Connected to MongoDB: {:?}", client);

        let db = client.database("trader_controller");

        let traders = db.collection::<Trader>("traders");
        let configs = db.collection::<TraderConfig>("configs");

        Ok(Self { traders, configs })
    }

    pub async fn get_config(
        &self,
        id: u32,
    ) -> Result<Option<TraderConfig>, Box<dyn std::error::Error>> {
        info!("Getting config with id: {}", id);
        let result = self.configs.find_one(doc! {"id": id}, None).await?;

        Ok(result)
    }

    pub async fn add_config(&self, config: TraderConfig) -> Result<(), Box<dyn std::error::Error>> {
        info!("Adding config: {:?}", config);
        self.configs.insert_one(config, None).await?;

        Ok(())
    }

    pub async fn add_trader(&self, trader: Trader) -> Result<(), Box<dyn std::error::Error>> {
        info!("Adding trader: {:?}", trader);
        self.traders.insert_one(trader, None).await?;

        Ok(())
    }

    pub async fn get_trader(&self, id: u32) -> Result<Option<Trader>, Box<dyn std::error::Error>> {
        info!("Getting trader with id: {}", id);
        let result = self.traders.find_one(doc! {"id": id}, None).await?;

        Ok(result)
    }
}
