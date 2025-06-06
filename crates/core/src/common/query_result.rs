use std::any::TypeId;

use crate::common::chain::Chain;
//use alloy::primitives::{Address, Bloom, Bytes, FixedBytes, B256, U256};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use sui_types::base_types::{ObjectType, SequenceNumber, SuiAddress};
use sui_types::digests::TransactionDigest;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct QueryResult {
    pub result: ExpressionResult,
}

impl QueryResult {
    pub fn new(result: ExpressionResult) -> QueryResult {
        QueryResult { result }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ExpressionResult {
    #[serde(rename = "account")]
    Account(Vec<AccountQueryRes>),
    #[serde(rename = "checkpoint")]
    Checkpoint(Vec<CheckpointQueryRes>),
    #[serde(rename = "transaction")]
    Transaction(Vec<TransactionQueryRes>),
    #[serde(rename = "coin")]
    Coin(Vec<CoinQueryRes>),
    #[serde(rename = "object")]
    Object(Vec<ObjectQueryRes>),
}

// TODO: should this be replaced with Alloy's Block?
#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CheckpointQueryRes {
    pub chain: Option<Chain>,
    pub number: Option<u64>,
    pub timestamp: Option<u64>,
    pub transactions: Option<usize>,
    pub epoch: Option<u64>,
    pub digest: Option<String>,
    pub previous_digest: Option<String>,
    pub computation_cost: Option<u64>,
    pub storage_cost: Option<u64>,
    pub storage_rebate: Option<u64>,
    pub non_refundable_storage_fee: Option<u64>,
    pub network_total_transactions: Option<u64>,
    pub validator_signature: Option<String>,
}

impl Default for CheckpointQueryRes {
    fn default() -> Self {
        Self {
            chain: None,
            number: None,
            timestamp: None,
            transactions: None,
            epoch: None,
            digest: None,
            computation_cost: None,
            storage_cost: None,
            storage_rebate: None,
            non_refundable_storage_fee: None,
            previous_digest: None,
            network_total_transactions: None,
            validator_signature: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AccountQueryRes {
    pub chain: Option<Chain>,
    pub sui_balance: Option<u128>,
    pub coin_owned: Option<usize>,
    pub staked_amount: Option<u128>,
    pub address: Option<SuiAddress>,
    pub active_delegations: Option<usize>,
}

impl Default for AccountQueryRes {
    fn default() -> Self {
        Self {
            chain: None,
            sui_balance: None,
            coin_owned: None,
            staked_amount: None,
            address: None,
            active_delegations: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CoinQueryRes {
    pub chain: Option<Chain>,
    pub id: Option<String>,          // Coin type ID
    pub name: Option<String>,        // "Usdc"
    pub symbol: Option<String>,      // "USDC"
    pub description: Option<String>, // "Stable coin."
    pub decimals: Option<u8>,        // 9
    pub icon_url: Option<String>,    // Might be null or Some(url)
}

impl Default for CoinQueryRes {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            symbol: None,
            description: None,
            decimals: None,
            icon_url: None,
            chain: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct EventQueryRes {
    pub chain: Option<Chain>,
    pub tx_digest: Option<String>,
    pub event_seq: Option<u64>,
    pub package_id: Option<String>,
    pub module: Option<String>,
    pub sender: Option<String>,
    pub event_type: Option<String>,
    pub bcs_encoding: Option<String>,
    pub bcs: Option<String>,
}

impl Default for EventQueryRes {
    fn default() -> Self {
        Self {
            chain: None,
            tx_digest: None,
            event_seq: None,
            package_id: None,
            module: None,
            sender: None,
            event_type: None,
            bcs_encoding: None,
            bcs: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ObjectQueryRes {
    pub chain: Option<Chain>,
    pub object_id: Option<String>,
    pub version: Option<SequenceNumber>,
    pub digest: Option<String>,
    pub owner: Option<SuiAddress>,
    pub previous_transaction: Option<TransactionDigest>,
    pub storage_rebate: Option<u64>,
}

impl Default for ObjectQueryRes {
    fn default() -> Self {
        Self {
            object_id: None,
            version: None,
            digest: None,
            owner: None,
            previous_transaction: None,
            storage_rebate: None,
            chain: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct TransactionQueryRes {
    pub chain: Option<Chain>,
    pub r#kind: Option<String>,
    pub recepient: Option<SuiAddress>,
    pub digest: Option<TransactionDigest>,
    pub sender: Option<SuiAddress>,
    pub to: Option<SuiAddress>,
    pub executed_epoch: Option<u64>,
    pub computation_cost: Option<u64>,
    pub storage_cost: Option<u64>,
    pub storage_rebate: Option<u64>,
    pub gas_price: Option<u64>,
    pub gas_budget: Option<u64>,
    pub gas_used: Option<u64>,
    pub checkpoint: Option<u64>,
    pub status: Option<bool>,
    pub timestamp_ms: Option<u64>,
    pub total_events: Option<usize>,
}

impl Default for TransactionQueryRes {
    fn default() -> Self {
        Self {
            chain: None,
            r#kind: None,
            digest: None,
            sender: None,
            to: None,
            computation_cost: None,
            storage_cost: None,
            storage_rebate: None,
            gas_price: None,
            gas_budget: None,
            gas_used: None,
            checkpoint: None,
            status: None,
            timestamp_ms: None,
            total_events: None,
            executed_epoch: None,
            recepient: None,
        }
    }
}

impl TransactionQueryRes {
    pub fn has_value(&self) -> bool {
        self.chain.is_some()
            || self.r#kind.is_some()
            || self.digest.is_some()
            || self.sender.is_some()
            || self.to.is_some()
            || self.computation_cost.is_some()
            || self.storage_cost.is_some()
            || self.storage_rebate.is_some()
            || self.gas_price.is_some()
            || self.gas_budget.is_some()
            || self.gas_used.is_some()
            || self.checkpoint.is_some()
            || self.status.is_some()
            || self.timestamp_ms.is_some()
            || self.total_events.is_some()
            || self.recepient.is_some()
            || self.executed_epoch.is_some()
    }

    pub fn get_field_values(&self) -> Vec<(&'static str, String)> {
        let mut fields = Vec::new();
        if let Some(chain) = &self.chain {
            fields.push(("chain", chain.to_string()));
        }
        if let Some(r#kind) = &self.r#kind {
            fields.push(("kind", r#kind.to_string()));
        }
        if let Some(digest) = &self.digest {
            fields.push(("digest", format!("{digest:?}")));
        }
        if let Some(sender) = &self.sender {
            fields.push(("sender", sender.to_string()));
        }
        if let Some(to) = &self.to {
            fields.push(("to", to.to_string()));
        }
        if let Some(computation_cost) = self.computation_cost {
            fields.push(("computation_cost", computation_cost.to_string()));
        }
        if let Some(storage_cost) = self.storage_cost {
            fields.push(("storage_cost", storage_cost.to_string()));
        }
        if let Some(storage_rebate) = self.storage_rebate {
            fields.push(("storage_rebate", storage_rebate.to_string()));
        }
        if let Some(gas_price) = self.gas_price {
            fields.push(("gas_price", gas_price.to_string()));
        }
        if let Some(gas_budget) = self.gas_budget {
            fields.push(("gas_budget", gas_budget.to_string()));
        }
        if let Some(gas_used) = self.gas_used {
            fields.push(("gas_used", gas_used.to_string()));
        }
        if let Some(checkpoint) = self.checkpoint {
            fields.push(("checkpoint", checkpoint.to_string()));
        }
        if let Some(status) = self.status {
            fields.push(("status", status.to_string()));
        }
        if let Some(timestamp_ms) = self.timestamp_ms {
            fields.push(("timestamp_ms", timestamp_ms.to_string()));
        }
        if let Some(total_events) = self.total_events {
            fields.push(("total_events", total_events.to_string()));
        }
        fields
    }
}

impl Serialize for TransactionQueryRes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields = self.get_field_values();
        let mut state = serializer.serialize_struct("TransactionQueryRes", fields.len())?;
        for (field_name, value) in fields {
            state.serialize_field(field_name, &value)?;
        }
        state.end()
    }
}

// #[serde_with::skip_serializing_none]
// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
// pub struct LogQueryRes {
//     pub chain: Option<Chain>,
//     pub address: Option<Address>,
//     pub topic0: Option<FixedBytes<32>>,
//     pub topic1: Option<FixedBytes<32>>,
//     pub topic2: Option<FixedBytes<32>>,
//     pub topic3: Option<FixedBytes<32>>,
//     pub data: Option<Bytes>,
//     pub block_hash: Option<B256>,
//     pub block_number: Option<u64>,
//     pub block_timestamp: Option<u64>,
//     pub transaction_hash: Option<B256>,
//     pub transaction_index: Option<u64>,
//     pub log_index: Option<u64>,
//     pub removed: Option<bool>,
// }

// impl Default for LogQueryRes {
//     fn default() -> Self {
//         Self {
//             chain: None,
//             address: None,
//             topic0: None,
//             topic1: None,
//             topic2: None,
//             topic3: None,
//             data: None,
//             block_hash: None,
//             block_number: None,
//             block_timestamp: None,
//             transaction_hash: None,
//             transaction_index: None,
//             log_index: None,
//             removed: None,
//         }
//     }
// }

// fn serialize_option_u256<S>(option: &Option<U256>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     match option {
//         Some(u256) => serializer.serialize_some(&u256.to_string()),
//         None => serializer.serialize_none(),
//     }
// }

// #[cfg(test)]
// mod test {
//     use std::str::FromStr;

//     use super::serialize_option_u256;
//     use alloy::primitives::U256;
//     use serde::Serialize;
//     use serde_json::json;

//     #[derive(Serialize)]
//     struct U256Serializable {
//         #[serde(serialize_with = "serialize_option_u256")]
//         pub value: Option<U256>,
//     }

//     #[test]
//     fn test_u256_serialization() {
//         let value = U256::from_str("100").expect("Unable to parse value to U256");
//         let u256 = U256Serializable { value: Some(value) };
//         let u256_str = json!(u256).to_string();
//         assert_eq!("{\"value\":\"100\"}", u256_str);
//     }
// }
