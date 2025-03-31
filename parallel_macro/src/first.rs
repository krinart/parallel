extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parse::Parse, parse::ParseStream, parse_macro_input, token, Expr, Result, Token,
};

struct FirstInput {
    futures_block: FuturesBlock,
    error_handler: ErrorHandler,
}

struct FuturesBlock {
    brace_token: token::Brace,
    expressions: Vec<Expr>,
}

struct ErrorHandler {
    else_token: Token![else],
    error_expr: Expr,
}

impl Parse for FuturesBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let brace_token = braced!(content in input);
        
        let mut expressions = Vec::new();
        while !content.is_empty() {
            let expr = content.parse::<Expr>()?;
            expressions.push(expr);
            
            if content.is_empty() {
                break;
            }
            
            content.parse::<Token![,]>()?;
        }
        
        if expressions.is_empty() {
            return Err(syn::Error::new(brace_token.span.join(), "futures block cannot be empty"));
        }
        
        Ok(FuturesBlock {
            brace_token,
            expressions,
        })
    }
}

impl Parse for ErrorHandler {
    fn parse(input: ParseStream) -> Result<Self> {
        let else_token = input.parse::<Token![else]>()?;
        let error_expr = input.parse::<Expr>()?;
        
        Ok(ErrorHandler {
            else_token,
            error_expr,
        })
    }
}

impl Parse for FirstInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let futures_block = input.parse::<FuturesBlock>()?;
        let error_handler = input.parse::<ErrorHandler>()?;
        
        Ok(FirstInput {
            futures_block,
            error_handler,
        })
    }
}

pub(crate) fn first(input: TokenStream) -> TokenStream {
    let FirstInput { futures_block, error_handler } = parse_macro_input!(input as FirstInput);
    
    // let futures = futures_block.expressions.iter();
    let error_expr = &error_handler.error_expr;
    
    // Create a tokio::select! based implementation instead
    let future_vars = futures_block.expressions.iter().enumerate().map(|(i, _)| {
        let var_name = format!("future_{}", i);
        syn::Ident::new(&var_name, proc_macro2::Span::call_site())
    }).collect::<Vec<_>>();
    
    let future_assignments = futures_block.expressions.iter().zip(future_vars.iter()).map(|(expr, var)| {
        quote! { let mut #var = #expr; }
    });
    
    let select_branches = future_vars.iter().map(|var| {
        quote! { 
            val = #var => {
                return Ok(val);
            }
        }
    });
    
    let expanded = quote! {
        {
            async move {
                #(#future_assignments)*
                
                loop {
                    tokio::select! {
                        #(#select_branches)*
                        else => {
                            return Err(#error_expr);
                        }
                    }
                }
            }.await
        }
    };
    
    TokenStream::from(expanded)
}