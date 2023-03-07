use syn::{Attribute, Lit, Meta, MetaList, MetaNameValue, NestedMeta};

#[derive(Clone, Default)]
pub struct Attributes {
    pub table_name: Option<String>,
    pub primary_key: Option<String>,
    pub extras: Vec<String>,
}

impl From<Vec<Attribute>> for Attributes {
    fn from(input: Vec<Attribute>) -> Attributes {
        let mut attrs = Attributes::default();
        let nested_attrs = input
            .into_iter()
            .filter(|attr| attr.path.is_ident("storage"))
            .filter_map(|attr| attr.parse_meta().ok())
            .flat_map(|meta| match meta {
                Meta::List(MetaList { nested, .. }) => nested,
                _ => panic!(),
            });

        for attr in nested_attrs {
            match attr {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    path,
                    lit: Lit::Str(s),
                    ..
                })) => {
                    if path.is_ident("table") {
                        attrs.table_name = Some(s.value());
                    } else if path.is_ident("primary_key") {
                        attrs.primary_key = Some(s.value());
                    }
                }
                NestedMeta::Lit(Lit::Str(s)) => attrs.extras.push(s.value()),
                _ => (),
            }
        }

        attrs
    }
}
