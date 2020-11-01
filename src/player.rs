use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerInfo {
    pub steamd_id: String,
    pub nickname: String,
    pub avatar_hash: String,
}
