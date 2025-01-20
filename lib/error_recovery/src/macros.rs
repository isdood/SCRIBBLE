//! Macros for automatic error recovery
//! Created: 2025-01-20 23:50:32
//! Author: isdood

/// Macro for automatically applying recovery strategies to a function
#[macro_export]
macro_rules! auto_recover {
    (
        $strategy:expr,
     $max_attempts:expr,
     $body:expr
    ) => {{
        use $crate::{RecoveryManager, RecoveryConfig};
        use std::time::Duration;

        let config = RecoveryConfig {
            max_attempts: $max_attempts,
            retry_delay: Duration::from_millis(100),
            use_backoff: true,
        };

        let mut manager = RecoveryManager::new(config);
        manager.register_strategy($strategy);

        let context = error_integration::context::ErrorContext::new();
        manager.attempt_recovery($body(), &context).await
    }};
}

/// Macro for defining a recovery strategy
#[macro_export]
macro_rules! define_recovery_strategy {
    (
        $name:ident,
     $error_type:ty,
     $can_handle:expr,
     $recovery_logic:expr
    ) => {
        pub struct $name {
            config: $crate::RecoveryConfig,
        }

        impl $name {
            pub fn new(config: $crate::RecoveryConfig) -> Self {
                Self { config }
            }
        }

        #[async_trait::async_trait]
        impl $crate::RecoveryStrategy for $name {
            type Error = $error_type;

            async fn attempt_recovery(
                &self,
                error: &Self::Error,
                context: &error_integration::context::ErrorContext,
            ) -> Result<(), $crate::RecoveryError> {
                ($recovery_logic)(error, context).await
            }

            fn can_handle(&self, error: &Self::Error) -> bool {
                ($can_handle)(error)
            }

            fn config(&self) -> &$crate::RecoveryConfig {
                &self.config
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RecoveryConfig, RecoveryError};
    use error_integration::context::ErrorContext;
    use std::time::Duration;

    #[tokio::test]
    async fn test_auto_recover_macro() {
        define_recovery_strategy!(
            TestStrategy,
            std::io::Error,
            |error| error.kind() == std::io::ErrorKind::WouldBlock,
                                  |_error, _context| async { Ok(()) }
        );

        let strategy = TestStrategy::new(RecoveryConfig::default());
        let result = auto_recover!(
            strategy,
            3,
            || async {
                Err(std::io::Error::new(
                    std::io::ErrorKind::WouldBlock,
                    "test error"
                ))
            }
        );

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_define_recovery_strategy_macro() {
        define_recovery_strategy!(
            CustomStrategy,
            String,
            |error| error.contains("recoverable"),
                                  |_error, _context| async { Ok(()) }
        );

        let strategy = CustomStrategy::new(RecoveryConfig {
            max_attempts: 3,
            retry_delay: Duration::from_millis(100),
                                           use_backoff: true,
        });

        assert!(strategy.can_handle(&"recoverable error".to_string()));
        assert!(!strategy.can_handle(&"fatal error".to_string()));
    }
}
