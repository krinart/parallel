extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Token, parse::{Parse, ParseStream}, Result, bracketed, ExprBlock};

enum TimeoutFallback {
    None,
    Else(Expr),
}

struct TimeoutInput {
    duration: Expr,
    body: Expr,
    fallback: TimeoutFallback,
}

impl Parse for TimeoutInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse duration
        let duration = input.parse()?;
        input.parse::<Token![,]>()?;
        
        // Parse body
        let body = input.parse()?;
        
        // Parse optional else clause
        let fallback = if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            TimeoutFallback::Else(input.parse()?)
        } else {
            TimeoutFallback::None
        };
        
        Ok(TimeoutInput {
            duration,
            body,
            fallback,
        })
    }
}

pub(crate) fn timeout(input: TokenStream) -> TokenStream {
    let TimeoutInput { duration, body, fallback } = parse_macro_input!(input as TimeoutInput);
    
    let expanded = match fallback {
        TimeoutFallback::None => {
            // Return Result for basic timeout usage
            quote! {
                {
                    use tokio::time::timeout;
                    use std::time::Duration;
                    
                    // Convert the numeric duration to seconds
                    let duration_secs = Duration::from_secs(#duration as u64);
                    
                    // Wrap the body in an async block and apply timeout
                    let timeout_future = timeout(duration_secs, async { #body });
                    
                    // Return a Result
                    match timeout_future.await {
                        Ok(result) => Ok(result),
                        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, 
                            format!("Operation timed out after {} seconds", #duration))),
                    }
                }
            }
        },
        TimeoutFallback::Else(fallback_expr) => {
            // Use custom fallback on timeout, but wrap in Result
            quote! {
                {
                    use tokio::time::timeout;
                    use std::time::Duration;
                    
                    // Convert the numeric duration to seconds
                    let duration_secs = Duration::from_secs(#duration as u64);
                    
                    // Wrap the body in an async block and apply timeout
                    let timeout_future = timeout(duration_secs, async { #body });
                    
                    // Await and return wrapped in Result
                    match timeout_future.await {
                        Ok(result) => Ok(result),
                        Err(_) => Err({
                            #fallback_expr
                        })
                    }
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}