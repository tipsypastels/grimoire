use anyhow::Result;

mod artifact;
mod binary;
mod tailwind;

#[tokio::main]
async fn main() -> Result<()> {
    artifact::init().await?;
    binary::init().await?;

    Ok(())
}
