#[derive(Copy, PartialEq, Debug, Clone)]
pub enum Selector {
    Css,
    XPath,
    TagName,
    LinkText,
    PartialLinkText,
}

impl Into<&'static str> for &Selector {
    fn into(self) -> &'static str {
        match self {
            Selector::Css => "css selector",
            Selector::XPath => "xpath",
            Selector::TagName => "tag name",
            Selector::LinkText => "link text",
            Selector::PartialLinkText => "partial link text",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Browser {
    Firefox,
    Chrome,
}

impl Into<&'static str> for &Browser {
    fn into(self) -> &'static str {
        match self {
            Browser::Firefox => "firefox",
            Browser::Chrome => "chrome",
        }
    }
}

impl Browser {
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Platform {
    Linux,
    Windows,
    Unknow,
}

impl Platform {
    pub fn current() -> Platform {
        if cfg!(unix) {
            Platform::Linux
        } else if cfg!(windows) {
            Platform::Windows
        } else {
            Platform::Unknow
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl Into<&'static str> for &Platform {
    fn into(self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::Windows => "windows",
            Platform::Unknow => "unknow",
        }
    }
}

pub trait WebdriverObject: PartialEq {
    fn get_id(&self) -> &String;
}
