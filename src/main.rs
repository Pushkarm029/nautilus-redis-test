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

        let currencies_rx = redis.load_currencies()?;
        let instruments_rx = redis.load_instruments()?;
        let synthetics_rx = redis.load_synthetics()?;
        let accounts_rx = redis.load_accounts()?;
        let orders_rx = redis.load_orders()?;

        println!("currencies count: {}", currencies_rx.recv().unwrap().len());
        println!("instruments count: {}", instruments_rx.recv().unwrap().len());
        println!("synthetics count: {}", synthetics_rx.recv().unwrap().len());
        println!("accounts count: {}", accounts_rx.recv().unwrap().len());
        println!("orders count: {}", orders_rx.recv().unwrap().len());

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
// 134.010523351s: load_all 5
// 4. complete async impl (not possible because our python binding doesnt support it)
