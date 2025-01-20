use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use darling::{FromDeriveInput, FromMeta, FromVariant};

#[derive(Debug, FromMeta)]
struct DiagnoseOpts {
    #[darling(default)]
    detect: String,
    #[darling(default)]
    suggestion: String,
    #[darling(default)]
    quick_fix: String,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(diagnose))]
struct DiagnoseVariant {
    ident: syn::Ident,
    #[darling(default)]
    attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(error_path), supports(enum_any))]
struct ErrorDeriveOpts {
    ident: syn::Ident,
    data: darling::ast::Data<DiagnoseVariant, ()>,
    #[darling(default)]
    path: String,
}

/// Derives the Diagnose trait for error types
#[proc_macro_derive(Diagnose, attributes(diagnose, error_path))]
pub fn derive_diagnose(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match ErrorDeriveOpts::from_derive_input(&input) {
        Ok(val) => val,
        Err(err) => { return TokenStream::from(err.write_errors()); }
    };

    let name = &opts.ident;
    let error_path = &opts.path;

    let variants = match opts.data {
        darling::ast::Data::Enum(variants) => variants,
        _ => panic!("Diagnose can only be derived for enums"),
    };

    let match_arms = variants.into_iter().map(|variant| {
        let variant_name = &variant.ident;
        let diagnose_attr = variant.attrs.iter().find(|attr| attr.path().is_ident("diagnose"));

        if let Some(attr) = diagnose_attr {
            let meta = attr.parse_args::<DiagnoseOpts>().unwrap_or_else(|_| DiagnoseOpts {
                detect: String::new(),
                                                                        suggestion: String::new(),
                                                                        quick_fix: String::new(),
            });

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
