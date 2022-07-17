mod kitsu;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let results = kitsu::client::search("kimi no na wa").await?;
    dbg!(results);
    Ok(())
}
