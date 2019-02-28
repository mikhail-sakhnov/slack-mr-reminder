
use restson::RestPath;
use restson::Error;

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum MergeRequestCollection {
    Array(Vec<MergeRequest>)
}

#[derive(Deserialize,Debug)]
pub struct MergeRequest {
    id: u32,
    title: String,
    author: Author,
    description: String,
    work_in_progress: bool,
    web_url: String
}

#[derive(Deserialize,Debug)]
pub struct Author {
    name: String,
    username: String,
    avatar_url: String
}

impl RestPath<u32> for MergeRequestCollection {
    fn get_path(id: u32) -> Result<String,Error> { 
        Ok(format!("/api/v4/projects/{}/merge_requests", id)) 
    }
}

