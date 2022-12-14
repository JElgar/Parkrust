use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Fields, FieldsNamed, Ident, ItemStruct};

#[proc_macro_attribute]
pub fn table_data_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemStruct);
    impl_table_data_type(ast)
}

fn impl_table_data_type(struct_: ItemStruct) -> TokenStream {
    let name = &struct_.ident;
    let header_idents: Vec<Ident> = match &struct_.fields {
        Fields::Named(FieldsNamed { named, .. }) => named
            .iter()
            .map(|field| field.ident.clone().unwrap())
            .collect(),
        _ => panic!("Only named structs are supported"),
    };
    let header_strings: Vec<String> = header_idents
        .iter()
        .map(|ident| ident.to_string())
        .collect();

    quote! {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(PartialEq)]
        #struct_

        impl crate::components::table::TableDataType for #name {
            fn get_headers() -> Vec<crate::components::table::TableHeaderData> {
                vec![#( #header_strings ),*]
            }

            fn get_row(&self) -> Vec<crate::components::table::TableCellData> {
                vec![#( self.#header_idents.clone() ),*]
            }
        }
    }
    .into()
}
