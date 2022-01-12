use doson::DataValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub code: String,
    pub result: DataValue,
}
