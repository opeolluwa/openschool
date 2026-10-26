use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, LitStr, Type};

const VARBINARY_NONE: &str = "VarBinary(StringLen::None)";

pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let mut input = input;

    let data_struct = match &mut input.data {
        Data::Struct(data) => data,
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "stringify_ulid_identifiers can only be applied to structs",
            ));
        }
    };

    let fields = match &mut data_struct.fields {
        Fields::Named(fields) => &mut fields.named,
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "stringify_ulid_identifiers only supports structs with named fields",
            ));
        }
    };

    let struct_ident = input.ident.clone();
    let helper = format_ident!("__ulid_serde_for_{}", struct_ident);

    let serialize_with = LitStr::new(
        &format!("self::{helper}::serialize"),
        proc_macro2::Span::call_site(),
    );
    let deserialize_with = LitStr::new(
        &format!("self::{helper}::deserialize"),
        proc_macro2::Span::call_site(),
    );

    for field in fields.iter_mut() {
        let name = field
            .ident
            .as_ref()
            .expect("named fields carry an identifier")
            .to_string();

        if field.attrs.iter().any(|attr| attr.path().is_ident("serde")) {
            continue;
        }

        let rename = to_camel_case(&name);

        if is_ulid_field(field) {
            field.attrs.push(syn::parse_quote!(#[serde(
                rename = #rename,
                serialize_with = #serialize_with,
                deserialize_with = #deserialize_with,
            )]));
        } else if is_option_ulid_field(field) {
            let opt_serialize_with = LitStr::new(
                &format!("self::{helper}::serialize_option"),
                proc_macro2::Span::call_site(),
            );
            let opt_deserialize_with = LitStr::new(
                &format!("self::{helper}::deserialize_option"),
                proc_macro2::Span::call_site(),
            );
            field.attrs.push(syn::parse_quote!(#[serde(
                rename = #rename,
                serialize_with = #opt_serialize_with,
                deserialize_with = #opt_deserialize_with,
            )]));
        } else {
            field
                .attrs
                .push(syn::parse_quote!(#[serde(rename = #rename)]));
        }
    }

    let helper_module = ulid_helper_module(&helper);

    Ok(quote!(
        #helper_module
        #input
    ))
}

fn to_camel_case(input: &str) -> String {
    let mut camel = String::with_capacity(input.len());
    let mut capitalize = false;

    for character in input.chars() {
        if character == '_' {
            capitalize = true;
            continue;
        }

        if capitalize {
            camel.extend(character.to_uppercase());
            capitalize = false;
        } else {
            camel.push(character);
        }
    }

    camel
}

fn ulid_helper_module(helper: &Ident) -> TokenStream {
    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, dead_code)]
        mod #helper {
            use ::serde::{Deserialize, Deserializer, Serializer};
            use ::ulid::Ulid;

            pub(super) fn to_ulid(bytes: &[u8]) -> Option<Ulid> {
                let bytes: [u8; 16] = bytes.try_into().ok()?;

                Some(Ulid::from_bytes(bytes))
            }

            pub(super) fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: AsRef<[u8]> + ?Sized,
            {
                let text = to_ulid(value.as_ref())
                    .map(|ulid| ulid.to_string())
                    .ok_or_else(|| {
                        ::serde::ser::Error::custom(
                            "identifier is not a valid 16 byte ULID",
                        )
                    })?;

                serializer.serialize_str(&text)
            }

            pub(super) fn deserialize<'de, D>(
                deserializer: D,
            ) -> Result<Vec<u8>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let text = String::deserialize(deserializer)?;

                Ulid::from_string(&text)
                    .map(|ulid| ulid.to_bytes().to_vec())
                    .map_err(|_| {
                        ::serde::de::Error::custom(format!(
                            "`{text}` is not a valid ULID",
                        ))
                    })
            }

            pub(super) fn serialize_option<S>(
                value: &Option<Vec<u8>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match value {
                    Some(bytes) => serialize(bytes, serializer),
                    None => serializer.serialize_none(),
                }
            }

            pub(super) fn deserialize_option<'de, D>(
                deserializer: D,
            ) -> Result<Option<Vec<u8>>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let text: Option<String> = Option::deserialize(deserializer)?;

                match text {
                    Some(t) => Ulid::from_string(&t)
                        .map(|ulid| Some(ulid.to_bytes().to_vec()))
                        .map_err(|_| {
                            ::serde::de::Error::custom(format!(
                                "`{t}` is not a valid ULID",
                            ))
                        }),
                    None => Ok(None),
                }
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                const BYTES: [u8; 16] = [
                    1, 160, 219, 107, 176, 164, 143, 12, 144, 118, 32, 238, 10, 81, 191, 219,
                ];
                const ULID: &str = "01M3DPQC54HW690XH0XR553FYV";

                #[test]
                fn round_trips_through_the_canonical_string() {
                    let ulid = to_ulid(&BYTES).expect("16 bytes is a ULID");

                    assert_eq!(ulid.to_string(), ULID);
                    assert_eq!(Ulid::from_string(ULID).expect("parses").to_bytes(), BYTES);
                }
            }
        }
    }
}

