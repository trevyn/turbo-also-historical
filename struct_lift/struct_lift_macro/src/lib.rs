use proc_macro2::TokenStream;
use quote::quote;

pub fn struct_lift(attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
 // Implement your proc-macro logic here. :)
 Ok(item)
}
