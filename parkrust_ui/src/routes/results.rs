use parkrust::models::parkrun::{RunResult, ResultsQuery, Listable};
use parkrust::client::{ParkrunClient, AuthenticatedParkrunClient, Token};
use parkrust_ui_derive::table_data_type;

use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch}; 
use material_yew::{MatTextField, MatButton};

use crate::components::Table;
use crate::components::table::TableDataType;
use crate::services::parkrun::{get_client, get_user_results, use_results};
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
    let results_state = use_results();

    match &*results_state {
        Some(results) => {
            let table_data = results.iter().map(|result| ResultTableData::from_parkrun_result(result)).collect::<Vec<ResultTableData>>();
            html! {
                <Table<ResultTableData> data={table_data} />
            }
        },
        None => {
            html! {
                <div> { "Loading..." } </div> 
            }
        }
    }
}
