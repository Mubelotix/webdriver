use crate::tab::*;
use crate::error::*;
use crate::enums::{Selector, WebdriverObject};
use json::*;
use std::result::Result;
use log::{debug, info, warn, error};
use std::rc::Rc;
use crate::http_requests::{execute_script_sync, click_on_element, get_element_text, send_text_to_element, get_selected_tab, select_tab,
    get_element_attribute, get_element_css_value, get_element_property, get_element_tag_name, is_element_enabled, get_element_rect};

pub struct Element {
    id: String,
    session_id: Rc<String>,
    tab_id: Rc<String>
}

impl Element {
    pub fn new(id: String, session_id: Rc<String>, tab_id: Rc<String>) -> Self {
        Element{
            id,
            session_id,
            tab_id
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

    pub fn get_attribute(&self, attribute_name: &str) -> Result<String, WebdriverError> {
        self.select_tab()?;
        get_element_attribute(&self.session_id, &self.id, attribute_name)
    }

    pub fn get_tag_name(&self) -> Result<String, WebdriverError> {
        self.select_tab()?;
        get_element_tag_name(&self.session_id, &self.id)
    }

    pub fn get_css_value(&self, property_name: &str) -> Result<String, WebdriverError> {
        self.select_tab()?;
        get_element_css_value(&self.session_id, &self.id, property_name)
    }

    pub fn get_property(&self, property_name: &str) -> Result<String, WebdriverError> {
        self.select_tab()?;
        get_element_property(&self.session_id, &self.id, property_name)
    }

    pub fn get_rect(&self) -> Result<((usize, usize), (usize, usize)), WebdriverError> {
        self.select_tab()?;
        get_element_rect(&self.session_id, &self.id)
    }

    pub fn is_enabled(&self) -> Result<bool, WebdriverError> {
        self.select_tab()?;
        is_element_enabled(&self.session_id, &self.id)
    }

    pub fn click(&mut self) -> Result<(), WebdriverError> {
        self.select_tab()?;
        
        // TODO watch the bug
        warn!("Using javascript click because of a bug in geckodriver where and error hapen but is not reported to us.");
        if let Ok(()) = execute_script_sync(&self.session_id, "arguments[0].click();", vec![self.as_json_object()]) {
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

    pub fn as_json_object(&self) -> JsonValue {
        object!{ "element-6066-11e4-a52e-4f735466cecf" => self.id.as_str() }
    }

    pub fn scroll_into_view(&self) -> Result<(), WebdriverError> {
        execute_script_sync(&self.session_id, "", vec![self.as_json_object()])
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