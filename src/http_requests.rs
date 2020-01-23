use crate::timeouts::Timeouts;
use crate::error::WebdriverError;
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

/// -> take timeouts
/// set timeouts
pub(crate) fn set_timeouts(session_id: &str, timeouts: Timeouts) -> Result<(), WebdriverError> {
    debug!("setting timeouts to {:#?} on session with id {}", timeouts, session_id);

    let json = post(&format!("http://localhost:4444/session/{}/timeouts", session_id), &timeouts.to_json().to_string())?;

    if json["value"].is_null() {
        debug!("setting timeouts succeed");
        Ok(())
    } else {
        error!("response to timeouts change request was not understood: {}", json);
        Err(WebdriverError::InvalidResponse)
    }
}
