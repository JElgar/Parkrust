use gloo::{storage::{LocalStorage, Storage, errors::StorageError}, console::log};
use parkrust::client::{AuthenticatedParkrunClient, ParkrunClient, Token};
use std::rc::Rc;
use yew::prelude::*;
use chrono::prelude::*;

const ACCESS_TOKEN_KEY: &str = "access_token";
const REFRESH_TOKEN_KEY: &str = "refresh_token";
const ALTHLETE_ID_KEY: &str = "athlete_id";

#[derive(Clone, PartialEq)]
pub struct AuthData {
    pub athlete_id: String,
    pub token: Token,
}

#[derive(Default, Clone, PartialEq)]
pub struct AuthState {
    pub data: Option<AuthData>,
}

pub enum AuthAction {
    Login(AuthData),
}

pub type AuthContext = UseReducerHandle<AuthState>;

impl Reducible for AuthState {
    /// Reducer Action Type
    type Action = AuthAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_auth_data = match action {
            AuthAction::Login(auth_data) => Some(auth_data),
        };

        Self {
            data: new_auth_data,
        }
        .into()
    }
}

pub async fn login(id: &str, password: &str) -> Token {
    let client = ParkrunClient::new()
        .authenticate(id, password)
        .await
        .unwrap();
   
    store_auth_data(&client.token, &id).unwrap_or(log!("Failed to store parkrun auth data"));
    client.token
}

pub fn store_auth_data(token: &Token, athlete_id: &str) -> Result<(), StorageError> {
    LocalStorage::set(ACCESS_TOKEN_KEY, token.access_token.clone())?;
    LocalStorage::set(REFRESH_TOKEN_KEY, token.refresh_token.clone())?;
    LocalStorage::set(ALTHLETE_ID_KEY, athlete_id.clone())?;
    Ok(())
}

pub fn get_auth_data_from_local_storage() -> Option<AuthData> {
    let access_token: String = LocalStorage::get(ACCESS_TOKEN_KEY).ok()?;
    let athlete_id: String = LocalStorage::get(ALTHLETE_ID_KEY).ok()?;
    let refresh_token: String = LocalStorage::get(REFRESH_TOKEN_KEY).ok()?;

    log!("Auth data found!");
    Some(AuthData {
        athlete_id,
        token: Token {
            access_token,
            refresh_token,
            expires_at: Utc::now(),
        },
    })
}
