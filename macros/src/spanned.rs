use proc_macro::{Span, TokenStream};

use syn::{
    parse::{self, Parse, ParseStream},
    spanned::Spanned,
    Error, Expr, Lit,
};

fn add_span(mut ls: String) -> String {
    let span = Span::call_site();

    let file = span.source_file().path();
    let lc = span.start();

    ls.push_str(&format!(", loc: {}:{}", file.display(), lc.line));

    ls
}

struct Input {
    first: Expr,
    second: Option<(Token![,], Expr)>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let first = input.parse()?;

        let lookahead = input.lookahead1();
        Ok(if lookahead.peek(Token![,]) {
            let comma = input.parse()?;
            let expr = input.parse()?;

            Input {
                first,
                second: Some((comma, expr)),
            }
        } else {
            Input {
                first,
                second: None,
            }
        })
    }
}

fn into_lit_str(e: Expr) -> Result<String, Error> {
    match e {
        Expr::Lit(e) => match e.lit {
            Lit::Str(ls) => Ok(ls.value()),
            l => Err(Error::new(l.span(), "expected a string literal")),
        },
        e => Err(Error::new(e.span(), "expected a string literal")),
    }
}

pub fn common(input: TokenStream, level: &str) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    let (logger, message) = if let Some((_, e)) = input.second {
        (Some(input.first), e)
    } else {
        (None, input.first)
    };

    let symbol = match into_lit_str(message) {
        Ok(s) => add_span(s),
        Err(e) => return e.to_compile_error().into(),
    };

    let section = format!(".stlog.{}", level);
    if let Some(logger) = logger {
        quote!(unsafe {
            #[export_name = #symbol]
            #[link_section = #section]
            static SYMBOL: u8 = 0;

            stlog::Log::log(&mut #logger, &SYMBOL as *const u8 as usize as u8)
        })
        .into()
    } else {
        quote!(unsafe {
            extern "Rust" {
                #[link_name = "stlog::GLOBAL_LOGGER"]
                static LOGGER: &'static stlog::GlobalLog;
            }

            #[export_name = #symbol]
            #[link_section = #section]
            static SYMBOL: u8 = 0;

            stlog::GlobalLog::log(LOGGER, &SYMBOL as *const u8 as usize as u8)
        })
        .into()
    }
}
