use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, NestedMeta};
use darling::{FromDeriveInput, FromMeta};

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
    data: darling::ast::Data<(), ()>,
    #[darling(default)]
    error_path: Option<String>,
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
    let error_path = opts.error_path.unwrap_or_default();

    // Get enum variants
    let variants = if let syn::Data::Enum(data) = &input.data {
        &data.variants
    } else {
        panic!("Diagnose can only be derived for enums");
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let mut detect = String::new();
        let mut suggestion = String::new();
        let mut quick_fix = String::new();

        // Parse diagnose attributes
        for attr in &variant.attrs {
            if attr.path().is_ident("diagnose") {
                attr.parse_nested_meta(|meta| {
                    let path = meta.path.get_ident().unwrap().to_string();
                    let value = meta.value()?;
                    let str_val = value.parse::<syn::LitStr>()?.value();

                    match path.as_str() {
                        "detect" => detect = str_val,
                        "suggestion" => suggestion = str_val,
                        "quick_fix" => quick_fix = str_val,
                        _ => {}
                    }
                    Ok(())
                }).unwrap_or_default();
            }
        }

        if !detect.is_empty() {
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
