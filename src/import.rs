use rand::seq::SliceRandom;
use rand::thread_rng;
use scylla::Session;
use scylla::SessionBuilder;
use std::error::Error;
use std::time::Instant;

mod models;

async fn create_keyspace(session: &Session) -> Result<(), Box<dyn Error>> {
    session
        .query_unpaged(
            r#"
                CREATE KEYSPACE IF NOT EXISTS steam
                    WITH REPLICATION = {
                        'class': 'SimpleStrategy',
                        'replication_factor': 1
                    };
                "#,
            (),
        )
        .await
        .map(|_| ())
        .map_err(From::from)
}

async fn create_table(session: &Session) -> Result<(), Box<dyn Error>> {
    session
        .query_unpaged(
            r#"
                CREATE TABLE IF NOT EXISTS steam.games (
                    app_id bigint PRIMARY KEY,
                    name text,
                    PRIMARY KEY(app_id)
                );
            "#,
            (),
        )
        .await
        .map(|_| ())
        .map_err(From::from)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let session: Session = SessionBuilder::new().known_node(uri).build().await?;

    session.refresh_metadata().await?;

    create_keyspace(&session).await?;
    create_table(&session).await?;

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("data/games.csv")?;

    let mut count = 0;
    let start = Instant::now();

    let mut games = Vec::new();

    for result in rdr.deserialize() {
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

    session.query_unpaged(query, values);

    // for chunk in games.chunks(1024) {
    //     println!("chunk size: {}", chunk.len());
    // }

    Ok(())
}
