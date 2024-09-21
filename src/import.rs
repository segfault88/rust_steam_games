use scylla::Session;
use scylla::SessionBuilder;
use std::error::Error;
use std::time::Instant;
use rand::thread_rng;
use rand::seq::SliceRandom;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let session: Session = SessionBuilder::new().known_node(uri).build().await?;

    session.refresh_metadata().await?;

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("data/games.csv")?;

    let mut count = 0;
    let start = Instant::now();

    let mut games = Vec::new();

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let game: models::Game = result?;

        games.push(game);
        count += 1;
    }

    println!(
        "done count: {}, took: {:?}",
        count,
        Instant::now().duration_since(start)
    );

    games.shuffle(&mut thread_rng());

    println!("random game:");
    for game in games.iter().take(1) {
        println!("{:?}", game);
    }

    for chunk in games.chunks(1024) {
        println!("chunk size: {}", chunk.len());
    }

    Ok(())
}
