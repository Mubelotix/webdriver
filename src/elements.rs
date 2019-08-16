pub use crate::session::*;
pub use crate::windows::*;
use json::*;
use std::result::Result;

pub struct Element<'a> {
    id: String,
    tab: &'a Tab<'a>
}

impl<'a> Element<'a> {
    pub fn new(id: String, tab: &'a Tab) -> Self {
        Element{
            id,
            tab
        }
    }

    pub fn type_text(&mut self, text: &str) -> Result<(), String> {
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.tab.get_session_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/element/");
        request_url += &self.id;
        request_url.push_str("/value");

        let postdata = object! {
            "text" => text
        };

        let mut res = self.tab.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send()
            .expect("Can't send request to selenium.");

        let res = json::parse(&res.text().expect("Can't read response body.")).expect("Can't parson response body to json.");

        Ok(())
    }

    pub fn click(&mut self) -> Result<(), String> {
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.tab.get_session_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/element/");
        request_url += &self.id;
        request_url.push_str("/click");

        let postdata = object! {
        };

        let mut res = self.tab.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send()
            .expect("Can't send request to selenium.");

        let res = json::parse(&res.text().expect("Can't read response body.")).expect("Can't parson response body to json.");

        Ok(())
    }
}