/// # transaction
/// A struct that represents a transaction. It has fields for 
/// a sender, recipient, and an amount indicating how much the 
/// transaction was for

#[derive(Debug)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
}
