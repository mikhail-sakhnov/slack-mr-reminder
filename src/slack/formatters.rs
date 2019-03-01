use crate::gitlab::MergeRequest;
use crate::gitlab::MergeRequestCollection;

pub struct OpenMergeRequestsMessageFormatter {
    pub data: MergeRequestCollection,
}

impl OpenMergeRequestsMessageFormatter {
    pub fn format(&self) -> String {
        match &self.data {
            MergeRequestCollection::Array(data) => {
                let caption = format!("There are {} open merge requests: \n ", data.len());
                return data
                    .iter()
                    .filter(|mr| !mr.work_in_progress)
                    .map(OpenMergeRequestsMessageFormatter::format_merge_request)
                    .fold(caption, |acc, x| acc + &x + " \n ---- \n");
            }
        }
    }

    fn format_merge_request(mr: &MergeRequest) -> String {
        return format!(
            "*{}* #{} by {} \n\n *DESCRIPTION* \n {} \n\n {}",
            mr.title, mr.id, mr.author.name, mr.description, mr.web_url
        );
    }
}
