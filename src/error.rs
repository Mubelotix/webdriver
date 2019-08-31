#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum WebdriverError {
    UnsupportedPlatform,
    FailedRequest,
    InvalidResponse,
    Unknow,
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
    UnsupportedOperation,
    Custom(String)
}

impl WebdriverError {
    pub fn from(error: String) -> Self {
        match error.as_str() {
            "element click intercepted" => WebdriverError::ElementClickIntercepted,
            "element not interactable" => WebdriverError::ElementNotInteractable,
            "insecure certificate" => WebdriverError::InsecureCertificate,
            "invalid argument" => WebdriverError::InvalidArgument,
            "invalid cookie domain" => WebdriverError::InvalidCookieDomain,
            "invalid element state" => WebdriverError::InvalidElementState,
            "invalid selector" => WebdriverError::InvalidSelector,
            "invalid session id " => WebdriverError::InvalidSessionId,
            "javascript error" => WebdriverError::JavascriptError,
            "move target out of bounds" => WebdriverError::MoveTargetOutOfBounds,
            "no such alert" => WebdriverError::NoSuchAlert,
            "no such cookie" => WebdriverError::NoSuchCookie,
            "no such element" => WebdriverError::NoSuchElement,
            "no such frame" => WebdriverError::NoSuchFrame,
            "no such window" => WebdriverError::NoSuchWindow,
            "script timeout error" => WebdriverError::ScriptTimeoutError,
            "session not created" => WebdriverError::SessionNotCreated,
            "stale element reference" => WebdriverError::StaleElementReference,
            "timeout" => WebdriverError::Timeout,
            "unable to set cookie" => WebdriverError::UnnableToSetCookie,
            "unable to capture screen" => WebdriverError::UnableToCaptureScreen,
            "unexpected alert open" => WebdriverError::UnexpectedAlertOpen,
            "unknown command" => WebdriverError::UnknowCommand,
            "unknown error" => WebdriverError::Unknow,
            "unknown method" => WebdriverError::UnknowMethod,
            "unsupported operation" => WebdriverError::UnsupportedOperation,
            _ => WebdriverError::Custom(error),
        }
    }
}