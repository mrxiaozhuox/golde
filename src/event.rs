use doson::DataValue;

#[derive(Debug, Clone)]
pub struct Event {
    pub code: String,
    pub result: DataValue,
}