use json::*;
use reqwest::Client;
use std::time::Duration;
use std::result::Result;
use crate::enums::*;
use crate::timeouts::*;
pub use crate::tab::*;

pub struct Session<'a> {
    id: Option<String>,
    pub client: Client,
    pub tabs: Vec<Tab<'a>>
}

impl<'a> Session<'a> {
    pub fn new(browser: Browser) -> Result<Self, String> {
        // Detect platform
        let platform = Platform::current();
        match platform {
            Platform::Unknow => return Err(String::from("Can't detect platform.")),
            _ => ()
        }

        // Create http client
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build();
        if client.is_err() {
            return Err(String::from("Can't create http client."));
        }
        let client = client.unwrap();
        
        // Create session
        let mut session = Session {
            id: None,
            client,
            tabs: Vec::new()
        };
        let post_data = object!{
            "capabilities" => object!{
                "alwaysMatch" => object!{
                    "platformName" => platform.to_string(),
                    "browserName" => browser.to_string(),
                }
            }
        };
        let res = session
            .client
            .post("http://localhost:4444/wd/hub/session")
            .body(post_data.to_string())
            .send();
        if let Err(e) = res {
            return Err(format!("{}", e));
        }
        let mut res = res.unwrap();

        // Eventually read error
        let res = json::parse(&res.text().expect("Can't read response body.")).expect("Can't parse response body to json.");
        if res["value"]["error"] != JsonValue::Null {
            return Err(match res["value"]["error"].as_str().expect("Can't read error.") {
                _ => {
                    println!("{}", res["value"]["error"].as_str().expect("Can't read error."));
                    "Unknow Error".to_string()
                },
            });
        }

        session.id = Some(res["value"]["sessionId"].to_string());

        Ok(session)
    }

    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn get_all_tabs(&self) -> Result<Vec<Tab>, String> {
        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.get_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/window/handles");

        // send command
        let res = self
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
                if json["value"]["handles"] != JsonValue::Null {
                    let mut tabs: Vec<Tab> = Vec::new();
                    tabs.clear();
                    let mut i = 0;
                    while !json["value"]["handles"][i].is_null() {
                        i += 1;
                        tabs.push(Tab::new_from(json["value"]["handles"][i].to_string(), &self));
                    }
                    return Ok(tabs);
                } else {
                    println!("{}", json);
                    return Err(String::from("Selenium returned a null result."));
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }

    pub fn get_selected_tab(&self) -> Result<Tab, String> {
        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.get_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/window");

        // send command
        let res = self
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
                    return Ok(Tab::new_from(json["value"].to_string(), &self));
                } else {
                    eprintln!("{:?}", json);
                    return Err(String::from("Selenium returned a null result."));
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }

    pub fn get_timeouts(&self) -> Result<Timeouts, String> {
        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.get_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/timeouts");

        // send command
        let res = self
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
                if json["value"]["pageLoad"].is_number() &&
                    json["value"]["implicit"].is_number() 
                {
                    return Ok(Timeouts{
                        script: json["value"]["script"].as_usize(),
                        page_load: json["value"]["pageLoad"].as_usize().unwrap(),
                        implicit: json["value"]["implicit"].as_usize().unwrap(),
                    });
                } else {
                    return Err("unknow error".to_string());
                }
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }

    pub fn set_timeouts(&self, timeouts: Timeouts) -> Result<(), String> {
        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.get_id() {
            request_url += &id;
        } else {
            return Err(String::from("Session does not exist."));
        }
        request_url.push_str("/timeouts");
        let postdata = timeouts.to_json();

        // send command
        let res = self
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
                if json["value"].is_null() {
                    return Ok(());
                } else {
                    eprintln!("{}", json.to_string());
                    return Err("error".to_string());
                }
                
            } else {
                return Err(String::from("Can't parse selenium response to json."));
            }
        } else {
            return Err(String::from("Can't read selenium response."));
        }
    }
}