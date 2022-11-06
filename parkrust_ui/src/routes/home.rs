use crate::routes::events::Events;
use crate::{components::Card, routes::results::Results, services::parkrun::use_results};
use parkrust::client::requests::{average_time, duration_formatter, events, total_time};
use parkrust::models::parkrun::RunResult;
use yew::prelude::*;
use chrono::{Utc, Duration, Month};
use chrono::prelude::*;
use num_traits::cast::FromPrimitive;

#[function_component(Calendar)]
pub fn calendar() -> Html {
    let results_state = use_results();

    pub fn get_next_saturday(start_date: Date<Utc>) -> Date<Utc> {
        let days_till_next_saturday = (7 - start_date.weekday().num_days_from_sunday() + 6) % 7;
        start_date + Duration::days(days_till_next_saturday as i64)
    }

    fn get_saturdays_in_month(month: u32, year: i32) -> Vec<Date<Utc>> {
        let first_day_of_month = Date::from_utc(NaiveDate::from_ymd(year, month, 1), Utc);
        let first_saturday = get_next_saturday(first_day_of_month);

        let mut saturdays = Vec::new();
        let mut current_saturday = first_saturday; 
        while current_saturday.month() == month {
            saturdays.push(current_saturday);
            current_saturday = current_saturday + Duration::days(7);
        }
        saturdays 
    }

    fn result_on_day(day: &Date<Utc>, results: &Vec<RunResult>) -> Option<RunResult> {
        results.iter().find_map(|result| if &result.date() == day { Some(result.clone()) } else { None })
    }

    fn rows(results: &Vec<RunResult>) -> Html {
        let year = 2022;
        (1..=12).map(|month| {
            let day_tiles = get_saturdays_in_month(month, year).iter().map(|day| {
                let result = result_on_day(day, results);
                let classes = {
                    let background_colors = match result {
                        Some(_) => {
                            "bg-green-400"
                        },
                        None => {
                            if day >= &Utc::now().date() {
                                "dark:bg-slate-700"
                            } else {
                                "dark:bg-slate-500"
                            }
                        }
                    };
                    format!("lg:m-1 p-1 w-10 bg-white rounded-lg shadow-md dark:border-gray-700 text-center {}", background_colors)
                };

                html! {
                    <td>
                        <div class={classes}>
                            { day.day() } 
                        </div>
                    </td>
                }
            }).collect::<Html>();

            html! {
                <tr class="text-right" > 
                    <td>  
                        <div class="pr-2">
                            { Month::from_u32(month).unwrap().name() }
                        </div>
                    </td>
                    { day_tiles } 
                </tr>
            }
        }).collect::<Html>()
    }

    let calendar = match &*results_state {
        Some(results) => {
            html! {
                <table class="table-fixed">
                    { rows(results) }
                </table>
            }
        },
        None => {
            html! {
                <div> { "Loading..." } </div>
            }
        },
    };

    html! {
        <>
            <div class="mt-3 text-3xl font-bold leading-8 mb-6"> { "Calendar" } </div>
            { calendar }
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct StatCardProps {
    pub title: AttrValue,
    pub emoji: AttrValue,
    pub value: AttrValue,
}


#[function_component(StatCard)]
pub fn stat_card(StatCardProps { value, title, emoji }: &StatCardProps) -> Html {
    html! {
        <div class="transform col-span-6 lg:col-span-3">
            <Card>
                <div class="w-full">
                   <div class="mt-3 text-3xl font-bold leading-8"> { format!("{} {}", emoji, value) } </div>
                   <div class="mt-1 text-base text-gray-600 dark:text-white"> { title } </div>
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
                        <StatCard emoji="🏃" title="Total runs" value={ results.len().to_string() } />
                        <StatCard emoji="⏱" title="Average time" value={ duration_formatter(average_time(results)) } />
                        <StatCard emoji="📆" title="Total time" value={ duration_formatter(total_time(results)) } />
                        <StatCard emoji="📍" title="Locations" value={ events(results).len().to_string() } />
                        <div class="col-span-12 md:col-span-6">
                            <Card>
                                <Calendar />
                            </Card>
                        </div>
                        <div class="transform col-span-12 row-span-3 md:col-span-6">
                            <Card>
                                <div class="mt-3 text-3xl font-bold leading-8 mb-6"> { "Results" } </div>
                                <Results />
                            </Card>
                        </div>
                        <div class="transform col-span-12 md:col-span-6">
                            <Card>
                                <div class="mt-3 text-3xl font-bold leading-8 mb-6"> { "Results" } </div>
                                <Events />
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
