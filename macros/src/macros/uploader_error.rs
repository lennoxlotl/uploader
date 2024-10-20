use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, LitInt, Token};

type TokenStream2 = proc_macro2::TokenStream;

// The attributes which are extracted from the `RocketError` macro
#[derive(Debug, Clone)]
struct UploaderErrorAttributes {
    status_code: Option<u16>,
}

pub fn uploader_error_impl(input: DeriveInput) -> TokenStream {
    let mut output = TokenStream2::new();
    let mut match_stream = TokenStream2::new();
    let enum_name = &input.ident;

    // Parse all contents of the Enum
    if let Data::Enum(e) = input.data {
        for variant in e.variants {
            let ident = variant.ident;
            let attributes =
                extract_field_attr(&variant.attrs).expect("Unable to parse Enum variant");

            let status_code = attributes.status_code.unwrap();

            match_stream.extend(quote! {
                Self::#ident => ErrorAttributes {
                    status_code: #status_code
                },
            });
        }
    }

    output.extend(quote! {
        impl #enum_name {
            pub fn error_attr(&self) -> ErrorAttributes {
                match self {
                    #match_stream
                }
            }
        }
    });
    output.into()
}

/// Extracts the attributes (#[uploader(...)]) from a field
///
/// # Arguments
///
/// * `attr` - The attributes of a field
///
/// # Returns
///
/// The extracted attributes or an error if the attributes could not be parsed
fn extract_field_attr(attr: &[Attribute]) -> syn::Result<UploaderErrorAttributes> {
    let mut status_code: Option<u16> = None;
    let attrs: Vec<&Attribute> = attr
        .iter()
        .filter(|attr| attr.path().is_ident("uploader"))
        .collect();

    for attr in attrs {
        attr.parse_nested_meta(|meta| {
            let path = meta.path;
            let input = meta.input;
            if path.is_ident("status_code") {
                input.parse::<Token![=]>()?;
                let value: LitInt = input.parse()?;
                let int_value = value.base10_parse::<u16>()?;
                status_code = Some(int_value);
            }
            Ok(())
        })?
    }

    Ok(UploaderErrorAttributes { status_code })
}
