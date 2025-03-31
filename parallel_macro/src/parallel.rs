extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::Parse, parse::ParseStream, Expr, Result, Token};

struct ParallelInput {
    expressions: Vec<Expr>,
}

impl Parse for ParallelInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expressions = Vec::new();
        
        while !input.is_empty() {
            let expr = input.parse::<Expr>()?;
            expressions.push(expr);
            
            if input.is_empty() {
                break;
            }
            
            input.parse::<Token![,]>()?;
        }
        
        Ok(ParallelInput { expressions })
    }
}

pub fn parallel(input: TokenStream) -> TokenStream {
    let ParallelInput { expressions } = parse_macro_input!(input as ParallelInput);
    
    // Generate a tuple with the correct types
    let expr_tokens = expressions.iter();
    
    let expanded = quote! {
        {
            use futures::future::Future;
            use futures::future::join_all;
            
            // Create a tuple of awaited futures
            tokio::join!(
                #(#expr_tokens),*
            )
        }
    };
    
    TokenStream::from(expanded)
}