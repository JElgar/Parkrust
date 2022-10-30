use yew::prelude::*;
use crate::{components::Card, routes::results::Results}; 

#[derive(Clone, PartialEq, Properties)]
pub struct StatCardProps {
    pub title: AttrValue,
    pub value: AttrValue,
}

#[function_component(StatCard)]
pub fn stat_card(StatCardProps { value, title }: &StatCardProps) -> Html {
    html! {
        <Card> 
            <div class="w-full">
               <div class="mt-3 text-3xl font-bold leading-8"> { value } </div>
               <div class="mt-1 text-base text-gray-600"> { title } </div>
            </div>
        </Card>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! { 
        <div class="grid grid-cols-12 gap-6 mt-5">
            <div class="transform col-span-3"> <StatCard title="Total runs" value="10" /> </div>
            <div class="transform col-span-3"> <Card> { "Hello 2" } </Card> </div>
            <div class="transform col-span-3"> <Card> { "Hello 3" } </Card> </div>
            <div class="transform col-span-3"> <Card> { "Hello 4" } </Card> </div>
            <div class="transform col-span-12"> <Card> <Results /> </Card> </div>
        </div>
    }
}

