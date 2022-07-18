mod kitsu;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Searching ...");
    let results = kitsu::client::search("kimi no na wa").await?;
    let first_anime = results.first().unwrap();

    println!("Loading anime data ...");
    let anime = kitsu::client::get_anime(&first_anime.slug).await?;
    dbg!(anime);
    Ok(())
}
