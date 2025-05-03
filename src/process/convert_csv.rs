use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::cli::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // Use PascalCase to match CSV header
                                    //使用serde方式时，将记录映射为结构体
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

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut output_rows: Vec<Value> = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    // let rows = reader
    // .deserialize()
    // .map(|row|row.unwrap())
    // .collect::<Vec<Row>>();

    //使用deserialize()需要提前定义好Row结构体，如果换一个csv文件，则需要重新定义结构体，为了让格式转换更具通用性，使用records()方法
    // for string_record in reader.deserialize() {
    //     let row: Row = string_record?;
    //     output_rows.push(row);
    // }

    for result in reader.records() {
        let record = result?;
        let record_value = headers
            .iter()
            .zip(record.iter())
            .map(|(header, value)| {
                let header = header.to_string();
                let value = value.to_string();
                (header, value)
            })
            .collect::<Value>();
        output_rows.push(record_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&output_rows)?,
        OutputFormat::Yaml => serde_yaml::to_string(&output_rows)?,
    };

    fs::write(output, content)?;

    Ok(())
}
