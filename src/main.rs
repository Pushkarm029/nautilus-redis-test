// src/main.rs
use anyhow::Result;
use futures::future::join_all;
use futures::StreamExt;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::{collections::HashMap, time::Instant};
use tokio;
// Simulated data structure
#[derive(Debug, Clone)]
struct Currency {
    code: String,
    name: String,
    precision: i32,
}

struct RedisDemo {
    conn: MultiplexedConnection,
}

impl RedisDemo {
    async fn new() -> Result<Self> {
        let client = Client::open("redis://127.0.0.1/")?;
        let conn = client.get_multiplexed_tokio_connection().await?;
        Ok(Self { conn })
    }

    // // Sequential approach
    // fn load_currencies_sync(&mut self) -> Result<HashMap<String, Currency>> {
    //     let mut currencies = HashMap::new();
    //     let pattern = "currency:*".to_string();

    //     let mut keys: Vec<String> = Vec::new();

    //     self.conn
    //         .scan_match::<String, String>(pattern)
    //         .await
    //         .unwrap()
    //         .for_each(|key| {
    //             if keys.len() % 1000 == 0 {
    //                 println!("Processed {} currencies so far...", keys.len());
    //             }
    //             keys.push(key);
    //         });

    //     for key in keys {
    //         let code = key.split(':').nth(1).unwrap_or_default().to_string();
    //         if currencies.len() % 1000 == 0 {
    //             println!("Processed {} currencies so far...", currencies.len());
    //         }
    //         if let Ok(currency_data) = self.conn.get::<_, String>(&key) {
    //             // Simulate some processing time
    //             // std::thread::sleep(std::time::Duration::from_micros(1));

    //             let currency = Currency {
    //                 code: code.clone(),
    //                 name: currency_data,
    //                 precision: 2,
    //             };
    //             currencies.insert(code, currency);
    //         }
    //     }

    //     Ok(currencies)
    // }

    // // Concurrent approach
    async fn load_currencies_async(&mut self) -> Result<HashMap<String, Currency>> {
        let mut currencies = HashMap::new();
        let pattern = "currency:*".to_string();

        // for key in scan_keys(&mut self.database.con, pattern)? {
        //     let parts: Vec<&str> = key.as_str().rsplitn(2, ':').collect();
        //     let currency_code = Ustr::from(parts.first().unwrap());
        //     let result = self.load_currency(&currency_code)?;
        //     match result {
        //         Some(currency) => {
        //             currencies.insert(currency_code, currency);
        //         }
        //         None => {
        //             log::error!("Currency not found: {currency_code}");
        //         }
        //     }
        // }

        let iter_keys: Vec<String> = self
            .conn
            .scan_match::<String, String>(pattern)
            .await
            .unwrap()
            .collect()
            .await;

        let futures = iter_keys.iter().map(|key| {
            let mut conn = self.conn.clone();
            async move {
                let code = key.split(':').nth(1).unwrap_or_default().to_string();
                let currency_data = conn.get::<_, String>(key).await?;
                let currency = Currency {
                    code: code.clone(),
                    name: currency_data,
                    precision: 2,
                };

                Ok((code, currency))
            }
        });

        let results: Vec<Result<(String, Currency)>> = join_all(futures).await;

        for result in results.into_iter().flatten() {
            currencies.insert(result.0, result.1);
        }

        println!("Currencies loaded: {}", currencies.len());

        Ok(currencies)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut redis_demo = RedisDemo::new().await?;

    // Test sync approach
    // let start = Instant::now();
    // let sync_results = redis_demo.load_currencies_sync()?;
    // let sync_duration = start.elapsed();
    // println!("Sync approach took: {:?}", sync_duration);
    // println!("Sync results count: {}", sync_results.len());

    // for 10^5
    // 113.169598496s

    // Test async approach
    let start = Instant::now();
    let async_results = redis_demo.load_currencies_async().await?;
    let async_duration = start.elapsed();
    println!("Async approach took: {:?}", async_duration);
    println!("Async results count: {}", async_results.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_comparison() -> Result<()> {
        let mut redis_demo = RedisDemo::new().await?;
        // Test sync approach
        // let start = Instant::now();
        // let sync_results = redis_demo.load_currencies_sync()?;
        // let sync_duration = start.elapsed();

        // Test async approach
        // let start = Instant::now();
        // let async_results = redis_demo.load_currencies_async().await?;
        // let async_duration = start.elapsed();

        // println!("Sync duration: {:?}", sync_duration);
        // println!("Async duration: {:?}", async_duration);

        // assert_eq!(sync_results.len(), async_results.len());
        // assert!(
        //     async_duration < sync_duration,
        //     "Async should be faster. Sync: {:?}, Async: {:?}",
        //     sync_duration,
        //     async_duration
        // );

        Ok(())
    }
}
