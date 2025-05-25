use std::fmt;

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self{
        Transaction{sender, recipient, amount}
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}->{}:{}", self.sender, self.recipient, self.amount)
    }
}