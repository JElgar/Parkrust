mod app;

use app::App;

use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}
