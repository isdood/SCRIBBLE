use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Lit};
use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any))]
struct ErrorDeriveOpts {
    ident: syn::Ident,
}

/// Derives the Diagnose trait for error types
///
/// # Example
///
/// ```rust
/// use error_derive::Diagnose;
///
/// #[derive(Debug, Diagnose)]
/// #[error_path = "test/errors"]
/// enum TestError {
///     #[diagnose(detect = "value < 0", suggestion = "Value must be positive", quick_fix = "set_positive_value()")]
///     NegativeValue,
///     SimpleError,
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

    // Parse error_path attribute
    let error_path = input.attrs.iter()
    .find(|attr| attr.path().is_ident("error_path"))
    .and_then(|attr| {
        let meta = attr.meta.require_name_value().ok()?;
        if let Expr::Lit(expr_lit) = &meta.value {
            if let Lit::Str(lit_str) = &expr_lit.lit {
                Some(lit_str.value())
            } else {
                None
            }
        } else {
            None
        }
    })
    .unwrap_or_default();

    // Get enum variants
    let variants = if let syn::Data::Enum(data) = &input.data {
        &data.variants
    } else {
        panic!("Diagnose can only be derived for enums");
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        // Parse diagnose attributes
        if let Some(attr) = variant.attrs.iter().find(|attr| attr.path().is_ident("diagnose")) {
            let mut detect = String::new();
            let mut suggestion = String::new();
            let mut quick_fix = String::new();

            attr.parse_nested_meta(|meta| {
                let path = meta.path.get_ident().unwrap().to_string();
                let value = meta.value()?.parse::<Lit>()?;

                if let Lit::Str(s) = value {
                    let value = s.value();
                    match path.as_str() {
                        "detect" => detect = value,
                        "suggestion" => suggestion = value,
                        "quick_fix" => quick_fix = value,
                        _ => {}
                    }
                }

                Ok(())
            }).ok();

            if !detect.is_empty() {
                return quote! {
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
                };
            }
        }

        quote! {
            Self::#variant_name => error_core::DiagnosticReport::new()
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
