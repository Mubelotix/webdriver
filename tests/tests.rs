use lw_webdriver::session::*;
use lw_webdriver::enums::*;
use lw_webdriver::tab::*;
use std::{thread, time::Duration};

static BROWSER: Browser = Browser::Firefox;

#[test]
fn navigation() {
    //env_logger::init();
    let session = Session::new(BROWSER, false).expect("Echec de création de la session");
    let mut tab = session.get_selected_tab().unwrap();
    tab.navigate("http://example.com/").unwrap();
    assert_eq!(tab.get_url().unwrap(), String::from("http://example.com/"));
    tab.navigate("https://www.google.com/").unwrap();
    assert_eq!(tab.get_url().unwrap(), String::from("https://www.google.com/"));
    tab.back().unwrap();
    assert_eq!(tab.get_url().unwrap(), String::from("http://example.com/"));
    tab.forward().unwrap();
    assert_eq!(tab.get_url().unwrap(), String::from("https://www.google.com/"));
    tab.refresh().unwrap();
    assert_eq!(tab.get_url().unwrap(), String::from("https://www.google.com/"));
}

#[test]
fn getters() {
    //env_logger::init();
    let session = Session::new(BROWSER, false).expect("Echec de création de la session");
    let mut tab = session.get_selected_tab().unwrap();
    tab.navigate("http://example.com/").unwrap();
    assert_eq!(tab.get_url().unwrap(), String::from("http://example.com/"));
    assert_eq!(tab.get_title().unwrap(), String::from("Example Domain"));
}

#[test]
fn tabs() {
    //env_logger::init();
    let session = Session::new(BROWSER, false).expect("Echec de création de la session");

    let mut window1 = session.get_selected_tab().unwrap();
    window1.navigate("https://www.mozilla.org/fr/").unwrap();
    assert_eq!(window1.get_url().unwrap(), String::from("https://www.mozilla.org/fr/"));

    let mut window2 = Tab::new(&session).unwrap();
    window2.navigate("http://example.com/").unwrap();
    assert_eq!(window2.get_url().unwrap(), String::from("http://example.com/"));
    window1.navigate("https://www.google.com/").unwrap();
    assert_eq!(window1.get_url().unwrap(), String::from("https://www.google.com/"));

    window2.close().unwrap();
    window1.select().unwrap();
}

#[test]
fn timeouts() {
    //env_logger::init();
    let mut session = Session::new(BROWSER, false).expect("Echec de création de la session");

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

#[test]
fn elements() {
    //env_logger::init();
    let session = Session::new(BROWSER, false).expect("Echec de création de la session");

    let mut tab = session.get_selected_tab().unwrap();
    tab.navigate("https://www.mozilla.org/fr/").unwrap();

    let mut element1 = tab.find(Selector::XPath, "//*[@id=\"id_email\"]").unwrap().unwrap();
    let mut element2 = tab.find(Selector::XPath, "/html/body/div[3]/main/div[1]/div/aside/div[2]/form/fieldset/div/fieldset/p/label[2]").unwrap().unwrap();
    
    element1.type_text("test@example.com").unwrap();
    assert_eq!("Texte", element2.get_text().unwrap());
    element2.click().unwrap();
}

#[test]
fn execute_javascript() {
    //env_logger::init();
    let session = Session::new(BROWSER, false).expect("Echec de création de la session");

    let mut tab = session.get_selected_tab().unwrap();
    tab.navigate("http://example.com").unwrap();
    thread::sleep(Duration::from_secs(1));
    
    tab.execute_script("document.querySelector(arguments[0]).click();", vec!["html>body>div>p>a"]).unwrap();

    thread::sleep(Duration::from_secs(10));
}

#[test]
fn element_obscured() {
    //env_logger::init();
    let session = Session::new(BROWSER, false).expect("Echec de création de la session");

    let mut tab = session.get_selected_tab().unwrap();
    tab.navigate("https://mubelotix.dev/webdriver_tests/element_obscured.html").unwrap();
    
    let mut element = tab.find(Selector::XPath, "/html/body/p/a").unwrap().unwrap();
    element.click().unwrap();
    thread::sleep(Duration::from_secs(3));
}