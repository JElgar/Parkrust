use std::num::NonZeroU32;

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
    pub page_size: Option<usize>,
}

// TODO Create a macro for table type then the above T: TableType
//
// The macro should define, T::get_headers() and T().get_row()
// TODO Either have hte TableDataType return data (I think this is best) so get_row ->
// Vec<CellData> or have it return html

#[function_component(Table)]
pub fn table<T: TableDataType>(TableProps { data, page_size }: &TableProps<T>) -> Html {
    let current_page = use_state(|| 1);
    let num_pages = match page_size { Some(page_size) => data.len() / page_size, None => 1 }; 
    // let page_data = match page_size {
    //     Some(page_size) => {
    //         data.iter().skip((*current_page - 1) * page_size).take(*page_size).collect::<Vec<T>>()
    //     },
    //     None => data.to_vec()
    // };

    html! {
        <div class="overflow-x-auto relative">
          <table class="min-w-full table-auto">
              <thead class="bg-gray-100 dark:bg-gray-700">
                <tr>
                  {
                      T::get_headers().iter().enumerate().map(|(index, header)| {
                        html! {
                            <th scope="col" class={"py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase dark:text-gray-400" }>{ header }</th>
                        }
                      }).collect::<Html>()
                  }
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                  {
                      data.iter().skip((*current_page - 1) * page_size.unwrap_or(0)).take(page_size.unwrap_or( usize::MAX )).map(|value| {
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
          if num_pages > 1 {
            <div class="flex items-center justify-center">
                <nav class="isolate inline-flex -space-x-px rounded-md shadow-sm" aria-label="Pagination">
                  <a href="#" class="relative inline-flex items-center rounded-l-md border border-gray-300 bg-white px-2 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 focus:z-20">
                    <span class="sr-only"> { "Previous" } </span>
                    <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                      <path fill-rule="evenodd" d="M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z" clip-rule="evenodd" />
                    </svg>
                  </a>
                  
                  {
                      (1..=num_pages).map(|page| {
                          let classes = {
                              if page == *current_page {
                                  "relative z-10 inline-flex items-center border border-indigo-500 bg-indigo-50 px-4 py-2 text-sm font-medium text-indigo-600 focus:z-20"
                              }
                              else if page <= num_pages - 3 && page > 3 {
                                  "hidden"
                              }
                              else if page <= num_pages - 2 && page > 2 {
                                  "relative hidden items-center border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 focus:z-20 lg:inline-flex"
                              }
                              else {
                                  "relative inline-flex items-center border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 focus:z-20"
                              }
                          };
                          html! {
                              <>
                                  <button 
                                      onclick={
                                          let current_page = current_page.clone();
                                          Callback::from(move |_| current_page.set(page))
                                      }
                                      class={classes}> { page.to_string() }
                                  </button>
                                  {
                                      if page == (num_pages as f64 / 2.0).floor() as usize {
                                          if num_pages > 6 {
                                              html! {
                                                  <span class="relative inline-flex items-center border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700">{"..."}</span>
                                              }
                                          }
                                          else if num_pages > 4 {
                                              html! {
                                                  <span class="relative inline-flex lg:hidden items-center border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700">{"..."}</span>
                                              }
                                          } 
                                          else { html! {} }
                                      } else { html! {} }
                                  }
                              </>
                          }
                      }).collect::<Html>()
                  }

                  <a href="#" class="relative inline-flex items-center rounded-r-md border border-gray-300 bg-white px-2 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 focus:z-20">
                    <span class="sr-only">{ "Next" } </span>
                    <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                      <path fill-rule="evenodd" d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z" clip-rule="evenodd" />
                    </svg>
                  </a>
                </nav>
              </div>
          }
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
