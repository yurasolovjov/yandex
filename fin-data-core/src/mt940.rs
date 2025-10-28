use crate::{FinancialRecord, ParserRecord,  ParseError, SerializeError, SerializeRecord};
use std::io::{BufRead, BufReader, Read, Write};
use serde_json;

use std::collections::{HashMap};
use std::collections::hash_map::Entry;
use crate::ParseError::Mt940;

pub struct Mt940Parser;

#[derive(Debug, Default)]
enum ParseState{
    #[default]
    Init,
    StartCode(String),
    FinCode(String),
    StartValue(String),
    FinValue(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Mt940Line {
    Tag(String),
    Value(String),
}

impl ParserRecord for Mt940Parser {
    fn parse<R: Read>(&self, mut input: R) -> Result<Vec<FinancialRecord>, ParseError> {
        let buf_reader = BufReader::new(input);
        let mut financial_records = Vec::new();
        let raw_records = raw_parser_mt940(buf_reader)?;
        for (tag, line) in raw_records {
            let record: FinancialRecord = parse_line_mt940(&*line)?;
        }
        Ok(financial_records)
    }
}

fn raw_parser_mt940<R: BufRead>(input: R) -> Result<HashMap<Mt940Line, Vec<Mt940Line>>, ParseError> {
    let mut records: HashMap<Mt940Line,Vec<Mt940Line>> = HashMap::new();
    for line in input.lines() {
        let line = line?;
        let (tag, value) = raw_line_parser_mt940(&*line)?;
        match records.entry(tag) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().push(value);
            }
            Entry::Vacant(e) => {
                e.insert(vec![value]);
            }
        }
    }
    Ok(records)
}

fn raw_line_parser_mt940<R: Read>(line: &str) -> Result<(Mt940Line, Mt940Line), ParseError> {
    let mut state: ParseState  = ParseState::default();
    let mut code: Option<Mt940Line> = None;
    let mut value: Option<Mt940Line> = None;
    let mut raw_code = String::new();
    let mut raw_value = String::new();

    for value_char in line.chars() {
        match state {
            ParseState::Init => {
                match value_char {
                    ':' => {
                        state = ParseState::StartCode(value_char.to_string());
                    },
                    _ => {
                        return Err(ParseError::Mt940("incorrect format".to_string()));
                    }
                }
            }
            ParseState::StartCode(_) => {
                match value_char {
                    '0'..='9' => {
                        raw_code.push(value_char);
                    }
                    ':' => {
                        state = ParseState::FinCode(value_char.to_string());
                    }
                    _ => {
                        return Err(ParseError::Mt940("incorrect format".to_string()));
                    }
                }
                raw_code.push(value_char);
            }
            ParseState::FinCode(_) => {
                match code {
                    None => {
                        code = Some(Mt940Line::Tag(raw_code.to_string()));
                        state = ParseState::StartValue(value_char.to_string());
                        raw_value.push(value_char);
                    }
                    Some(line) => {
                        return Err(ParseError::Mt940("incorrect format".to_string()));
                    }
                }
            }
            ParseState::StartValue(_) => {
                match value_char {
                    '\n' => {
                        state = ParseState::FinValue(value_char.to_string());
                        value = Some(Mt940Line::Value(raw_value.to_string()))
                    }
                    _ => raw_value.push(value_char)
                }
            }
            _ => {
                return Err(ParseError::Mt940("incorrect format".to_string()));
            }
        }
    }
    Ok((code.expect("Expected tag :xx: but got empty"), value.expect("Expected value after tag but got empty")))
}

fn parse_line_mt940(line: &str) -> Result<FinancialRecord, ParseError> {
    Ok(FinancialRecord{
        amount: 0.0,
        debit_credit: "C".try_into().unwrap(),
        value_date: "".to_string(),
        entry_date: None,
        description: "".to_string(),
        reference: None,
        transaction_type: "".to_string(),
    })
}

#[derive(Default)]
pub struct Mt940Serializer;

impl SerializeRecord for Mt940Serializer {
    fn serialize<W: Write, I: IntoIterator<Item=FinancialRecord>>(&self, records: I, writer: &mut W) -> Result<(), SerializeError> {
        let vec: Vec<FinancialRecord> = records.into_iter().collect();
        serde_json::to_writer_pretty(&mut *writer, &vec)?;
        Ok(())
    }
}
