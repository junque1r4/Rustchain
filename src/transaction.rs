use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: f64) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Transaction {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
            timestamp,
        }
    }
}

pub struct TransactionPool {
    pub transactions: Vec<Transaction>,
}

impl TransactionPool {
    pub fn new() -> Self {
        TransactionPool {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        // Simple validation, could be more complex in a real system
        if transaction.amount <= 0.0 {
            return false;
        }

        self.transactions.push(transaction);
        true
    }

    pub fn get_transactions(&mut self, limit: usize) -> Vec<Transaction> {
        let available = self.transactions.len().min(limit);
        if available == 0 {
            return Vec::new();
        }

        // Take the first 'limit' transactions
        let txs: Vec<Transaction> = self.transactions.drain(0..available).collect();
        txs
    }
}
