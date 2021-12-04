use crate::prelude::*;
use clap::Parser;
use reqwest::header::{HeaderValue, COOKIE};

#[derive(Parser)]
pub(crate) struct Opts {
    session: String,
    #[clap(short, long, default_value = "https://adventofcode.com/2021/day/{}/input")]
    url_template: String,
    #[clap(subcommand)]
    pub(crate) challenge: Challenge
}

#[derive(Parser, Clone, Copy)]
pub(crate) enum Challenge {
    Day1 = 1,
    Day2 = 2,
    Day3 = 3,
}

impl Opts {
    pub(crate) fn session_value(&self) -> Result<HeaderValue> {
        let header = HeaderValue::from_str(format!("session={}", self.session).as_str());
        Ok(header?)
    }
    
    pub(crate) fn url(&self) -> String {
        self.url_template.as_str().replace(r#"{}"#, &(self.challenge as u8).to_string())
    }
}

pub(crate) async fn input(opts: &Opts) -> Result<Vec<String>> {
    let res = reqwest::Client::new()
        .get(opts.url())
        .header(COOKIE, opts.session_value()?)
        .send()
        .await?
        .text()
        .await?
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(res)
}
