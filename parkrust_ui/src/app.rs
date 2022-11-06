use parkrust_ui::services::parkrun::{
    get_auth_data_from_local_storage, AuthContext, AuthData, AuthState, Cache,
};
use yew::prelude::*;
use yew_router::prelude::Redirect;
use yew_router::{BrowserRouter, Switch};

use parkrust_ui::{
    routes::{home::Home, login::Login, results::Results},
    utils::router::Route,
};

fn switch(routes: Route, auth_data: Option<AuthData>) -> Html {
    // TODO this doesnt work because 404s cant be handled
    // Authenticated routes
    match (routes, auth_data) {
        // Unauthenticated Routes
        (Route::Login, _) => html! { <Login /> },
        // Redirect when trying to access any other route
        (_, None) => html! { <Redirect<Route> to={Route::Login} /> },
        // Authenticated Routes
        (Route::Home, _) => html! { <Home /> },
        (Route::Results, _) => html! { <Results /> },
        (Route::NotFound, _) => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let auth_state = use_reducer(|| AuthState {
        data: get_auth_data_from_local_storage(),
        cache: Cache::default(),
    });
    html! {
        <ContextProvider<AuthContext> context={auth_state}>
            <Router />
        </ContextProvider<AuthContext>>
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    let auth_ctx = use_context::<AuthContext>().unwrap();

    let render = {
        let auth_data = auth_ctx.data.clone();
        move |route: Route| switch(route, auth_data.clone())
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={render} />
        </BrowserRouter>
    }
}
