extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, ArgCaptured, Block, FnArg, ItemFn, Pat, PatIdent};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn hello_trace(attr: TokenStream, input: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        panic!("no parameters needed")
    }
    let the_fn: ItemFn = syn::parse(input).unwrap();
    let fn_block = the_fn.block.clone();
    let mut gen = the_fn.clone();
    let fn_ident = the_fn.ident.clone();
    let mut input_idents = vec![];
    for input in the_fn.decl.inputs.clone() {
        match input {
            FnArg::Captured(ArgCaptured {
                pat: Pat::Ident(PatIdent { ident, .. }),
                ..
            }) => input_idents.push(ident),
            _ => unimplemented!("input matching"),
        }
    }
    let input_idents2 = input_idents.clone();
    let new_block = quote! {
        {
            let __trace_name = stringify!(#fn_ident);
            print!("[debug {}]", __trace_name);
            #(
                print!(" {}: {:?}", stringify!(#input_idents), #input_idents2);
            )*
            println!();
            let __trace_start = std::time::Instant::now();
            let __trace_returns = #fn_block;
            let __trace_end = std::time::Instant::now();
            println!(
                "[debug {}] return: {:?} (took {:?})",
                __trace_name,
                __trace_returns,
                __trace_end - __trace_start,
            );
            __trace_returns
        }
    };
    let new_block: Block = syn::parse2(new_block).unwrap();
    gen.block = Box::new(new_block);
    gen.into_token_stream().into()
}
