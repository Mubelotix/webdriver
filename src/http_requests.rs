use crate::timeouts::Timeouts;
use crate::error::WebdriverError;
use crate::enums::Selector;
use json::{JsonValue, object};
use log::{debug, info, warn, error};

/// used by requests sending data
fn post(url: &str, body: &str) -> Result<JsonValue, WebdriverError> {
    let res = minreq::post(url)
        .with_body(body.to_string())
        .send();

    if let Ok(res) = res {
        if let Ok(text) = res.as_str() {
            if let Ok(json) = json::parse(text) {
                if !json["value"]["error"].is_string() {
                    Ok(json)
                } else {
                    error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                    Err(WebdriverError::from(json["value"]["error"].to_string()))
                }
            } else {
                error!("WebdriverError::InvalidResponse (not json), text: {}, error: {:?}", text, json::parse(text));
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            error!("WebdriverError::InvalidResponse (not utf8), error: {:?}", res.as_str());
            Err(WebdriverError::InvalidResponse)
        }
    } else {
        error!("WebdriverError::FailedRequest, error: {:?}", res);
        Err(WebdriverError::FailedRequest)
    }
}

/// use by requests getting data
fn get(url: &str) -> Result<JsonValue, WebdriverError> {
    let res = minreq::get(url)
        .send();

    if let Ok(res) = res {
        if let Ok(text) = res.as_str() {
            if let Ok(json) = json::parse(text) {
                if !json["value"]["error"].is_string() {
                    Ok(json)
                } else {
                    error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                    Err(WebdriverError::from(json["value"]["error"].to_string()))
                }
            } else {
                error!("WebdriverError::InvalidResponse (not json), text: {}, error: {:?}", text, json::parse(text));
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            error!("WebdriverError::InvalidResponse (not utf8), error: {:?}", res.as_str());
            Err(WebdriverError::InvalidResponse)
        }
    } else {
        error!("WebdriverError::FailedRequest, error: {:?}", res);
        Err(WebdriverError::FailedRequest)
    }
}

/// use by requests using delete http requests
fn delete(url: &str) -> Result<JsonValue, WebdriverError> {
    let res = minreq::delete(url)
        .send();

    if let Ok(res) = res {
        if let Ok(text) = res.as_str() {
            if let Ok(json) = json::parse(text) {
                if !json["value"]["error"].is_string() {
                    Ok(json)
                } else {
                    error!("{:?}, response: {}", WebdriverError::from(json["value"]["error"].to_string()), json);
                    Err(WebdriverError::from(json["value"]["error"].to_string()))
                }
            } else {
                error!("WebdriverError::InvalidResponse (not json), text: {}, error: {:?}", text, json::parse(text));
                Err(WebdriverError::InvalidResponse)
            }
        } else {
            error!("WebdriverError::InvalidResponse (not utf8), error: {:?}", res.as_str());
            Err(WebdriverError::InvalidResponse)
        }
    } else {
        error!("WebdriverError::FailedRequest, error: {:?}", res);
        Err(WebdriverError::FailedRequest)
    }
}

/// -> take capabilities (options)
/// create a session
/// -> return created session id
pub(crate) fn new_session(capabilities: &str) -> Result<String, WebdriverError> {
    debug!("session creation request with capabilities {}", capabilities);

    let json = post("http://localhost:4444/session", capabilities)?;

    if json["value"]["sessionId"].is_string() {
        let session_id = json["value"]["sessionId"].to_string();
        debug!("session created (id: {})", session_id);
        Ok(session_id)
    } else {
        error!("response to session creation request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// create a tab on this session
/// -> return created tab id
pub(crate) fn new_tab(session_id: &str) -> Result<String, WebdriverError> {
    debug!("tab creation request on session with id {}", session_id);

    let json = post(&format!("http://localhost:4444/session/{}/window/new", session_id), "{}")?;

    if json["value"]["handle"].is_string() {
        let session_id = json["value"]["handle"].to_string();
        debug!("tab created (id: {})", session_id);
        Ok(session_id)
    } else {
        error!("response to session creation request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return every open tab ids
pub(crate) fn get_open_tabs(session_id: &str) -> Result<Vec<String>, WebdriverError> {
    debug!("getting ids of open tabs on session with id {}", session_id);

    let json = get(&format!("http://localhost:4444/session/{}/window/handles", session_id))?;

    if !json["value"].is_null() {
        let mut tabs: Vec<String> = Vec::new();
        let mut i = 0;
        while !json["value"][i].is_null() {
            tabs.push(json["value"][i].to_string());
            i += 1;
        }
        debug!("ids of open tabs: {:?}", session_id);
        Ok(tabs)
    } else {
        error!("response to open tab ids request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return selected tab id
pub(crate) fn get_selected_tab(session_id: &str) -> Result<String, WebdriverError> {
    debug!("getting id of the selected tab on session with id {}", session_id);

    let json = get(&format!("http://localhost:4444/session/{}/window", session_id))?;

    if json["value"].is_string() {
        let id = json["value"].to_string();
        debug!("the selected tab id is {}", id);
        Ok(id)
    } else {
        error!("response to selected tab id request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return timeouts
pub(crate) fn get_timeouts(session_id: &str) -> Result<Timeouts, WebdriverError> {
    debug!("getting timeouts on session with id {}", session_id);

    let json = get(&format!("http://localhost:4444/session/{}/timeouts", session_id))?;

    if json["value"]["pageLoad"].is_number() && json["value"]["implicit"].is_number() {
        let timeouts = Timeouts{
            script: json["value"]["script"].as_usize(),
            page_load: json["value"]["pageLoad"].as_usize().unwrap(),
            implicit: json["value"]["implicit"].as_usize().unwrap(),
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
    debug!("setting timeouts to {:?} on session with id {}", timeouts, session_id);

    let json = post(&format!("http://localhost:4444/session/{}/timeouts", session_id), &timeouts.to_json().to_string())?;

    if json["value"].is_null() {
        debug!("setting timeouts succeed");
        Ok(())
    } else {
        error!("response to timeouts change request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id and tab id
/// select tab
pub(crate) fn select_tab(session_id: &str, tab_id: &str) -> Result<(), WebdriverError> {
    debug!("selecting tab with id {} on session with id {}", tab_id, session_id);

    let json = post(&format!("http://localhost:4444/session/{}/window", session_id), &object! {
        "handle" => tab_id,
    }.to_string())?;

    if json["value"].is_null() {
        debug!("selecting tab succeed");
        Ok(())
    } else {
        error!("response to tab selection request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id and a valid url
/// load a website in the selected tab
pub(crate) fn navigate(session_id: &str, url: &str) -> Result<(), WebdriverError> {
    debug!("navigating to {} on session with id {}", url, session_id);

    let json = post(&format!("http://localhost:4444/session/{}/url", session_id), &object! {
        "url" => url,
    }.to_string())?;

    if json["value"].is_null() {
        debug!("navigation succeed");
        Ok(())
    } else {
        error!("response to navigation request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// close active tab
pub(crate) fn close_active_tab(session_id: &str) -> Result<(), WebdriverError> {
    debug!("closing active tab on session with id {}", session_id);

    let json = delete(&format!("http://localhost:4444/session/{}/window", session_id))?;

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
pub(crate) fn find_element(session_id: &str, selector: Selector, value: &str) -> Result<String, WebdriverError> {
    debug!("selecting element by {} with value {} on session with id {}", selector.to_string(), value, session_id);

    let json = post(&format!("http://localhost:4444/session/{}/element", session_id), &object! {
        "using" => selector.to_string(),
        "value" => value
    }.to_string())?;

    if !json["value"]["element-6066-11e4-a52e-4f735466cecf"].is_null() {
        debug!("element found");
        Ok(json["value"]["element-6066-11e4-a52e-4f735466cecf"].to_string())
    } else {
        error!("response to element search request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

/// -> take session id
/// -> return url of the active tab
pub(crate) fn get_active_tab_url(session_id: &str) -> Result<String, WebdriverError> {
    debug!("getting url of active tab on session with id {}", session_id);

    let json = get(&format!("http://localhost:4444/session/{}/url", session_id))?;

    if json["value"].is_string() {
        let url = json["value"].to_string();
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
    debug!("getting title of active tab on session with id {}", session_id);

    let json = get(&format!("http://localhost:4444/session/{}/title", session_id))?;

    if json["value"].is_string() {
        let url = json["value"].to_string();
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
    debug!("navigating backward on active tab on session with id {}", session_id);

    let json = post(&format!("http://localhost:4444/session/{}/back", session_id), "{}")?;

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
    debug!("navigating forward on active tab on session with id {}", session_id);

    let json = post(&format!("http://localhost:4444/session/{}/forward", session_id), "{}")?;

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
    debug!("refreshing the active tab on session with id {}", session_id);

    let json = post(&format!("http://localhost:4444/session/{}/refresh", session_id), "{}")?;

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
pub(crate) fn execute_script_sync(session_id: &str, script: &str, args: Vec<&str>) -> Result<(), WebdriverError> {
    debug!("executing script on selected tab on session with id {}", session_id);

    let json = post(&format!("http://localhost:4444/session/{}/execute/sync", session_id), &object!{
        "script" => script,
        "args" => args
    }.to_string())?;

    if json["value"].is_null() {
        debug!("script successfully executed");
        Ok(())
    } else {
        error!("response to refresh request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn click_on_element(session_id: &str, element_id: &str) -> Result<(), WebdriverError> {
    debug!("clicking on element with id {} on session with id {}", session_id, element_id);
    warn!("click_on_element function may fail silently in firefox");

    let json = post(&format!("http://localhost:4444/session/{}/element/{}/click", session_id, element_id), "{}")?;

    if json["value"].is_null() {
        debug!("clicked successfully");
        Ok(())
    } else {
        error!("response to click request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_text(session_id: &str, element_id: &str) -> Result<String, WebdriverError> {
    debug!("getting text of element with id {} on session with id {}", session_id, element_id);

    let json = get(&format!("http://localhost:4444/session/{}/element/{}/text", session_id, element_id))?;

    if json["value"].is_string() {
        let text = json["value"].to_string();
        debug!("text of element with id {} is {}", element_id, text);
        Ok(text)
    } else {
        error!("response to text request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn send_text_to_element(session_id: &str, element_id: &str, text: &str) -> Result<(), WebdriverError> {
    debug!("sending text ({}) to element with id {} on session with id {}", text, session_id, element_id);

    let json = post(&format!("http://localhost:4444/session/{}/element/{}/value", session_id, element_id), &object!{
        "text" => text,
    }.to_string())?;

    if json["value"].is_null() {
        debug!("success");
        Ok(())
    } else {
        error!("response to send text request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_attribute(session_id: &str, element_id: &str, attribute_name: &str) -> Result<String, WebdriverError> {
    debug!("getting attribute {} of element with id {} on session with id {}", attribute_name, session_id, element_id);

    let json = get(&format!("http://localhost:4444/session/{}/element/{}/attribute/{}", session_id, element_id, attribute_name))?;

    if json["value"].is_string() {
        let value = json["value"].to_string();
        debug!("attribute {} is {}", attribute_name, value);
        Ok(value)
    } else {
        error!("response to get element attribute request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_property(session_id: &str, element_id: &str, property_name: &str) -> Result<String, WebdriverError> {
    debug!("getting property {} of element with id {} on session with id {}", property_name, session_id, element_id);

    let json = get(&format!("http://localhost:4444/session/{}/element/{}/property/{}", session_id, element_id, property_name))?;

    if !json["value"].is_null() {
        let value = json["value"].to_string();
        debug!("property {} is {}", property_name, value);
        Ok(value)
    } else {
        error!("response to get element property request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_css_value(session_id: &str, element_id: &str, property_name: &str) -> Result<String, WebdriverError> {
    debug!("getting css value of property {} of element with id {} on session with id {}", property_name, session_id, element_id);

    let json = get(&format!("http://localhost:4444/session/{}/element/{}/css/{}", session_id, element_id, property_name))?;

    if json["value"].is_string() {
        let value = json["value"].to_string();
        debug!("css value for {} is {}", property_name, value);
        Ok(value)
    } else {
        error!("response to get element css value request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_tag_name(session_id: &str, element_id: &str) -> Result<String, WebdriverError> {
    debug!("getting tag name of element with id {} on session with id {}", session_id, element_id);

    let json = get(&format!("http://localhost:4444/session/{}/element/{}/name", session_id, element_id))?;

    if json["value"].is_string() {
        let value = json["value"].to_string();
        debug!("tag name is {}", value);
        Ok(value)
    } else {
        error!("response to get element tag name request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_element_rect(session_id: &str, element_id: &str) -> Result<((usize, usize), (usize, usize)), WebdriverError> {
    debug!("getting rect of element with id {} on session with id {}", session_id, element_id);

    let json = get(&format!("http://localhost:4444/session/{}/element/{}/rect", session_id, element_id))?;

    if json["value"]["x"].is_number() && json["value"]["y"].is_number() && json["value"]["width"].is_number() && json["value"]["height"].is_number() {
        let value = ((json["value"]["x"].as_usize().unwrap(), json["value"]["y"].as_usize().unwrap()), (json["value"]["width"].as_usize().unwrap(), json["value"]["height"].as_usize().unwrap()));
        debug!("rect is {:?}", value);
        Ok(value)
    } else {
        error!("response to get element rect request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn is_element_enabled(session_id: &str, element_id: &str) -> Result<bool, WebdriverError> {
    debug!("checking if element with id {} on session with id {} is enabled", element_id, session_id);

    let json = get(&format!("http://localhost:4444/session/{}/element/{}/enabled", session_id, element_id))?;

    if json["value"].is_boolean() {
        let value = json["value"].as_bool().unwrap();
        Ok(value)
    } else {
        error!("response to is element enabled request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn get_all_cookies(session_id: &str) -> Result<Vec<(String, usize, bool, String, String, bool, String)>, WebdriverError> {
    debug!("getting cookies on session with id {}", session_id);

    let json = get(&format!("http://localhost:4444/session/{}/cookie", session_id))?;

    if json["value"].is_array() {
        let mut i = 0;
        let mut cookies = Vec::new();
        while json["value"][i].is_object() {
            if let (domain, Some(expiry), Some(http_only), name, path, Some(secure), value) =
            (json["value"][i]["domain"].to_string(), json["value"][i]["expiry"].as_usize(), json["value"][i]["httpOnly"].as_bool(), json["value"][i]["name"].to_string(), json["value"][i]["path"].to_string(), json["value"][i]["secure"].as_bool(), json["value"][i]["value"].to_string()) {
                cookies.push((domain, expiry, http_only, name, path, secure, value))
            } else {
                warn!("a cookie was invalid; result: {:?}", (json["value"][i]["domain"].to_string(), json["value"][i]["expiry"].as_usize(), json["value"][i]["httpOnly"].as_bool(), json["value"][i]["name"].to_string(), json["value"][i]["path"].to_string(), json["value"][i]["secure"].as_bool(), json["value"][i]["value"].to_string()))
            }
            i += 1;
        }
        
        debug!("cookies: {:?}", cookies);

        Ok(cookies)
    } else {
        error!("response to cookies request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}

pub(crate) fn set_cookie(session_id: &str, cookie: (String, usize, bool, String, String, bool, String)) -> Result<(), WebdriverError> {
    debug!("setting cookie {} to {} on session with id {}", cookie.3, cookie.6, session_id);

    let json = post(&format!("http://localhost:4444/session/{}/cookie", session_id), &object!{
        "cookie" => object!{
            "domain" => cookie.0,
            "expiry" => cookie.1,
            "httpOnly" => cookie.2,
            "name" => cookie.3,
            "path" => cookie.4,
            "secure" => cookie.5,
            "value" => cookie.6
        }
    }.to_string())?;

    if json["value"].is_null() {
        debug!("success");
        Ok(())
    } else {
        error!("response to add cookie request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}