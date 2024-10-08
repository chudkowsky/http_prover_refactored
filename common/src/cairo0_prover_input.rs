use serde::{Deserialize, Serialize};

use crate::ProverInput;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cairo0ProverInput {
    pub program: Cairo0CompiledProgram,
    pub program_input: serde_json::Value,
    pub layout: String,
}

impl ProverInput for Cairo0ProverInput {
    fn serialize(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cairo0CompiledProgram {
    pub attributes: Vec<String>,
    pub builtins: Vec<String>,
    pub compiler_version: String,
    pub data: Vec<String>,
    pub debug_info: serde_json::Value,
    pub hints: serde_json::Value,
    pub identifiers: serde_json::Value,
    pub main_scope: String,
    pub prime: String,
    pub reference_manager: serde_json::Value,
}
