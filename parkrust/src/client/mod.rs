use chrono::{DateTime, Duration, Utc};
use reqwest::{Method, RequestBuilder, Response, Url};
use std::collections::HashMap;

use crate::models::parkrun::{AuthResponse, RefreshTokenResponse};

pub mod requests;

#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

pub struct ParkrunClient {
    pub base_url: Url,
    pub request_client: reqwest::Client,
}

pub struct AuthenticatedParkrunClient {
    pub base_url: Url,
    pub request_client: reqwest::Client,
    pub token: Token,
}

#[cfg(target_arch = "wasm32")]
fn get_base_url() -> Url {
    Url::parse("https://parkrun-proxy.x2.workers.dev/").unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_base_url() -> Url {
    Url::parse("https://api.parkrun.com").unwrap()
}

impl Token {
    pub fn from_auth_response(response: AuthResponse) -> Self {
        Token {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at: Utc::now() + Duration::seconds(response.expires_in.parse().unwrap()),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.timestamp() <= Utc::now().timestamp()
    }
}

impl Default for ParkrunClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkrunClient {
    pub fn new() -> Self {
        ParkrunClient {
            base_url: get_base_url(),
            request_client: reqwest::Client::builder().build().unwrap(),
        }
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
        mut athlete_id: &str,
        password: &str,
    ) -> Result<AuthenticatedParkrunClient, Box<dyn std::error::Error>> {
        if athlete_id.starts_with('A') || athlete_id.starts_with('a') {
            athlete_id = &athlete_id[1..athlete_id.len()]
        }

        let body = HashMap::from([
            ("username", athlete_id),
            ("password", password),
            ("scope", "app"),
            ("grant_type", "password"),
        ]);

        // TODO Handle possible errors here
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
            token: Token::from_auth_response(response),
        })
    }

    pub async fn refresh_token(
        &mut self,
        refresh_token: &str,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        let body = HashMap::from([
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ]);

        let response = self
            .request(Method::POST, "/auth/refresh")
            .form(&body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?
            .json::<RefreshTokenResponse>()
            .await?;

        println!("Refresh token response is: {:?}", response);
        Ok(Token {
            expires_at: Utc::now() + Duration::seconds(response.expires_in.into()),
            access_token: response.access_token,
            refresh_token: String::from(refresh_token),
        })
    }
}

impl AuthenticatedParkrunClient {
    pub fn new(token: Token) -> Self {
        AuthenticatedParkrunClient {
            base_url: get_base_url(),
            request_client: reqwest::Client::builder().build().unwrap(),
            token,
        }
    }

    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let request_url = self.base_url.join(path).unwrap();
        self.request_client
            .request(method, request_url)
            .header("X-Powered-By", "Park Rust")
            .query(&[("access_token", self.token.access_token.clone())])
    }

    pub async fn send_request_with_refresh(
        &mut self,
        request: RequestBuilder,
    ) -> Result<Response, reqwest::Error> {
        if self.token.is_expired() {
            self.refresh_token().await.expect("Failed to refresh token");
        }

        request.send().await
    }

    pub async fn refresh_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.token = ParkrunClient::new()
            .refresh_token(&self.token.refresh_token)
            .await?;
        Ok(())
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
