use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        Transaction { sender, recipient, amount }
    }
}
