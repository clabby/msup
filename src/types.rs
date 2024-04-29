//! This module contains the definiteions for the upgrade JSON schema.

use std::collections::HashMap;
use alloy_json_abi::Function;
use alloy_primitives::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

/// Describes the definition of a multisig batch.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigBatch {
    /// The chain ID of the upgrade
    pub chain_id: u64,
    /// The upgrade metadata
    pub metadata: ObjectMetadata,
    /// The upgrade transactions
    pub transactions: Vec<BatchTransaction>,
}

/// Describes the metadata of an object.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetadata {
    /// The name of the object
    pub name: String,
    /// The description of the object
    pub description: String,
}

/// Describes a transaction within a [MultisigBatch].
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BatchTransaction {
    /// The metadata of the transaction
    pub metadata: ObjectMetadata,
    /// The address of the contract to call
    pub to: Address,
    /// The value to send with the transaction
    pub value: U256,
    /// The data to send with the transaction
    pub data: Bytes,
    /// The function signature of the contract to call
    pub contract_method: Function,
    /// The input names and values for the transaction's function call
    pub contract_inputs_values: HashMap<String, String>,
}