fn is_ulid_field(field: &syn::Field) -> bool {
    let mut declared_column_type = false;

    for attr in &field.attrs {
        if !attr.path().is_ident("sea_orm") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if !meta.path.is_ident("column_type") {
                return Ok(());
            }

            declared_column_type = true;

            if let Ok(value) = meta.value()
                && let Ok(lit) = value.parse::<syn::LitStr>()
                && lit.value() == VARBINARY_NONE
            {
                declared_column_type = false;
            }

            Ok(())
        });
    }

    if declared_column_type {
        return false;
    }

    is_vec_u8(&field.ty)
}

fn is_vec_u8(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    if segment.ident != "Vec" {
        return false;
    }

    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };

    arguments
        .args
        .iter()
        .any(|argument| matches!(argument, syn::GenericArgument::Type(inner) if is_u8(inner)))
}

fn is_option_ulid_field(field: &syn::Field) -> bool {
    let mut has_varbinary_none = false;
    let mut has_column_type = false;

    for attr in &field.attrs {
        if !attr.path().is_ident("sea_orm") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("column_type") {
                has_column_type = true;
                if let Ok(value) = meta.value()
                    && let Ok(lit) = value.parse::<syn::LitStr>()
                    && lit.value() == VARBINARY_NONE
                {
                    has_varbinary_none = true;
                }
            }
            Ok(())
        });
    }

    if has_column_type && !has_varbinary_none {
        return false;
    }

    is_option_vec_u8(&field.ty)
}

fn is_option_vec_u8(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    if segment.ident != "Option" {
        return false;
    }

    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };

    arguments
        .args
        .iter()
        .any(|argument| matches!(argument, syn::GenericArgument::Type(inner) if is_vec_u8(inner)))
}

fn is_u8(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    type_path
        .path
        .segments
        .last()
        .map(|segment| segment.ident == "u8")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    fn first_field(input: proc_macro2::TokenStream) -> syn::Field {
        let parsed: DeriveInput = syn::parse2(input).expect("parses");

        match parsed.data {
            Data::Struct(data) => match data.fields {
                Fields::Named(fields) => fields.named.first().cloned().expect("has a field"),
                _ => panic!("expected named fields"),
            },
            _ => panic!("expected a struct"),
        }
    }

    #[test]
    fn matches_a_varbinary_none_primary_key() {
        let field = first_field(quote! {
            struct S {
                #[sea_orm(primary_key, auto_increment = false, column_type = "VarBinary(StringLen::None)")]
                identifier: Vec<u8>,
            }
        });

        assert!(is_ulid_field(&field));
    }

    #[test]
    fn matches_a_varbinary_none_foreign_key() {
        let field = first_field(quote! {
            struct S {
                #[sea_orm(column_type = "VarBinary(StringLen::None)")]
                form_template_identifier: Vec<u8>,
            }
        });

        assert!(is_ulid_field(&field));
    }

    #[test]
    fn ignores_other_binary_column_types() {
        let field = first_field(quote! {
            struct S {
                #[sea_orm(column_type = "VarBinary(255)")]
                payload: Vec<u8>,
            }
        });

        assert!(!is_ulid_field(&field));
    }

    #[test]
    fn matches_a_bare_vec_u8_without_a_column_type() {
        let field = first_field(quote! {
            struct S {
                identifier: Vec<u8>,
            }
        });

        assert!(is_ulid_field(&field));
    }

    #[test]
    fn ignores_non_byte_columns() {
        let field = first_field(quote! {
            struct S {
                #[sea_orm(column_type = "Text")]
                name: String,
            }
        });

        assert!(!is_ulid_field(&field));
    }

    #[test]
    fn converts_column_names_to_camel_case() {
        assert_eq!(to_camel_case("identifier"), "identifier");
        assert_eq!(to_camel_case("form_type"), "formType");
        assert_eq!(
            to_camel_case("form_template_identifier"),
            "formTemplateIdentifier"
        );
        assert_eq!(to_camel_case("created_at"), "createdAt");
    }

    #[test]
    fn generates_a_helper_module_and_field_serde_attrs() {
        let input: DeriveInput = syn::parse2(quote! {
            #[sea_orm(table_name = "admission_applicant")]
            struct Model {
                #[sea_orm(primary_key, auto_increment = false, column_type = "VarBinary(StringLen::None)")]
                identifier: Vec<u8>,
                created_at: String,
            }
        })
        .expect("parses");

        let expanded = expand(input)
            .expect("expands")
            .to_token_stream()
            .to_string();

        assert!(expanded.contains("__ulid_serde_for_Model"));
        assert!(expanded.contains("serialize_with"));
        assert!(expanded.contains("rename = \"createdAt\""));
        assert!(expanded.contains("Ulid"));
    }
}
