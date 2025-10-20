use crate::{FinancialRecord, ParserRecord, ParseError, SerializeError, SerializeRecord};
use std::io::{Read, Write};
use uuid::Uuid;


#[derive(Default)]
pub struct Mt940Parser;

impl ParserRecord for Mt940Parser {
    fn parse<R: Read>(&self, mut input: R) -> Result<Vec<FinancialRecord>, ParseError> {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer)
            .map_err(ParseError::Io)?;

        // Простой пример парсинга: ищем строки с ":61:"
        let mut records = Vec::new();
        for line in buffer.lines() {
            if line.starts_with(":61:") {
                // Формат: :61:YYMMDD...C...1234,56N...ID...
                // Это упрощённая реализация!
                let parts: Vec<&str> = line.split('C').collect();
                if parts.len() < 2 {
                    continue;
                }
                let amount_str = parts[1].split('N').next().unwrap_or("0");
                let value = amount_str.replace(',', ".").parse::<f64>()
                    .map_err(|_| ParseError::Mt940("invalid amount".to_string()))?;
                let id = "mt940-".to_string() + &Uuid::new_v4().to_string();
                records.push(FinancialRecord {
                    transaction_id: id,
                    amount: value,
                    description: "MT940 transaction".to_string(),
                    date_at: "".to_string(),
                });
            }
        }
        Ok(records)
    }
}

#[derive(Default)]
pub struct Mt940Serializer;

impl SerializeRecord for Mt940Serializer {

    fn serialize<W: Write, I: IntoIterator<Item=FinancialRecord>>(&self, records: I, writer: &mut W) -> Result<(), SerializeError> {
        Err(SerializeError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "MT940 serialization not implemented",
        )))
    }
}