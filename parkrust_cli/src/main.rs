mod cli;

use parkrust::client::ParkrunClient;
use parkrust::models::parkrun::{Event, EventsQuery, Listable};

use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Command::Run { id, password } => {
            let mut client = ParkrunClient::new().authenticate(id, password).await?;

            // println!("{:?}", client.get_me().await?);

            // RunResult::list(ResultsQuery{ athlete_id: id.clone() }, &mut client).await.unwrap();
            // println!("{:?}", RunResult::list(ResultsQuery{ athlete_id: id.clone() }, &client).await.unwrap());
            println!(
                "{:?}",
                Event::list(
                    EventsQuery {
                        athlete_id: id.clone()
                    },
                    &mut client
                )
                .await
                .unwrap()
            );
        }
    }

    // println!("{:?}", client.get_events("718005").await?);
    Ok(())
}
