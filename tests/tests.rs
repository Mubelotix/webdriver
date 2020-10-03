#![allow(unused_must_use)]

use lw_webdriver::session::*;
use lw_webdriver::enums::*;
use std::panic::catch_unwind;
use log::{info};

#[test]
fn navigation() {
    catch_unwind(|| {
        env_logger::init();
    });
    
    for i in 0..2 {
        let mut session = match i {
            0 => {
                info!("testing with Firefox");
                Session::new(Browser::Firefox, false).unwrap()
            },
            _ => {
                info!("testing with Chrome");
                Session::new(Browser::Chrome, false).unwrap()
            }
        };

        session.tabs[0].navigate("http://example.com/").unwrap(); // There is one tab already opened
        assert_eq!(&session.tabs[0].get_url().unwrap(), "http://example.com/");

        session.tabs[0].navigate("https://mubelotix.dev/").unwrap();
        assert_eq!(&session.tabs[0].get_url().unwrap(), "https://mubelotix.dev/");

        session.tabs[0].back().unwrap();
        assert_eq!(&session.tabs[0].get_url().unwrap(), "http://example.com/");

        session.tabs[0].forward().unwrap();
        assert_eq!(&session.tabs[0].get_url().unwrap(), "https://mubelotix.dev/");

        session.tabs[0].refresh().unwrap();
        assert_eq!(&session.tabs[0].get_url().unwrap(), "https://mubelotix.dev/");
    }
}

#[test]
fn tabs() {
    catch_unwind(|| {
        env_logger::init();
    });
    
    for i in 0..2 {
        let mut session = match i {
            0 => {
                info!("testing with Firefox");
                Session::new(Browser::Firefox, false).unwrap()
            },
            _ => {
                info!("testing with Chrome");
                Session::new(Browser::Chrome, false).unwrap()
            }
        };

        session.tabs[0].navigate("https://mubelotix.dev/").unwrap(); // There is one tab already opened
        assert_eq!(session.tabs.len(), 1);

        session.open_tab().unwrap();
        assert_eq!(session.tabs.len(), 2);

        session.tabs[1].navigate("https://mubelotix.dev/webdriver_tests/open_tab.html").unwrap();
        assert_eq!(session.tabs.len(), 2); // the website opened a tab but the webdriver ignore it, preventing using the wrong tab

        let source = session.tabs[1].get_page_source().unwrap();
        assert!(source.len() > 100);

        session.update_tabs().unwrap();    // however we can ask the webdriver to update tabs
        assert_eq!(session.tabs.len(), 3); // and the tab opened by the webdriver is accessible

        session.tabs.remove(2);            // drop the tab => close the tab
        assert_eq!(session.tabs.len(), 2);
    }
}

#[test]
fn timeouts() {
    catch_unwind(|| {
        env_logger::init();
    });
    
    for i in 0..2 {
        let mut session = match i {
            0 => {
                info!("testing with Firefox");
                Session::new(Browser::Firefox, false).unwrap()
            },
            _ => {
                info!("testing with Chrome");
                Session::new(Browser::Chrome, false).unwrap()
            }
        };

        let mut timeouts = session.get_timeouts().unwrap();
        assert_eq!(Some(30000), timeouts.script);
        assert_eq!(300_000, timeouts.page_load);
        assert_eq!(0, timeouts.implicit);
        
        timeouts.script = None;
        timeouts.page_load = 299_999;
        timeouts.implicit = 1;

        session.set_timeouts(timeouts).unwrap();

        timeouts = session.get_timeouts().unwrap();
        assert_eq!(None, timeouts.script);
        assert_eq!(299_999, timeouts.page_load);
        assert_eq!(1, timeouts.implicit);
    }
}

#[test]
fn elements() {
    catch_unwind(|| {
        env_logger::init();
    });
    
    for i in 0..2 {
        let mut session = match i {
            0 => {
                info!("testing with Firefox");
                Session::new(Browser::Firefox, false).unwrap()
            },
            _ => {
                info!("testing with Chrome");
                Session::new(Browser::Chrome, false).unwrap()
            }
        };

        session.open_tab();
        session.tabs[0].navigate("https://www.mozilla.org/fr/").unwrap();
        session.tabs[1].navigate("https://mubelotix.dev/").unwrap();

        let mut email_input = session.tabs[0].find(Selector::XPath, "//*[@id=\"id_email\"]").unwrap().unwrap();
        let mut label = session.tabs[0].find(Selector::XPath, "/html/body/div[3]/main/div[1]/div/aside/div[2]/form/fieldset/div/fieldset/p/label[2]").unwrap().unwrap();
        assert_eq!(email_input.get_tag_name().unwrap(), "input");
        assert_eq!(email_input.is_enabled().unwrap(), true);
        assert!(email_input.get_rect().is_ok());

        let project_element = session.tabs[1].find(Selector::XPath, "/html/body/main/div[1]").unwrap().unwrap();
        assert_eq!(project_element.get_tag_name().unwrap(), "div");
        assert_eq!(project_element.get_attribute("class").unwrap(), "project");
        assert_eq!(project_element.get_css_value("display").unwrap(), "flex");
        assert_eq!(project_element.get_property("draggable").unwrap(), "false");

        email_input.type_text("test@example.com").unwrap();
        assert_eq!("Texte", label.get_text().unwrap());
        label.click().unwrap();
    }
}

#[test]
fn execute_javascript() {
    catch_unwind(|| {
        env_logger::init();
    });
    
    for i in 0..2 {
        let mut session = match i {
            0 => {
                info!("testing with Firefox");
                Session::new(Browser::Firefox, false).unwrap()
            },
            _ => {
                info!("testing with Chrome");
                Session::new(Browser::Chrome, false).unwrap()
            }
        };

        session.tabs[0].navigate("http://example.com").unwrap();
        let element = session.tabs[0].find(Selector::Css, "html>body>div>p>a").unwrap().unwrap();
        session.tabs[0].execute_script("arguments[0].click();", vec![element.as_json_object()]).unwrap();
    }
}

#[test]
fn element_obscured() {
    catch_unwind(|| {
        env_logger::init();
    });
    
    for i in 0..2 {
        let mut session = match i {
            0 => {
                info!("testing with Firefox");
                Session::new(Browser::Firefox, false).unwrap()
            },
            _ => {
                info!("testing with Chrome");
                Session::new(Browser::Chrome, false).unwrap()
            }
        };

        session.tabs[0].navigate("https://mubelotix.dev/webdriver_tests/element_obscured.html").unwrap();
        
        let mut element_obscured = session.tabs[0].find(Selector::XPath, "/html/body/p/a").unwrap().unwrap();
        element_obscured.click().unwrap();
    }
}