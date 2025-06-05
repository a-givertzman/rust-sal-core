use syn::punctuated::Punctuated;

///
/// ### Extracts `TokenStream` for function-like proc macro
/// 
/// Used for passing function arguments into `println(fmt, vals)` or `log::info(fmt, vals)`
/// 
/// Where function signature:
/// ```ignore
/// fn debug("{}: {}", a, b)
/// ```
/// 
/// **`"{}: {}"`** - will be extracted into `self.fmt`
/// 
/// **`a`, `b`** - will be extracted into `self.values`
pub(super) struct ErrPassMacroInput {
    pub err: syn::Expr,
    pub fmt: Option<syn::PatLit>,
    pub vals: Option<Punctuated<syn::Expr, syn::Token![,]>>,
}
//
//
impl syn::parse::Parse for ErrPassMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let err: syn::Expr = input.parse()?;
        let _comma: Result<syn::Token![,], syn::Error> = input.parse();
        // let fmt: syn::PatLit = input.parse()?;
        let (fmt, vals) = match input.parse() {
            Ok(fmt) => {
                let fmt: syn::PatLit = fmt;
                let _comma: Result<syn::Token![,], syn::Error> = input.parse();
                let vals = input.parse_terminated(syn::Expr::parse, syn::Token![,])?;
                (Some(fmt), Some(vals))
            }
            Err(_) => (None, None),
        };
        Ok(Self {
            err,
            fmt,
            vals, //: input.parse_terminated(syn::Expr::parse, syn::Token![,])?,
        })
    }
}
