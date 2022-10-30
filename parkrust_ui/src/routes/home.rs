use crate::{components::Card, routes::results::Results, services::parkrun::use_results};
use parkrust::client::requests::{average_time, duration_formatter, events, total_time};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct StatCardProps {
    pub title: AttrValue,
    pub value: AttrValue,
}

#[function_component(StatCard)]
pub fn stat_card(StatCardProps { value, title }: &StatCardProps) -> Html {
    html! {
        <div class="transform sm:col-span-3 col-span-6">
            <Card>
                <div class="w-full">
                   <div class="mt-3 text-3xl font-bold leading-8"> { value } </div>
                   <div class="mt-1 text-base text-gray-600"> { title } </div>
                </div>
            </Card>
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let results_state = use_results();

    match &*results_state {
        Some(results) => {
            html! {
                <div class="p-8">
                    <div class="grid grid-cols-12 gap-6">
                        <StatCard title="Total runs" value={ results.len().to_string() } />
                        <StatCard title="Average time" value={ duration_formatter(average_time(results)) } />
                        <StatCard title="Total time" value={ duration_formatter(total_time(results)) } />
                        <StatCard title="Locations" value={ events(results).len().to_string() } />
                        <div class="transform col-span-12">
                            <Card>
                                <div class="mt-3 text-3xl font-bold leading-8 mb-6"> { "Results" } </div>
                                <Results />
                            </Card>
                        </div>
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <div> { "Loading..." } </div>
            }
        }
    }
}
