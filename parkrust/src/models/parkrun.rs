use async_trait::async_trait;
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::time::Duration as StdDuration;

use crate::client::AuthenticatedParkrunClient;
use parkrust_derive::{parkrun_list, parkrun_model, parkrun_request_args};

#[async_trait(?Send)]
pub trait Listable<Args: Serialize + Send> {
    async fn list(
        args: Args,
        parkrun_client: &mut AuthenticatedParkrunClient,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error + Send + Sync>>
    where
        Self: Sized;
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
    pub links: Vec<ListResponseLink>,
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
    pub athlete_id: String,
}

#[parkrun_model()]
#[parkrun_list(
    endpoint = "/v1/events",
    args_type = "EventsQuery",
    data_key = "events"
)]
pub struct Event {
    pub event_number: String,
    pub event_name: String,
    pub event_short_name: String,
    pub event_long_name: String,
    pub event_location: String,
    pub country_code: String,
    pub preferred_language: String,
    #[serde(rename(serialize = "SeriesID", deserialize = "SeriesID"))]
    pub series_id: String,
    pub next_anniversary: String,
    pub home_run_selection: String,
    pub status_live: String,
    pub anniversary_saturday_of_month: String,
    pub event_status: String,
    pub user_favourite: Option<String>,
    pub accessible_to_public: String,
}

#[parkrun_request_args()]
pub struct ResultsQuery {
    pub athlete_id: String,
}

#[parkrun_model()]
#[parkrun_list(
    endpoint = "/v1/results",
    args_type = "ResultsQuery",
    data_key = "results"
)]
pub struct RunResult {
    #[serde(rename(serialize = "SeriesID", deserialize = "SeriesID"))]
    pub series_id: String, // Int
    pub event_number: String,    // Int
    pub run_id: String,          // Int
    pub finish_position: String, // Int
    pub gender_position: String, // Int,
    pub event_date: String,      // Date  (2018-03-10)
    #[serde(rename(serialize = "AthleteID", deserialize = "AthleteID"))]
    pub athlete_id: String, // Int
    pub run_time: String,        // Duration
    pub was_pb_run: String,      // Boolean
    pub age_grading: String,     // Float
    pub age_category: String,    // Enum (JM15-17)
    pub first_timer: String,     // Boolean
    #[serde(rename(serialize = "GenuinePB", deserialize = "GenuinePB"))]
    pub genuine_pb: String, // Boolean
    pub updated: String,         // Date time
    pub assisted: Option<String>, // Not sure? "0" or probably "1"
}

impl RunResult {
    pub fn duration(&self) -> Duration {
        let duration_splits = self.run_time.split(':').collect::<Vec<&str>>();
        let mins: u32 = String::from(*duration_splits.get(1).unwrap())
            .parse()
            .unwrap();
        let seconds: u32 = String::from(*duration_splits.get(2).unwrap())
            .parse()
            .unwrap();

        let duration = StdDuration::new((seconds + mins * 60).into(), 0);
        Duration::from_std(duration).unwrap()
    }

    pub fn date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(self.event_date.as_str(), "%Y-%m-%d").unwrap()
    }

    pub fn position(&self) -> usize {
        self.finish_position.parse().unwrap()
    }

    /// Return speed. The result is the duration per km
    pub fn speed(&self) -> Duration {
        self.duration() / 5
    }
}
