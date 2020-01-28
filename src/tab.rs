//! Tabs allow you to control elements

use json::*;
use std::result::Result;
use crate::elements::*;
use crate::session::*;
use crate::enums::*;
use crate::error::*;
use log::{info, error};
use std::rc::Rc;
use crate::http_requests::{get_selected_tab, select_tab, navigate, close_active_tab, find_element,
    get_active_tab_url, get_active_tab_title, back, forward, refresh, execute_script_sync};

/// Tabs are used to load a site and get informations.
/// 
/// ```rust
/// # use lw_webdriver::{session::Session, enums::Browser};
/// 
/// let mut session = Session::new(Browser::Firefox, false).unwrap();
/// 
/// // using the default tab
/// session.tabs[0].navigate("https://www.mozilla.org/fr/").unwrap();
/// ```
pub struct Tab {
    pub(crate) id: String,
    pub(crate) session_id: Rc<String>
}

impl Tab {
    pub fn new_from(id: String, session_id: Rc<String>) -> Tab {
        Tab {
            id,
            session_id
        }
    }

    pub fn get_session_id(&self) -> Rc<String> {
        Rc::clone(&self.session_id)
    }

    /// Create a new tab in a session.
    /// This return an immutable reference (in a Result) because the tab is stored in the session.
    pub fn new(session: &mut Session) -> Result<&Tab, WebdriverError> {
        let tab_id = session.open_tab()?;
        Ok(&session.tabs[tab_id])
    }

    /// Select this tab.
    /// Selection is done automatically by this crate when you get informations.
    pub fn select(&self) -> Result<(), WebdriverError> {
        // check if it is needed to select the tab
        if let Ok(id) = get_selected_tab(&self.session_id) {
            if id == self.id {
                return Ok(());
            }
        }

        // select tab
        select_tab(&self.session_id, &self.id)
    }

    /// Load a website
    pub fn navigate(&mut self, url: &str) -> Result<(), WebdriverError> {
        self.select()?;
        navigate(&self.session_id, url)
    }

    /// Find an element in the tab, selected by a [Selector](../enums/enum.Selector.html).
    pub fn find<'a>(&'a self, selector: Selector, tofind: &'a str) -> Result<Option<Element<'a>>, WebdriverError> {
        self.select()?;
        match find_element(&self.session_id, selector, tofind) {
            Ok(id) => {
                Ok(Some(Element::new(id, &self, (selector, tofind))))
            },
            Err(error) if error == WebdriverError::NoSuchElement => {
                Ok(None)
            },
            Err(error) => {
                return Err(error)
            }
        }
    }

    /// Return the url of the current web page.
    pub fn get_url(&self) -> Result<String, WebdriverError> {
        self.select()?;
        get_active_tab_url(&self.session_id)
    }

    /// Return the title of the tab.
    pub fn get_title(&self) -> Result<String, WebdriverError> {
        self.select()?;
        get_active_tab_title(&self.session_id)
    }

    /// Navigate to the previous page.
    pub fn back(&mut self) -> Result<(), WebdriverError> {
        self.select()?;
        back(&self.session_id)
    }

    /// Navigate forward.
    pub fn forward(&mut self) -> Result<(), WebdriverError> {
        self.select()?;
        forward(&self.session_id)
    }

    /// Refresh the page.
    pub fn refresh(&mut self) -> Result<(), WebdriverError> {
        self.select()?;
        refresh(&self.session_id)
    }

    // TODO mutability
    pub fn execute_script(&mut self, script: &str, args: Vec<&str>) -> Result<(), WebdriverError> {
        self.select()?;
        execute_script_sync(&self.session_id, script, args)
    }
}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl WebdriverObject for Tab {
    fn get_id(&self) -> &String {
        &self.id
    }
}

impl Drop for Tab {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        if let Ok(()) = self.select() {
            close_active_tab(&self.session_id);
        }
    }
}