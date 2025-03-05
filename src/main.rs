mod api;
mod blockchain;
mod discovery;
mod network;
mod transaction;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use api::start_server;
use blockchain::{Block, Blockchain};
use transaction::{Transaction, TransactionPool};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create shared state with Arc<Mutex<>>
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let tx_pool = Arc::new(Mutex::new(TransactionPool::new()));

    // Clone for the API server
    let blockchain_api = Arc::clone(&blockchain);
    let tx_pool_api = Arc::clone(&tx_pool);

    println!("Starting blockchain node on port 8080");

    // Start automatic mining in a background task
    tokio::spawn(async move {
        loop {
            // Wait a bit before mining to collect transactions
            tokio::time::sleep(Duration::from_secs(15)).await;

            // Check if there are transactions to mine
            let transactions = {
                let mut pool = tx_pool.lock().unwrap();
                if pool.transactions.is_empty() {
                    // Generate some test transactions if none available
                    let tx = Transaction::new("system", "miner", 1.0);
                    pool.add_transaction(tx);
                }
                pool.get_transactions(10)
            };

            if !transactions.is_empty() {
                let tx_data = serde_json::to_string(&transactions).unwrap();

                // Mine the block
                let mut chain = blockchain.lock().unwrap();
                println!(
                    "Mining new block with {} transactions...",
                    transactions.len()
                );
                let start = Instant::now();
                let block = chain.add_block(tx_data);
                let duration = start.elapsed();

                println!("Block mined in {:?} - Hash: {}", duration, block.hash);
            }
        }
    });

    // Start API server
    start_server(blockchain_api, tx_pool_api, 8080).await
}
