use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use darling::FromDeriveInput;
use darling::FromMeta;

#[derive(Debug, FromMeta)]
struct DiagnoseOpts {
    detect: String,
    suggestion: String,
    quick_fix: String,
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any), attributes(error_path))]
struct ErrorDeriveOpts {
    ident: syn::Ident,
    data: darling::ast::Data<(), DiagnoseVariant>,
    #[darling(default)]
    error_path: Option<String>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(diagnose))]
struct DiagnoseVariant {
    ident: syn::Ident,
    #[darling(default)]
    diagnose: Option<DiagnoseOpts>,
}

/// Derives the Diagnose trait for error types
///
/// # Example
///
/// ```rust
/// use error_derive::Diagnose;
/// use error_core::Diagnose as _;
///
/// #[derive(Debug, Diagnose)]
/// #[error_path = "quantum/errors"]
/// pub enum QuantumError {
///     #[diagnose(
///         detect = "quantum_state < 0.5",
///         suggestion = "Consider increasing coherence threshold",
///         quick_fix = "set_min_coherence(0.5)"
///     )]
///     InvalidState,
/// }
///
/// let error = QuantumError::InvalidState;
/// let report = error.diagnose();
/// assert!(!report.quick_fixes.is_empty());
/// ```
#[proc_macro_derive(Diagnose, attributes(diagnose, error_path))]
pub fn derive_diagnose(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match ErrorDeriveOpts::from_derive_input(&input) {
        Ok(val) => val,
        Err(err) => { return TokenStream::from(err.write_errors()); }
    };

    let name = &opts.ident;
    let error_path = opts.error_path.unwrap_or_else(|| String::from(""));

    let variants = match opts.data {
        darling::ast::Data::Enum(variants) => variants,
        _ => panic!("Diagnose can only be derived for enums"),
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        if let Some(diagnose) = &variant.diagnose {
            let detect = &diagnose.detect;
            let suggestion = &diagnose.suggestion;
            let quick_fix = &diagnose.quick_fix;

            quote! {
                Self::#variant_name => {
                    let mut report = error_core::DiagnosticReport::new();
                    report.message = format!("Detected condition: {}", #detect);
                    report.suggestions.push(#suggestion.to_string());
                    report.quick_fixes.push(error_core::QuickFix {
                        description: #suggestion.to_string(),
                                            code: #quick_fix.to_string(),
                    });
                    report
                }
            }
        } else {
            quote! {
                Self::#variant_name => error_core::DiagnosticReport::new()
            }
        }
    });

    let expanded = quote! {
        impl error_core::Diagnose for #name {
            fn diagnose(&self) -> error_core::DiagnosticReport {
                match self {
                    #(#match_arms),*
                }
            }

            fn get_quick_fixes(&self) -> Vec<error_core::QuickFix> {
                self.diagnose().quick_fixes
            }

            fn check_at_compile_time() -> Option<error_core::CompileTimeError> {
                if #error_path.is_empty() {
                    Some(error_core::CompileTimeError {
                        message: "No error_path attribute specified".to_string(),
                         location: format!("enum {}", stringify!(#name)),
                    })
                } else {
                    None
                }
            }
        }
    };

    TokenStream::from(expanded)
}
