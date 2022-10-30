use chrono::Duration;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::time::Duration as StdDuration;

use parkrust_derive::{parkrun_model, parkrun_list, parkrun_request_args};
use serde_json::from_str;

use crate::client::AuthenticatedParkrunClient;
use parse_duration::parse;

#[async_trait(?Send)]
pub trait Listable<Args: Serialize + Send> {
    async fn list(args: Args, parkrun_client: &mut AuthenticatedParkrunClient) -> Result<Vec<Self>, Box<dyn std::error::Error + Send + Sync>> where Self:Sized;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponseLink {
    pub rel: String,
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse<T> {
    pub data: T,
    pub links: Vec<ListResponseLink>
}

#[parkrun_model()]
pub struct Athlete {
    #[serde(rename(serialize = "AthleteID", deserialize = "AthleteID"))]
    pub athlete_id: String,
    pub first_name: String,
    pub last_name: String,
}

#[parkrun_model()]
pub struct ListAthletes {
    pub athletes: Vec<Athlete>,
}

#[parkrun_request_args()]
pub struct EventsQuery {
    pub athlete_id: String
}

#[parkrun_model()]
#[parkrun_list(endpoint="/v1/events", args_type="EventsQuery", data_key="events")]
pub struct Event {
    event_number: String,
    event_name: String,
    event_short_name: String, 
    event_long_name: String,
    event_location: String,
    country_code: String,
    preferred_language: String,
    #[serde(rename(serialize = "SeriesID", deserialize = "SeriesID"))]
    series_id: String,
    next_anniversary: String,
    home_run_selection: String,
    status_live: String,
    anniversary_saturday_of_month: String,
    event_status: String,
    user_favourite: Option<String>,
    accessible_to_public: String,
}

#[parkrun_request_args()]
pub struct ResultsQuery {
    pub athlete_id: String
}

#[parkrun_model()]
#[parkrun_list(endpoint="/v1/results", args_type="ResultsQuery", data_key="results")]
pub struct RunResult {
    #[serde(rename(serialize = "SeriesID", deserialize = "SeriesID"))]
    pub series_id: String, // Int
    pub event_number: String, // Int
    pub run_id: String, // Int
    pub finish_position: String, // Int
    pub gender_position: String, // Int,
    pub event_date: String, // Date  (2018-03-10)
    #[serde(rename(serialize = "AthleteID", deserialize = "AthleteID"))]
    pub athlete_id: String, // Int
    pub run_time: String, // Duration
    pub was_pb_run: String, // Boolean
    pub age_grading: String, // Float
    pub age_category: String, // Enum (JM15-17)
    pub first_timer: String, // Boolean
    #[serde(rename(serialize = "GenuinePB", deserialize = "GenuinePB"))]
    pub genuine_pb: String, // Boolean
    pub updated: String, // Date time
    pub assisted: Option<bool> // Not sure?
}

impl RunResult {
    pub fn duration(&self) -> Duration {
        let duration_splits = self.run_time.split(":").collect::<Vec<&str>>();
        let mins: u32 = from_str(duration_splits.get(1).unwrap()).unwrap_or(0);
        let seconds: u32 = from_str(duration_splits.get(2).unwrap()).unwrap_or(0);

        let duration = StdDuration::new((seconds + mins * 60).into(), 0);
        Duration::from_std(duration).unwrap()
    }
}
