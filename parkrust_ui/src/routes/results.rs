use parkrust::models::parkrun::RunResult;
use parkrust_ui_derive::table_data_type;

use yew::prelude::*;

use crate::components::Table;
use crate::services::parkrun::use_results;

#[table_data_type()]
pub struct ResultTableData {
    date: String,
    time: String,
}

impl ResultTableData {
    pub fn from_parkrun_result(
        RunResult {
            event_date,
            run_time,
            ..
        }: &RunResult,
    ) -> Self {
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
            let table_data = results
                .iter()
                .map(ResultTableData::from_parkrun_result)
                .collect::<Vec<ResultTableData>>();
            html! {
                <Table<ResultTableData> data={table_data} page_size={10}/>
            }
        }
        None => {
            html! {
                <div> { "Loading..." } </div>
            }
        }
    }
}
