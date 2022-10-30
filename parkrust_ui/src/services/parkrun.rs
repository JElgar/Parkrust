use gloo::{storage::{LocalStorage, Storage, errors::StorageError}, console::log};
use parkrust::client::{AuthenticatedParkrunClient, ParkrunClient, Token};
use std::rc::Rc;
use yew::prelude::*;
use chrono::prelude::*;

const ACCESS_TOKEN_KEY: &str = "access_token";
const REFRESH_TOKEN_KEY: &str = "refresh_token";
const TOKEN_EXPIRES_AT_KEY: &str = "token_expires_at";
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
    Refresh(Token),
}

pub type AuthContext = UseReducerHandle<AuthState>;

// #[hook]
// pub fn use_auth() {
//     let auth_ctx = use_context::<AuthContext>().unwrap();
// 
//     let _get_token = {
//         let auth_ctx = auth_ctx.clone();
//         move |token: Token| async {
//             // TODO remove the unwrap
//             if auth_ctx.data.unwrap().token.is_expired() {
//             }
//         }
//     };
// }

pub async fn get_client(auth_ctx: UseReducerHandle<AuthState>) -> Option<AuthenticatedParkrunClient> {
    let mut token = auth_ctx.clone().data.clone()?.token;
    if token.is_expired() {
        log!("Token is expired");
        log!("{:?}", token.expires_at.to_string());
        token = refresh_token(&token.refresh_token).await;
        log!("Got new token!", token.expires_at.to_string());
        log!("{:?}", token.expires_at.to_string());
        auth_ctx.dispatch(AuthAction::Refresh(token.clone()));
    }

    Some(AuthenticatedParkrunClient::new(token))
}

impl Reducible for AuthState {
    /// Reducer Action Type
    type Action = AuthAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::Login(auth_data) => {
                store_token_data(&auth_data.token).unwrap();
                store_althlete_id(&auth_data.athlete_id).unwrap();
                // store_token_data(&auth_data.token).unwrap_or(log!("Failed to store token data"));
                // store_althlete_id(&auth_data.athlete_id).unwrap_or(log!("Failed to store athlete id"));
                Self { data: Some(auth_data) }.into()
            },
            AuthAction::Refresh(token) => {
                let athlete_id = &self.data.as_ref().unwrap().athlete_id;
                Self { data: Some(AuthData { athlete_id: String::from(athlete_id), token }) }.into()
            }
        }
    }
}

pub async fn login(id: &str, password: &str) -> Token {
    let client = ParkrunClient::new()
        .authenticate(id, password)
        .await
        .unwrap();
   
    client.token
}

pub async fn refresh_token(refresh_token: &str) -> Token {
    ParkrunClient::new().refresh_token(refresh_token).await.unwrap()
}

pub fn store_token_data(token: &Token) -> Result<(), StorageError> {
    LocalStorage::set(ACCESS_TOKEN_KEY, token.access_token.clone())?;
    LocalStorage::set(REFRESH_TOKEN_KEY, token.refresh_token.clone())?;
    LocalStorage::set(TOKEN_EXPIRES_AT_KEY, token.expires_at.clone())?;
    Ok(())
}
    
pub fn store_althlete_id(athlete_id: &str) -> Result<(), StorageError> {
    LocalStorage::set(ALTHLETE_ID_KEY, athlete_id)?;
    Ok(())
}

pub fn get_auth_data_from_local_storage() -> Option<AuthData> {
    let access_token: String = LocalStorage::get(ACCESS_TOKEN_KEY).ok()?;
    let athlete_id: String = LocalStorage::get(ALTHLETE_ID_KEY).ok()?;
    let refresh_token: String = LocalStorage::get(REFRESH_TOKEN_KEY).ok()?;
    let expires_at: DateTime<Utc> = LocalStorage::get(TOKEN_EXPIRES_AT_KEY).ok()?;

    log!("Auth data found!");
    Some(AuthData {
        athlete_id,
        token: Token {
            access_token,
            refresh_token,
            expires_at,
        },
    })
}
