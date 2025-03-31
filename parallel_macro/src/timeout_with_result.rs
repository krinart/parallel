extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Expr, Token, parse::{Parse, ParseStream}, Result};

enum TimeoutFallback {
    None,
    Else(Expr),
}




// Input struct for the standard timeout macro (optional fallback)
struct TimeoutInput {
    duration: Expr,
    body: Expr,
    fallback: TimeoutFallback,
}

impl Parse for TimeoutInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse duration
        let duration = input.parse()?;
        
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

// Input struct for timeout_fallback macro (required fallback)
struct TimeoutFallbackInput {
    duration: Expr,
    body: Expr,
    fallback: Expr,
}

impl Parse for TimeoutFallbackInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse duration
        let duration = input.parse()?;
        
        // Parse body
        let body = input.parse()?;
        
        // Parse required else clause
        input.parse::<Token![else]>()?;
        let fallback = input.parse()?;
        
        Ok(TimeoutFallbackInput {
            duration,
            body,
            fallback,
        })
    }
}

/// Original timeout macro that returns a Result
pub(crate) fn timeout_with_result(input: TokenStream) -> TokenStream {
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
                    
                    // Check if we're inside a runtime or need to create one
                    if let Ok(handle) = tokio::runtime::Handle::try_current() {
                        // We're in a runtime, use the current handle to enter it
                        let _guard = handle.enter();
                        
                        // Apply timeout to the future and execute it "immediately" using spawn_blocking
                        tokio::task::block_in_place(|| {
                            handle.block_on(async {
                                let body_future = #body;
                                let timeout_future = timeout(duration_secs, body_future);
                                
                                match timeout_future.await {
                                    Ok(result) => match result {
                                        Ok(val) => TimeoutResult::Success(val),
                                        Err(e) => TimeoutResult::Error(e),
                                    },
                                    Err(_) => TimeoutResult::TimedOut,
                                }
                            })
                        })
                    } else {
                        // Not in a runtime, create a new one
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(async {
                                let body_future = #body;
                                let timeout_future = timeout(duration_secs, body_future);
                                
                                match timeout_future.await {
                                    Ok(result) => match result {
                                        Ok(val) => TimeoutResult::Success(val),
                                        Err(e) => TimeoutResult::Error(e),
                                    },
                                    Err(_) => TimeoutResult::TimedOut,
                                }
                            })
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
                    use parallel_macro_core::TimeoutResult;
                    
                    // Convert the numeric duration to seconds
                    let duration_secs = Duration::from_secs(#duration as u64);
                    
                    // Check if we're inside a runtime or need to create one
                    if let Ok(handle) = tokio::runtime::Handle::try_current() {
                        // We're in a runtime, use the current handle to enter it
                        let _guard = handle.enter();
                        
                        // Apply timeout to the future and execute it "immediately" using spawn_blocking
                        tokio::task::block_in_place(|| {
                            handle.block_on(async {
                                let body_future = #body;
                                let timeout_future = timeout(duration_secs, body_future);
                                
                                match timeout_future.await {
                                    Ok(result) => match result {
                                        Ok(val) => TimeoutResult::Success(val),
                                        Err(e) => TimeoutResult::Error(e),
                                    },
                                    Err(_) => {
                                        let fallback_result = #fallback_expr;

                                        match fallback_result {
                                            Ok(result) => TimeoutResult::Success(result),
                                            Err(err) => TimeoutResult::Error(err),
                                        }
                                    },
                                }
                            })
                        })
                    } else {
                        // Not in a runtime, create a new one
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(async {
                                let body_future = #body;
                                let timeout_future = timeout(duration_secs, body_future);
                                
                                match timeout_future.await {
                                    Ok(result) => match result {
                                        Ok(val) => TimeoutResult::Success(val),
                                        Err(e) => TimeoutResult::Error(e),
                                    },
                                    Err(_) => {
                                        let fallback_result = #fallback_expr;

                                        match fallback_result {
                                            Ok(result) => TimeoutResult::Success(result),
                                            Err(err) => TimeoutResult::Error(err),
                                        }
                                    },
                                }
                            })
                    }
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
