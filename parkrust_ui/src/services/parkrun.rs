use gloo::{storage::{LocalStorage, Storage, errors::StorageError}, console::log};
use parkrust::{client::{AuthenticatedParkrunClient, ParkrunClient, Token}, models::parkrun::{RunResult, Listable, ResultsQuery}};
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
    pub results_cache: Option<Vec<RunResult>>
}

pub enum AuthAction {
    Login(AuthData),
    Refresh(Token),
    CacheResults(Vec<RunResult>),
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


#[hook]
pub fn use_results() -> UseStateHandle<Option<Vec<RunResult>>> {
    let results = use_state(|| None);
    let auth_ctx = use_context::<AuthContext>().unwrap();

    {
        let results = results.clone();
        println!("Getting stuff");
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                results.set(Some(get_user_results(&auth_ctx).await));
            });
            || ()
        }, ());
    }

    results
}

pub async fn get_client(auth_ctx: &UseReducerHandle<AuthState>) -> Option<AuthenticatedParkrunClient> {
    let mut token = auth_ctx.data.clone()?.token;
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

pub async fn get_user_results(auth_ctx: &UseReducerHandle<AuthState>) -> Vec<RunResult> {
    let athlete_id = auth_ctx.data.as_ref().unwrap().athlete_id.clone();
    if let Some(results) = &auth_ctx.results_cache {
        return results.to_vec();
    }

    let mut client = get_client(auth_ctx).await.unwrap();
    let results = RunResult::list(ResultsQuery{ athlete_id }, &mut client).await.unwrap();
    auth_ctx.dispatch(AuthAction::CacheResults(results.clone()));
    results
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
                Self { data: Some(auth_data), results_cache: None }.into()
            },
            AuthAction::Refresh(token) => {
                let athlete_id = &self.data.as_ref().unwrap().athlete_id;
                let results_cache = self.results_cache.clone();
                Self { data: Some(AuthData { athlete_id: String::from(athlete_id), token }), results_cache }.into()
            },
            AuthAction::CacheResults(results) => {
                let auth_data = self.data.clone();
                Self { data: auth_data, results_cache: Some(results) }.into()
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
