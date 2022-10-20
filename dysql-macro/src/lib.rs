use proc_macro::TokenStream;

#[derive(Debug)]
struct SqlClosure {
    dto: Option<proc_macro2::Ident>,
    dialect: proc_macro2::Literal,
    body: proc_macro2::TokenStream,
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as SqlClosure);
    
    match expand(st) {
        Ok(ret) => ret.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

impl syn::parse::Parse for SqlClosure {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

fn expand(st: SqlClosure) -> syn::Result<TokenStream> {
    todo!()
}