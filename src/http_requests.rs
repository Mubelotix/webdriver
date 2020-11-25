use crate::{enums::Selector, error::*, timeouts::Timeouts};
use log::{debug, warn};
use serde_json::{self, json, Value};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use std::convert::TryFrom;

type MinReqResult = Result<minreq::Response, minreq::Error>;

pub enum Method<B: Serialize> {
    Get,
    Delete,
    Post(B),
}

/// used by requests sending data
#[inline]
fn request<B: Serialize, V: DeserializeOwned>(url: &str, method: Method<B>) -> Result<V, WebdriverError> {
    let result = match method {
        Method::Get => minreq::get(url),
        Method::Post(body) => minreq::post(url).with_body(serde_json::to_string(&body).unwrap()),
        Method::Delete => minreq::delete(url),
    }.send();

    match result {
        Ok(result) => {
            let text = result.as_str().map_err(|e| WebdriverError::HttpRequestError(e))?;
            let mut json = match serde_json::from_str::<Value>(text) {
                Ok(json) => json,
                Err(_e) => {
                    todo!()
                }
            };

            let value = json["value"].take();

            match serde_json::from_value::<V>(value) {
                Ok(value) => {
                    Ok(value)
                }
                Err(_err) => {
                    if let Ok(e) = serde_json::from_str::<Value>(text) {
                        if let Some(e) = e["value"]["error"].as_str() {
                            if let Ok(error) = BrowserError::try_from(e) {
                                return Err(WebdriverError::BrowserError(error))
                            }
                        }
                    }
                    Err(WebdriverError::InvalidBrowserResponse)
                }
            }
        },
        Err(e) => Err(WebdriverError::HttpRequestError(e))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSessionResponse {
    pub sessionId: String,
    pub capabilities: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWindowResponse {
    pub handle: String,
    #[serde(rename = "type")]
    pub window_type: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub secure: Option<bool>,
    pub http_only: Option<bool>,
    pub expiry: Option<u64>,
    pub same_site: Option<String>
}

pub(crate) fn new_session(capabilities: Value) -> Result<NewSessionResponse, WebdriverError> {
    debug!(
        "session creation request with capabilities {}",
        capabilities
    );

    request("http://localhost:4444/session", Method::Post(capabilities))
}

pub(crate) fn new_tab(session_id: &str) -> Result<NewWindowResponse, WebdriverError> {
    debug!("tab creation request on session with id {}", session_id);

    request(
        &format!("http://localhost:4444/session/{}/window/new", session_id),
        Method::Post(()),
    )
}

pub(crate) fn get_open_tabs(session_id: &str) -> Result<Vec<String>, WebdriverError> {
    debug!("getting ids of open tabs on session with id {}", session_id);

    request::<(), Vec<String>>(
        &format!("http://localhost:4444/session/{}/window/handles",session_id),
        Method::Get,
    )
}

pub(crate) fn get_selected_tab(session_id: &str) -> Result<String, WebdriverError> {
    debug!(
        "getting id of the selected tab on session with id {}",
        session_id
    );

    request::<(), String>(&format!("http://localhost:4444/session/{}/window",session_id), Method::Get)
}

pub(crate) fn get_timeouts(session_id: &str) -> Result<Timeouts, WebdriverError> {
    debug!("getting timeouts on session with id {}", session_id);

    request::<(), Timeouts>(&format!(
        "http://localhost:4444/session/{}/timeouts",
        session_id
    ), Method::Get)
}

pub(crate) fn set_timeouts(session_id: &str, timeouts: Timeouts) -> Result<(), WebdriverError> {
    debug!(
        "setting timeouts to {:?} on session with id {}",
        timeouts, session_id
    );

    request(&format!("http://localhost:4444/session/{}/timeouts", session_id), Method::Post(timeouts))
}

pub(crate) fn select_tab(session_id: &str, tab_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "selecting tab with id {} on session with id {}",
        tab_id, session_id
    );

    request(&format!("http://localhost:4444/session/{}/window", session_id), Method::Post(json! ({
        "handle": tab_id
    })))
}

pub(crate) fn navigate(session_id: &str, url: &str) -> Result<(), WebdriverError> {
    debug!("navigating to {} on session with id {}", url, session_id);

    request(&format!("http://localhost:4444/session/{}/url", session_id), Method::Post(json! ({
        "url": url
    })))
}

pub(crate) fn close_active_tab(session_id: &str) -> Result<Vec<String>, WebdriverError> {
    debug!("closing active tab on session with id {}", session_id);

    request::<(), Vec<String>>(&format!("http://localhost:4444/session/{}/window", session_id), Method::Delete)
}

pub(crate) fn find_elements(
    session_id: &str,
    selector: &Selector,
    value: &str,
) -> Result<Vec<String>, WebdriverError> {
    let selector: &str = selector.into();

    debug!(
        "selecting element by {} with value {} on session with id {}",
        selector, value, session_id
    );

    request(&format!("http://localhost:4444/session/{}/element", session_id), Method::Post(json!({
        "using": selector,
        "value": value,
    })))
}

pub(crate) fn get_active_tab_url(session_id: &str) -> Result<String, WebdriverError> {
    debug!(
        "getting url of active tab on session with id {}",
        session_id
    );

    request::<(), String>(&format!("http://localhost:4444/session/{}/url", session_id), Method::Get)
}

pub(crate) fn get_active_tab_title(session_id: &str) -> Result<String, WebdriverError> {
    debug!(
        "getting title of active tab on session with id {}",
        session_id
    );

    request::<(), String>(&format!("http://localhost:4444/session/{}/title", session_id), Method::Get)
}

pub(crate) fn back(session_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "navigating backward on active tab on session with id {}",
        session_id
    );

    request::<(), ()>(&format!("http://localhost:4444/session/{}/back", session_id), Method::Post(()))
}

pub(crate) fn forward(session_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "navigating forward on active tab on session with id {}",
        session_id
    );

    request::<(), ()>(&format!("http://localhost:4444/session/{}/forward", session_id), Method::Post(()))
}

