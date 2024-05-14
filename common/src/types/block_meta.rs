use serde::{Deserialize, Serialize};
use solana_sdk::commitment_config::CommitmentLevel;
use solana_transaction_status::Reward;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SlotMeta {
    pub slot: u64,
    pub parent: u64,
    pub commitment_level: CommitmentLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BlockMeta {
    pub parent_slot: u64,
    pub slot: u64,
    pub parent_blockhash: String,
    pub blockhash: String,
    pub rewards: Vec<Reward>,
    pub block_height: Option<u64>,
    pub executed_transaction_count: u64,
    pub entries_count: u64,
}