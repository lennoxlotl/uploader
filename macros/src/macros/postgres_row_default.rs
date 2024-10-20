extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, Attribute, Data, DeriveInput, Fields, LitStr, Stmt, Token};

// The attributes which are extracted from the `PostgresRow` macro
#[derive(Debug, Clone)]
struct PostgresFieldAttributes {
    rename: Option<String>,
    flatten: bool,
}

// Implements the `PostgresRow` proc macro
pub fn postgres_row_default_impl(input: DeriveInput) -> TokenStream {
    let struct_ident = &input.ident;
    let named_fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("This proc-macro only supports named fields!"),
        },
        _ => panic!("This proc-macro only supports structs!"),
    };

    let names = named_fields.iter().map(|field| &field.ident);
    let safe_gets = named_fields.iter().map(|field| -> Stmt {
        let field_ident = &field.ident;
        let field_type = &field.ty;
        let field_attr =
            extract_field_attr(&field.attrs).expect("Unable to parse field attributes");

        // Allows for usage of the `#[postgres(rename = "column_name")]` attribute
        let column_name = if let Some(renamed) = field_attr.rename {
            renamed
        } else {
            field_ident
                .clone()
                .expect("Unable to extract field name for Column")
                .to_string()
        };

        // Executes a safe get from the row by utilizing default values for unwrapping
        //
        // I have no idea why this is not a part of the library itself, if it had this
        // this entire proc_macro would not be required
        //
        // (well they do, but you have to define it for every field making it insanely stupid to use)
        if field_attr.flatten {
            parse_quote!(
                let #field_ident = #field_type::from_row(row)?;
            )
        } else {
            parse_quote!(
                let #field_ident = row.try_get::<#field_type, &str>(#column_name)
                    .unwrap_or_default();
            )
        }
    });

    TokenStream::from(quote! {
        impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for #struct_ident {
            fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
                #(#safe_gets)*

                Ok(#struct_ident {
                    #(#names),*
                })
            }
        }
    })
}

/// Extracts the attributes (#[postgres(...)]) from a field
///
/// # Arguments
///
/// * `attr` - The attributes of a field
///
/// # Returns
///
/// The extracted attributes or an error if the attributes could not be parsed
fn extract_field_attr(attr: &[Attribute]) -> syn::Result<PostgresFieldAttributes> {
    let mut rename: Option<String> = None;
    let mut flatten: bool = false;
    let attrs: Vec<&Attribute> = attr
        .iter()
        .filter(|attr| attr.path().is_ident("postgres"))
        .collect();

    for attr in attrs {
        attr.parse_nested_meta(|meta| {
            let path = meta.path;
            let input = meta.input;
            if path.is_ident("rename") {
                input.parse::<Token![=]>()?;
                let value: LitStr = input.parse()?;
                rename = Some(value.value());
            } else if path.is_ident("flatten") {
                flatten = true;
            }
            Ok(())
        })?
    }

    Ok(PostgresFieldAttributes { rename, flatten })
}
