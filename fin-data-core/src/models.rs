use crate::{ParseError, SerializeError};

pub struct FinancialRecord {
    pub transaction_id: String,
    pub amount: f64,
    pub description: String,
    pub date_at: String,
}

impl FinancialRecord {
    pub fn new(transaction_id: String, amount: f64, description: String, date_at: String) -> Self {
        Self{
            transaction_id,
            amount,
            description,
            date_at,
        }
    }

    pub fn to_csv_line(&self) -> String {
        let desc = if self.description.contains(|c| c == ',' || c == '"' || c == '\n') {
            format!("\"{}\"", self.description.replace('"', "\"\""))
        } else {
            self.description.clone()
        };

        format!(
            "{},{},{},{}\n",
            self.transaction_id,
            self.amount,
            desc,
            self.date_at
        )
    }
}
impl TryFrom<String> for FinancialRecord  {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let fields: Vec<String> = value
            .split(',')
            .map(|f| f.to_string())
            .collect();
        match fields.len() {
            4 => {
                Ok(Self::new(
                    fields[0].to_string(),
                    fields[1].parse::<f64>().unwrap_or(0.0),
                    fields[2].to_string(),
                    fields[3].to_string(),
                ))
            }
            _ => {
                Err(ParseError::InvalidFormat("can't parse financial record".to_string()))
            }
        }
    }
}
