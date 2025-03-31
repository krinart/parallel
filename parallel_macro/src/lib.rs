// src/lib.rs
use proc_macro::TokenStream;

mod parallel;
mod timeout;

#[proc_macro]
pub fn parallel(input: TokenStream) -> TokenStream {
    parallel::parallel(input)
}

#[proc_macro]
pub fn timeout(input: TokenStream) -> TokenStream {
    timeout::timeout(input)
}

// #[proc_macro]
// pub fn timeout_dynamic(input: TokenStream) -> TokenStream {
//     timeout::timeout_dynamic(input)
// }