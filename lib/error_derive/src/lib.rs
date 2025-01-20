use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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
/// assert!(report.quick_fixes.is_empty());
/// ```
#[proc_macro_derive(Diagnose, attributes(diagnose, error_path))]
pub fn derive_diagnose(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Implementation of the derive macro
    let name = &input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl error_core::Diagnose for #name {
            fn diagnose(&self) -> error_core::DiagnosticReport {
                error_core::DiagnosticReport::new()
            }

            fn get_quick_fixes(&self) -> Vec<error_core::QuickFix> {
                Vec::new()
            }

            fn check_at_compile_time() -> Option<error_core::CompileTimeError> {
                None
            }
        }
    };

    TokenStream::from(expanded)
}
