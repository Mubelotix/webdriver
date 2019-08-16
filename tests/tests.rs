use webdriver::session::*;
use webdriver::enums::*;
use webdriver::windows::*;
use std::rc::Rc;
use webdriver::timeouts::*;

static BROWSER: Browser = Browser::Firefox;

#[test]
fn navigation() {
    let webdriver = Session::new(BROWSER).expect("Echec de création de la session");
    let mut tab = webdriver.get_selected_tab().unwrap();
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
    let webdriver = Session::new(BROWSER).expect("Echec de création de la session");
    let mut tab = webdriver.get_selected_tab().unwrap();
    tab.navigate("http://example.com/").unwrap();
    assert_eq!(tab.get_url().unwrap(), String::from("http://example.com/"));
    assert_eq!(tab.get_title().unwrap(), String::from("Example Domain"));
}

#[test]
fn windows() {
    let webdriver = Session::new(BROWSER).expect("Echec de création de la session");

    let mut window1 = webdriver.get_selected_tab().unwrap();
    window1.navigate("https://www.mozilla.org/fr/").unwrap();
    assert_eq!(window1.get_url().unwrap(), String::from("https://www.mozilla.org/fr/"));

    let mut window2 = Tab::new(&webdriver).unwrap();
    window2.navigate("http://example.com/").unwrap();
    assert_eq!(window2.get_url().unwrap(), String::from("http://example.com/"));
    window1.navigate("https://www.google.com/").unwrap();
    assert_eq!(window1.get_url().unwrap(), String::from("https://www.google.com/"));

    window2.close().unwrap();
    window1.select().unwrap();
}

#[test]
fn timeouts() {
    let session = Session::new(BROWSER).expect("Echec de création de la session");

    let mut timeouts = session.get_timeouts().unwrap();
    assert_eq!(Some(30000), timeouts.script);
    assert_eq!(300000, timeouts.page_load);
    assert_eq!(0, timeouts.implicit);
    
    timeouts.script = None;
    timeouts.page_load = 299999;
    timeouts.implicit = 1;

    session.set_timeouts(timeouts).unwrap();

    timeouts = session.get_timeouts().unwrap();
    assert_eq!(None, timeouts.script);
    assert_eq!(299999, timeouts.page_load);
    assert_eq!(1, timeouts.implicit);
}