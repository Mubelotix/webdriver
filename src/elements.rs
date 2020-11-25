use log::{error, warn};
use serde_json::{json, Value};
use std::{rc::Rc, result::Result};

use crate::{
    enums::WebdriverObject,
    error::*,
    http_requests::*,
};

pub const ELEMENT_ID: &str = "element-6066-11e4-a52e-4f735466cecf";

pub struct Element {
    id: String,
    session_id: Rc<String>,
    tab_id: Rc<String>,
}

impl Element {
    pub(crate) fn new(id: String, session_id: Rc<String>, tab_id: Rc<String>) -> Self {
        Element {
            id,
            session_id,
            tab_id,
        }
    }

    pub fn select_tab(&self) -> Result<(), WebdriverError> {
        select_tab(&self.session_id, &self.tab_id)
    }

    pub fn type_text(&mut self, text: &str) -> Result<(), WebdriverError> {
        self.select_tab()?;
        send_text_to_element(&self.session_id, &self.id, text)
    }

    pub fn switch_to(&mut self) -> Result<(), WebdriverError> {
        self.select_tab()?;
        switch_to_frame(&self.session_id, &self.id)
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

    pub fn get_rect(&self) -> Result<ElementRect, WebdriverError> {
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
        if let Ok(()) = execute_script_sync(
            &self.session_id,
            "arguments[0].click();",
            vec![self.as_json_object()],
        ) {
            return Ok(());
        } else {
            error!("Failed to click with javascript. Using normal method.");
        }

        match click_on_element(&self.session_id, &self.id) {
            Err(WebdriverError::BrowserError(BrowserError::ElementNotInteractable))
            | Err(WebdriverError::BrowserError(BrowserError::ElementClickIntercepted)) => Ok(()),
            whatever => whatever,
        }
    }

    pub fn as_json_object(&self) -> Value {
        json!({
            ELEMENT_ID: self.id.as_str(),
        })
    }

    pub fn scroll_into_view(&self) -> Result<(), WebdriverError> {
        execute_script_sync(
            &self.session_id,
            "arguments[0].scrollIntoView();",
            vec![self.as_json_object()],
        )
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl WebdriverObject for Element {
    fn get_id(&self) -> &String {
        &self.id
    }
}
