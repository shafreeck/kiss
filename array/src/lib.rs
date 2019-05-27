extern crate proc_macro;

use syn;
use quote::quote;
use crate::proc_macro::TokenStream;

#[proc_macro]
pub fn init(input :TokenStream) -> TokenStream{
    let repeat :syn::ExprRepeat= syn::parse(input).unwrap();
    let obj :syn::Expr = *repeat.expr;
    let size = match *repeat.len{
        syn::Expr::Lit(l)=>{
            match l.lit{
                syn::Lit::Int(size)=>{size}
                _=>{panic!("integer expected")}
            }
        }
        _=>{panic!("literal exptecd")}
    };

    let iter = std::iter::repeat(obj).take(size.value() as usize);
    let expanded = quote!{
        [#(#iter),*]
    };
    expanded.into()
}
