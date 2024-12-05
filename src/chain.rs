use blake3::Hash;
use serde::{Deserialize, Serialize};

use crate::transactions::Transaction;

type Chain = Vec<Block>;

#[derive(Serialize, Deserialize)]
struct Block {
    transactions: Vec<Transaction>,
    hash: Hash,
}
