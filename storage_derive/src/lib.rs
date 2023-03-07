use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod schema;
mod types;
mod upsert;

use attrs::Attributes;
use schema::Schema;

#[proc_macro_derive(Store, attributes(storage))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    try_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn try_derive(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let type_name = input.ident;
    let attrs = Attributes::from(input.attrs);

    let schema = Schema::new(&attrs, input.data)?;
    let table_name = schema.table_name.as_str();

    let schema_sql = format!("{}", schema);
    let upsert_sql = format!("{}", schema.upsert());
    let upsert = schema.upsert().implementation();

    Ok(quote! {
        #[automatically_derived]
        impl Store for #type_name {
            fn initialize(conn: &mut ::rusqlite::Connection) -> ::rusqlite::Result<()> {
                conn.execute_batch(#schema_sql)
            }

            fn upsert_statement<'a>(conn: &'a ::rusqlite::Connection) -> ::rusqlite::Result<::rusqlite::Statement<'a>> {
                conn.prepare(#upsert_sql)
            }

            #upsert

            const PROGRESS_NAME: &'static str = #table_name;
        }
    }
    .into())
}
