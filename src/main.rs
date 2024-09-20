use scylla::Session;
use std::error::Error;
use std::time::Instant;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("data/games.csv")?;

    let mut count = 0;
    let start = Instant::now();

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let _game: models::Game = result?;

        if count < 1 {
            println!("{:?}", _game);
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
