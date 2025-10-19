mod models;
mod csv;
mod mt940;
mod camt053;
mod errors;

pub use models::{FinancialRecord};
pub use errors::{
    ParseError, SerializeError
};
pub use csv::{
    CSVReader
};
use std::io::{ Write, Read};
use csv::{
    financial_records_to_csv_reader
};


pub trait ParserRecord{
    fn parse<R:Read>(&self, input: R) -> Result<Vec<FinancialRecord>, ParseError>;
}

pub trait SerializeRecord{
    fn serialize<W:Write, I: IntoIterator<Item=FinancialRecord>>(&self, records: I, writer: W) -> Result<(), SerializeError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let parser = CSVReader;

        let records = vec![
            FinancialRecord {
                transaction_id: "txn_001".to_string(),
                amount: 150.75,
                description: "Grocery shopping".to_string(),
                date_at: "2025-10-01".to_string(),
            },
            FinancialRecord {
                transaction_id: "txn_002".to_string(),
                amount: -89.99,
                description: "Online subscription".to_string(),
                date_at: "2025-10-03".to_string(),
            },
            FinancialRecord {
                transaction_id: "txn_003".to_string(),
                amount: 2500.00,
                description: "Salary deposit".to_string(),
                date_at: "2025-10-05".to_string(),
            },
            FinancialRecord {
                transaction_id: "txn_004".to_string(),
                amount: -45.50,
                description: "Gas station".to_string(),
                date_at: "2025-10-07".to_string(),
            },
            FinancialRecord {
                transaction_id: "txn_005".to_string(),
                amount: 120.00,
                description: "Freelance payment".to_string(),
                date_at: "2025-10-10".to_string(),
            },
            FinancialRecord {
                transaction_id: "txn_006".to_string(),
                amount: -299.99,
                description: "New headphones".to_string(),
                date_at: "2025-10-12".to_string(),
            },
            FinancialRecord {
                transaction_id: "txn_007".to_string(),
                amount: 50.25,
                description: "Refund from store".to_string(),
                date_at: "2025-10-15".to_string(),
            },
        ];

        let cursor = financial_records_to_csv_reader(records);

        use ParserRecord;

        let data = parser.parse(cursor).unwrap();

        for record in data {
            println!("{}", record.description);
        }
    }
}
