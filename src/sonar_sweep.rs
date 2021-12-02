use std::io::Write;

use crate::prelude::*;
use reqwest::header::{HeaderValue, COOKIE};

pub async fn eval() -> Result<()> {
    // count the number of times the depth increases from one sweep to the next

    let session = get_session_value()?;
    let measurements = input("https://adventofcode.com/2021/day/1/input", &session).await?;

    println!("count is: {}", count(measurements));

    Ok(())
}

fn get_session_value() -> Result<String> {
    let mut session = String::new();
    print!("Paste in session cookie value: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut session)?;

    Ok(format!("session={}", session.trim_end()))
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

fn count(measurements: Vec<i32>) -> i32 {
    let sums: Vec<i32> = measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .zip(measurements.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();
    sums.iter().zip(sums.iter().skip(1)).fold(0, |acc, elem| {
        if elem.1 > elem.0 {
            return acc + 1;
        }
        acc
    })
}
