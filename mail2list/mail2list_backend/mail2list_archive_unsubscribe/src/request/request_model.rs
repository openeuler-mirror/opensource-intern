use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscribeMailListQuery {
    pub id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArchiveMailListQuery {
    pub ids: Option<Vec<i64>>,
}