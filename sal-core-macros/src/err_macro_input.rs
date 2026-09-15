///
/// ### Extracts attributes `TokenStream` for function-like proc macro
/// 
/// Used for passing macro attributes like `dbg` or `self.dbg`
/// 
/// Where function signature:
/// ```ignore
/// #[dbg(dbg)]
/// fn some_function1(...)

/// #[dbg(self.dbg)]
/// fn some_function2(&self...)
/// ```
pub(super) enum ErrMacroInput {
    Field(String),
    #[allow(unused)]
    Var(String),
}
//
//
impl syn::parse::Parse for ErrMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr = syn::ExprLit::parse(input)?;
        if let Some(attr) = attr.lit.span().source_text() {
            let attr = attr.replace('\\', "");
            let attr = attr.replace('"', "");
            let mut parts = attr.split('.');
            if let Some(first) = parts.next() {
                if let Some(second) = parts.next() {
                    return Ok(Self::Field(second.to_string()));
                }
                return Ok(Self::Var(first.to_string()));
            }
        }
        Err(syn::Error::new(input.span(), "Supported: `err(\"self.dbg\")` of `err(\"dbg\")`"))
    }
}
