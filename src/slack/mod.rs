pub mod formatters;

use restson::Error;
use restson::RestPath;

#[derive(Serialize, Deserialize)]
pub struct SlackSendMessage {
    pub text: String,
}

#[derive(Deserialize)]
struct SlackSendMessageResp {}

impl RestPath<&str> for SlackSendMessage {
    fn get_path(path: &str) -> Result<String, Error> {
        Ok(format!("services/{}", path))
    }
}
