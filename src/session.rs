//! Sessions allow you to control tabs

use json::*;
use std::time::Duration;
use std::result::Result;
use crate::enums::*;
use crate::timeouts::*;
use crate::tab::*;
use crate::error::*;
use std::process::{Command, Stdio};
use std::thread;
use log::{info, warn, error};

pub struct Session {
    id: Option<String>,
    webdriver_process: Option<std::process::Child>,
}

impl Session {
    pub fn new(browser: Browser, headless: bool) -> Result<Self, WebdriverError> {
        info!{"Creating a session..."};
        let result = Session::new_session(browser, headless);

        if let Err(WebdriverError::FailedRequest) = result {
            warn!{"No webdriver launched."}
            if cfg!(unix) {
                if browser == Browser::Firefox {
                    info!{"Launching geckodriver..."}
                    let p = Command::new("./geckodriver")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                        .expect("Failed to start process.");
                    thread::sleep(Duration::from_millis(2000));
                    let result = Session::new_session(browser, headless);
                    if let Ok(mut result) = result {
                        info!{"Session created successfully."}
                        result.webdriver_process = Some(p);
                        return Ok(result);
                    } else if let Err(e) = result {
                        error!("Failed to create session. error : {:?}.", e);
                        return Err(e);
                    }
                } else {
                    info!{"Launching chromedriver..."}
                    let p = Command::new("./chromedriver")
                        .arg("--port=4444")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                        .expect("Failed to start process");
                    thread::sleep(Duration::from_millis(2000));
                    let result = Session::new_session(browser, headless);
                    if let Ok(mut result) = result {
                        info!{"Session created successfully."}
                        result.webdriver_process = Some(p);
                        return Ok(result);
                    } else if let Err(e) = result{
                        error!("Failed to create session. error : {:?}.", e);
                        return Err(e);
                    }
                }
            } else {
                panic!("Please launch the webdriver manually.")
            }
        } else {
            return result;
        }
        
        result
    }

