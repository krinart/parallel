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
                                    Ok(result) => Ok(result),
                                    Err(_) => Err(format!("Operation timed out after {} seconds", #duration)),
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
                                    Ok(result) => Ok(result),
                                    Err(_) => Err(format!("Operation timed out after {} seconds", #duration)),
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
                                    Ok(result) => Ok(result),
                                    Err(_) => Err({
                                        #fallback_expr
                                    }),
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
                                    Ok(result) => Ok(result),
                                    Err(_) => Err({
                                        #fallback_expr
                                    }),
                                }
                            })
                    }
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// New timeout_fallback macro that directly returns the fallback value
/// This always requires an else clause and does not need to be awaited
pub(crate) fn timeout_fallback(input: TokenStream) -> TokenStream {
    let TimeoutFallbackInput { duration, body, fallback } = parse_macro_input!(input as TimeoutFallbackInput);
    
    // Use custom fallback on timeout - direct return, no Result wrapping
    let expanded = quote! {
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
                            Ok(result) => result,
                            Err(_) => {
                                #fallback
                            }
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
                            Ok(result) => result,
                            Err(_) => {
                                #fallback
                            }
                        }
                    })
            }
        }
    };
    
    TokenStream::from(expanded)
}


pub(crate) fn timeout_value(input: TokenStream) -> TokenStream {
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
                        
                        // Wrap the body expression in a task and apply timeout
                        tokio::task::block_in_place(|| {
                            handle.block_on(async {
                                // Create a task that performs the evaluation
                                let task = tokio::task::spawn(async move {
                                    #body
                                });
                                
                                // Apply timeout to the task
                                match timeout(duration_secs, task).await {
                                    Ok(Ok(value)) => Ok(value),
                                    Ok(Err(_)) => Err(format!("Task panicked")),
                                    Err(_) => Err(format!("Operation timed out after {} seconds", #duration)),
                                }
                            })
                        })
                    } else {
                        // Not in a runtime, create a new one
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(async {
                                // Create a task that performs the evaluation
                                let task = tokio::task::spawn(async move {
                                    #body
                                });
                                
                                // Apply timeout to the task
                                match timeout(duration_secs, task).await {
                                    Ok(Ok(value)) => Ok(value),
                                    Ok(Err(_)) => Err(format!("Task panicked")),
                                    Err(_) => Err(format!("Operation timed out after {} seconds", #duration)),
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
                    
                    // Convert the numeric duration to seconds
                    let duration_secs = Duration::from_secs(#duration as u64);
                    
                    // Check if we're inside a runtime or need to create one
                    if let Ok(handle) = tokio::runtime::Handle::try_current() {
                        // We're in a runtime, use the current handle to enter it
                        let _guard = handle.enter();
                        
                        // Wrap the body expression in a task and apply timeout
                        tokio::task::block_in_place(|| {
                            handle.block_on(async {
                                // Create a task that performs the evaluation
                                let task = tokio::task::spawn(async move {
                                    #body
                                });
                                
                                // Apply timeout to the task
                                match timeout(duration_secs, task).await {
                                    Ok(Ok(value)) => Ok(value),
                                    Ok(Err(_)) => Err({
                                        #fallback_expr
                                    }),
                                    Err(_) => Err({
                                        #fallback_expr
                                    }),
                                }
                            })
                        })
                    } else {
                        // Not in a runtime, create a new one
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(async {
                                // Create a task that performs the evaluation
                                let task = tokio::task::spawn(async move {
                                    #body
                                });
                                
                                // Apply timeout to the task
                                match timeout(duration_secs, task).await {
                                    Ok(Ok(value)) => Ok(value),
                                    Ok(Err(_)) => Err({
                                        #fallback_expr
                                    }),
                                    Err(_) => Err({
                                        #fallback_expr
                                    }),
                                }
                            })
                    }
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
