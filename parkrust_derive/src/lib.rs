use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::{self, parse_macro_input, AttributeArgs, ItemStruct, Ident};
use darling::FromMeta;

#[derive(Debug, FromMeta)]
struct ParkrunModelArgs {}

#[proc_macro_attribute]
pub fn parkrun_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(item as ItemStruct);
    let args_ast = parse_macro_input!(attr as AttributeArgs);
    // println!("Args: {:?}", args);
    let args = ParkrunModelArgs::from_list(&args_ast).unwrap();

    // Build the trait implementation
    impl_parkrun_model(ast, args)
}

fn impl_parkrun_model(ast: ItemStruct, args: ParkrunModelArgs) -> TokenStream {
    let struct_def = quote! {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(rename_all = "PascalCase")]
        #ast
    };

    quote! {
        #struct_def
    }.into()
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

        #[async_trait]
        impl Listable<#args_type> for #name {
            async fn list(args: #args_type, parkrun_client: &AuthenticatedParkrunClient) -> Result<Vec<#name>, Box<dyn std::error::Error>> {
                let response = parkrun_client
                    .request(reqwest::Method::GET, #endpoint)
                    .query(&[("athleteId", args.athlete_id)])
                    .send()
                    .await?;

                println!("Response is: {:?}", response);
                
                Ok(response.json::<ListResponse<#list_response_ident>>()
                    .await?
                    .data
                    .#data_key)
            }
        }
    };
    return quote! {
        #ast
        #list_def
    }.into();
}
