use std::rc::Rc;

use parkrust::models::parkrun::{RunResult, ResultsQuery, Listable};
use parkrust::client::{ParkrunClient, AuthenticatedParkrunClient, Token};

use parkrust_ui::services::parkrun::{AuthContext, AuthState};
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch}; 
use material_yew::{MatTextField, MatButton};

use parkrust_ui::{
    utils::router::Route,
    routes::{
        login::Login,
        results::Results,
    }
};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Login => html! { <Login /> },
        Route::Results => html! { <Results /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let token = use_reducer(|| AuthState { data: None });
    html! {
        <ContextProvider<AuthContext> context={token}>
            <BrowserRouter>
                <Switch<Route> render={|route: Route| switch(route)} />
            </BrowserRouter>
        </ContextProvider<AuthContext>>
    }
}
