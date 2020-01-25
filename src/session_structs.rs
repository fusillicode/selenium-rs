// Session Request Object
#[derive(Serialize, Deserialize)]
pub struct NewSessionRequest<T> {
    #[serde(rename = "desiredCapabilities")]
    desired_capabilities: T,
}

#[derive(Serialize, Deserialize)]
enum DesiredCapabilitiesRequest {
    ChromeDesiredCapabilitiesRequest,
    FirefoxDesiredCapabilitiesRequest
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChromeDesiredCapabilitiesRequest {
    browser_name: String,
    chrome_options: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FirefoxDesiredCapabilitiesRequest {
    browser_name: String,
    firefox_options: Vec<String>,
}

impl NewSessionRequest<DesiredCapabilitiesRequest> {
    pub fn new(browser_name: &str) -> NewSessionRequest<DesiredCapabilitiesRequest> {
        match &*browser_name {
            "chrome" => NewSessionRequest::<ChromeDesiredCapabilitiesRequest as DesiredCapabilitiesRequest> { desired_capabilities: ChromeDesiredCapabilitiesRequest { browser_name: browser_name.to_string(), chrome_options: vec!["TODO".to_string()] }},
            "firefox" => NewSessionRequest::<FirefoxDesiredCapabilitiesRequest as DesiredCapabilitiesRequest> { desired_capabilities: FirefoxDesiredCapabilitiesRequest { browser_name: browser_name.to_string(), firefox_options: vec!["TODO".to_string()] }}
        }
    }
}


// Session Response object
#[derive(Serialize, Deserialize)]
pub struct NewSessionResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
}

impl NewSessionResponse {
    pub fn get_session_id(self) -> String {
        self.session_id
    }
}

// Title Response Object
#[derive(Deserialize)]
pub struct TitleResponse {
    value: String,
}

impl TitleResponse {
    pub fn get_title(self) -> String {
        self.value
    }
}
