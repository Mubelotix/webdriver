use crate::tab::*;
use crate::error::*;
use crate::enums::{Selector, WebdriverObject};
use json::*;
use std::result::Result;
use log::{debug, info, warn, error};
use crate::http_requests::{execute_script_sync, click_on_element, get_element_text};

pub struct Element<'a> {
    id: String,
    tab: &'a Tab,
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
        request_url += &self.tab.get_session_id();
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
        self.tab.select()?;
        get_element_text(&self.tab.session_id, &self.id)
    }

    pub fn click(&mut self) -> Result<(), WebdriverError> {
        self.tab.select()?;
        
        // TODO watch the bug
        warn!("Using javascript click because of a bug in geckodriver where and error hapen but is not reported to us.");
        if let Ok(()) = execute_script_sync(&self.tab.session_id, "var element = document.evaluate(arguments[0], document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;element.click();", vec![self.stored_selector.1]) {
            return Ok(());
        } else {
            error!("Failed to click with javascript. Using normal method.");
        }

        match click_on_element(&self.tab.session_id, &self.id) {
            Ok(()) => {
                Ok(())
            }
            Err(error) if error == WebdriverError::ElementNotInteractable || error == WebdriverError::ElementClickIntercepted => {
                Ok(())
            },
            Err(error) => {
                return Err(error)
            }
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