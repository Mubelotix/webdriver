use serde_json::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Debug, PartialEq)]
pub struct Timeouts {
    pub script: Option<usize>,
    pub page_load: usize,
    pub implicit: usize
}

impl Timeouts {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "script": self.script,
            "pageLoad": self.page_load,
            "implicit": self.implicit
        })
    }
}