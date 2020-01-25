// Session Request Object
#[derive(Serialize, Deserialize)]
pub struct NewSessionRequest {
    #[serde(rename = "desiredCapabilities")]
    desired_capabilities: DesiredCapabilitiesRequest,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DesiredCapabilitiesRequest {
    browser_name: String,
    chrome_options: Vec<String>,
}

impl NewSessionRequest {
    pub fn new(browser_name: &str, browser_options: &Vec<String>) -> NewSessionRequest {
        NewSessionRequest {
            desired_capabilities: DesiredCapabilitiesRequest::create(browser_name.to_string(), browser_options.to_vec())
        }
    }
}

impl DesiredCapabilitiesRequest {
    pub fn create(browser_name: String, browser_options: Vec<String>) -> DesiredCapabilitiesRequest {
        DesiredCapabilitiesRequest { browser_name, chrome_options: browser_options }
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
