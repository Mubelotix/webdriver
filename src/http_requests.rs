use crate::{elements::ELEMENT_ID, enums::Selector, error::WebdriverError, timeouts::Timeouts};
use log::{debug, error, warn};
use serde_json::{self, json, Value};

type MinReqResult = Result<minreq::Response, minreq::Error>;

fn handle_response(result: MinReqResult) -> Result<Value, WebdriverError> {
    if let Ok(result) = result {
        if let Ok(text) = result.as_str() {
            match serde_json::from_str::<Value>(text) {
                Ok(json) => {
                    let error_value = &json["value"]["error"];

                    if error_value.is_string() {
                        let webdriver_error = WebdriverError::from(error_value.to_string());
                        error!("{:?}, response: {}", webdriver_error, json);
                        Err(webdriver_error)
                    } else {
                        Ok(json)
                    }
                }
                Err(err) => {
                    error!(
                        "WebdriverError::InvalidResponse (not json), text: {:?}, error: {:?}",
                        text, err,
                    );
                    Err(WebdriverError::InvalidResponse)
                }
            }
        } else {
            error!(
                "WebdriverError::InvalidResponse (not utf8), error: {:?}",
                result.as_str()
            );
            Err(WebdriverError::InvalidResponse)
        }
    } else {
        error!("WebdriverError::FailedRequest, error: {:?}", result);
        Err(WebdriverError::FailedRequest)
    }
}

/// used by requests sending data
#[inline]
fn post(url: &str, body: &str) -> Result<Value, WebdriverError> {
    handle_response(minreq::post(url).with_body(body.to_owned()).send())
}

/// use by requests getting data
#[inline]
fn get(url: &str) -> Result<Value, WebdriverError> {
    handle_response(minreq::get(url).send())
}

/// use by requests using delete http requests
#[inline]
fn delete(url: &str) -> Result<Value, WebdriverError> {
    handle_response(minreq::delete(url).send())
}

#[test]
fn test() {
    println!(
        "{:?}",
        post(
            "http://localhost:4444/session/b1191cdf-b297-4fb3-b073-f1dc28e9adde/window/new",
            "{}"
        )
    );
}

