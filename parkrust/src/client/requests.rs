use chrono::{DateTime, Duration, Utc};
use std::time::Duration as StdDuration;

use crate::models::parkrun::{RunResult, ResultsQuery, Listable};

use super::AuthenticatedParkrunClient;

pub async fn total_time(client: &AuthenticatedParkrunClient, athlete_id: &String) -> Duration {
    let results = RunResult::list(ResultsQuery{ athlete_id: athlete_id.clone() }, &client).await.unwrap();
    let seconds: i64 = results.iter().map(|result| result.duration().num_seconds()).sum();
    Duration::from_std(StdDuration::from_secs(seconds.try_into().unwrap())).unwrap()
}

pub async fn average_time(client: &AuthenticatedParkrunClient, athlete_id: &String) -> Duration {
    total_time(&client, &athlete_id).await / RunResult::list(ResultsQuery{ athlete_id: athlete_id.clone() }, &client).await.unwrap().len().try_into().unwrap()

}
