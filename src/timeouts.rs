use serde::{Serialize, Deserialize};
use serde_json::*;

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Timeouts {
    #[serde(default)]
    pub script: Option<usize>,
    #[serde(default)]
    #[serde(rename = "pageLoad")]
    pub page_load: Option<usize>,
    #[serde(default)]
    pub implicit: Option<usize>,
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
