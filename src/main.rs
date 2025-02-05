// src/main.rs
mod queries;

use queries::models::CacheDatabaseAdapter;
use queries::RedisCacheDatabase;
use std::time::Instant;
use tokio::{self, runtime};

fn main() {
    let runtime = runtime::Runtime::new().unwrap();
    let start = Instant::now();
    if let Err(e) = runtime.block_on(async {
        let mut redis = RedisCacheDatabase::new().await?;

        let currencies = redis.load_currencies()?;
        println!("currencies count: {}", currencies.len());

        let instruments = redis.load_instruments()?;
        println!("instruments count: {}", instruments.len());

        let synthetics = redis.load_synthetics()?;
        println!("synthetics count: {}", synthetics.len());

        let accounts = redis.load_accounts()?;
        println!("accounts count: {}", accounts.len());

        let orders = redis.load_orders()?;
        println!("orders count: {}", orders.len());

        let async_duration = start.elapsed();
        println!("Async approach took: {:?}", async_duration);

        anyhow::Ok(())
    }) {
        println!("Error: {}", e);
    };
}

// 100000 keys

// 1. current
// Currencies loaded: 100000
// Async approach took: 4.558472675s
// Async results count: 100000

// 2. current nauti-impl
// 1-4ms: connection impl
// 55.546447462s: load_currencies
// 359.081090153s: for all 5

// 3. current sql type approach
// 353.774379093s: load_all 5


// 3. async with layer by layer impl
// 4. complete async impl (not possible because our python binding doesnt support it)