pub(crate) fn refresh(session_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "refreshing the active tab on session with id {}",
        session_id
    );

    request::<(), ()>(&format!("http://localhost:4444/session/{}/refresh", session_id), Method::Post(()))
}

pub(crate) fn execute_script_sync(
    session_id: &str,
    script: &str,
    args: Vec<Value>,
) -> Result<(), WebdriverError> {
    debug!(
        "executing script on selected tab on session with id {}",
        session_id
    );

    request::<Value, ()>(&format!("http://localhost:4444/session/{}/execute/sync", session_id), Method::Post(json!({
        "script": script,
        "args": args,
    })))
}

pub(crate) fn click_on_element(session_id: &str, element_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "clicking on element with id {} on session with id {}",
        session_id, element_id
    );
    warn!("click_on_element function may fail silently in firefox");

    request::<(), ()>(&format!(
        "http://localhost:4444/session/{}/element/{}/click",
        session_id, element_id
    ), Method::Post(()))
}

pub(crate) fn get_element_text(
    session_id: &str,
    element_id: &str,
) -> Result<String, WebdriverError> {
    debug!(
        "getting text of element with id {} on session with id {}",
        session_id, element_id
    );

    request::<(), String>(&format!(
        "http://localhost:4444/session/{}/element/{}/text",
        session_id, element_id
    ), Method::Get)
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

    request(&format!(
        "http://localhost:4444/session/{}/element/{}/value",
        session_id, element_id
    ), Method::Post(json!({
        "text": text,
    })))
}

pub(crate) fn switch_to_frame(session_id: &str, element_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "switching to frame with id {} on session with id {}",
        element_id, session_id
    );

    request(&format!("http://localhost:4444/session/{}/frame", session_id), Method::Post(json!({
        "id": {
            "element-6066-11e4-a52e-4f735466cecf": element_id
        },
    })))
}

pub(crate) fn switch_to_parent_frame(session_id: &str) -> Result<(), WebdriverError> {
    debug!(
        "switching to parent frame on session with id {}",
        session_id
    );

    request(&format!("http://localhost:4444/session/{}/frame/parent", session_id), Method::Post(()))
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

    request::<(), String>(&format!(
        "http://localhost:4444/session/{}/element/{}/attribute/{}",
        session_id, element_id, attribute_name
    ), Method::Get)
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

    request::<(), String>(&format!(
        "http://localhost:4444/session/{}/element/{}/property/{}",
        session_id, element_id, property_name
    ), Method::Get)
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

    request::<(), String>(&format!(
        "http://localhost:4444/session/{}/element/{}/css/{}",
        session_id, element_id, property_name
    ), Method::Get)
}

pub(crate) fn get_element_tag_name(
    session_id: &str,
    element_id: &str,
) -> Result<String, WebdriverError> {
    debug!(
        "getting tag name of element with id {} on session with id {}",
        session_id, element_id
    );

    request::<(), String>(&format!(
        "http://localhost:4444/session/{}/element/{}/name",
        session_id, element_id
    ), Method::Get)
}

pub(crate) fn get_element_rect(
    session_id: &str,
    element_id: &str,
) -> Result<ElementRect, WebdriverError> {
    debug!(
        "getting rect of element with id {} on session with id {}",
        session_id, element_id
    );

    request::<(), ElementRect>(&format!(
        "http://localhost:4444/session/{}/element/{}/rect",
        session_id, element_id
    ), Method::Get)
}

pub(crate) fn is_element_enabled(
    session_id: &str,
    element_id: &str,
) -> Result<bool, WebdriverError> {
    debug!(
        "checking if element with id {} on session with id {} is enabled",
        element_id, session_id
    );

    request::<(), bool>(&format!(
        "http://localhost:4444/session/{}/element/{}/enabled",
        session_id, element_id
    ), Method::Get)
}

pub(crate) fn get_all_cookies(session_id: &str) -> Result<Vec<Cookie>, WebdriverError> {
    debug!("getting cookies on session with id {}", session_id);

    request::<(), Vec<Cookie>>(&format!(
        "http://localhost:4444/session/{}/cookie",
        session_id
    ), Method::Get)
}

pub(crate) fn set_cookie(session_id: &str, cookie: Cookie) -> Result<(), WebdriverError> {
    debug!(
        "setting cookie {:?} on session with id {}",
        cookie, session_id
    );
    
    request::<serde_json::Value, ()>(&format!(
        "http://localhost:4444/session/{}/cookie",
        session_id
    ), Method::Post(json!({
        "cookie": cookie,
    })))
}
