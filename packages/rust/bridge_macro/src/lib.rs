use proc_macro::TokenStream;

/// This attribute does not generate or modify anything. It is only used as a market attribute for bridge_gen to generate TypeScript definitions.
/// You may use it on struct definitions or functions definitions.
#[proc_macro_attribute]
pub fn bridge(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}