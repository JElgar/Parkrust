use crate::routes::events::Events;
use crate::{
    components::{Card, LoadingSpinner},
    routes::results::Results,
    services::parkrun::use_results,
};
use chrono::prelude::*;
use chrono::{Duration, Month, Utc};
use num_traits::cast::FromPrimitive;
use parkrust::client::requests::{
    average_speed, average_time, duration_formatter, events, fastest_time, total_time,
};
use parkrust::models::parkrun::RunResult;
use yew::prelude::*;

#[function_component(Calendar)]
pub fn calendar() -> Html {
    let results_state = use_results();

    pub fn get_next_saturday(start_date: NaiveDate) -> NaiveDate {
        let days_till_next_saturday = (7 - start_date.weekday().num_days_from_sunday() + 6) % 7;
        start_date + Duration::days(days_till_next_saturday as i64)
    }

    fn get_saturdays_in_month(month: u32, year: i32) -> Vec<NaiveDate> {
        let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let first_saturday = get_next_saturday(first_day_of_month);

        let mut saturdays = Vec::new();
        let mut current_saturday = first_saturday;
        while current_saturday.month() == month {
            saturdays.push(current_saturday);
            current_saturday += Duration::days(7);
        }
        saturdays
    }

    fn result_on_day(day: &NaiveDate, results: &[RunResult]) -> Option<RunResult> {
        results.iter().find_map(|result| {
            if &result.date() == day {
                Some(result.clone())
            } else {
                None
            }
        })
    }

    fn rows(results: &[RunResult]) -> Html {
        (1..=12).map(|month| {
            let day_tiles = get_saturdays_in_month(month, Local::now().year()).iter().map(|day| {
                let result = result_on_day(day, results);
                let classes = {
                    let background_colors = match result {
                        Some(_) => {
                            "bg-green-400"
                        },
                        None => {
                            if day >= &Utc::now().naive_local().date() {
                                "bg-white dark:bg-slate-700"
                            } else {
                                "bg-slate-100 dark:bg-slate-500"
                            }
                        }
                    };
                    format!("lg:m-1 p-1 w-10 rounded-lg shadow-md dark:border-gray-700 text-center {background_colors}")
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
        }
        None => {
            html! {
                <div> { "Loading..." } </div>
            }
        }
    };

    html! {
        <>
            <div class="mt-3 text-3xl font-bold leading-8 mb-6"> { "Calendar" } </div>
            { calendar }
        </>
    }
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct StatCardProps {
    pub title: AttrValue,
    pub emoji: AttrValue,
    pub value: AttrValue,
}

#[function_component(StatCard)]
pub fn stat_card(
    StatCardProps {
        value,
        title,
        emoji,
    }: &StatCardProps,
) -> Html {
    html! {
        <div class="transform col-span-6 lg:col-span-3">
            <Card>
                <div class="w-full">
                   <div class="mt-3 text-3xl font-bold leading-8 flex flex-col sm:flex-row">
                    <div> { emoji } </div>
                    <div class="mt-3 sm:mt-0 sm:ml-1"> { value } </div>
                   </div>
                   <div class="mt-1 text-base text-gray-600 dark:text-white"> { title } </div>
                </div>
            </Card>
        </div>
    }
}

fn format_total_time(total_time: Duration) -> String {
    if total_time.num_minutes() < 100 {
        format!("{}m", total_time.num_minutes())
    } else if total_time.num_hours() < 1000 {
        format!("{}h", total_time.num_hours())
    } else {
        format!("{}d", total_time.num_days())
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
                        <StatCard emoji="⏱" title="Avg time" value={ duration_formatter(average_time(results)) } />
                        <StatCard emoji="⏳" title="Total time" value={ format_total_time(total_time(results)) } />
                        <StatCard emoji="📍" title="Locations" value={ events(results).len().to_string() } />
                        <StatCard emoji="🚀" title="Fastest time" value={ duration_formatter(fastest_time(results)) } />
                        <StatCard emoji="📅" title="This year" value={ results.iter().filter(|result| result.date().year() ==  Local::now().year()).count().to_string() } />
                        <StatCard emoji="👪" title="Best position" value={ results.iter().map(|result| result.position()).min().unwrap().to_string() } />
                        <StatCard emoji="⌚" title="Avg min/km" value={ duration_formatter(average_speed(results)) } />

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
                <div> <LoadingSpinner /> </div>
            }
        }
    }
}
