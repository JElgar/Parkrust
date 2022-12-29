use parkrust::models::parkrun::{ RunResult, Event };
use parkrust_ui_derive::table_data_type;

use yew::prelude::*;

use crate::components::Table;
use crate::services::parkrun::{use_results, use_events};

#[table_data_type()]
pub struct ResultTableData {
    date: String,
    time: String,
    event_name: String,
}

impl ResultTableData {
    pub fn from_parkrun_result(
        RunResult {
            event_date,
            run_time,
            ..
        }: &RunResult,
        run_event: &Event,
    ) -> Self {
        ResultTableData {
            date: event_date.clone(),
            time: run_time.clone(),
            event_name: run_event.event_short_name.clone(),
        }
    }
}

#[function_component(Results)]
pub fn results() -> Html {
    let results_state = use_results();
    let events_state = use_events();

    match (&*results_state, &*events_state) {
        (Some(results), Some(events)) => {
            let table_data = results
                .iter()
                .map(|result| {
                    let event = events.iter().find(|event| event.event_number == result.event_number).unwrap();
                    ResultTableData::from_parkrun_result(result, event)
                })
                .collect::<Vec<ResultTableData>>();
            html! {
                <Table<ResultTableData> data={table_data} page_size={10}/>
            }
        }
        _ => {
            html! {
                <div> { "Loading..." } </div>
            }
        }
    }
}
