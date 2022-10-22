use parkrust::models::parkrun::{RunResult, ResultsQuery, Listable};
use parkrust::client::{ParkrunClient, AuthenticatedParkrunClient, Token};

use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch}; 
use material_yew::{MatTextField, MatButton};

use crate::{
    utils::router::Route,
    routes::login::Login,
    services::parkrun::{AuthContext, AuthState},
};

#[function_component(Results)]
pub fn results() -> Html {
    let results = use_state(|| vec![]);
    let auth_ctx = use_context::<AuthContext>().unwrap();

    let id = "";
    // let password = "";
                    
    println!("Building app");

    {
        let results = results.clone();
        println!("Getting stuff");
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let auth_data = auth_ctx.data.clone().unwrap();
                let client = AuthenticatedParkrunClient::new(auth_data.token);
                let response: Vec<RunResult> = RunResult::list(ResultsQuery{ athlete_id: auth_data.athlete_id }, &client).await.unwrap();

                results.set(response);
            });
            || ()
        }, ());
    }

    html! {
        <table class="table-auto">
            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
              <tr>
                <th>{"Date"}</th>
                <th>{"Time"}</th>
              </tr>
            </thead>
            <tbody>
                { 
                    results.iter().map(|result| {
                        html!{
                            <tr key={result.run_id.clone()} class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                                <td>
                                    { result.event_date.clone() }
                                </td>
                                <td>
                                    { result.run_time.clone() }
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
