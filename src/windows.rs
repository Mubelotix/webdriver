use json::*;
use std::result::Result;
use std::time::Duration;
use crate::session::*;
use std::rc::{Weak, Rc};

pub struct Tab<'a> {
    id: String,
    session: &'a Session<'a>
}

impl<'a> Tab<'a> {
    pub fn new_from(id: String, session: &'a Session) -> Tab<'a> {
        Tab {
            id,
            session
        }
    }

    pub fn new(session: &'a Session) -> Result<Tab<'a>, String> {
        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = session.get_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/window/new");
        let postdata = object! {};

        // send command
        let res = session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        if let Err(e) = res {
            return Err(format!("{}", e));
        }
        let mut res = res.unwrap();

        // read response
        if let Ok(text) = &res.text() {
            if let Ok(json) = json::parse(text) {
                if json["value"]["handle"] != JsonValue::Null {
                    return Ok(Tab{
                        id: json["value"]["handle"].to_string(),
                        session: session
                    });
                } else if json["value"]["error"] != JsonValue::Null {
                    return Err(format!("{}", json["value"]["error"]));
                } else {
                    return Err(format!("Missing field(s) from selenium response. {}", json));
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }

    pub fn select(&mut self) -> Result<(), String> {
        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/window");
        let postdata = object! {
            "handle" => self.id.clone(),
        };

        // send command
        let res = self.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        if let Err(e) = res {
            return Err(format!("{}", e));
        }
        let mut res = res.unwrap();

        // read response
        if let Ok(text) = &res.text() {
            if let Ok(json) = json::parse(text) {
                if json["value"] == JsonValue::Null {
                    return Ok(());
                } else if json["error"] != JsonValue::Null {
                    return Err(json["error"].to_string());
                } else {
                    return Err(String::from("Selenium returned a empty response."));
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }

    pub fn navigate(&mut self, url: &str) -> Result<(), String> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/url");
        let postdata = object! {
            "url" => url,
        };

        // send command
        let res = self.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        if let Err(e) = res {
            return Err(format!("{}", e));
        }
        let mut res = res.unwrap();

        // read response
        if let Ok(text) = &res.text() {
            if let Ok(json) = json::parse(text) {
                if json["value"] == JsonValue::Null {
                    return Ok(());
                } else if json["error"] != JsonValue::Null {
                    return Err(json["error"].to_string());
                } else {
                    return Err(String::from("Selenium returned a empty response."));
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/window");

        // send command
        let res = self.session
            .client
            .delete(&request_url)
            .send();
        if let Err(e) = res {
            return Err(format!("{}", e));
        }
        let mut res = res.unwrap();

        // read response
        if let Ok(text) = &res.text() {
            if let Ok(json) = json::parse(text) {
                if json["error"] == JsonValue::Null {
                    return Ok(());
                } else {
                    return Err(json["error"].to_string());
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }
}