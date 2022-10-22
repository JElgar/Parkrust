use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch}; 

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/results")]
    Results,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}
