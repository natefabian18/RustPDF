/**
 * Author: Nate Fabian
 * Purpose: Hold all the types and implementations needed for report processing
 */
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CellValue {
    Double(f64),
    String(String),
    Null(()),
}

impl Default for CellValue {
    fn default() -> Self {
        CellValue::Null(())
    }
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result: String;
        match self {
            CellValue::Double(val) => result = format!("{:?}", val),
            CellValue::String(val) => result = format!("{}", val),
            CellValue::Null(_val) => result = format!(""),
        }

        write!(f, "{}", result)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    #[serde(default)]
    pub value: CellValue,
    #[serde(default)]
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportData {
    pub name: String,
    pub data: Vec<Vec<Cell>>,
}