/// # transaction
/// A struct that represents a transaction. It has fields for 
/// a sender, recipient, and an amount indicating how much the 
/// transaction was for

use std::hash::{Hash, Hasher};

#[derive(Debug)]
#[derive(Clone)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
}

// Manually implement because float can't be hashed
impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sender.hash(state);
        self.recipient.hash(state);

        // convert float to string since floating point literals can't be 
        // hashed themselves (because NaN != NaN)
        let amount_str = self.amount.to_string();
        amount_str.hash(state);
    }
}
