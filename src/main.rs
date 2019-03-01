mod gitlab;
mod slack;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

use crate::gitlab::MergeRequestCollection;
use restson::RestClient;

use clap::{App, Arg};

use crate::slack::formatters::OpenMergeRequestsMessageFormatter;
use crate::slack::SlackSendMessage;

fn main() {
    let matches = App::new("GitLab Mr Reminder")
        .version("1.0")
        .author("Mikhail S. <sahnov.m@gmail.com>")
        .about("Sends notifications about opened Merge Requests into the slack")
        .arg(
            Arg::with_name("gitlab-project-id")
                .help("Gitlab project id to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("gitlab-token")
                .help("Gitlab token to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("slack-hook-credentials")
            .required(true)
            .help("Slack webhook credentials (e.g. https://hooks.slack.com/services/{credentials}) to send message")
        )
        .arg(
            Arg::with_name("gitlab-host")
                .required(false)
                .help("Sets the API host"),
        )
        .get_matches();
    let base_url = matches
        .value_of("gitlab-host")
        .unwrap_or("https://gitlab.com");
    let token = matches.value_of("gitlab-token").unwrap();

    let mut client = RestClient::new(base_url).unwrap();
    let project_id = value_t!(matches, "gitlab-project-id", u32).unwrap();

    let query = vec![("private_token", token), ("state", "opened")];
    let data: MergeRequestCollection = client.get_with(project_id, &query).unwrap();

    let formatter = OpenMergeRequestsMessageFormatter { data: data };

    let slack_hook_credentials = matches.value_of("slack-hook-credentials").unwrap();
    let mut slack_client = RestClient::new("https://hooks.slack.com/").unwrap();

    let data = SlackSendMessage {
        text: formatter.format(),
    };
    slack_client.post(slack_hook_credentials, &data).unwrap();
}
