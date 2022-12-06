use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    system_patch(attr, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn system_patch(_arg: TokenStream, _item: TokenStream) -> Result<TokenStream2> {
    todo!()
}
