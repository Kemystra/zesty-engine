use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let name = ast.ident;
    let generated_impl = quote! {
        impl Component for #name {}
    };

    generated_impl.into()
}

#[proc_macro_derive(ComponentType)]
pub fn component_type_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let name = ast.ident;
    let generated_impl = quote! {
        impl ComponentType for #name {
            const TYPE: String = stringify!(#name);
        }
    };

    generated_impl.into()
}
