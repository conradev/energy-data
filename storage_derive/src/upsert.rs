use quote::{quote, ToTokens};
use std::fmt::Display;

use crate::schema::Schema;

pub struct Upsert<'a> {
    pub schema: &'a Schema,
}

impl<'a> Upsert<'a> {
    pub fn implementation(&self) -> impl ToTokens {
        let params = self.schema.fields.iter().map(|field| {
            let ident = &field.ident;
            quote! {
                (&self.#ident as &dyn ::rusqlite::ToSql),
            }
        });
        quote! {
            fn upsert(&self, statement: &mut ::rusqlite::Statement) -> ::rusqlite::Result<()> {
                statement
                    .execute(&[
                        #(#params)*
                    ])
                    .map(|_| ())
            }
        }
    }
}

impl<'a> Display for Upsert<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let fields: String = self
            .schema
            .fields
            .iter()
            .map(|f| f.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let values: String = self
            .schema
            .fields
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "REPLACE INTO {}({}) VALUES ({});",
            self.schema.table_name, fields, values
        )
    }
}
