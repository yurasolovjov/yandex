use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FinancialRecord {
    pub transaction_id: String,
    pub amount: f64,
    pub description: String,
    pub date_at: String,
}

impl FinancialRecord {
    pub fn new(transaction_id: String, amount: f64, description: String, date_at: String) -> Self {
        Self{
            transaction_id,
            amount,
            description,
            date_at,
        }
    }
}