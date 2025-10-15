mod models;
mod csv;
mod mt940;
mod camt053;
mod errors;

pub use models::{FinancialRecord};
pub use errors::*;
use std::io::{ Write, Read};


pub trait ParserRecord{
    fn parse<R:Read>(&self, input: R) -> Result<FinancialRecord, ParseError>;
}

pub trait SerializeRecord{
    fn serialize<W:Write, I: IntoIterator<Item=FinancialRecord>>(&self, records: I, writer: W) -> Result<(), SerializeError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
