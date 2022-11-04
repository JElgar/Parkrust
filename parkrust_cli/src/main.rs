mod cli;

use parkrust::client::ParkrunClient;
use parkrust::client::requests::{total_time, average_time};
use parkrust::models::parkrun::{RunResult, Listable, ResultsQuery, Event, EventsQuery};

use cli::{Cli, Command};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

     let args = Cli::parse();

     match &args.command {
        Command::Run{ id, password } => {
            let mut client = ParkrunClient::new()
                .authenticate(id, password)
                .await?;

            // println!("{:?}", client.get_me().await?);

            // RunResult::list(ResultsQuery{ athlete_id: id.clone() }, &mut client).await.unwrap();
            // println!("{:?}", RunResult::list(ResultsQuery{ athlete_id: id.clone() }, &client).await.unwrap());
            // println!("Total time: {:?}", total_time(&client, &id).await);
            // println!("Average time: {:?}", average_time(&client, &id).await);
            println!("{:?}", Event::list(EventsQuery{ athlete_id: id.clone() }, &mut client).await.unwrap());
        },
     }

    // println!("{:?}", client.get_events("718005").await?);
    Ok(())
}
