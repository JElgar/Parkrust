use yew_router::Routable;

#[derive(Clone, Routable, PartialEq, Eq)]
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
