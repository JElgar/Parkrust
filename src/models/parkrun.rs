use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use parkrust_derive::{parkrun_model, parkrun_list};

use crate::client::AuthenticatedParkrunClient;

#[async_trait]
pub trait Listable<Args> {
    async fn list(args: Args, parkrun_client: AuthenticatedParkrunClient) -> Result<Vec<Self>, Box<dyn std::error::Error>> where Self:Sized;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: String,
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

#[parkrun_model()]
pub struct ListEvents {
    pub events: Vec<Event>,
}

#[parkrun_model()]
pub struct ListResults {
    pub results: Vec<RunResult>,
}

#[parkrun_model()]
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

pub struct ResultsQuery {
    pub athlete_id: String
}

#[parkrun_model()]
#[parkrun_list(endpoint="/v1/results", args_type="ResultsQuery", data_key="results")]
pub struct RunResult {
    #[serde(rename(serialize = "SeriesID", deserialize = "SeriesID"))]
    pub series_id: String, // Int
    event_number: String, // Int
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
