use yew::prelude::*;
use parkrust_ui_derive::table_data_type;

#[table_data_type()]
pub struct TableThing {
    pub test: String,
    pub test2: String,
}

pub trait TableDataType {
    fn get_headers() -> Vec<&'static str>;
    fn get_row(&self) -> Html;
}

// Probably use a map for the data instead? Or create a macro!
#[derive(Clone, PartialEq, Properties)]
pub struct TableProps<T: PartialEq> {
    data: Vec<T>
}

// TODO Create a macro for table type then the above T: TableType
//
// The macro should define, T::get_headers() and T().get_row()
// TODO Either have hte TableDataType return data (I think this is best) so get_row ->
// Vec<CellData> or have it return html


#[function_component(Table)]
pub fn table<T: TableDataType>(TableProps { data }: &TableProps<T>) -> Html {
    html! {
        <table class="table-auto">
            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
              <tr>
                <th>{"Date"}</th>
                <th>{"Time"}</th>
              </tr>
            </thead>
            <tbody>
                { 
                    data.iter().map(|value| {
                        value.get_row()
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_headers() {
        #[table_data_type()]
        struct TestType {
            pub test: String,
            pub test2: String,
        }

        assert_eq!(TestType::get_headers(), vec!["test", "test2"]);
    }
}
