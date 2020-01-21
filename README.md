# lw-webdriver

This crate allows you to control a web browser (Firefox or chrome) easily.
It does not use selenium, which is much more lightweight.
It only uses geckodriver or chromedriver (you have to download the one you want to use depending on your browser and place it in your program's directory).
This crate can launch the driver and kill his process after, but if one is already running, it will be used.
A lot of improvements can be done. Feel free to contribute.

## Example

```rust
use lw_webdriver::{session::Session, enums::{Browser, Selector}};
use std::{thread, time::Duration};

// start session
let session = Session::new(Browser::Firefox, false).unwrap();

// load a website
let mut tab = session.get_selected_tab().unwrap();
tab.navigate("https://mubelotix.dev/").unwrap();

// click a link
let mut link = tab.find(Selector::XPath, "//a[@href='https://www.kerbalspaceprogram.com/']").unwrap().unwrap();
link.click().unwrap();

thread::sleep(Duration::from_secs(5));
```

## Running tests

Run tests one by one:

```rust
cargo test -- --test-threads=1
```

License: MIT
