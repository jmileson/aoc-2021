mod input;
mod sonar_sweep;
mod dive;

pub mod prelude {
    pub use anyhow::{anyhow,Result};
    pub use clap::Parser;
}

use input::Challenge;
use prelude::*;
use crate::input::{Opts, input};

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    
    let challenge_input = input(&opts).await?;

    match opts.challenge {
        Challenge::Day1 => sonar_sweep::eval(challenge_input).await,
        Challenge::Day2 => dive::eval(challenge_input).await,
    }?;

    Ok(())
}
