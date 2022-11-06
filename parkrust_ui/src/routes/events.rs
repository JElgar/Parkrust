use parkrust::models::parkrun::{Event, RunResult};
use parkrust_ui_derive::table_data_type;

use yew::prelude::*;

use crate::components::Table;
use crate::services::parkrun::{use_events, use_results};

#[table_data_type()]
pub struct EventTableData {
    pub event_name: String,
    pub run_count: String,
}

impl EventTableData {
    pub fn from_parkrun_models(
        Event {
            event_short_name,
            event_number,
            ..
        }: &Event,
        results: &[RunResult],
    ) -> Self {
        EventTableData {
            event_name: event_short_name.clone(),
            run_count: results
                .iter()
                .filter(|result| &result.event_number == event_number)
                .count()
                .to_string(),
        }
    }
}

#[function_component(Events)]
pub fn events() -> Html {
    let results = use_results();
    let events_state = use_events();

    match &*events_state {
        Some(events) => {
            let results = results.as_ref().unwrap();

            let mut table_data = events
                .iter()
                .map(|event| EventTableData::from_parkrun_models(event, results))
                .collect::<Vec<EventTableData>>();

            table_data.sort_by_key(|event| -event.run_count.parse::<i32>().unwrap());
            html! {
                <Table<EventTableData> data={table_data} />
            }
        }
        None => {
            html! {
                <div> { "Loading..." } </div>
            }
        }
    }
}
