use darling::FromMeta;
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{self, parse_macro_input, AttributeArgs, Ident, ItemStruct};

#[proc_macro_attribute]
pub fn parkrun_model(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemStruct);

    quote! {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(rename_all = "PascalCase")]
        #ast
    }
    .into()
}

#[proc_macro_attribute]
pub fn parkrun_request_args(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemStruct);

    quote! {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(rename_all = "camelCase")]
        #ast
    }
    .into()
}

#[derive(Debug, FromMeta)]
struct ParkrunListArgs {
    endpoint: String,
    args_type: Ident,
    data_key: Ident,
}

#[proc_macro_attribute]
pub fn parkrun_list(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(item as ItemStruct);
    let args_ast = parse_macro_input!(attr as AttributeArgs);
    // println!("Args: {:?}", args);
    let args = ParkrunListArgs::from_list(&args_ast).unwrap();

    // Build the trait implementation
    impl_parkrun_list(ast, args)
}

fn impl_parkrun_list(ast: ItemStruct, args: ParkrunListArgs) -> TokenStream {
    // Object name
    let name = &ast.ident;
    let endpoint = args.endpoint;
    let args_type = &args.args_type;
    let data_key = &args.data_key;

    let list_response_ident = Ident::new(format!("List{}Response", name).as_str(), name.span());

    let list_def = quote! {
        #[parkrun_model()]
        struct #list_response_ident {
            pub #data_key: Vec<#name>,
        }

        #[async_trait(?Send)]
        impl Listable<#args_type> for #name {
            async fn list(args: #args_type, parkrun_client: &mut AuthenticatedParkrunClient) -> Result<Vec<#name>, Box<dyn std::error::Error + Send + Sync>> {

                // Make list call with params
                let request = parkrun_client
                    .request(reqwest::Method::GET, #endpoint)
                    .query(&args);

                let mut response = parkrun_client.send_request_with_refresh(request)
                    .await?
                    .json::<ListResponse<#list_response_ident>>()
                    .await?;

                let mut items: Vec<#name> = Vec::new();
                items.extend(response.data.#data_key);

                // While there is a next page token
                while let Some(next_page) = response.links.iter().find_map(|link| {
                    if link.rel == "next" {
                        Some(link.href.clone())
                    } else {
                        None
                    }
                }) {
                    // Update the response with the next page response
                    let request = parkrun_client.request(reqwest::Method::GET, &next_page.as_str()[2..]);
                    response = parkrun_client.send_request_with_refresh(request)
                        .await?
                        .json::<ListResponse<#list_response_ident>>()
                        .await?;

                    // And push the items to the output list 
                    items.extend(response.data.#data_key);
                }

                Ok(items)
            }
        }
    };
    return quote! {
        #ast
        #list_def
    }
    .into();
}
