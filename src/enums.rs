use os_info::Type;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Selector {
    Css,
    XPath,
    TagName,
    LinkText,
    PartialLinkText
}

impl Selector {
    pub fn to_string(&self) -> &'static str {
        match self {
            Selector::Css => "css selector",
            Selector::XPath => "xpath",
            Selector::TagName => "tag name",
            Selector::LinkText => "link text",
            Selector::PartialLinkText => "partial link text"
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Browser {
    Firefox,
    Chrome
}

impl Browser {
    pub fn to_string(&self) -> &'static str {
        match self {
            Browser::Firefox => "firefox",
            Browser::Chrome => "chrome"
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Platform {
    Linux,
    Windows,
    Unknow
}

impl Platform {
    pub fn to_string(&self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::Windows => "windows",
            Platform::Unknow => "unknow"
        }
    }

    pub fn current() -> Platform {
        let info = os_info::get();
        match info.os_type() {
            Type::Windows => Platform::Windows,
            Type::Linux | Type::Ubuntu | Type::Debian | Type::Fedora | Type::Redhat | Type::Arch | Type::Centos | Type::Alpine => Platform::Linux,
            _ => Platform::Unknow,
        }
    }
}
