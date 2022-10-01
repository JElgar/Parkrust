use parkrust::client::ParkrunClient;
use parkrust::models::parkrun::{RunResult, Listable, ResultsQuery, Event, EventsQuery};
use parkrust::cli::{Cli, Command};
use clap::Parser;

/// TODO
/// [ ] Write macro to add serialization to fields
/// [ ] Write macro to generate requests from endpoints + types

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

     let args = Cli::parse();

     match &args.command {
        Command::Run{ id, password } => {
            let client = ParkrunClient::new()
                .authenticate(id, password)
                .await?;

            // println!("{:?}", client.get_me().await?);

            println!("{:?}", RunResult::list(ResultsQuery{ athlete_id: id.clone() }, &client).await?);
            println!("{:?}", Event::list(EventsQuery{ athlete_id: id.clone() }, &client).await?);
        },
     }


    // println!("{:?}", client.get_events("718005").await?);
    Ok(())
}
