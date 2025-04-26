#![allow(unused)]

use std::{fs, path::Path};

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // Use PascalCase to match CSV header
struct Row {
    //上面使用了PascalCase，这里就无需再映射了
    name: String,

    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    nationality: String,

    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn copy_file(
    source: &str,
    target: &str,
    mode: &str,
    from: u8,
    length: u8,
    replica: u8,
) -> anyhow::Result<()> {
    let source_path = Path::new(source);
    let target_path = Path::new(target);

    if !source_path.exists() {
        return Err(anyhow::anyhow!("Source file does not exist"));
    }

    if target_path.exists() {
        return Err(anyhow::anyhow!("Target file already exists"));
    }

    fs::copy(source_path, target_path)?;
    Ok(())
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut output_rows = Vec::with_capacity(128);
    // let rows = reader
    // .deserialize()
    // .map(|row|row.unwrap())
    // .collect::<Vec<Row>>();
    for string_record in reader.deserialize() {
        let row: Row = string_record?;
        output_rows.push(row);
    }
    let json = serde_json::to_string_pretty(&output_rows)?;
    fs::write(output, json)?;
    Ok(())
}
