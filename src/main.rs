use parkrust::client::ParkrunClient;
use parkrust::models::parkrun::{RunResult, Listable, ResultsQuery};

/// TODO
/// [ ] Write macro to add serialization to fields
/// [ ] Write macro to generate requests from endpoints + types

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ParkrunClient::new()
        .authenticate("", "")
        .await?;
    // println!("{:?}", client.get_me().await?);

    println!("{:?}", RunResult::list(ResultsQuery{ athlete_id: String::from("718005") }, client).await?);

    // println!("{:?}", client.get_events("718005").await?);
    Ok(())
}
