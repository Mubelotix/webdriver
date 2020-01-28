use crate::tab::*;
use crate::error::*;
use crate::enums::{Selector, WebdriverObject};
use json::*;
use std::result::Result;
use log::{debug, info, warn, error};
use std::rc::Rc;
use crate::http_requests::{execute_script_sync, click_on_element, get_element_text, send_text_to_element, get_selected_tab, select_tab};

pub struct Element {
    id: String,
    session_id: Rc<String>,
    tab_id: Rc<String>,
    stored_selector: (Selector, String)
}

impl Element {
    pub fn new(id: String, session_id: Rc<String>, tab_id: Rc<String>, selector: (Selector, String)) -> Self {
        Element{
            id,
            session_id,
            tab_id,
            stored_selector: selector
        }
    }

    fn select_tab(&self) -> Result<(), WebdriverError> {
        // check if it is needed to select the tab
        if let Ok(id) = get_selected_tab(&self.session_id) {
            if id == *self.tab_id {
                return Ok(());
            }
        }

        // select tab
        select_tab(&self.session_id, &self.tab_id)
    }

    pub fn type_text(&mut self, text: &str) -> Result<(), WebdriverError> {
        self.select_tab()?;
        send_text_to_element(&self.session_id, &self.id, text)
    }

    pub fn get_text(&self) -> Result<String, WebdriverError> {
        self.select_tab()?;
        get_element_text(&self.session_id, &self.id)
    }

    pub fn click(&mut self) -> Result<(), WebdriverError> {
        self.select_tab()?;
        
        // TODO watch the bug
        warn!("Using javascript click because of a bug in geckodriver where and error hapen but is not reported to us.");
        if let Ok(()) = execute_script_sync(&self.session_id, "var element = document.evaluate(arguments[0], document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;element.click();", vec![&self.stored_selector.1]) {
            return Ok(());
        } else {
            error!("Failed to click with javascript. Using normal method.");
        }

        match click_on_element(&self.session_id, &self.id) {
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

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl WebdriverObject for Element {
    fn get_id(&self) -> &String {
        &self.id
    }
}