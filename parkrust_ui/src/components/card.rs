use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub children: Children,
}

#[function_component(Card)]
pub fn card(CardProps{ children }: &CardProps) -> Html {
    html! {
        <div class="p-6 bg-slate-500 rounded-lg shadow-md dark:bg-gray-800 dark:border-gray-700">
            { for children.iter() }
        </div>
    }
}
