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
use log::{debug, info, warn, error};
use std::rc::Rc;
use crate::http_requests::*;

pub struct Session {
    id: Rc<String>,
    pub tabs: Vec<Tab>,
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

        // Generate capabilities
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
        
        // Send request
        let session_id = new_session(&post_data.to_string())?;
        let mut session = Session {
            id: Rc::new(session_id),
            tabs: Vec::new(),
            webdriver_process: None
        };

        session.update_tabs()?;

        Ok(session)
    }

    pub fn open_tab(&mut self) -> Result<usize, WebdriverError> {
        let tab_id = new_tab(&self.id)?;
        let new_tab = Tab::new_from(tab_id, Rc::clone(&self.id));
        self.tabs.push(new_tab);

        Ok(self.tabs.len() - 1)
    }

    pub fn update_tabs(&mut self) -> Result<(), WebdriverError> {
        let tabs_id = get_open_tabs(&self.id)?;
        for tab_id in tabs_id {
            if self.tabs.iter().position(|element| element.id == tab_id).is_none() {
                self.tabs.push(Tab::new_from(tab_id, Rc::clone(&self.id)));
            }
        }

        Ok(())
    }

    pub fn get_timeouts(&self) -> Result<Timeouts, WebdriverError> {
        Ok(get_timeouts(&self.id)?)
    }

    pub fn set_timeouts(&mut self, timeouts: Timeouts) -> Result<(), WebdriverError> {
        Ok(set_timeouts(&self.id, timeouts)?)
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl WebdriverObject for Session {
    fn get_id(&self) -> &String {
        &self.id
    }
}

impl Drop for Session {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        for tab in &mut self.tabs {
            tab.close();
        }
        if self.webdriver_process.is_some() {
            warn!("Killing webdriver process (may fail silently)");
            self.webdriver_process.take().unwrap().kill();
        }
    }
}