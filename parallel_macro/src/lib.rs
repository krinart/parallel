// src/lib.rs
use proc_macro::TokenStream;

mod parallel;
mod timeout;
mod first;
mod timeout_with_result;

#[proc_macro]
pub fn parallel(input: TokenStream) -> TokenStream {
    parallel::parallel(input)
}

#[proc_macro]
pub fn timeout(input: TokenStream) -> TokenStream {
    timeout::timeout(input)
}

#[proc_macro]
pub fn timeout_fallback(input: TokenStream) -> TokenStream {
    timeout::timeout_fallback(input)
}

#[proc_macro]
pub fn timeout_value(input: TokenStream) -> TokenStream {
    timeout::timeout_value(input)
}

#[proc_macro]
pub fn first(input: TokenStream) -> TokenStream {
    first::first(input)
}

#[proc_macro]
pub fn timeout_with_result(input: TokenStream) -> TokenStream {
    timeout_with_result::timeout_with_result(input)
}


use quote::{quote};
use syn::{parse_macro_input, Expr, Token, parse::{Parse, ParseStream}, Result as SynResult};

enum TimeoutFallback {
    None,
    Else(Expr),
}

struct TimeoutInput {
    // duration: Expr,
    // body: Expr,
    // fallback: TimeoutFallback,
}


impl Parse for TimeoutInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // Parse duration

        let forked = input.fork();
        let tokens = forked.to_string();
        println!("Current stream content: {}", tokens);

        let duration: Expr = input.parse()?;
        // input.parse::<Token![!]>()?;
        
        // // Parse body
        let body: Expr = input.parse()?;

        // // Parse duration
        // let duration = input.parse()?;
        
        // // Parse body
        // let content;
        // braced!(content in input);
        // let body = content.parse()?;

        
        // Parse optional else clause
        let fallback:TimeoutFallback = if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            TimeoutFallback::Else(input.parse()?)
        } else {
            TimeoutFallback::None
        };
        
        Ok(TimeoutInput {
            // duration,
            // body,
            // fallback,
        })
    }
}

#[proc_macro]
pub fn my_test_timeout(input: TokenStream) -> TokenStream {
    // let TimeoutInput { duration, body, fallback } = parse_macro_input!(input as TimeoutInput);
    let TimeoutInput {  } = parse_macro_input!(input as TimeoutInput);

    println!("123");

    let expanded = quote! {
        println!("expanded")
    };

    TokenStream::from(expanded)
}