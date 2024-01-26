use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    sender_address: String,
    recipient_address: String,
    value: f32,
}

impl Transaction {
    pub fn new(sender_address: String, recipient_address: String, value: f32) -> Self {
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
    pub fn value(&self) -> f32 {
        self.value
    }
}
