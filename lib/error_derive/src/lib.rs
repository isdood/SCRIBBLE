use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, parse::Parse, parse::ParseStream};
use darling::{FromDeriveInput, FromMeta, FromVariant};

#[derive(Debug, FromMeta)]
struct DiagnoseOpts {
    detect: String,
    suggestion: String,
    quick_fix: String,
}

// Implement Parse for DiagnoseOpts
impl Parse for DiagnoseOpts {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        Ok(DiagnoseOpts {
            detect: content.parse()?,
           suggestion: {
               content.parse::<syn::Token![,]>()?;
               content.parse()?
           },
           quick_fix: {
               content.parse::<syn::Token![,]>()?;
               content.parse()?
           },
        })
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(diagnose))]
struct DiagnoseVariant {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any), attributes(error_path))]
struct ErrorDeriveOpts {
    ident: syn::Ident,
    data: darling::ast::Data<DiagnoseVariant, ()>,
    path: Option<String>,
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
/// #[error_path(path = "quantum/errors")]
/// pub enum QuantumError {
///     #[diagnose(detect = "quantum_state < 0.5", suggestion = "Consider increasing coherence threshold", quick_fix = "set_min_coherence(0.5)")]
///     InvalidState,
/// }
/// ```
#[proc_macro_derive(Diagnose, attributes(diagnose, error_path))]
pub fn derive_diagnose(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match ErrorDeriveOpts::from_derive_input(&input) {
        Ok(val) => val,
        Err(err) => { return TokenStream::from(err.write_errors()); }
    };

    let name = &opts.ident;
    let error_path = opts.path.unwrap_or_default();

    let variants = match opts.data {
        darling::ast::Data::Enum(variants) => variants,
        _ => panic!("Diagnose can only be derived for enums"),
    };

    let match_arms = variants.into_iter().map(|variant| {
        let variant_name = &variant.ident;
        let diagnose_attr = variant.attrs.iter().find(|attr| attr.path().is_ident("diagnose"));

        if let Some(attr) = diagnose_attr {
            if let Ok(meta) = attr.parse_args::<DiagnoseOpts>() {
                let detect = &meta.detect;
                let suggestion = &meta.suggestion;
                let quick_fix = &meta.quick_fix;

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
