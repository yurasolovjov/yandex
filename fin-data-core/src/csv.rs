use std::io::Read;
use crate::{
    FinancialRecord, ParserRecord, ParseError
};

pub struct CSVReader;

impl ParserRecord for CSVReader {
    fn parse<R: Read>(&self, input: R) -> Result<FinancialRecord, String> {

    }
}
