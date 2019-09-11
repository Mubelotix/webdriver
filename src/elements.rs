use crate::tab::*;
use crate::error::*;
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

    pub fn type_text(&mut self, text: &str) -> Result<(), WebdriverError> {
        // select tab
        self.tab.select()?;

        // Build request
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.tab.get_session_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/element/");
        request_url += &self.id;
        request_url.push_str("/value");
        let postdata = object! {
            "text" => text
        };

        // Send request
        let res = self.tab.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn get_text(&self) -> Result<String, WebdriverError> {
        // select tab
        self.tab.select()?;

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.tab.get_session_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/element/");
        request_url += &self.id;
        request_url.push_str("/text");

        // send command
        let res = self
            .tab
            .session
            .client
            .get(&request_url)
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_string() {
                        Ok(json["value"].to_string())
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn click(&mut self) -> Result<(), WebdriverError> {
        // select tab
        self.tab.select()?;

        // Build request
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.tab.get_session_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/element/");
        request_url += &self.id;
        request_url.push_str("/click");
        let postdata = object! {
        };

        // Send request
        let res = self.tab.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        

        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }
}