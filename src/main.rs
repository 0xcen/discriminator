use anchor_lang::prelude::*;
use convert_case::{Case, Casing};
use serde_json::{Result, Value};
use std::io::Read;
use std::{fs::File, io::Write};

// fn main() -> () {

// let mut namespace = "global".to_string();
// let mut name = None;
// for arg in std::env::args().skip(1) {
//     if arg == "-n" {
//         namespace = std::env::args().nth(2).expect("no namespace given");
//     } else {
//         name = Some(arg);
//     }
// }
// let name = name.expect("no name given");
// let hash = get_hash(&namespace, &name);

// // print result
// println!("namespace: {}", namespace);
// println!("name: {}", name);
// println!("hash: {:?}\n", hash);
// ()
// }

pub fn get_hash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}
fn camel_to_snake_case(camel_case: &str) -> String {
    camel_case.to_case(Case::Snake)
}

// Function to read JSON data from a file, process instruction names, and compute sighash
fn process_instructions(file_path: &str, namespace: &str) -> Result<Value> {
    // Read the JSON file
    let mut file = File::open(file_path).unwrap();
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).unwrap();

    // Deserialize JSON into a serde_json::Value
    let mut value: Value = serde_json::from_str(&json_data)?;

    // Process instruction names and compute sighash
    let mut processed_instructions = serde_json::Map::new();
    if let Some(instructions) = value.get_mut("instructions").and_then(|v| v.as_array_mut()) {
        for instruction in instructions {
            if let Some(name) = instruction["name"].as_str() {
                // Convert camel case name to snake case
                let snake_case_name = camel_to_snake_case(name);

                // Compute sighash using get_hash function
                let sighash = get_hash(namespace, &snake_case_name);

                // Insert the instruction name and sighash into the processed JSON object
                processed_instructions.insert(
                    name.to_string(),
                    Value::Array(
                        sighash
                            .iter()
                            .map(|&b| Value::Number(serde_json::Number::from(b)))
                            .collect(),
                    ),
                );
            }
        }
    }

    Ok(Value::Object(processed_instructions))
}

// Function to write the processed JSON data to a new file
fn write_processed_json_to_file(value: &Value, output_file_path: &str) -> Result<()> {
    let output_json = serde_json::to_string_pretty(value)?;
    let mut output_file = File::create(output_file_path).unwrap();
    output_file.write_all(output_json.as_bytes()).unwrap();
    Ok(())
}

fn main() {
    let file_path = "/path/to/your/json/file.json";
    let namespace = "global";
    let output_file_path = "/path/to/your/output/file.json";

    match process_instructions(file_path, namespace) {
        Ok(processed_data) => {
            match write_processed_json_to_file(&processed_data, output_file_path) {
                Ok(_) => println!(
                    "Processed JSON data has been written to {}",
                    output_file_path
                ),
                Err(e) => println!("Error writing processed JSON data: {:?}", e),
            }
        }
        Err(e) => println!("Error reading JSON data: {:?}", e),
    }
}
// Make sure to replace "path/to/your/json/file.json" with the actual file path of your JSON data. The function extract_instruction_names_from_file will read the JSON data from the specified file, deserialize it into the JSONData struct, and then extract and return the names of the instructions as a vector of strings. The main function demonstrates how to use this function and print the extracted instruction names.

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JSONData {
    pub version: String,
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub accounts: Vec<Account>,
    pub types: Vec<Type>,
    pub errors: Vec<Error>,
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Instruction {
    pub name: String,
    pub accounts: Vec<AccountEntry>,
    pub args: Vec<Argument>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountEntry {
    pub name: String,
    pub isMut: bool,
    pub isSigner: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Argument {
    pub name: String,
    pub r#type: ArgumentType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ArgumentType {
    Defined(String),
    Vec { defined: String },
    U64,
    String,
    Bytes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub r#type: AccountType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AccountType {
    Struct { fields: Vec<StructField> },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StructField {
    pub name: String,
    pub r#type: StructFieldType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StructFieldType {
    PublicKey,
    Vec(String),
    Option(String),
    U64,
    I64,
    Bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
    pub name: String,
    pub r#type: TypeKind,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TypeKind {
    Struct { fields: Vec<StructField> },
    Enum { variants: Vec<EnumVariant> },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<Vec<StructField>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: u64,
    pub name: String,
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub address: String,
}

impl JSONData {
    // You can define methods or additional functionality for the struct here.
}
