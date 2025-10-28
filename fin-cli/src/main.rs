use clap::{Parser, ValueEnum};
use fin_data_core::{ParserRecord, SerializeRecord, CSVParser, CSVSerializer, Mt940Parser, Camt053Parser, Camt053Serializer, ParseError, SerializeError, Mt940Serializer};
use std::fs::{self, File};
use std::io::{stdout, Write, Read};

#[derive(ValueEnum, Clone)]
enum Format {
    Csv,
    Mt940,
    Camt053,
}

#[derive(Parser)]
struct Cli {
    #[arg(long, value_enum)]
    from: Format,
    #[arg(long, value_enum)]
    to: Format,
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let input_data = match &cli.input {
        Some(path) => {
            fs::read(path)?
        }
        None => {
            let mut buffer = Vec::new();
            std::io::stdin().read_to_end(&mut buffer)?;
            buffer
        }
    };

    let records = match cli.from {
        Format::Csv => CSVParser.parse(&*input_data)?,
        Format::Mt940 => Mt940Parser.parse(&*input_data)?,
        Format::Camt053 => Camt053Parser.parse(&*input_data)?,
    };

    println!("parsed {} records", records.len());

    let mut output: Box<dyn Write> = match &cli.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(stdout()),
    };

    match cli.to {
        Format::Csv => {
            CSVSerializer.serialize(records.into_iter(), &mut output)?;
        }
        Format::Mt940 => {
            Mt940Serializer.serialize(records.into_iter(), &mut output)?;
        }
        Format::Camt053 => {
            Camt053Serializer.serialize(records.into_iter(), &mut output)?;
        }
    }

    Ok(())
}