extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod macros;

// Exposes the `PostgresRow` proc macro
#[proc_macro_derive(PostgresRow, attributes(postgres))]
pub fn from_row_default_derive(input: TokenStream) -> TokenStream {
    return crate::macros::postgres_row_default::postgres_row_default_impl(parse_macro_input!(
        input as DeriveInput
    ));
}

// Exposes the `UploaderError` proc macro
#[proc_macro_derive(UploaderError, attributes(uploader))]
pub fn rocket_error(input: TokenStream) -> TokenStream {
    return crate::macros::uploader_error::uploader_error_impl(parse_macro_input!(
        input as DeriveInput
    ));
}
