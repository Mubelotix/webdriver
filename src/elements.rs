use crate::tab::*;
use crate::error::*;
use crate::enums::{Selector, WebdriverObject};
use json::*;
use std::result::Result;
use log::{debug, info, warn, error};

pub struct Element<'a> {
    id: String,
    tab: &'a Tab<'a>,
    stored_selector: (Selector, &'a str)
}

impl<'a> Element<'a> {
    pub fn new(id: String, tab: &'a Tab, selector: (Selector, &'a str)) -> Self {
        Element{
            id,
            tab,
            stored_selector: selector
        }
    }

    pub fn type_text(&mut self, text: &str) -> Result<(), WebdriverError> {
        info!("Sending \"{}\" to element...", text);

        // select tab
        self.tab.select()?;

        // Build request
        let mut request_url = String::from("http://localhost:4444/session/");
        request_url += &self.tab.get_session().get_id().to_string();
        request_url.push_str("/element/");
        request_url += &self.id.to_string();
        request_url.push_str("/value");
        let postdata = object! {
            "text" => text
        };

        // Send request
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();
        
        // Read response
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["error"].is_string() {
                        error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Ok(())
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

    pub fn get_text(&self) -> Result<String, WebdriverError> {
        info!("Getting text of element...");

        // select tab
        self.tab.select()?;

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        request_url += &self.tab.get_session().get_id().to_string();
        request_url.push_str("/element/");
        request_url += &self.id.to_string();
        request_url.push_str("/text");

        // send command
        let res = minreq::get(&request_url)
            .send();
        
        // Read response
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

    pub fn click(&mut self) -> Result<(), WebdriverError> {
        info!("Clicking on element...");
        
        // TODO watch the bug
        warn!("Using javascript click because of a bug in geckodriver where and error hapen but is not reported to us.");
        if let Ok(()) = self.tab.execute_script("var element = document.evaluate(arguments[0], document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;element.click();", vec![self.stored_selector.1]) {
            return Ok(());
        } else {
            error!("Failed to click with javascript. Using normal method.");
        }

        // select tab
        debug!("Selecting tab");
        self.tab.select()?;

        // Build request
        debug!("Building request");
        let mut request_url = String::from("http://localhost:4444/session/");
        request_url += &self.tab.get_session().get_id().to_string();
        request_url.push_str("/element/");
        request_url += &self.id.to_string();
        request_url.push_str("/click");
        let postdata = object! {
        };

        // Send request
        debug!("Sending request");
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();
        
        // Read response
        debug!("Reading response");
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["error"].is_string() {
                        let e = WebdriverError::from(json["value"]["error"].to_string());
                        if e == WebdriverError::ElementClickIntercepted {
                            warn!("Element not interractable with webdriver command : trying with javascript.");

                            match self.stored_selector.0 {
                                Selector::XPath => {
                                    if let Ok(()) = self.tab.execute_script("var element = document.evaluate(arguments[0], document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;element.click();", vec![self.stored_selector.1]) {
                                        info!("Successfully clicked with javascript.");
                                        return Ok(());
                                    } else {
                                        error!("Failed to click with javascript.");
                                    }
                                },
                                _ => error!("unimplemented selector {}", self.stored_selector.0.to_string()),
                            };
                        }
                        error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        debug!("Clicked successfully {}", json);
                        Ok(())
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

impl PartialEq for Element<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl WebdriverObject for Element<'_> {
    fn get_id(&self) -> &String {
        &self.id
    }
}