use crate::{FinancialRecord, ParserRecord,  ParseError, SerializeError, SerializeRecord};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use serde_json;

pub struct Camt053Parser;

impl ParserRecord for Camt053Parser {
    fn parse<R: Read>(&self, mut input: R) -> Result<Vec<FinancialRecord>, ParseError> {
        let records = serde_json::from_reader(input)?;
        Ok(records)
    }
}

#[derive(Default)]
pub struct Camt053Serializer;

impl SerializeRecord for Camt053Serializer {
    fn serialize<W: Write, I: IntoIterator<Item=FinancialRecord>>(&self, records: I, writer: &mut W) -> Result<(), SerializeError> {
        let vec: Vec<FinancialRecord> = records.into_iter().collect();
        serde_json::to_writer_pretty(&mut *writer, &vec)?;
        Ok(())
    }
}