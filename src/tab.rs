use json::*;
use std::result::Result;
use crate::elements::*;
use crate::session::*;
use crate::enums::*;

pub struct Tab<'a> {
    id: String,
    pub session: &'a Session<'a>
}

impl<'a> Tab<'a> {
    pub fn new_from(id: String, session: &'a Session) -> Tab<'a> {
        Tab {
            id,
            session
        }
    }

    pub fn get_session_id(&self) -> Option<&String> {
        self.session.get_id()
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

    pub fn select(&self) -> Result<(), String> {
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

    pub fn find(&self, selector: Selector, tofind: &str) -> Result<Option<Element>, String> {
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
        request_url.push_str("/element");
        let postdata = object! {
            "using" => selector.to_string(),
            "value" => tofind
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
                if json["value"]["element-6066-11e4-a52e-4f735466cecf"] != JsonValue::Null {
                    let inter = &*self;
                    return Ok(Some(Element::new(json["value"]["element-6066-11e4-a52e-4f735466cecf"].to_string(), inter)));
                } else if json["value"]["error"] != JsonValue::Null {
                    match json["value"]["error"].as_str() {
                        Some("no such element") => return Ok(None),
                        Some(_) => return Err("error".to_string()),
                        None => return Err("unknow error".to_string())
                    }
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

    pub fn get_url(&self) -> Result<String, String> {
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

        // send command
        let res = self
            .session
            .client
            .get(&request_url)
            .send();
        if let Err(e) = res {
            return Err(format!("{}", e));
        }
        let mut res = res.unwrap();

        // read response
        if let Ok(text) = &res.text() {
            if let Ok(json) = json::parse(text) {
                if json["value"] != JsonValue::Null {
                    return Ok(json["value"].to_string());
                } else {
                    return Err(String::from("Selenium returned a null result."));
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }

    pub fn get_title(&self) -> Result<String, String> {
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
        request_url.push_str("/title");

        // send command
        let res = self
            .session
            .client
            .get(&request_url)
            .send();
        if let Err(e) = res {
            return Err(format!("{}", e));
        }
        let mut res = res.unwrap();

        // read response
        if let Ok(text) = &res.text() {
            if let Ok(json) = json::parse(text) {
                if json["value"] != JsonValue::Null {
                    return Ok(json["value"].to_string());
                } else {
                    return Err(String::from("Selenium returned a null result."));
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }

    pub fn back(&mut self) -> Result<(), String> {
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
        request_url.push_str("/back");
        let postdata = object! {};

        // send command
        let res = self
            .session
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

    pub fn forward(&mut self) -> Result<(), String> {
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
        request_url.push_str("/forward");
        let postdata = object! {};

        // send command
        let res = self
            .session
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

    pub fn refresh(&mut self) -> Result<(), String> {
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
        request_url.push_str("/refresh");
        let postdata = object! {};

        // send command
        let res = self
            .session
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
}

impl<'a> Drop for Tab<'a> {
    fn drop(&mut self) {
        self.close();
    }
}