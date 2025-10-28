use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FinancialRecord {
    pub amount: f64,

    pub debit_credit: char,

    pub value_date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_date: Option<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    pub transaction_type: String,
}