/// -> take capabilities (options)
/// create a session
/// -> return created session id
pub(crate) fn new_session(capabilities: &str) -> Result<String, WebdriverError> {
    debug!(
        "session creation request with capabilities {}",
        capabilities
    );

    let json = post("http://localhost:4444/session", capabilities)?;
    let session_id_value = &json["value"]["sessionId"];

    if session_id_value.is_string() {
        let session_id = session_id_value.as_str().unwrap().to_string();
        debug!("session created (id: {:?})", session_id);
        Ok(session_id)
    } else {
        error!(
            "response to session creation request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// create a tab on this session
/// -> return created tab id
pub(crate) fn new_tab(session_id: &str) -> Result<String, WebdriverError> {
    debug!("tab creation request on session with id {}", session_id);

    debug!(
        "url {:?}",
        format!("http://localhost:4444/session/{}/window/new", session_id)
    );
    let json = post(
        &format!("http://localhost:4444/session/{}/window/new", session_id),
        "{}\n",
    )?;

    let handle_value = &json["value"]["handle"];

    if handle_value.is_string() {
        let session_id = handle_value.as_str().unwrap().to_string();
        debug!("tab created (id: {})", session_id);
        Ok(session_id)
    } else {
        error!(
            "response to session creation request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return every open tab ids
pub(crate) fn get_open_tabs(session_id: &str) -> Result<Vec<String>, WebdriverError> {
    debug!("getting ids of open tabs on session with id {}", session_id);

    debug!(
        "url {:?}",
        format!(
            "http://localhost:4444/session/{}/window/handles",
            session_id
        )
    );
    let json = get(&format!(
        "http://localhost:4444/session/{}/window/handles",
        session_id
    ))?;

    let value = &json["value"];

    if value.is_array() {
        let values = value.as_array().unwrap();
        let mut tabs = Vec::with_capacity(values.len());

        for string_value in values.iter() {
            tabs.push(string_value.as_str().unwrap().to_owned());
        }

        debug!("ids of open tabs: {:?}", tabs);
        Ok(tabs)
    } else {
        error!(
            "response to open tab ids request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return selected tab id
pub(crate) fn get_selected_tab(session_id: &str) -> Result<String, WebdriverError> {
    debug!(
        "getting id of the selected tab on session with id {}",
        session_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/window",
        session_id
    ))?;

    let value = &json["value"];

    if value.is_string() {
        let id = value.as_str().unwrap().to_string();
        debug!("the selected tab id is {}", id);
        Ok(id)
    } else {
        error!(
            "response to selected tab id request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return timeouts
pub(crate) fn get_timeouts(session_id: &str) -> Result<Timeouts, WebdriverError> {
    debug!("getting timeouts on session with id {}", session_id);

    let json = get(&format!(
        "http://localhost:4444/session/{}/timeouts",
        session_id
    ))?;

    let value = &json["value"];
    let page_load_value = &value["pageLoad"];
    let implicit_value = &value["implicit"];

    if page_load_value.is_number() && implicit_value.is_number() {
        let script_value = &value["script"];

        let timeouts = Timeouts {
            script: script_value.as_u64().map(|v| v as usize),
            page_load: page_load_value.as_u64().map(|v| v as usize).unwrap(),
            implicit: implicit_value.as_u64().map(|v| v as usize).unwrap(),
        };
        debug!("timeouts are {:?}", timeouts);
        Ok(timeouts)
    } else {
        error!("response to timeouts request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id and timeouts
/// set timeouts
pub(crate) fn set_timeouts(session_id: &str, timeouts: Timeouts) -> Result<(), WebdriverError> {
    debug!(
        "setting timeouts to {:?} on session with id {}",
        timeouts, session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/timeouts", session_id),
        &serde_json::to_string(&timeouts).unwrap(),
    )?;

    if json["value"].is_null() {
        debug!("setting timeouts succeed");
        Ok(())
    } else {
        error!(
            "response to timeouts change request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id and tab id
/// select tab
pub(crate) fn select_tab(session_id: &str, tab_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "selecting tab with id {} on session with id {}",
        tab_id, session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/window", session_id),
        &json!({
            "handle": tab_id,
        })
        .to_string(),
    )?;

    if json["value"].is_null() {
        debug!("selecting tab succeed");
        Ok(())
    } else {
        error!(
            "response to tab selection request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id and a valid url
/// load a website in the selected tab
pub(crate) fn navigate(session_id: &str, url: &str) -> Result<(), WebdriverError> {
    debug!("navigating to {} on session with id {}", url, session_id);

    let json = post(
        &format!("http://localhost:4444/session/{}/url", session_id),
        &json!({
            "url": url,
        })
        .to_string(),
    )?;

    if json["value"].is_null() {
        debug!("navigation succeed");
        Ok(())
    } else {
        error!(
            "response to navigation request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// close active tab
pub(crate) fn close_active_tab(session_id: &str) -> Result<(), WebdriverError> {
    debug!("closing active tab on session with id {}", session_id);

    let json = delete(&format!(
        "http://localhost:4444/session/{}/window",
        session_id
    ))?;

    if json["value"].is_array() || json["value"].is_null() {
        debug!("tab closed successfully");
        Ok(())
    } else {
        error!("response to close request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id, a selector and a value
/// search for elements
/// -> return id of the first element found
pub(crate) fn find_element(
    session_id: &str,
    selector: Selector,
    value: &str,
) -> Result<String, WebdriverError> {
    let selector: &str = selector.into();

    debug!(
        "selecting element by {} with value {} on session with id {}",
        selector, value, session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/element", session_id),
        &json!({
            "using": selector,
            "value": value,
        })
        .to_string(),
    )?;

    let element_value = &json["value"][ELEMENT_ID];

    if element_value.is_string() {
        debug!("element found");
        Ok(element_value.as_str().unwrap().to_string())
    } else {
        error!(
            "response to element search request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return url of the active tab
pub(crate) fn get_active_tab_url(session_id: &str) -> Result<String, WebdriverError> {
    debug!(
        "getting url of active tab on session with id {}",
        session_id
    );

    let json = get(&format!("http://localhost:4444/session/{}/url", session_id))?;
    let value = &json["value"];

    if value.is_string() {
        let url = value.as_str().unwrap().to_string();
        debug!("active tab url is {}", url);
        Ok(url)
    } else {
        error!("response to url request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return title of the active tab
pub(crate) fn get_active_tab_title(session_id: &str) -> Result<String, WebdriverError> {
    debug!(
        "getting title of active tab on session with id {}",
        session_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/title",
        session_id
    ))?;

    let value = &json["value"];

    if value.is_string() {
        let url = value.as_str().unwrap().to_string();
        debug!("active tab title is {}", url);
        Ok(url)
    } else {
        error!("response to title request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// navigate backward on the selected tab
pub(crate) fn back(session_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "navigating backward on active tab on session with id {}",
        session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/back", session_id),
        "{}",
    )?;

    if json["value"].is_null() {
        debug!("successfully navigated backward");
        Ok(())
    } else {
        error!("response to back request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// navigate forward on the selected tab
pub(crate) fn forward(session_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "navigating forward on active tab on session with id {}",
        session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/forward", session_id),
        "{}",
    )?;

    if json["value"].is_null() {
        debug!("successfully navigated forward");
        Ok(())
    } else {
        error!("response to forward request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// refresh the selected tab
pub(crate) fn refresh(session_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "refreshing the active tab on session with id {}",
        session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/refresh", session_id),
        "{}",
    )?;

    if json["value"].is_null() {
        debug!("tab successfully refreshed");
        Ok(())
    } else {
        error!("response to refresh request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id, script and args
/// execute the script on the active tab
pub(crate) fn execute_script_sync(
    session_id: &str,
    script: &str,
    args: Vec<Value>,
) -> Result<(), WebdriverError> {
    debug!(
        "executing script on selected tab on session with id {}",
        session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/execute/sync", session_id),
        &json!({
            "script": script,
            "args": args,
        })
        .to_string(),
    )?;

    if json["value"].is_null() {
        debug!("script successfully executed");
        Ok(())
    } else {
        error!("response to refresh request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn click_on_element(session_id: &str, element_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "clicking on element with id {} on session with id {}",
        session_id, element_id
    );
    warn!("click_on_element function may fail silently in firefox");

    let json = post(
        &format!(
            "http://localhost:4444/session/{}/element/{}/click",
            session_id, element_id
        ),
        "{}",
    )?;

    if json["value"].is_null() {
        debug!("clicked successfully");
        Ok(())
    } else {
        error!("response to click request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_text(
    session_id: &str,
    element_id: &str,
) -> Result<String, WebdriverError> {
    debug!(
        "getting text of element with id {} on session with id {}",
        session_id, element_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/element/{}/text",
        session_id, element_id
    ))?;

    if json["value"].is_string() {
        let text = json["value"].as_str().unwrap().to_string();
        debug!("text of element with id {} is {}", element_id, text);
        Ok(text)
    } else {
        error!("response to text request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn send_text_to_element(
    session_id: &str,
    element_id: &str,
    text: &str,
) -> Result<(), WebdriverError> {
    debug!(
        "sending text ({}) to element with id {} on session with id {}",
        text, session_id, element_id
    );

    let json = post(
        &format!(
            "http://localhost:4444/session/{}/element/{}/value",
            session_id, element_id
        ),
        &json!({
            "text": text,
        })
        .to_string(),
    )?;

    if json["value"].is_null() {
        debug!("success");
        Ok(())
    } else {
        error!("response to send text request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn switch_to_frame(session_id: &str, element_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "switching to frame with id {} on session with id {}",
        element_id, session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/frame", session_id),
        &json!({
            "id": {
                "element-6066-11e4-a52e-4f735466cecf": element_id
            },
        })
        .to_string(),
    )?;

    if json["value"].is_null() {
        debug!("success");
        Ok(())
    } else {
        error!(
            "response to switch to frame request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn switch_to_parent_frame(session_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "switching to parent frame on session with id {}",
        session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/frame/parent", session_id),
        "{}",
    )?;

    if json["value"].is_null() {
        debug!("success");
        Ok(())
    } else {
        error!(
            "response to switch to parent frame request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_attribute(
    session_id: &str,
    element_id: &str,
    attribute_name: &str,
) -> Result<String, WebdriverError> {
    debug!(
        "getting attribute {} of element with id {} on session with id {}",
        attribute_name, session_id, element_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/element/{}/attribute/{}",
        session_id, element_id, attribute_name
    ))?;

    if json["value"].is_string() {
        let value = json["value"].as_str().unwrap().to_string();
        debug!("attribute {} is {}", attribute_name, value);
        Ok(value)
    } else {
        error!(
            "response to get element attribute request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_property(
    session_id: &str,
    element_id: &str,
    property_name: &str,
) -> Result<String, WebdriverError> {
    debug!(
        "getting property {} of element with id {} on session with id {}",
        property_name, session_id, element_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/element/{}/property/{}",
        session_id, element_id, property_name
    ))?;

    if !json["value"].is_null() {
        let value = json["value"].as_str().unwrap().to_string();
        debug!("property {} is {}", property_name, value);
        Ok(value)
    } else {
        error!(
            "response to get element property request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_css_value(
    session_id: &str,
    element_id: &str,
    property_name: &str,
) -> Result<String, WebdriverError> {
    debug!(
        "getting css value of property {} of element with id {} on session with id {}",
        property_name, session_id, element_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/element/{}/css/{}",
        session_id, element_id, property_name
    ))?;

    if json["value"].is_string() {
        let value = json["value"].as_str().unwrap().to_string();
        debug!("css value for {} is {}", property_name, value);
        Ok(value)
    } else {
        error!(
            "response to get element css value request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_tag_name(
    session_id: &str,
    element_id: &str,
) -> Result<String, WebdriverError> {
    debug!(
        "getting tag name of element with id {} on session with id {}",
        session_id, element_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/element/{}/name",
        session_id, element_id
    ))?;

    if json["value"].is_string() {
        let value = json["value"].as_str().unwrap().to_string();
        debug!("tag name is {}", value);
        Ok(value)
    } else {
        error!(
            "response to get element tag name request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub type ElementRect = ((usize, usize), (usize, usize));

pub(crate) fn get_element_rect(
    session_id: &str,
    element_id: &str,
) -> Result<ElementRect, WebdriverError> {
    debug!(
        "getting rect of element with id {} on session with id {}",
        session_id, element_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/element/{}/rect",
        session_id, element_id
    ))?;

    let value = &json["value"];
    let x = &value["x"];
    let y = &value["y"];
    let w = &value["width"];
    let h = &value["height"];

    if x.is_number() && y.is_number() && w.is_number() && h.is_number() {
        let value = (
            (
                x.as_u64().map(|v| v as usize).unwrap(),
                y.as_u64().map(|v| v as usize).unwrap(),
            ),
            (
                w.as_u64().map(|v| v as usize).unwrap(),
                h.as_u64().map(|v| v as usize).unwrap(),
            ),
        );
        debug!("rect is {:?}", value);
        Ok(value)
    } else {
        error!(
            "response to get element rect request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn is_element_enabled(
    session_id: &str,
    element_id: &str,
) -> Result<bool, WebdriverError> {
    debug!(
        "checking if element with id {} on session with id {} is enabled",
        element_id, session_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/element/{}/enabled",
        session_id, element_id
    ))?;

    if json["value"].is_boolean() {
        let value = json["value"].as_bool().unwrap();
        Ok(value)
    } else {
        error!(
            "response to is element enabled request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub type CookieData = (String, usize, bool, String, String, bool, String);

pub(crate) fn get_all_cookies(session_id: &str) -> Result<Vec<CookieData>, WebdriverError> {
    debug!("getting cookies on session with id {}", session_id);

    let json = get(&format!(
        "http://localhost:4444/session/{}/cookie",
        session_id
    ))?;

    let value = &json["value"];

    if value.is_array() {
        let values = value.as_array().unwrap();
        let mut cookies = Vec::with_capacity(values.len());

        for value in values.iter() {
            let object = value.as_object().unwrap();

            let tuple = (
                object["domain"].as_str().unwrap().to_string(),
                object["expiry"].as_u64().map(|v| v as usize),
                object["httpOnly"].as_bool(),
                object["name"].as_str().unwrap().to_string(),
                object["path"].as_str().unwrap().to_string(),
                object["secure"].as_bool(),
                object["value"].as_str().unwrap().to_string(),
            );

            if let (domain, Some(expiry), Some(http_only), name, path, Some(secure), value) = tuple
            {
                cookies.push((domain, expiry, http_only, name, path, secure, value))
            } else {
                warn!("a cookie was invalid; result: {:?}", tuple)
            }
        }

        debug!("cookies: {:?}", cookies);

        Ok(cookies)
    } else {
        error!("response to cookies request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn set_cookie(session_id: &str, cookie: CookieData) -> Result<(), WebdriverError> {
    debug!(
        "setting cookie {} to {} on session with id {}",
        cookie.3, cookie.6, session_id
    );

    let json = post(
        &format!("http://localhost:4444/session/{}/cookie", session_id),
        &json!({
            "cookie": {
                "domain": cookie.0,
                "expiry": cookie.1,
                "httpOnly": cookie.2,
                "name": cookie.3,
                "path": cookie.4,
                "secure": cookie.5,
                "value": cookie.6,
            },
        })
        .to_string(),
    )?;

    if json["value"].is_null() {
        debug!("success");
        Ok(())
    } else {
        error!(
            "response to add cookie request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_page_source(session_id: &str) -> Result<String, WebdriverError> {
    debug!(
        "getting page source of active tab on session with id {}",
        session_id
    );

    let json = get(&format!(
        "http://localhost:4444/session/{}/source",
        session_id
    ))?;

    let value = &json["value"];

    if value.is_string() {
        let source = value.as_str().unwrap().to_string();
        debug!("page source is {}", source);
        Ok(source)
    } else {
        error!(
            "response to page source request was not understood: {}",
            json
        );
        Err(WebdriverError::InvalidResponse)
    }
}
