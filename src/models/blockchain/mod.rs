#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub previous_hash: String,
    pub block_hash: String,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
    pub hash: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub sign: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub account_states: Vec<AccountState>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccountState {
    pub account_id: String,
    pub balance: u64,
}
