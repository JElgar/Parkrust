use darling::FromMeta;
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{self, parse_macro_input, AttributeArgs, Ident, ItemStruct, Fields, FieldsNamed};

#[proc_macro_attribute]
pub fn table_data_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemStruct);
    impl_table_data_type(ast)
}

fn impl_table_data_type(struct_: ItemStruct) -> TokenStream {
    let name = &struct_.ident;
    let header_idents: Vec<Ident> = match &struct_.fields {
        Fields::Named(FieldsNamed{ named, .. }) => {
            named.iter().map(|field| field.ident.clone().unwrap()).collect()
        },
        _ => panic!("Only named structs are supported")
    };
    let header_strings: Vec<String> = header_idents.clone().iter().map(|ident| ident.to_string()).collect();

    quote! {
        #struct_

        impl TableDataType for #name {
            fn get_headers() -> Vec<&'static str> {
                vec![#( #header_strings ),*]
            }
            
            fn get_row(&self) -> Html {
                html! {
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                        #(
                            <td>
                                { self.#header_idents.clone() }
                            </td>
                        )*
                    </tr>
                }
            }
        } 
    }.into()
}
