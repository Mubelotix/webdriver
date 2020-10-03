#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Selector {
    Css,
    XPath,
    TagName,
    LinkText,
    PartialLinkText
}

impl Into<&str> for Selector {
    fn into(self) -> &'static str {
        match self {
            Selector::Css => "css selector",
            Selector::XPath => "xpath",
            Selector::TagName => "tag name",
            Selector::LinkText => "link text",
            Selector::PartialLinkText => "partial link text"
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Browser {
    Firefox,
    Chrome
}

impl Browser {
    pub fn to_string(self) -> &'static str {
        match self {
            Browser::Firefox => "firefox",
            Browser::Chrome => "chrome"
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Platform {
    Linux,
    Windows,
    Unknow
}

impl Platform {
    pub fn to_string(self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::Windows => "windows",
            Platform::Unknow => "unknow"
        }
    }

    pub fn current() -> Platform {
        if cfg!(unix) {
            Platform::Linux
        } else if cfg!(windows) {
            Platform::Windows
        } else {
            Platform::Unknow
        }
    }
}

pub trait WebdriverObject: PartialEq {
    fn get_id(&self) -> &String;
}