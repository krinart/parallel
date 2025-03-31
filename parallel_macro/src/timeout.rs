use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Block, Expr, ExprLit, Lit, LitInt, parse::Parse, parse::ParseStream, Token};

pub(crate) fn timeout(input: TokenStream) -> TokenStream {
    // Parse the input
    let input = parse_macro_input!(input as TimeoutInput);
    
    // Extract components
    let duration = input.duration;
    let block = input.block;
    
    // Generate unique identifier for the task
    let task_ident = format_ident!("__timeout_task");
    
    // Generate the output code
    let output = quote! {
        {
            use std::time::Duration;
            use tokio::time::timeout;
            
            async fn #task_ident() {
                #block
            }
            
            tokio::runtime::Handle::current().block_on(async {
                match timeout(Duration::from_secs(#duration), #task_ident()).await {
                    Ok(result) => result,
                    Err(_) => {
                        eprintln!("Operation timed out after {} seconds", #duration);
                        // Return a default value or throw an error
                    }
                }
            })
        }
    };
    
    output.into()
}

// Struct to hold the parsed components
struct TimeoutInput {
    duration: u64,
    block: Block,
}

// Parser implementation for the timeout macro
impl Parse for TimeoutInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the timeout duration
        let duration_expr: Expr = input.parse()?;
        
        // Extract numeric value
        let duration = match &duration_expr {
            Expr::Lit(ExprLit { lit: Lit::Int(lit_int), .. }) => {
                lit_int.base10_parse::<u64>()?
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    duration_expr,
                    "Expected a literal integer for timeout duration"
                ));
            }
        };
        
        // Parse the block
        let block: Block = input.parse()?;
        
        Ok(TimeoutInput {
            duration,
            block,
        })
    }
}