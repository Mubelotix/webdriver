//! Tabs allow you to control elements

use json::*;
use std::result::Result;
use crate::elements::*;
use crate::session::*;
use crate::enums::*;
use crate::error::*;
use log::{info, warn, error};

/// Tabs are used to load a site and get informations.
/// 
/// ```rust
/// let session = Session::new(Browser::Firefox).unwrap();
/// 
/// // when starting a session, browser open a tab wich is selected
/// let mut default_window = session.get_selected_tab().unwrap(); 
/// 
/// default_window.navigate("https://www.mozilla.org/fr/").unwrap();
/// ```
pub struct Tab<'a> {
    id: String,
    pub session: &'a Session
}

impl<'a> Tab<'a> {
    pub fn new_from(id: String, session: &'a Session) -> Tab<'a> {
        Tab {
            id,
            session
        }
    }

    pub fn get_session_id(&self) -> Option<&String> {
        self.session.get_id()
    }

    /// Create a new tab in a session.
    pub fn new(session: &'a Session) -> Result<Tab<'a>, WebdriverError> {
        info!("Creating tab...");

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/window/new");
        let postdata = object! {};

        // send command
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();

        // Read response
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["handle"].is_string() {
                        Ok(Tab{
                            id: json["value"]["handle"].to_string(),
                            session
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

    /// Select this tab.
    /// Selection is done automatically by this crate when you get informations.
    pub fn select(&self) -> Result<(), WebdriverError> {
        // check if it is needed to select the tab
        if let Ok(id) = self.session.get_selected_tab_id() {
            if id == self.id {
                return Ok(());
            }
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/window");
        let postdata = object! {
            "handle" => self.id.clone(),
        };

        // send command
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();
        
        // Read response
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

    /// Load a website
    pub fn navigate(&mut self, url: &str) -> Result<(), WebdriverError> {
        info!("Navigating to {}...", url);

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/url");
        let postdata = object! {
            "url" => url,
        };

        // send command
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();
        
        // Read response
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

    /// Close the tab.
    pub fn close(&mut self) -> Result<(), WebdriverError> {
        info!("Closing tab...");

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/window");

        // send command
        let res = minreq::delete(&request_url)
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

    /// Find an element in the tab, selected by a [Selector](../enums/enum.Selector.html).
    pub fn find(&self, selector: Selector, tofind: &'a str) -> Result<Option<Element>, WebdriverError> {
        info!("Finding {} with selector {}", tofind, selector.to_string());

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/element");
        let postdata = object! {
            "using" => selector.to_string(),
            "value" => tofind
        };

        // send command
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();

        // Read response
        if let Ok(res) = res {
            if let Ok(text) = res.as_str() {
                if let Ok(json) = json::parse(text) {
                    if !json["value"]["element-6066-11e4-a52e-4f735466cecf"].is_null() {
                        let inter = &*self; // TODO
                        Ok(Some(Element::new(json["value"]["element-6066-11e4-a52e-4f735466cecf"].to_string(), inter, (selector, tofind))))
                    } else if json["value"]["error"].is_string() {
                        let e = WebdriverError::from(json["value"]["error"].to_string());
                        error!("{:?}, response: {}", e, json);
                        if e == WebdriverError::NoSuchElement {
                            Ok(None)
                        } else {
                            Err(e)
                        }
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

    /// Return the url of the current web page.
    pub fn get_url(&self) -> Result<String, WebdriverError> {
        info!("Getting url...");

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/url");

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

    /// Return the title of the tab.
    pub fn get_title(&self) -> Result<String, WebdriverError> {
        info!("Getting title...");

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/title");

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

    /// Navigate to the previous page.
    pub fn back(&mut self) -> Result<(), WebdriverError> {
        info!("Navigating backward...");

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/back");
        let postdata = object! {};

        // send command
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();
        
        // Read response
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

    /// Navigate forward.
    pub fn forward(&mut self) -> Result<(), WebdriverError> {
        info!("Navigating forward...");

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/forward");
        let postdata = object! {};

        // send command
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();
        
        // Read response
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

    /// Refresh the page.
    pub fn refresh(&mut self) -> Result<(), WebdriverError> {
        info!("Refreshing tab...");

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/refresh");
        let postdata = object! {};

        // send command
        let res = minreq::post(&request_url)
            .with_body(postdata.to_string())
            .send();
        
        // Read response
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

    // TODO mutability
    pub fn execute_script(&self, script: &str, args: Vec<&str>) -> Result<(), WebdriverError> {
        info!("Executing javascript script...");

        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/execute/sync");
        let postdata = object!{
            "script" => script,
            "args" => args
        };

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

#[allow(unused_must_use)]
impl<'a> Drop for Tab<'a> {
    fn drop(&mut self) {
        self.close();
    }
}