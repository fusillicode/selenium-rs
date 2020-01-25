/*!
    This library provides a client API for the selenium webdriver specification:
    [](https://www.w3.org/TR/webdriver1/)

    #Usage

    ```toml
    [dependencies]
    selenium-rs = "0.1"
    ```

    and add this to your root crate:
    ```rust
    extern crate selenium_rs;
    ```


    ## Requirements

    Selenium-rs requires that there is an instance of the selenium-webdriver server running
    to make requests against.

    As of right now the easiest way to get this is to download the standalone from:
    [https://www.seleniumhq.org/download/], and then running it (requires Java 1.8+)


    # Example: Make a google search, programmatically

    ```rust
    use selenium_rs::webdriver::{Browser, WebDriver, Selector};

    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    driver.navigate("http://google.com");
    let search_bar = driver.find_element(Selector::CSS, "input[maxlength=\"2048\"]").unwrap();
    search_bar.type_text("selenium-rs github");
    let search_button = driver.find_element(Selector::CSS, "input[name=\"btnK\"]").unwrap();

    search_button.click();
    ```
*/

extern crate reqwest;
extern crate url;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod element;
pub mod webdriver;

mod element_structs;
mod session_structs;
mod utils;
