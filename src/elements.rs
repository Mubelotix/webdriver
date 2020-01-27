use crate::tab::*;
use crate::error::*;
use crate::enums::{Selector, WebdriverObject};
use json::*;
use std::result::Result;
use log::{debug, info, warn, error};
use crate::http_requests::{execute_script_sync, click_on_element, get_element_text, send_text_to_element};

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
        self.tab.select()?;
        send_text_to_element(&self.tab.session_id, &self.id, text)
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