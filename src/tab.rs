use json::*;
use std::result::Result;
use crate::elements::*;
use crate::session::*;
use crate::enums::*;
use crate::error::*;

pub struct Tab<'a> {
    id: String,
    pub session: &'a Session<'a>
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

    pub fn new(session: &'a Session) -> Result<Tab<'a>, WebdriverError> {
        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/window/new");
        let postdata = object! {};

        // send command
        let res = session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();

        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["handle"].is_string() {
                        Ok(Tab{
                            id: json["value"]["handle"].to_string(),
                            session
                        })
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn select(&self) -> Result<(), WebdriverError> {
        // check if it is needed to select the tab
        if let Ok(id) = self.session.get_selected_tab_id() {
            if id == self.id {
                return Ok(());
            }
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
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
        let res = self.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_null() {
                        Ok(())
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn navigate(&mut self, url: &str) -> Result<(), WebdriverError> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
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
        let res = self.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_null() {
                        Ok(())
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn close(&mut self) -> Result<(), WebdriverError> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/window");

        // send command
        let res = self.session
            .client
            .delete(&request_url)
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn find(&self, selector: Selector, tofind: &str) -> Result<Option<Element>, WebdriverError> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
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
        let res = self.session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();

        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if !json["value"]["element-6066-11e4-a52e-4f735466cecf"].is_null() {
                        let inter = &*self; // TODO
                        Ok(Some(Element::new(json["value"]["element-6066-11e4-a52e-4f735466cecf"].to_string(), inter)))
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn get_url(&self) -> Result<String, WebdriverError> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/url");

        // send command
        let res = self
            .session
            .client
            .get(&request_url)
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_string() {
                        Ok(json["value"].to_string())
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn get_title(&self) -> Result<String, WebdriverError> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/title");

        // send command
        let res = self
            .session
            .client
            .get(&request_url)
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_string() {
                        Ok(json["value"].to_string())
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn back(&mut self) -> Result<(), WebdriverError> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/back");
        let postdata = object! {};

        // send command
        let res = self
            .session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_null() {
                        Ok(())
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn forward(&mut self) -> Result<(), WebdriverError> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/forward");
        let postdata = object! {};

        // send command
        let res = self
            .session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_null() {
                        Ok(())
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            Err(WebdriverError::FailedRequest)
        }
    }

    pub fn refresh(&mut self) -> Result<(), WebdriverError> {
        // select tab
        if let Err(e) = self.select() {
            return Err(e);
        }

        // build command
        let mut request_url = String::from("http://localhost:4444/wd/hub/session/");
        if let Some(id) = self.session.get_id() {
            request_url += &id;
        } else {
            return Err(WebdriverError::NoSuchWindow);
        }
        request_url.push_str("/refresh");
        let postdata = object! {};

        // send command
        let res = self
            .session
            .client
            .post(&request_url)
            .body(postdata.to_string())
            .send();
        
        // Read response
        if let Ok(mut res) = res {
            if let Ok(text) = &res.text() {
                if let Ok(json) = json::parse(text) {
                    if json["value"].is_null() {
                        Ok(())
                    } else if json["value"]["error"].is_string() {
                        Err(WebdriverError::from(json["value"]["error"].to_string()))
                    } else {
                        Err(WebdriverError::InvalidResponse)
                    }
                } else {
                    Err(WebdriverError::InvalidResponse)
                }
            } else {
                Err(WebdriverError::InvalidResponse)
            }
        } else {
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