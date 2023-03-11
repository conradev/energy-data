use proc_macro2::Ident;
use std::fmt::Display;
use syn::{Data, DataStruct, Fields};

use crate::attrs::Attributes;
use crate::types::SqlType;
use crate::upsert::Upsert;

pub struct Schema {
    pub table_name: String,
    pub primary_key: String,
    pub fields: Vec<Field>,
    pub extras: Vec<String>,
}

impl Schema {
    pub fn new(attrs: &Attributes, data: Data) -> Result<Schema, syn::Error> {
        let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) = data else {
            panic!("lol");
            // return Err(syn::Error::new_spanned("Store can only be derived on structs with named fields"));
        };
        let table_name = match &attrs.table_name {
            Some(t) => t.clone(),
            None => {
                return Err(syn::Error::new_spanned(
                    fields,
                    "You must specify a table name with the table attribute",
                ))
            }
        };
        let primary_key = match &attrs.primary_key {
            Some(t) => t.clone(),
            None => {
                return Err(syn::Error::new_spanned(
                    fields,
                    "You must specify a primary key with the primary_key attribute",
                ))
            }
        };
        let fields: Result<Vec<_>, _> = fields
            .named
            .iter()
            .map(|field| {
                let mut field = Field::try_from(field);
                if let Ok(field) = field.as_mut() {
                    field.primary_key = attrs.primary_key.as_ref() == Some(&field.name);
                }
                field
            })
            .collect();
        let extras = attrs.extras.clone();

        Ok(Schema {
            table_name,
            primary_key,
            fields: fields?,
            extras,
        })
    }

    pub fn upsert(&self) -> Upsert {
        Upsert { schema: self }
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let schema_body = self
            .fields
            .iter()
            .map(|f| format!("    {f}"))
            .collect::<Vec<_>>()
            .join(",\n");

        write!(
            f,
            "CREATE TABLE IF NOT EXISTS {} (\n{}{}) WITHOUT ROWID;",
            self.table_name,
            schema_body,
            self.extras.join("\n")
        )
    }
}

pub struct Field {
    pub ident: Ident,
    pub name: String,
    pub sql_type: SqlType,
    pub primary_key: bool,
}

impl TryFrom<&syn::Field> for Field {
    type Error = syn::Error;

    fn try_from(value: &syn::Field) -> Result<Field, Self::Error> {
        let Some(ident) = value.ident.as_ref() else {
            return Err(syn::Error::new_spanned(value, "All fields in the structure must have a name"));
        };
        Ok(Field {
            ident: ident.clone(),
            name: ident.to_string(),
            sql_type: SqlType::try_from(&value.ty)?,
            primary_key: false,
        })
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let name = Some(self.name.as_str());
        let sql_type = Some(self.sql_type.sql_type.as_str());
        let primary_key = self.primary_key.then_some("PRIMARY KEY");
        let nullable = (!self.sql_type.nullable).then_some("NOT NULL");

        // name TYPE (PRIMARY KEY) (NOT NULL)
        let field = [name, sql_type, primary_key, nullable]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{field}")
    }
}
