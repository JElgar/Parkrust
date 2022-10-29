use parkrust_ui_derive::table_data_type;
use yew::prelude::*;

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
    pub data: Vec<T>,
}

// TODO Create a macro for table type then the above T: TableType
//
// The macro should define, T::get_headers() and T().get_row()
// TODO Either have hte TableDataType return data (I think this is best) so get_row ->
// Vec<CellData> or have it return html

#[function_component(Table)]
pub fn table<T: TableDataType>(TableProps { data }: &TableProps<T>) -> Html {
    html! {
        <div class="overflow-x-auto relative">
          <table class="min-w-full divide-y divide-gray-200 table-auto ixed dark:divide-gray-700">
              <thead class="bg-gray-100 dark:bg-gray-700">
                <tr>
                  {
                      T::get_headers().iter().enumerate().map(|(index, header)| {
                        let background_color_css = if index % 2 == 0 {
                            ""
                        } else {
                            "bg-gray-50 dark:bg-gray-800"
                        };

                        html! {
                            <th scope="col" class={"py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase dark:text-gray-400".to_owned() + background_color_css}>{ header }</th>
                        }
                      }).collect::<Html>()
                  }
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                  {
                      data.iter().map(|value| {
                          let rows = value.get_row().iter().map(|cell_data| {
                              html! {
                                  <td class="py-4 px-6 text-sm font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                      { cell_data }
                                  </td>
                              }
                          }).collect::<Html>();
                          html! {
                              <tr class="hover:bg-gray-100 dark:hover:bg-gray-700">
                                  { rows }
                              </tr>
                          }
                      }).collect::<Html>()
                  }
              </tbody>
          </table>
        </div>
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
