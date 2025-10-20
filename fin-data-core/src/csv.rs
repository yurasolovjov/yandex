use std::io::{self, Read, Write, BufReader, BufRead, Cursor, };
use crate::{FinancialRecord, ParserRecord, ParseError, SerializeRecord, SerializeError};
use csv::{ReaderBuilder, WriterBuilder};

use std::fs::File;
pub struct CSVReader;
pub struct CSVWriter;

impl ParserRecord for CSVReader {
    fn parse<R: Read>(&self, input: R) -> Result<Vec<FinancialRecord>, ParseError> {
        let mut rdr = ReaderBuilder::new().from_reader(input);
        let mut records = Vec::new();

        for result in rdr.deserialize() {
            records.push(result?);
        }
        Ok(records)
    }
}


impl SerializeRecord for CSVWriter {
    fn serialize<W: Write, I: IntoIterator<Item=FinancialRecord>>(&self, records: I, writer: &mut W) -> Result<(), SerializeError> {
        let mut w = WriterBuilder::new().from_writer(writer);
        for record in records {
            w.serialize(record)?;
        }
        w.flush()?;
        Ok(())
    }
}