//! Tabs allow you to control elements

use std::{rc::Rc, result::Result, unimplemented};

use serde_json::Value;

use crate::{elements::Element, enums::*, error::*, http_requests::*, session::*};

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
    pub(crate) id: Rc<String>,
    pub(crate) session_id: Rc<String>,
    pub(crate) close_on_drop: bool,
}

impl Tab {
    pub fn new_from(id: String, session_id: Rc<String>, close_on_drop: bool) -> Tab {
        Tab {
            id: Rc::new(id),
            session_id,
            close_on_drop
        }
    }

    pub fn get_session_id(&self) -> Rc<String> {
        Rc::clone(&self.session_id)
    }

    /// Create a new tab in a session.
    /// This return an immutable reference (in a Result) because the tab is stored in the session.
    pub fn new(session: &mut Session) -> Result<Tab, WebdriverError> {
        let tab = session.open_tab()?;
        Ok(tab)
    }

    /// Select this tab.
    /// Selection is done automatically by this crate when you get informations.
    pub fn select(&self) -> Result<(), WebdriverError> {
        // check if it is needed to select the tab
        if let Ok(id) = get_selected_tab(&self.session_id) {
            if id == *self.id {
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
    pub fn find(
        &mut self,
        _selector: Selector,
        _tofind: &str,
    ) -> Result<Option<Element>, WebdriverError> {
        self.select()?;
        unimplemented!()
        /*match find_element(&self.session_id, &selector, &tofind) {
            Ok(id) => Ok(Some(Element::new(
                id,
                Rc::clone(&self.session_id),
                Rc::clone(&self.id),
            ))),
            Err(WebdriverError::NoSuchElement) => Ok(None),
            Err(error) => Err(error),
        }*/
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

    pub fn execute_script(&self, script: &str, args: Vec<Value>) -> Result<(), WebdriverError> {
        self.select()?;
        execute_script_sync(&self.session_id, script, args)
    }

    pub fn switch_to_parent_frame(&self) -> Result<(), WebdriverError> {
        self.select()?;
        switch_to_parent_frame(&self.session_id)
    }

    pub fn get_cookies(&self) -> Result<Vec<Cookie>, WebdriverError> {
        self.select()?;
        get_all_cookies(&self.session_id)
    }

    pub fn set_cookie(
        &self,
        cookie: Cookie,
    ) -> Result<(), WebdriverError> {
        self.select()?;
        set_cookie(&self.session_id, cookie)
    }

    pub fn set_cookies(
        &self,
        cookies: Vec<Cookie>,
    ) -> Result<(), WebdriverError> {
        self.select()?;
        for cookie in cookies {
            set_cookie(&self.session_id, cookie)?
        }
        Ok(())
    }

    pub fn get_page_source(&self) -> Result<String, WebdriverError> {
        self.select()?;
        unimplemented!()
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
        if self.close_on_drop {
            if let Ok(()) = self.select() {
                close_active_tab(&self.session_id);
            }
        }
    }
}
