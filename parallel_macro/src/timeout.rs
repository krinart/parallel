extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Token, parse::{Parse, ParseStream}, Result};

struct TimeoutInput {
    duration: Expr,
    body: Expr,
}

impl Parse for TimeoutInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let duration = input.parse()?;
        input.parse::<Token![,]>()?;
        let body = input.parse()?;
        
        Ok(TimeoutInput {
            duration,
            body,
        })
    }
}

pub(crate) fn timeout(input: TokenStream) -> TokenStream {
    let TimeoutInput { duration, body } = parse_macro_input!(input as TimeoutInput);
    
    let expanded = quote! {
        {
            use tokio::time::timeout;
            use std::time::Duration;
            
            // Convert the numeric duration to seconds
            let duration_secs = Duration::from_secs(#duration as u64);
            
            // Wrap the body in an async block and apply timeout
            let timeout_future = timeout(duration_secs, async { #body });
            
            // Await and unwrap the result
            match timeout_future.await {
                Ok(result) => result,
                Err(_) => panic!("Operation timed out after {} seconds", #duration),
            }
        }
    };
    
    TokenStream::from(expanded)
}