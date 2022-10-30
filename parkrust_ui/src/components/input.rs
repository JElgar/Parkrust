use yew::prelude::*;
use yew::virtual_dom::AttrValue;

/// The `TextFieldType` type
#[derive(Debug, Clone, PartialEq)]
pub enum TextFieldType {
    Text,
    Search,
    Tel,
    Url,
    Email,
    Password,
    Date,
    Month,
    Week,
    Time,
    DatetimeLocal,
    Number,
    Color,
}

impl TextFieldType {
    pub fn as_str(&self) -> &'static str {
        use TextFieldType::*;
        match self {
            Text => "text",
            Search => "search",
            Tel => "tel",
            Url => "url",
            Email => "email",
            Password => "password",
            Date => "date",
            Month => "month",
            Week => "week",
            Time => "time",
            DatetimeLocal => "datetime-local",
            Number => "number",
            Color => "color",
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputProps {
    pub id: AttrValue,
    pub name: AttrValue,
    pub label: AttrValue,
    pub field_type: TextFieldType,
    pub auto_complete: AttrValue,
    pub required: bool,
    pub placeholder: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Input)]
pub fn input(
    InputProps {
        id,
        name,
        label,
        field_type,
        auto_complete,
        required,
        placeholder,
        onchange,
    }: &InputProps,
) -> Html {
    html! {
        <div>
            <label htmlFor={id.clone()} class="sr-only">
                {label.clone()}
            </label>
            <input
                id={id.clone()}
                name={name.clone()}
                type={field_type.as_str()}
                autoComplete={auto_complete.clone()}
                required={*required}
                class="mt-1 p-2 block w-full rounded-md border border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                placeholder={placeholder.clone()}
                {onchange}
            />
        </div>
    }
}
