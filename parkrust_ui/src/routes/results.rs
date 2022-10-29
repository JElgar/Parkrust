use parkrust::models::parkrun::{RunResult, ResultsQuery, Listable};
use parkrust::client::{ParkrunClient, AuthenticatedParkrunClient, Token};
use parkrust_ui_derive::table_data_type;

use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch}; 
use material_yew::{MatTextField, MatButton};

use crate::components::Table;
use crate::components::table::TableDataType;
use crate::{
    utils::router::Route,
    routes::login::Login,
    services::parkrun::{AuthContext, AuthState},
};

#[table_data_type()]
pub struct ResultTableData {
    date: String,
    time: String
}

impl ResultTableData {
    pub fn from_parkrun_result(RunResult { event_date, run_time, .. }: &RunResult) -> Self {
        ResultTableData {
            date: event_date.clone(),
            time: run_time.clone(),
        }
    }
}

#[function_component(Results)]
pub fn results() -> Html {
    let results = use_state(|| vec![]);
    let auth_ctx = use_context::<AuthContext>().unwrap();
    let table_data = results.iter().map(|result| ResultTableData::from_parkrun_result(result)).collect::<Vec<ResultTableData>>();

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
        <Table<ResultTableData> data={table_data} />
    }
}
