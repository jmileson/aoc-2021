use crate::prelude::*;
use clap::Parser;
use reqwest::header::{HeaderValue, COOKIE};
use std::io::Write;

#[derive(Parser)]
pub(crate) struct Opts {
    session: String,
}

#[derive(Parser)]
enum Challenge {
    Day1,
    Day2,
}

impl Opts {
    pub(crate) fn session_value(&self) -> Result<HeaderValue> {
        let header = HeaderValue::from_str(format!("session={}", self.session).as_str());
        Ok(header?)
    }
}

fn get_session_value(opts: &Opts) -> String {
    Ok(format!("session={}", opts.session))
}

async fn input(url: &str, session: &str) -> Result<Vec<i32>> {
    let res = reqwest::Client::new()
        .get(url)
        .header(COOKIE, HeaderValue::from_str(session)?)
        .send()
        .await?
        .text()
        .await?
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string().parse::<i32>().expect("always succeeds"))
        .collect();

    Ok(res)
}
