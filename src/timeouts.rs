use json::*;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Timeouts {
    pub script: Option<usize>,
    pub page_load: usize,
    pub implicit: usize
}

impl Timeouts {
    pub fn to_json(&self) -> json::JsonValue {
        object! {
            "script" => self.script,
            "pageLoad" => self.page_load,
            "implicit" => self.implicit
        }
    }
}