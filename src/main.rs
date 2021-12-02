mod input;
mod sonar_sweep;

pub mod prelude {
    pub use anyhow::Result;
}

use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    sonar_sweep::eval().await?;
    Ok(())
}
