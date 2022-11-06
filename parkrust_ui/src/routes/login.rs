use yew::prelude::*;
use yew_router::prelude::*;

use web_sys::HtmlInputElement;

use crate::{
    components::{Button, Input, TextFieldType},
    services::parkrun::{login, AuthAction, AuthContext, AuthData},
    utils::router::Route,
};

#[function_component(Login)]
pub fn login_view() -> Html {
    let athlete_id = use_state(|| "".to_owned());
    let password = use_state(|| "".to_owned());
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let athlete_id = athlete_id.clone();
        let password = password.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let athlete_id = athlete_id.clone();
            let password = password.clone();
            let auth_ctx = auth_ctx.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let athlete_id = (*athlete_id).as_str();
                let token = login(athlete_id, (*password).as_str()).await;
                auth_ctx.dispatch(AuthAction::Login(AuthData {
                    token,
                    athlete_id: athlete_id.to_string(),
                }));
                navigator.push(&Route::Home)
            });
        })
    };

    html! {
        <div class="flex min-h-full items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
            <div class="w-full max-w-md space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900 dark:text-white">
                        {"Sign in to your account"}
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600 dark:text-slate-400">
                        { "Sign in to your parkrun account. To create an account or reset your password head to the " }
                        <a href="https://www.parkrun.org.uk/register/" class="font-medium text-indigo-600 hover:text-indigo-500"> { "Parkrun website" } </a>
                        { "." }
                    </p>
                </div>
                <form class="mt-8 space-y-6" {onsubmit} >
                    <div class="rounded-md shadow-sm space-y-6">
                        <Input
                            id="athlete-id"
                            name="athlete id"
                            label="Athlete ID"
                            field_type={TextFieldType::Text}
                            auto_complete="username"
                            required=true
                            placeholder="Athlete ID"
                            onchange={
                                let athlete_id = athlete_id.clone();
                                Callback::from(move |event: Event| {
                                    let input: HtmlInputElement = event.target_unchecked_into();
                                    athlete_id.set(input.value());
                                })
                            }
                        />
                        <Input
                            id="password"
                            name="password"
                            label="Password"
                            field_type={TextFieldType::Password}
                            auto_complete="current-password"
                            required=true
                            placeholder="Password"
                            onchange={
                                let password = password.clone();
                                Callback::from(move |event: Event| {
                                    let input: HtmlInputElement = event.target_unchecked_into();
                                    password.set(input.value());
                                })
                            }
                        />
                    </div>
                    <Button text="Login" />
                </form>
            </div>
        </div>
    }
}
