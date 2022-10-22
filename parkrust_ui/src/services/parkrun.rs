use std::rc::Rc;
use yew::prelude::*;
use parkrust::client::{AuthenticatedParkrunClient, ParkrunClient, Token};

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

        Self { data: new_auth_data }.into()
    }
}

pub async fn login(id: &str, password: &str) -> Token {
    let client = ParkrunClient::new()
        .authenticate(id, password)
        .await.unwrap();
    client.token
}
