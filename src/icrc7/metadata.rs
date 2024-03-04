use std::collections::HashMap;

use candid::CandidType;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use serde::{Deserialize, Serialize};

pub type Icrc7CollectionMetadata = HashMap<String, MetadataValue>;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Icrc7TokenMetadata{
    pub metadata: HashMap<String, MetadataValue>
}