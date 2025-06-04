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
        if let Ok(attr) = input.parse() {
            let attr: syn::LitStr = attr;
            if let Some(attr) = attr.span().source_text() {
                let attr = attr.replace('\\', "");
                let attr = attr.replace('"', "");
                let mut parts = attr.split('.');
                if let Some(_first) = parts.next() {
                    if let Some(second) = parts.next() {
                        return Ok(Self::Field(second.to_string()));
                    }
                    // return Ok(Self::Var(first.to_string()));
                }
            }
        }
        Err(syn::Error::new(input.span(), "Attibute must be a name of the defined `self.dbg` field for example"))
    }
}
