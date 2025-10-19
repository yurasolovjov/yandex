use std::io::{self, Read, Write, BufReader, BufRead};
use crate::{FinancialRecord, ParserRecord, ParseError, SerializeRecord, SerializeError};
use std::io::Cursor;

use std::fs::File;
pub struct CSVReader;

impl ParserRecord for CSVReader {
    fn parse<R: Read>(&self, input: R) -> Result<Vec<FinancialRecord>, ParseError> {
        parse_csv(input)
    }
}

fn parse_csv<R: Read, C: TryFrom<String, Error = ParseError>>(reader: R) -> Result<Vec<C>, ParseError> {
    let mut csv_reader = BufReader::new(reader);
    let mut vec: Vec<C> = Vec::new();

    for (i, line) in csv_reader.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        vec.push(C::try_from(line)?);
    }
    Ok(vec)
}


impl SerializeRecord for FinancialRecord {
    fn serialize<W: Write, I: IntoIterator<Item=FinancialRecord>>(&self, records: I, writer: &mut W) -> Result<(), SerializeError> {
        for record in records {
            writer.write(record.to_csv_line().as_bytes())?;
        }
        Err(SerializeError::Csv("not implemented".to_string()))
    }
}

pub fn financial_records_to_csv_reader(records: Vec<FinancialRecord>) -> impl Read {
    let mut csv = String::from("transaction_id,amount,description,date_at\n");
    for record in records {
        csv.push_str(&record.to_csv_line());
    }
    Cursor::new(csv.into_bytes())
}