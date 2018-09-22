extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use syn::ItemStatic;

#[proc_macro_attribute]
pub fn global_logger(args: TokenStream, input: TokenStream) -> TokenStream {
    let var = parse_macro_input!(input as ItemStatic);

    assert_eq!(
        args.to_string(),
        "",
        "`global_logger` attribute takes no arguments"
    );

    assert!(
        var.mutability.is_none(),
        "`#[global_logger]` can't be used on `static mut` variables"
    );

    let attrs = var.attrs;
    let vis = var.vis;
    let ident = var.ident;
    let ty = var.ty;
    let expr = var.expr;

    // TODO use this when rust-lang/rust#54451 lands
    // quote!(
    //     #(#attrs)*
    //     #vis static #ident: #ty = {
    //         #[export_name = "stlog::GLOBAL_LOGGER"]
    //         static __STLOGGER__: &stlog::GlobalLog = &#ident;

    //         #expr
    //     };
    // ).into()

    quote!(
        #(#attrs)*
        #vis static #ident: #ty = #expr;

        #[export_name = "stlog::GLOBAL_LOGGER"]
        pub static __STLOG_GLOBAL_LOGGER__: &(stlog::GlobalLog) = {
            &#ident
        };
    ).into()
}
