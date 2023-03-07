use syn::{GenericArgument, PathArguments, Type, TypePath};

pub struct SqlType {
    pub sql_type: String,
    pub nullable: bool,
}

impl TryFrom<&Type> for SqlType {
    type Error = syn::Error;

    fn try_from(value: &Type) -> Result<SqlType, Self::Error> {
        if let Some(inner) = option_inner_type(value) {
            let mut sql_type = SqlType::try_from(inner)?;
            sql_type.nullable = true;
            return Ok(sql_type);
        }

        let Type::Path(TypePath { path, .. }) = value else {
            return Err(syn::Error::new_spanned(value, "Unsupported type"));
        };
        let type_mapping = [
            ("bool", "INTEGER"),
            ("u8", "INTEGER"),
            ("u16", "INTEGER"),
            ("u32", "INTEGER"),
            ("u64", "INTEGER"),
            ("NaiveDate", "TEXT"),
            ("String", "TEXT"),
            ("f32", "REAL"),
            ("f64", "REAL"),
        ];
        let Some((_, sql_type)) = type_mapping.iter().find(|(i, _)| path.is_ident(i)) else {
            return Err(syn::Error::new_spanned(path, "Unsupported type"));
        };

        Ok(SqlType {
            sql_type: sql_type.to_string(),
            nullable: false,
        })
    }
}

fn option_inner_type(path: &Type) -> Option<&Type> {
    let Type::Path(TypePath { path, .. }) = path else {
        return None;
    };
    if path.leading_colon.is_some() {
        return None;
    }
    if path.segments.len() != 1 || path.segments[0].ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(ab) = &path.segments[0].arguments else {
        return None;
    };
    if ab.args.len() != 1 {
        return None;
    }
    let GenericArgument::Type(t) = &ab.args[0] else {
        return None;
    };
    Some(t)
}
