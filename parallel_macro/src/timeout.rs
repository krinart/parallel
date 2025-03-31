extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, braced, Expr, Token, parse::Parse, parse::ParseStream, Result, Ident, Block, Stmt};

struct TimeoutElseInput {
    duration: Expr,
    main_block: Vec<Stmt>,
    else_block: Option<Vec<Stmt>>,
}

impl Parse for TimeoutElseInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the duration
        let duration = input.parse()?;
        input.parse::<Token![,]>()?;
        
        // Parse the main block within braces
        let main_content;
        let _ = braced!(main_content in input);
        
        // Parse all statements in the block
        let mut main_stmts = Vec::new();
        while !main_content.is_empty() {
            main_stmts.push(main_content.parse()?);
        }
        
        // Check if there's an else block
        let mut else_stmts = None;
        if !input.is_empty() {
            // Parse the 'else' keyword
            let else_kw: Ident = input.parse()?;
            if else_kw != "else" {
                return Err(syn::Error::new_spanned(else_kw, "Expected 'else' keyword"));
            }
            
            // Parse the else block within braces
            let else_content;
            let _ = braced!(else_content in input);
            
            // Parse all statements in the else block
            let mut stmts = Vec::new();
            while !else_content.is_empty() {
                stmts.push(else_content.parse()?);
            }
            else_stmts = Some(stmts);
        }
        
        Ok(TimeoutElseInput {
            duration,
            main_block: main_stmts,
            else_block: else_stmts,
        })
    }
}

pub(crate) fn timeout(input: TokenStream) -> TokenStream {
    let TimeoutElseInput { duration, main_block, else_block } = parse_macro_input!(input as TimeoutElseInput);
    
    let main_stmts = &main_block;
    
    let result = if let Some(else_stmts) = else_block {
        // With else block - use match for handling
        quote! {
            {
                use tokio::time::timeout;
                use std::time::Duration;
                
                let duration_secs = Duration::from_secs(#duration as u64);
                
                match timeout(duration_secs, async {
                    #(#main_stmts)*
                }).await {
                    Ok(result) => result,
                    Err(_) => {
                        #(#else_stmts)*
                    }
                }
            }
        }
    } else {
        // No else block - automatically unwrap the result (will panic on timeout)
        quote! {
            {
                use tokio::time::timeout;
                use std::time::Duration;
                
                let duration_secs = Duration::from_secs(#duration as u64);
                
                match timeout(duration_secs, async {
                    #(#main_stmts)*
                }).await {
                    Ok(result) => result,
                    Err(_) => panic!("Operation timed out after {} seconds", #duration),
                }
            }
        }
    };
    
    TokenStream::from(result)
}

// Non-panicking version that returns a Result
pub(crate) fn timeout_result(input: TokenStream) -> TokenStream {
    let TimeoutElseInput { duration, main_block, else_block } = parse_macro_input!(input as TimeoutElseInput);
    
    let main_stmts = &main_block;
    
    let result = if let Some(else_stmts) = else_block {
        quote! {
            {
                use tokio::time::timeout;
                use std::time::Duration;
                
                let duration_secs = Duration::from_secs(#duration as u64);
                
                match timeout(duration_secs, async {
                    #(#main_stmts)*
                }).await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        #(#else_stmts)*
                    }
                }
            }
        }
    } else {
        quote! {
            {
                use tokio::time::timeout;
                use std::time::Duration;
                
                let duration_secs = Duration::from_secs(#duration as u64);
                
                timeout(duration_secs, async {
                    #(#main_stmts)*
                }).await
            }
        }
    };
    
    TokenStream::from(result)
}