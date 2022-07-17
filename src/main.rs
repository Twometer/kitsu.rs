mod kitsu;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let results = kitsu::search("Your name").await?;
    dbg!(results);
    Ok(())
}
