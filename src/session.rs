//! Sessions allow you to control tabs

use crate::enums::*;
use crate::error::*;
use crate::http_requests::*;
use crate::tab::*;
use crate::timeouts::*;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use serde_json::*;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::result::Result;
use std::thread;
use std::time::Duration;

/// This is the more important object.
/// Tabs can be accessed within the session.
///
/// # Example
///
/// ```rust
/// use lw_webdriver::{session::Session, enums::Browser};
///
/// let mut session = Session::new(Browser::Firefox, false).unwrap();
///
/// // accessing default tab
/// session.tabs[0].navigate("http://example.com/").unwrap();
///
/// // creating a new tab and access it
/// session.open_tab().unwrap();
/// session.tabs[1].navigate("https://mubelotix.dev/").unwrap();
/// ```
pub struct Session {
    id: Rc<String>,
    webdriver_process: Option<std::process::Child>,
}

impl Session {
    /// Create a session of a specific [browser](https://to.do/).
    /// Headless mean that the browser will be opened but not displayed (useful for servers).
    /// The crate will request a webdriver server at http://localhost:4444.
    /// If no webdriver is listening, one will be launched, but the program ([geckodriver](https://to.do/) or [chromedriver](https://to.do/))
    /// must be located at the same place than the running program.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use lw_webdriver::{session::Session, enums::Browser};
    /// let mut session = Session::new(Browser::Firefox, false).unwrap();
    /// ```
    pub fn new(browser: Browser, headless: bool) -> Result<Self, WebdriverError> {
        info! {"Creating a session..."};
        let result = Session::new_session(browser, headless);

        if let Err(WebdriverError::HttpRequestError(_e)) = result {
            warn! {"No webdriver launched."}
            if cfg!(unix) {
                if browser == Browser::Firefox {
                    info! {"Launching geckodriver..."}
                    let p = Command::new("./geckodriver")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                        .expect("Failed to start process.");
                    thread::sleep(Duration::from_millis(2000));
                    let result = Session::new_session(browser, headless);
                    if let Ok(mut result) = result {
                        info! {"Session created successfully."}
                        result.webdriver_process = Some(p);
                        return Ok(result);
                    } else if let Err(e) = result {
                        error!("Failed to create session. error : {:?}.", e);
                        return Err(e);
                    } else {
                        unreachable!();
                    }
                } else {
                    info! {"Launching chromedriver..."}
                    let p = Command::new("./chromedriver")
                        .arg("--port=4444")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                        .expect("Failed to start process");
                    thread::sleep(Duration::from_millis(2000));
                    let result = Session::new_session(browser, headless);
                    if let Ok(mut result) = result {
                        info! {"Session created successfully."}
                        result.webdriver_process = Some(p);
                        return Ok(result);
                    } else if let Err(e) = result {
                        error!("Failed to create session. error : {:?}.", e);
                        return Err(e);
                    } else {
                        unreachable!();
                    }
                }
            } else {
                panic!("Please launch the webdriver manually.")
            }
        } else {
            result
        }
    }

    fn new_session(browser: Browser, headless: bool) -> Result<Self, WebdriverError> {
        // Detect platform
        let platform = Platform::current();
        if let Platform::Unknow = platform {
            return Err(WebdriverError::UnsupportedPlatform);
        }

        // Generate capabilities
        let post_data = match browser {
            Browser::Firefox => {
                if headless {
                    json!({
                        "capabilities": {
                            "alwaysMatch": {
                                "platformName": platform.as_str(),
                                "browserName": browser.as_str(),
                                "moz:firefoxOptions": {
                                    "args": ["-headless"]
                                },
                            }
                        }
                    })
                } else {
                    json!({
                        "capabilities": {
                            "alwaysMatch": {
                                "platformName": platform.as_str(),
                                "browserName": browser.as_str()
                            }
                        }
                    })
                }
            }
            Browser::Chrome => {
                if headless {
                    json!({
                        "capabilities": {
                            "alwaysMatch": {
                                "platformName": platform.as_str(),
                                "browserName": browser.as_str(),
                                "goog:chromeOptions": {
                                    "args": ["-headless"]
                                }
                            }
                        }
                    })
                } else {
                    json!({
                        "capabilities": {
                            "alwaysMatch": {
                                "platformName": platform.as_str(),
                                "browserName": browser.as_str()
                            }
                        }
                    })
                }
            }
        };

        // Send request
        let session_id = new_session(post_data)?.sessionId;
        let mut session = Session {
            id: Rc::new(session_id),
            webdriver_process: None,
        };

        session.update_tabs()?;

        Ok(session)
    }

    /// Create a new tab in the session.
    /// The tab will be directly accessible from the session (no call to [update_tabs()](https://to.do/) needed).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use lw_webdriver::{session::Session, enums::Browser};
    /// let mut session = Session::new(Browser::Firefox, false).unwrap();
    ///
    /// assert_eq!(session.tabs.len(), 1); // default tab is already opened
    /// session.open_tab().unwrap();
    /// assert_eq!(session.tabs.len(), 2); // new tab is accessible
    /// ```
    pub fn open_tab(&mut self) -> Result<Tab, WebdriverError> {
        let tab_id = new_tab(&self.id)?.handle;

        Ok(Tab::new_from(tab_id, Rc::clone(&self.id), true))
    }

    /// When a tab is created with [open_tab()](https://to.do/) method, it is accessible directly.
    /// But sometimes a tab is created by someone else (from a web page with javascript) and you don't want to care about it!
    /// This tab will not be accessible by your program because you never asked it.
    /// However if you want to access every open tab, call this function.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use lw_webdriver::{session::Session, enums::Browser};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// let mut session = Session::new(Browser::Firefox, false).unwrap();
    ///
    /// // only the default tab is open
    /// assert_eq!(session.tabs.len(), 1);
    ///
    /// // load a website
    /// session.tabs[0].navigate("https://mubelotix.dev/webdriver_tests/open_tab.html").unwrap();
    ///
    /// // observe what is happening
    /// sleep(Duration::from_secs(5));
    ///
    /// // a tab has been opened by another tab but you never asked for it
    /// // you can see two tabs displayed
    /// // but this crate don't show the useless one
    /// assert_eq!(session.tabs.len(), 1);
    ///
    /// // if you want to access it, call this function
    /// session.update_tabs().unwrap();
    ///
    /// // now you can access two tabs!
    /// assert_eq!(session.tabs.len(), 2);
    /// ```
    pub fn update_tabs(&mut self) -> Result<Vec<Tab>, WebdriverError> {
        let mut tabs = Vec::new();
        for tab_id in get_open_tabs(&self.id)? {
            tabs.push(Tab::new_from(tab_id, Rc::clone(&self.id), false));
        }

        Ok(tabs)
    }

    /// This is a simple method getting [timeouts](https://to.do/) of the session.
    pub fn get_timeouts(&self) -> Result<Timeouts, WebdriverError> {
        Ok(get_timeouts(&self.id)?)
    }

    /// This is a simple method setting [timeouts](https://to.do/) of the session.
    pub fn set_timeouts(&mut self, timeouts: Timeouts) -> Result<(), WebdriverError> {
        Ok(set_timeouts(&self.id, timeouts)?)
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl WebdriverObject for Session {
    fn get_id(&self) -> &String {
        &self.id
    }
}

impl Drop for Session {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        if self.webdriver_process.is_some() {
            warn!("Killing webdriver process (may fail silently)");
            self.webdriver_process.take().unwrap().kill();
        }
    }
}
