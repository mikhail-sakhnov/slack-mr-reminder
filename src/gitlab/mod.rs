use restson::Error;
use restson::RestPath;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MergeRequestCollection {
    Array(Vec<MergeRequest>),
}

#[derive(Deserialize, Debug)]
pub struct MergeRequest {
    pub id: u32,
    pub title: String,
    pub author: Author,
    pub description: String,
    pub work_in_progress: bool,
    pub web_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub username: String,
    pub avatar_url: String,
}

impl RestPath<u32> for MergeRequestCollection {
    fn get_path(id: u32) -> Result<String, Error> {
        Ok(format!("/api/v4/projects/{}/merge_requests", id))
    }
}
