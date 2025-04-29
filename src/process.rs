#![allow(unused)]

use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{
    ffi::{OsStr, OsString},
    fs,
    num::ParseIntError,
    path::{Path, MAIN_SEPARATOR},
    str::FromStr,
};

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
    replica: u8,
) -> anyhow::Result<()> {
    let source_path = Path::new(source);
    let mut target_path = Path::new(target);

    if !source_path.exists() {
        return Err(anyhow::anyhow!("Source file does not exist"));
    }

    if !source_path.is_file() {
        return Err(anyhow::anyhow!("Source is not a file"));
    }

    let mut file_base = OsStr::new("");
    let mut file_extension = OsStr::new("");

    if !(target_path.to_string_lossy().ends_with("/")
        || target_path.to_string_lossy().ends_with("\\"))
    {
        let parent_path = target_path.parent();
        if let Some(parent) = parent_path {
            fs::create_dir_all(parent)?;
        }
        file_base = target_path.file_stem().unwrap();
        file_extension = target_path.extension().unwrap_or(OsStr::new(""));
        target_path = parent_path.unwrap();
    } else {
        fs::create_dir_all(target_path)?;
        file_base = source_path.file_stem().unwrap();
        file_extension = source_path.extension().unwrap();
    }

    let file_base = file_base.to_string_lossy().into_owned();
    let mut dot = OsString::from(".");
    let file_extension = if file_extension.is_empty() {
        file_extension.to_string_lossy().into_owned()
    } else {
        dot.push(file_extension);
        dot.to_string_lossy().into_owned()
    };

    if mode == "incr" {
        let end = replica + from - 1;
        let width = end.to_string().len();
        for i in from..=end {
            let new_file_name = format!("{}_{:0>width$}{}", file_base, i, file_extension);
            let target_file_name = target_path.join(new_file_name);
            fs::copy(source_path, target_file_name)?;
        }
    } else {
        let new_file_name = format!("{}{}", file_base, file_extension);
        println!(
            "Copying file {:?} to {:?}",
            source_path,
            target_path.join(&new_file_name)
        );
        fs::copy(source_path, target_path.join(&new_file_name))?;
    }

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
