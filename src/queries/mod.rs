pub mod models;

use std::collections::HashMap;

use anyhow::Result;
use futures::StreamExt;
use models::CacheDatabaseAdapter;
use redis::AsyncCommands;
use redis::{
    aio::{ConnectionManager, ConnectionManagerConfig},
    Client, RedisError,
};

pub struct RedisCacheDatabase {
    conn: ConnectionManager,
}

impl RedisCacheDatabase {
    pub async fn new() -> anyhow::Result<RedisCacheDatabase> {
        let client = Client::open("redis://127.0.0.1/")?;
        let config = ConnectionManagerConfig::new().set_number_of_retries(10);
        let conn = client.get_connection_manager_with_config(config).await?;
        Ok(Self { conn })
    }
}

const CURRENCIES: &str = "currencies";
const INSTRUMENTS: &str = "instruments";
const SYNTHETICS: &str = "synthetics";
const ACCOUNTS: &str = "accounts";
const ORDERS: &str = "orders";

// conditions :
// these all cant be async
// but we need concurrency

impl CacheDatabaseAdapter for RedisCacheDatabase {
    fn load(&self) -> anyhow::Result<HashMap<String, String>> {
        todo!()
    }

    fn load_currencies(&mut self) -> Result<std::sync::mpsc::Receiver<HashMap<String, String>>> {
        let pattern = format!("{CURRENCIES}*");
        let mut conn = self.conn.clone();

        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let mut currencies = HashMap::new();
            for key in scan_keys(&mut conn, pattern).await.unwrap() {
                let code = key.split(':').nth(1).unwrap_or_default().to_string();
                let result = conn.get::<_, String>(key).await;
                match result {
                    Ok(currency) => {
                        currencies.insert(code, currency);
                    }
                    Err(_) => {
                        log::error!("Currency not found: {code}");
                    }
                }
            }

            if let Err(e) = tx.send(currencies) {
                log::error!("Error sending currencies: {e}");
            }
        });

        Ok(rx)
    }

    fn load_instruments(
        &mut self,
    ) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>> {
        let pattern = format!("{INSTRUMENTS}*");
        let mut conn = self.conn.clone();

        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let mut instruments = HashMap::new();
            for key in scan_keys(&mut conn, pattern).await.unwrap() {
                let instrument_id = key.split(':').nth(1).unwrap_or_default().to_string();
                let result = conn.get::<_, String>(key).await;
                match result {
                    Ok(instrument) => {
                        instruments.insert(instrument_id, instrument);
                    }
                    Err(_) => {
                        log::error!("Instrument not found: {instrument_id}");
                    }
                }
            }
            if let Err(e) = tx.send(instruments) {
                log::error!("Error sending instruments: {e}");
            }
        });

        Ok(rx)
    }

    fn load_synthetics(
        &mut self,
    ) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>> {
        let pattern = format!("{SYNTHETICS}*");
        let mut conn = self.conn.clone();
        let (tx, rx) = std::sync::mpsc::channel();

        tokio::spawn(async move {
            let mut synthetics = HashMap::new();
            for key in scan_keys(&mut conn, pattern).await.unwrap() {
                let instrument_id = key.split(':').nth(1).unwrap_or_default().to_string();
                let result = conn.get::<_, String>(key).await;
                match result {
                    Ok(synthetic) => {
                        synthetics.insert(instrument_id, synthetic);
                    }
                    Err(_) => {
                        log::error!("Synthetic not found: {instrument_id}");
                    }
                }
            }
            if let Err(e) = tx.send(synthetics) {
                log::error!("Error sending synthetics: {e}");
            }
        });

        Ok(rx)
    }

    fn load_accounts(
        &mut self,
    ) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>> {
        let pattern = format!("{ACCOUNTS}*");
        let mut conn = self.conn.clone();

        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let mut accounts = HashMap::new();
            for key in scan_keys(&mut conn, pattern).await.unwrap() {
                let account_id = key.split(':').nth(1).unwrap_or_default().to_string();
                let result = conn.get::<_, String>(key).await;
                match result {
                    Ok(account) => {
                        accounts.insert(account_id, account);
                    }
                    Err(_) => {
                        log::error!("Account not found: {account_id}");
                    }
                }
            }
            if let Err(e) = tx.send(accounts) {
                log::error!("Error sending accounts: {e}");
            }
        });

        Ok(rx)
    }

    fn load_orders(
        &mut self,
    ) -> anyhow::Result<std::sync::mpsc::Receiver<HashMap<String, String>>> {
        let pattern = format!("{ORDERS}*");
        let mut conn = self.conn.clone();

        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let mut orders = HashMap::new();
            for key in scan_keys(&mut conn, pattern).await.unwrap() {
                let order_id = key.split(':').nth(1).unwrap_or_default().to_string();
                let result = conn.get::<_, String>(key).await;
                match result {
                    Ok(order) => {
                        orders.insert(order_id, order);
                    }
                    Err(_) => {
                        log::error!("Order not found: {order_id}");
                    }
                }
            }
            if let Err(e) = tx.send(orders) {
                log::error!("Error sending orders: {e}");
            }
        });

        Ok(rx)
    }

    fn load_positions(&mut self) -> anyhow::Result<HashMap<String, String>> {
        todo!()
    }
}

async fn scan_keys(
    conn: &mut ConnectionManager,
    pattern: String,
) -> Result<Vec<String>, RedisError> {
    Ok(conn
        .scan_match::<String, String>(pattern)
        .await
        .unwrap()
        .collect()
        .await)
}
