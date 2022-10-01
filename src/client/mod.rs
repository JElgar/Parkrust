use chrono::{DateTime, Duration, Utc};
use reqwest::{Method, RequestBuilder, Url};
use std::collections::HashMap;

use crate::models::parkrun::AuthResponse;

pub struct Token {
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

pub struct ParkrunClient {
    base_url: Url,
    request_client: reqwest::Client,
}

pub struct AuthenticatedParkrunClient {
    base_url: Url,
    request_client: reqwest::Client,
    token: Token,
}

impl ParkrunClient {
    pub fn new() -> Self {
        return ParkrunClient {
            base_url: Url::parse("https://api.parkrun.com").unwrap(),
            request_client: reqwest::Client::builder().build().unwrap(),
        };
    }

    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        self.request_client
            .request(method, self.base_url.join(path).unwrap())
            .basic_auth(
                "netdreams-iphone-s01",
                Some("gfKbDD6NJkYoFmkisR(iVFopQCKWzbQeQgZAZZKK"),
            )
            .header("X-Powered-By", "Park Rust")
    }

    pub async fn authenticate(
        self,
        athlete_id: &str,
        password: &str,
    ) -> Result<AuthenticatedParkrunClient, Box<dyn std::error::Error>> {
        // headers: { "Content-Type": "application/x-www-form-urlencoded" },
        let body = HashMap::from([
            ("username", athlete_id),
            ("password", password),
            ("scope", "app"),
            ("grant_type", "password"),
        ]);

        let response = self
            .request(Method::POST, "/user_auth.php")
            .form(&body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?
            .json::<AuthResponse>()
            .await?;

        Ok(AuthenticatedParkrunClient {
            base_url: self.base_url,
            request_client: self.request_client,
            token: Token {
                access_token: response.access_token,
                refresh_token: response.refresh_token,
                expires_at: Utc::now() + Duration::seconds(response.expires_in.parse().unwrap()),
            },
        })
    }
}

impl AuthenticatedParkrunClient {
    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let request_url = self.base_url.join(path).unwrap();
        self.request_client
            .request(method, request_url)
            .header("X-Powered-By", "Park Rust")
            .query(&[("access_token", self.token.access_token.clone())])
    }

    // pub async fn get_me(&self) -> Result<Athlete, Box<dyn std::error::Error>> {
    //     let athletes = self
    //         .request(Method::GET, "/v1/me")
    //         .send()
    //         .await?
    //         .json::<ListResponse<ListAthletes>>()
    //         .await?
    //         .data
    //         .athletes;
    //     Ok(athletes.get(0).unwrap().clone())
    // }
}
