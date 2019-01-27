//! Macros part of the stlog logging framework

#![cfg_attr(feature = "spanned", feature(proc_macro_span))]
#![deny(warnings)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::{Span, TokenStream};

use syn::{spanned::Spanned, Error, ItemStatic};

#[cfg(feature = "spanned")]
mod spanned;

/// An attribute to declare a global logger
///
/// This attribute can only be applied to `static` variables that implement the
/// [`GlobalLog`](../stlog/trait.GlobalLog.html) trait.
#[proc_macro_attribute]
pub fn global_logger(args: TokenStream, input: TokenStream) -> TokenStream {
    let var = parse_macro_input!(input as ItemStatic);

    if !args.is_empty() {
        return Error::new(
            Span::call_site().into(),
            "`global_logger` attribute takes no arguments",
        )
        .to_compile_error()
        .into();
    }

    if var.mutability.is_some() {
        return Error::new(
            var.span(),
            "`#[global_logger]` can't be used on `static mut` variables",
        )
        .to_compile_error()
        .into();
    }

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
    )
    .into()
}

#[cfg(feature = "spanned")]
#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    spanned::common(input, "error")
}

#[cfg(feature = "spanned")]
#[proc_macro]
pub fn warning(input: TokenStream) -> TokenStream {
    spanned::common(input, "warning")
}

#[cfg(feature = "spanned")]
#[proc_macro]
pub fn info(input: TokenStream) -> TokenStream {
    spanned::common(input, "info")
}

#[cfg(feature = "spanned")]
#[proc_macro]
pub fn debug(input: TokenStream) -> TokenStream {
    spanned::common(input, "debug")
}

#[cfg(feature = "spanned")]
#[proc_macro]
pub fn trace(input: TokenStream) -> TokenStream {
    spanned::common(input, "trace")
}
