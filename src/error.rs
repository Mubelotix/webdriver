use std::convert::TryFrom;

#[derive(Debug)]
pub enum WebdriverError {
    UnsupportedPlatform,
    BrowserError(BrowserError),
    InvalidBrowserError,
    InvalidBrowserResponse,
    HttpRequestError(minreq::Error),
}

#[derive(PartialEq, Debug, Clone)]
pub enum BrowserError {
    Unknown,
    ElementClickIntercepted,
    ElementNotInteractable,
    InsecureCertificate,
    InvalidArgument,
    InvalidCookieDomain,
    InvalidElementState,
    InvalidSelector,
    InvalidSessionId,
    JavascriptError,
    MoveTargetOutOfBounds,
    NoSuchAlert,
    NoSuchCookie,
    NoSuchElement,
    NoSuchFrame,
    NoSuchWindow,
    ScriptTimeoutError,
    SessionNotCreated,
    StaleElementReference,
    Timeout,
    UnnableToSetCookie,
    UnableToCaptureScreen,
    UnexpectedAlertOpen,
    UnknowCommand,
    UnknowError,
    UnknowMethod,
    UnsupportedOperation
}

impl TryFrom<&str> for BrowserError {
    type Error = WebdriverError;

    fn try_from(error: &str) -> Result<Self, WebdriverError> {
        match error {
            "element click intercepted" => Ok(BrowserError::ElementClickIntercepted),
            "element not interactable" => Ok(BrowserError::ElementNotInteractable),
            "insecure certificate" => Ok(BrowserError::InsecureCertificate),
            "invalid argument" => Ok(BrowserError::InvalidArgument),
            "invalid cookie domain" => Ok(BrowserError::InvalidCookieDomain),
            "invalid element state" => Ok(BrowserError::InvalidElementState),
            "invalid selector" => Ok(BrowserError::InvalidSelector),
            "invalid session id " => Ok(BrowserError::InvalidSessionId),
            "javascript error" => Ok(BrowserError::JavascriptError),
            "move target out of bounds" => Ok(BrowserError::MoveTargetOutOfBounds),
            "no such alert" => Ok(BrowserError::NoSuchAlert),
            "no such cookie" => Ok(BrowserError::NoSuchCookie),
            "no such element" => Ok(BrowserError::NoSuchElement),
            "no such frame" => Ok(BrowserError::NoSuchFrame),
            "no such window" => Ok(BrowserError::NoSuchWindow),
            "script timeout error" => Ok(BrowserError::ScriptTimeoutError),
            "session not created" => Ok(BrowserError::SessionNotCreated),
            "stale element reference" => Ok(BrowserError::StaleElementReference),
            "timeout" => Ok(BrowserError::Timeout),
            "unable to set cookie" => Ok(BrowserError::UnnableToSetCookie),
            "unable to capture screen" => Ok(BrowserError::UnableToCaptureScreen),
            "unexpected alert open" => Ok(BrowserError::UnexpectedAlertOpen),
            "unknown command" => Ok(BrowserError::UnknowCommand),
            "unknown error" => Ok(BrowserError::Unknown),
            "unknown method" => Ok(BrowserError::UnknowMethod),
            "unsupported operation" => Ok(BrowserError::UnsupportedOperation),
            _ => Err(WebdriverError::InvalidBrowserError),
        }
    }
}
