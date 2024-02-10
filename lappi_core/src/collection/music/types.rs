use serde::{Serialize, Deserialize};
use crate::collection::types::ItemId;

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ExternalSrcFileDesc {
    pub id: ItemId,
    pub path: String,
}