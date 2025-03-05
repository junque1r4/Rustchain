use crate::{transaction::Transaction, Block, Blockchain, TransactionPool};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct AppState {
    blockchain: Arc<Mutex<Blockchain>>,
    tx_pool: Arc<Mutex<TransactionPool>>,
}

#[derive(Serialize)]
struct BlockchainStats {
    block_count: usize,
    latest_hash: String,
    pending_transactions: usize,
}

#[derive(Deserialize)]
struct MiningParams {
    num_transactions: Option<usize>,
}

#[derive(Deserialize)]
struct QueryParams {
    limit: Option<usize>,
}

// Get the entire blockchain
async fn get_blockchain(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(&blockchain.chain)
}

// Get blockchain stats
async fn get_blockchain_stats(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    let tx_pool = data.tx_pool.lock().unwrap();

    let stats = BlockchainStats {
        block_count: blockchain.chain.len(),
        latest_hash: blockchain
            .chain
            .last()
            .map_or("none".to_string(), |block| block.hash.clone()),
        pending_transactions: tx_pool.transactions.len(), // Changed this line
    };

    HttpResponse::Ok().json(stats)
}

// Get a specific block by index
async fn get_block(path: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    let index = path.into_inner();

    if index < blockchain.chain.len() {
        HttpResponse::Ok().json(&blockchain.chain[index])
    } else {
        HttpResponse::NotFound().body("Block not found")
    }
}

// Get latest block
async fn get_latest_block(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();

    if let Some(block) = blockchain.chain.last() {
        HttpResponse::Ok().json(block)
    } else {
        HttpResponse::NotFound().body("No blocks in the chain")
    }
}

// Add a new transaction to the pool
async fn add_transaction(tx: web::Json<Transaction>, data: web::Data<AppState>) -> impl Responder {
    let mut tx_pool = data.tx_pool.lock().unwrap();
    if tx_pool.add_transaction(tx.into_inner()) {
        HttpResponse::Ok().body("Transaction added")
    } else {
        HttpResponse::BadRequest().body("Invalid transaction")
    }
}

// Mine a new block
async fn mine_block(params: web::Query<MiningParams>, data: web::Data<AppState>) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    let mut tx_pool = data.tx_pool.lock().unwrap();

    let num_tx = params.num_transactions.unwrap_or(10);
    let transactions = tx_pool.get_transactions(num_tx);

    if transactions.is_empty() {
        return HttpResponse::BadRequest().body("No transactions to mine");
    }

    let tx_data = serde_json::to_string(&transactions).unwrap();
    let start_time = Instant::now();

    let new_block = blockchain.add_block(tx_data);
    let mining_time = start_time.elapsed();

    HttpResponse::Ok().json(json!({
        "block": new_block,
        "mining_time_ms": mining_time.as_millis()
    }))
}

// Validate the blockchain
async fn validate_blockchain(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();

    if blockchain.is_valid() {
        HttpResponse::Ok().body("Blockchain is valid")
    } else {
        HttpResponse::Ok().body("Blockchain is invalid")
    }
}

pub async fn start_server(
    blockchain: Arc<Mutex<Blockchain>>,
    tx_pool: Arc<Mutex<TransactionPool>>,
    port: u16,
) -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        blockchain,
        tx_pool,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/blockchain", web::get().to(get_blockchain))
            .route("/blockchain/stats", web::get().to(get_blockchain_stats))
            .route("/blockchain/validate", web::get().to(validate_blockchain))
            .route("/block/latest", web::get().to(get_latest_block))
            .route("/block/{index}", web::get().to(get_block))
            .route("/transaction", web::post().to(add_transaction))
            .route("/mine", web::post().to(mine_block))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
