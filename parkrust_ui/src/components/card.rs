use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub children: Children,
}

#[function_component(Card)]
pub fn card(CardProps { children }: &CardProps) -> Html {
    html! {
        <div class="p-6 bg-white rounded-lg shadow-md dark:bg-slate-800 dark:border-gray-700 dark:text-white">
            { for children.iter() }
        </div>
    }
}
