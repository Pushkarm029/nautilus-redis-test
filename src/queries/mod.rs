pub mod models;

use std::collections::HashMap;

use anyhow::Result;
use futures::StreamExt;
use models::CacheDatabaseAdapter;
use redis::Commands;
use redis::{
    aio::{ConnectionManager, ConnectionManagerConfig},
    Client, RedisError,
};
use redis::{AsyncCommands, Connection};

pub struct RedisCacheDatabase {
    con: Connection,
}

impl RedisCacheDatabase {
    pub fn new() -> anyhow::Result<RedisCacheDatabase> {
        let client = Client::open("redis://127.0.0.1/")?;
        // let config = ConnectionManagerConfig::new().set_number_of_retries(10);
        // let con = client.get_connection_manager_with_config(config).await?;
        let con = client.get_connection()?;
        Ok(Self { con })
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

    fn load_currencies(&mut self) -> Result<HashMap<String, String>> {
        let mut currencies = HashMap::new();
        let pattern = format!("{CURRENCIES}*");

        for key in scan_keys(&mut self.con, pattern)? {
            let code = key.split(':').nth(1).unwrap_or_default().to_string();
            let result = self.con.get::<_, String>(key);
            match result {
                Ok(currency) => {
                    currencies.insert(code, currency);
                }
                Err(e) => {
                    log::error!("Currency not found: {code}");
                    return Err(e.into());
                }
            }
        }
        Ok(currencies)
    }

    fn load_instruments(&mut self) -> anyhow::Result<HashMap<String, String>> {
        let mut instruments = HashMap::new();
        let pattern = format!("{INSTRUMENTS}*");

        for key in scan_keys(&mut self.con, pattern)? {
            let instrument_id = key.split(':').nth(1).unwrap_or_default().to_string();
            let result = self.con.get::<_, String>(key);
            match result {
                Ok(instrument) => {
                    instruments.insert(instrument_id, instrument);
                }
                Err(e) => {
                    log::error!("Instrument not found: {instrument_id}");
                    return Err(e.into());
                }
            }
        }
        Ok(instruments)
    }

    fn load_synthetics(&mut self) -> anyhow::Result<HashMap<String, String>> {
        let mut synthetics = HashMap::new();
        let pattern = format!("{SYNTHETICS}*");

        for key in scan_keys(&mut self.con, pattern)? {
            let instrument_id = key.split(':').nth(1).unwrap_or_default().to_string();
            let result = self.con.get::<_, String>(key);
            match result {
                Ok(synthetic) => {
                    synthetics.insert(instrument_id, synthetic);
                }
                Err(e) => {
                    log::error!("Synthetic not found: {instrument_id}");
                    return Err(e.into());
                }
            }
        }
        Ok(synthetics)
    }

    fn load_accounts(&mut self) -> anyhow::Result<HashMap<String, String>> {
        let mut accounts = HashMap::new();
        let pattern = format!("{ACCOUNTS}*");

        for key in scan_keys(&mut self.con, pattern)? {
            let account_id = key.split(':').nth(1).unwrap_or_default().to_string();
            let result = self.con.get::<_, String>(key);
            match result {
                Ok(account) => {
                    accounts.insert(account_id, account);
                }
                Err(e) => {
                    log::error!("Account not found: {account_id}");
                    return Err(e.into());
                }
            }
        }
        Ok(accounts)
    }

    fn load_orders(&mut self) -> anyhow::Result<HashMap<String, String>> {
        let mut orders = HashMap::new();
        let pattern = format!("{ORDERS}*");

        for key in scan_keys(&mut self.con, pattern)? {
            let order_id = key.split(':').nth(1).unwrap_or_default().to_string();
            let result = self.con.get::<_, String>(key);
            match result {
                Ok(order) => {
                    orders.insert(order_id, order);
                }
                Err(e) => {
                    log::error!("Order not found: {order_id}");
                    return Err(e.into());
                }
            }
        }
        Ok(orders)
    }

    fn load_positions(&mut self) -> anyhow::Result<HashMap<String, String>> {
        todo!()
    }
}

fn scan_keys(con: &mut Connection, pattern: String) -> Result<Vec<String>, RedisError> {
    Ok(con.scan_match::<String, String>(pattern)?.collect())
}
