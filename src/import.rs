use scylla::Session;
use scylla::SessionBuilder;
use std::error::Error;
use std::time::Instant;

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

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let game: models::Game = result?;

        if count < 1 {
            println!("{:?}", game);
        }
        count += 1;
    }

    println!(
        "done count: {}, took: {:?}",
        count,
        Instant::now().duration_since(start)
    );

    Ok(())
}
