use yew::prelude::*;
use parkrust_ui_derive::table_data_type;

#[table_data_type()]
pub struct TableThing {
    pub test: String,
    pub test2: String,
}

pub type TableHeaderData = &'static str;
pub type TableCellData = String;
pub trait TableDataType: PartialEq {
    fn get_headers() -> Vec<TableHeaderData>;
    fn get_row(&self) -> Vec<TableCellData>;
}

// Probably use a map for the data instead? Or create a macro!
#[derive(Clone, PartialEq, Properties)]
pub struct TableProps<T: PartialEq> {
    pub data: Vec<T>
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
                { 
                    T::get_headers().iter().map(|header| {
                        html! {
                            <th>{ header }</th>
                        }
                    }).collect::<Html>()
                }
              </tr>
            </thead>
            <tbody>
                { 
                    data.iter().map(|value| {
                        let rows = value.get_row().iter().map(|cell_data| {
                            html! {
                                <td>
                                    { value.get_row() }
                                </td>
                            }
                        }).collect::<Html>();
                        html! { 
                            <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                                { rows }
                            </tr>
                        }
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