    fn new_session(browser: Browser, headless: bool)  -> Result<Self, WebdriverError> {
        // Detect platform
        let platform = Platform::current();
        if let Platform::Unknow = platform {
            return Err(WebdriverError::UnsupportedPlatform);
        }

        // Create session
        let mut session = Session {
            id: None,
            webdriver_process: None,
        };
        let post_data = match browser {
            Browser::Firefox => {
                if headless {
                    object!{
                        "capabilities" => object!{
                            "alwaysMatch" => object!{
                                "platformName" => platform.to_string(),
                                "browserName" => browser.to_string(),
                                "moz:firefoxOptions" => object! {
                                    "args" => array!{"-headless"}
                                },
                            }
                        }
                    }
                } else {
                    object!{
                        "capabilities" => object!{
                            "alwaysMatch" => object!{
                                "platformName" => platform.to_string(),
                                "browserName" => browser.to_string()
                            }
                        }
                    }
                }
            },
            Browser::Chrome => {
                if headless {
                    object!{
                        "capabilities" => object!{
                            "alwaysMatch" => object!{
                                "platformName" => platform.to_string(),
                                "browserName" => browser.to_string(),
                                "goog:chromeOptions" => object! {
                                    "args" => array!{"-headless"}
                                }
                            }
                        }
                    }
                } else {
                    object!{
                        "capabilities" => object!{
                            "alwaysMatch" => object!{
                                "platformName" => platform.to_string(),
                                "browserName" => browser.to_string()
                            }
                        }
                    }
                }
            }
        };

        info!("Creating session... capabilities = {}", post_data);
        
        let res = minreq::post("http://localhost:4444/session")
            .with_body(post_data.to_string())
            .send();

        // Read error
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["sessionId"].is_string() {
                        session.id = Some(json["value"]["sessionId"].to_string());
                        Ok(session)
                    } else if json["value"]["error"].is_string() {
                        error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        error!("WebdriverError::InvalidResponse, response: {}", json);
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    error!("WebdriverError::InvalidResponse, error: {:?}", json::parse(text));
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                error!("WebdriverError::InvalidResponse, error: {:?}", res.as_str());
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            error!("WebdriverError::FailedRequest, error: {:?}", res);
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn get_all_tabs(&self) -> Result<Vec<Tab>, WebdriverError> {
        info!("Getting all tabs...");

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/window/handles");

        // send command
        let res = minreq::get(&request_url)
            .send();
        
        // Read response
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if !json["value"]["handles"].is_null() {
                        let mut tabs: Vec<Tab> = Vec::new();
                        tabs.clear();
                        let mut i = 0;
                        while !json["value"]["handles"][i].is_null() {
                            i += 1;
                            tabs.push(Tab::new_from(json["value"]["handles"][i].to_string(), &self));
                        }
                        Ok(tabs)
                    } else if json["value"]["error"].is_string() {
                        error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        error!("WebdriverError::InvalidResponse, response: {}", json);
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    error!("WebdriverError::InvalidResponse, error: {:?}", json::parse(text));
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                error!("WebdriverError::InvalidResponse, error: {:?}", res.as_str());
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            error!("WebdriverError::FailedRequest, error: {:?}", res);
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn get_selected_tab(&self) -> Result<Tab, WebdriverError> {
        info!("Getting selected tab...");
        Ok(Tab::new_from(self.get_selected_tab_id()?, self))
    }

    pub fn get_selected_tab_id(&self) -> Result<String, WebdriverError> {
        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/window");

        // send command
        let res = minreq::get(&request_url)
            .send();
        
        // Read error
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_string() {
                        Ok(json["value"].to_string())
                    } else if json["value"]["error"].is_string() {
                        error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        error!("WebdriverError::InvalidResponse, response: {}", json);
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    error!("WebdriverError::InvalidResponse, error: {:?}", json::parse(text));
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                error!("WebdriverError::InvalidResponse, error: {:?}", res.as_str());
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            error!("WebdriverError::FailedRequest, error: {:?}", res);
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn get_timeouts(&self) -> Result<Timeouts, WebdriverError> {
        info!("Getting timeouts...");

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/timeouts");

        // send command
        let res = minreq::get(&request_url)
            .send();
        
        // Read error
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["pageLoad"].is_number() && json["value"]["implicit"].is_number() {
                        Ok(Timeouts{
                            script: json["value"]["script"].as_usize(),
                            page_load: json["value"]["pageLoad"].as_usize().unwrap(),
                            implicit: json["value"]["implicit"].as_usize().unwrap(),
                        })
                    } else if json["value"]["error"].is_string() {
                        error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        error!("WebdriverError::InvalidResponse, response: {}", json);
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    error!("WebdriverError::InvalidResponse, error: {:?}", json::parse(text));
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                error!("WebdriverError::InvalidResponse, error: {:?}", res.as_str());
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            error!("WebdriverError::FailedRequest, error: {:?}", res);
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn set_timeouts(&mut self, timeouts: Timeouts) -> Result<(), WebdriverError> {
        info!("Setting timeouts : {:?}", timeouts);

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/timeouts");
        let postdata = timeouts.to_json();

        // send command
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();
        
        // Read error
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_null() {
                        Ok(())
                    } else if json["value"]["error"].is_string() {
                        error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        error!("WebdriverError::InvalidResponse, response: {}", json);
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    error!("WebdriverError::InvalidResponse, error: {:?}", json::parse(text));
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                error!("WebdriverError::InvalidResponse, error: {:?}", res.as_str());
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            error!("WebdriverError::FailedRequest, error: {:?}", res);
            Err(WebdriverError::FailedRequest)
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.webdriver_process.is_some() {
            warn!("Killing webdriver process (may fail silently)");
            self.webdriver_process.take().unwrap().kill();
        }
    }
}