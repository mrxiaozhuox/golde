use doson::DataValue;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub code: String,
    pub result: DataValue,
}