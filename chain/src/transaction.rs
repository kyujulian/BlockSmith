use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Transaction {
    sender_address: String,
    recipient_address: String,
    value: i64,
}

impl Transaction {
    pub fn new(sender_address: String, recipient_address: String, value: i64) -> Self {
        Self {
            sender_address,
            recipient_address,
            value,
        }
    }
}
