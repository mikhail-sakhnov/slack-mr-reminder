#[macro_use]
extern crate serde_derive;

use restson::{RestClient};
use crate::api::MergeRequestCollection;

extern crate clap;
use clap::{Arg, App};

mod api;


fn main() {
    let matches = App::new("GitLab Mr Reminder")
                            .version("1.0")
                            .author("Mikhail S. <sahnov.m@gmail.com>")
                            .about("Sends notifications about opened Merge Requests into the slack")
                            .arg(Arg::with_name("gitlab-token")
                                .help("Gitlab token to use")
                                .required(true)
                                .index(1))
                            .arg(Arg::with_name("gitlab-host")
                                .required(false)
                                .help("Sets the API host")
                                )
                            .get_matches();
    let base_url = matches.value_of("gitlab-host").unwrap_or("https://gitlab.com");
    let token = matches.value_of("gitlab-token").unwrap();
    let mut client = RestClient::new(base_url).unwrap();
    let query = vec![("private_token",token), ("state", "opened")];
    let data: MergeRequestCollection = client.get_with(11084518, &query).unwrap();
    println!("{:#?}", data);
}
