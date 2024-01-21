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

    pub fn sender_address(&self) -> String {
        self.sender_address.clone()
    }

    pub fn recipient_address(&self) -> String {
        self.recipient_address.clone()
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}
