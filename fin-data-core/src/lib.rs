mod models;
mod csv;
mod mt940;
mod camt053;
mod errors;

pub use models::{FinancialRecord};
pub use errors::{
    ParseError,
    SerializeError
};
pub use csv::{
    CSVParser,
    CSVSerializer
};
pub use mt940::{
    Mt940Parser,
    Mt940Serializer
};
pub use camt053::{
    Camt053Parser,
    Camt053Serializer
};

use std::io::{ Write, Read};
use tempfile::NamedTempFile;


pub trait ParserRecord{
    fn parse<R:Read>(&self, input: R) -> Result<Vec<FinancialRecord>, ParseError>;
}

pub trait SerializeRecord{
    fn serialize<W:Write, I: IntoIterator<Item=FinancialRecord>>(&self, records: I, writer: &mut W) -> Result<(), SerializeError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_read_write_tests() {
        let parser = CSVParser;
        let serializer = CSVSerializer;

        let input_records = vec![
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

        use ParserRecord;
        use SerializeRecord;

        let mut tmp_file = NamedTempFile::new().unwrap();
        match serializer.serialize(input_records.clone(), &mut tmp_file) {
            Err(SerializeError::Csv(csv_error)) => {
                println!("{:?}", csv_error);
            },
            Err(SerializeError::Json(json_error)) => {
                println!("{:?}", json_error);
            }
            Err(SerializeError::Io(io_error)) => {
                println!("{:?}", io_error);
            }
            Ok(()) => {
                println!("OK");
            }
        }
        let read_tmp_file = tmp_file.reopen().unwrap();

        let output_records =  match parser.parse(read_tmp_file) {
            Ok(out) => {
                out
            }
            Err(ParseError::Csv(err)) => {
                panic!("{}", err)
            }
            _ => {
                panic!("Parse error")
            }
        };

        let zipped = output_records
            .into_iter().zip(input_records.into_iter());

        for (i, (in_rec, out_rec)) in zipped.enumerate(){
            assert_eq!(in_rec, out_rec, "FinancialRecords should be equal");
        }
    }
    #[test]
    fn json_read_write_tests() {
        let parser = Camt053Parser;
        let serializer = Camt053Serializer;

        let input_records = vec![
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

        use ParserRecord;
        use SerializeRecord;

        let mut tmp_file = NamedTempFile::new().unwrap();
        match serializer.serialize(input_records.clone(), &mut tmp_file) {
            Ok(()) => {
                println!("OK");
            }
            Err(SerializeError::Json(csv_error)) => {
                println!("{:?}", csv_error);
            }
            Err(SerializeError::Io(io_error)) => {
                println!("{:?}", io_error);
            }
            _ => {
                panic!("Serialize error");
            }
        }
        let read_tmp_file = tmp_file.reopen().unwrap();

        let output_records =  match parser.parse(read_tmp_file) {
            Ok(out) => {
                out
            }
            Err(ParseError::Camt053(err)) => {
                println!("{}", err);
                vec![]
            }
            _ => {
                panic!("Parse error")
            }
        };

        let zipped = output_records
            .into_iter().zip(input_records.into_iter());

        for (i, (in_rec, out_rec)) in zipped.enumerate(){
            assert_eq!(in_rec, out_rec, "FinancialRecords should be equal");
        }
    }
}
