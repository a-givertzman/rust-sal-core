mod err_macro_input;
mod err_pass_macro_input;
mod log_macro_input;
use log_macro_input::LogMacroInput;
use err_macro_input::ErrMacroInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

use crate::err_pass_macro_input::ErrPassMacroInput;


///
/// Define `#[dbg()]` attribute above the class method,
/// to make possible using a log macros as
/// 
/// ```ignore
/// struct MyStruct {
///     dbg: String,    // any type implements Display, the name of this field must be `dbg`
/// }
/// impl MyStruct {
///     #[dbg()]
///     pub fn show(&self, val: usize) {
///         sal_core::log::debug!("val: {}", val);
///     }
/// }
/// fn main() {
///     let my_struct = MyStruct { dbg: "MyStruct".into() };
///     my_struct.show(12);       // MyStruct.show | val: 12
/// }
/// ```
#[proc_macro_attribute]
pub fn dbg(_: TokenStream, input: TokenStream) -> TokenStream {
    // log_duration_impl(args, item)
    let item = input.clone();
    let item = syn::parse_macro_input!(item as ItemFn);

    let ItemFn {
        // The function signature
        sig,
        // The visibility specifier of this function
        vis,
        // The function block or body
        block,
        // Other attributes applied to this function
        attrs,
    } = item;

    // Extract statements in the body of the functions
    let statements = block.stmts;
    // Store the function identifier for logging
    let function_identifier = sig.ident.span().source_text();
    quote!(
        // Reapply all the other attributes on this function.
        // The compiler doesn't include the macro we are
        // currently working in this list.
        #(#attrs)*
        // Reconstruct the function declaration
        #vis #sig {
            // Defining variable containing the function identifier
            let __fn_label = #function_identifier;

            // println!("function: {}", stringify!(#function_identifier));
            
            #(#statements)*
        }
    ).into()
}
///
/// Define `#[err()]` attribute above the class method,
/// to make possible using a log macros as
/// 
/// ```ignore
/// struct MyStruct {
///     dbg: String,    // any type implements Display, the name of this field must be `dbg`
/// }
/// impl MyStruct {
///     #[err()]
///     pub fn show(&self, val: usize) -> Result<(), Error> {
///         Err(sal_core::dbg::err!("Error in {} seconds", val))
///     }
/// }
/// fn main() {
///     let my_struct = MyStruct { dbg: "MyStruct".into() };
///     let err = my_struct.show(12);       // Err("MyStruct.show | val: 12")
/// }
/// ```
#[proc_macro_attribute]
pub fn err(args: TokenStream, input: TokenStream) -> TokenStream {
    // log_duration_impl(args, item)
    let args = syn::parse_macro_input!(args as ErrMacroInput);

    let item = input.clone();
    let item = syn::parse_macro_input!(item as ItemFn);

    let ItemFn {
        // The function signature
        sig,
        // The visibility specifier of this function
        vis,
        // The function block or body
        block,
        // Other attributes applied to this function
        attrs,
    } = item;

    // Extract statements in the body of the functions
    let statements = block.stmts;
    // Store the function identifier for logging
    let function_identifier = sig.ident.span().source_text();
    // println!("dbg: {}", dbg);
    match args {
        ErrMacroInput::Field(field) => {
            let field: proc_macro2::TokenStream = field.parse().unwrap();
            // println!("self.dbg: {}", quote! {self.#field} );
            // println!("Field: {:?}", field);
            // let dbg_field = Some(field);
            quote!(
                // Reapply all the other attributes on this function.
                // The compiler doesn't include the macro we are
                // currently working in this list.
                #(#attrs)*
                // Reconstruct the function declaration
                #vis #sig {
                    // Defining variable containing the function identifier
                    let __err = Error::new(&self.#field, #function_identifier);
        
                    // println!("function: {}", stringify!(#function_identifier));
                    
                    #(#statements)*
                }
            ).into()
        }
        ErrMacroInput::Var(dbg_var) => {
            // println!("Var: {:?}", dbg_var);
            let dbg_var: proc_macro2::TokenStream = dbg_var.parse().unwrap();
            println!("dbg: {}", quote! {#dbg_var} );
            quote!(
                // Reapply all the other attributes on this function.
                // The compiler doesn't include the macro we are
                // currently working in this list.
                #(#attrs)*
                // Reconstruct the function declaration
                #vis #sig {
                    // Defining variable containing the function identifier
                    let __err = Error::new(#dbg_var.to_string(), #function_identifier);
        
                    // println!("function: {}", stringify!(#function_identifier));
                    
                    #(#statements)*
                }
            ).into()
        }
    }

}
///
/// Error::err | Returns just happens error.
#[proc_macro]
pub fn err_new(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        __err.err(
            format!(
                #f,
                #(#vals)*
            )
        )
    ).into()
}
///
/// Error::pass | Returns error created from nested entities
/// - optionally you can pass with additional message
#[proc_macro]
pub fn err_pass(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as ErrPassMacroInput);
    let value_err = value.err;
    // let value_err = quote! { format!("{:?}", #value_err); };
    // let f = value.fmt;
    match value.fmt.clone() {
        Some(fmt) => {
            let vals = value.vals.unwrap().into_pairs();
            quote!(
                __err.pass_with(
                    format!(
                        #fmt,
                        #(#vals)*
                    ),
                    #value_err,
                )
            ).into()
        }
        None => {
            quote!(
                __err.pass(#value_err)
            ).into()
        }
    }
}
// ///
// /// Error::pass_with | Returns error created from nested entities with additional message
// #[proc_macro]
// pub fn errpasswith(tokens: TokenStream) -> TokenStream {
//     let value = syn::parse_macro_input!(tokens as LogMacroInput);
//     let f = value.fmt;
//     let vals = value.vals.into_pairs();
//     quote!(
//         log::info!(
//             "{}.{} | {:?}",
//             self.dbg,
//             __fn_label,
//             format!(
//                 #f,
//                 #(#vals)*
//             )
//         );
//     ).into()
// }

///
/// Logs a message at the info level.
#[proc_macro]
pub fn info(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::info!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
///
/// Logs a message at the debug level.
#[proc_macro]
pub fn debug(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::debug!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
///
/// Logs a message at the trace level.
#[proc_macro]
pub fn trace(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::trace!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
///
/// Logs a message at the warn level.
#[proc_macro]
pub fn warn(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::warn!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
///
/// Logs a message at the error level.
#[proc_macro]
pub fn error(tokens: TokenStream) -> TokenStream {
    let value = syn::parse_macro_input!(tokens as LogMacroInput);
    let f = value.fmt;
    let vals = value.vals.into_pairs();
    quote!(
        log::error!(
            "{}.{} | {:?}",
            self.dbg,
            __fn_label,
            format!(
                #f,
                #(#vals)*
            )
        );
    ).into()
}
