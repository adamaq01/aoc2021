use proc_macro::TokenStream;
use proc_macro2::Punct;
use syn::{parse::Parse, parse_macro_input, FnArg, Ident, LitInt, PathArguments, ReturnType, Type};

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

struct PuzzleParams {
    day: LitInt,
    _token: Punct,
    stage: Ident,
    _token2: Punct,
    parse: Ident,
}

impl Parse for PuzzleParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(PuzzleParams {
            day: input.parse()?,
            _token: input.parse()?,
            stage: input.parse()?,
            _token2: input.parse()?,
            parse: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn puzzle(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as syn::ItemFn);
    let name = &function.sig.ident;

    let input = &function.sig.inputs.first().expect("Couldn't get input");
    let input = if let FnArg::Typed(input) = input {
        input.ty.clone()
    } else {
        panic!("Bad input");
    };
    let output = &function.sig.output;
    let output = if let ReturnType::Type(_, output) = output {
        output.clone()
    } else {
        panic!("Bad output");
    };
    let output = if let Type::Path(output) = *output {
        output
            .path
            .segments
            .first()
            .expect("Bad output")
            .clone()
            .arguments
    } else {
        panic!("Bad output");
    };
    let output = if let PathArguments::AngleBracketed(output) = output {
        output.args.first().expect("Bad output").clone()
    } else {
        panic!("Bad output");
    };

    let params = parse_macro_input!(metadata as PuzzleParams);
    let day = params.day;
    let stage = params.stage.to_string();
    let parse_inputs = params.parse;

    let s = quote! {
        pub fn #name() -> FnPuzzle<#input, #output> {

            #function

            FnPuzzle::new(#day,
                Stage::new(#stage).expect("Couldn't parse stage"),
                Box::new(#parse_inputs),
                Box::new(#name),
            )
        }
    };

    s.into()
}
