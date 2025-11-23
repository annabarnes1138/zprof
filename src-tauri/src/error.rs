//! Error handling for IPC commands
//!
//! This module provides error types that are serializable to JSON
//! for transmission across the Tauri IPC boundary.

use serde::{Deserialize, Serialize};

/// Error type for IPC commands
///
/// All IPC commands return Result<T, IpcError> which can be serialized
/// to JSON for the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcError {
    /// Error code for programmatic handling
    pub code: ErrorCode,
    /// Human-readable error message
    pub message: String,
    /// Optional suggestion for how to resolve the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

/// Error codes for different error types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Profile not found
    ProfileNotFound,
    /// Profile already exists
    ProfileExists,
    /// Invalid profile name or configuration
    InvalidInput,
    /// Filesystem operation failed
    FilesystemError,
    /// Failed to parse manifest file
    ManifestError,
    /// Cannot perform operation on active profile
    ProfileActive,
    /// Framework-related error
    FrameworkError,
    /// Unknown/unhandled error
    Unknown,
}

impl IpcError {
    /// Create a new IPC error
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            suggestion: None,
        }
    }

    /// Add a suggestion to the error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Convert to a string suitable for Result<T, String>
    pub fn to_string_result(self) -> String {
        if let Some(suggestion) = self.suggestion {
            format!("{}\n\nSuggestion: {}", self.message, suggestion)
        } else {
            self.message
        }
    }
}

impl std::fmt::Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for IpcError {}

/// Convert anyhow::Error to IpcError
impl From<anyhow::Error> for IpcError {
    fn from(err: anyhow::Error) -> Self {
        let message = err.to_string();
        let message_lower = message.to_lowercase();

        // Try to classify the error based on message content (case-insensitive)
        let code = if message_lower.contains("not found") || message_lower.contains("does not exist") {
            ErrorCode::ProfileNotFound
        } else if message_lower.contains("already exists") {
            ErrorCode::ProfileExists
        } else if message_lower.contains("active profile") || message_lower.contains("cannot delete active") {
            ErrorCode::ProfileActive
        } else if message_lower.contains("invalid") || message_lower.contains("validation") {
            ErrorCode::InvalidInput
        } else if message_lower.contains("manifest") || message_lower.contains("toml") {
            ErrorCode::ManifestError
        } else if message_lower.contains("framework") {
            ErrorCode::FrameworkError
        } else if message_lower.contains("failed to") || message_lower.contains("permission") {
            ErrorCode::FilesystemError
        } else {
            ErrorCode::Unknown
        };

        Self {
            code,
            message,
            suggestion: None,
        }
    }
}

/// Convert IpcError to String for Tauri commands
impl From<IpcError> for String {
    fn from(err: IpcError) -> String {
        err.to_string_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_error_creation() {
        let err = IpcError::new(ErrorCode::ProfileNotFound, "Profile 'test' not found");
        assert_eq!(err.code, ErrorCode::ProfileNotFound);
        assert_eq!(err.message, "Profile 'test' not found");
        assert!(err.suggestion.is_none());
    }

    #[test]
    fn test_ipc_error_with_suggestion() {
        let err = IpcError::new(ErrorCode::ProfileNotFound, "Profile 'test' not found")
            .with_suggestion("Run 'zprof list' to see available profiles");

        assert_eq!(err.code, ErrorCode::ProfileNotFound);
        assert_eq!(err.suggestion, Some("Run 'zprof list' to see available profiles".to_string()));
    }

    #[test]
    fn test_to_string_result_without_suggestion() {
        let err = IpcError::new(ErrorCode::InvalidInput, "Invalid profile name");
        assert_eq!(err.to_string_result(), "Invalid profile name");
    }

    #[test]
    fn test_to_string_result_with_suggestion() {
        let err = IpcError::new(ErrorCode::InvalidInput, "Invalid profile name")
            .with_suggestion("Use only lowercase letters and hyphens");

        let result = err.to_string_result();
        assert!(result.contains("Invalid profile name"));
        assert!(result.contains("Suggestion: Use only lowercase letters and hyphens"));
    }

    #[test]
    fn test_from_anyhow_error() {
        let anyhow_err = anyhow::anyhow!("Profile 'work' not found");
        let ipc_err: IpcError = anyhow_err.into();

        assert_eq!(ipc_err.code, ErrorCode::ProfileNotFound);
        assert!(ipc_err.message.contains("not found"));
    }

    #[test]
    fn test_error_code_classification() {
        let test_cases = vec![
            ("Profile not found", ErrorCode::ProfileNotFound),
            ("Profile already exists", ErrorCode::ProfileExists),
            ("Cannot delete active profile", ErrorCode::ProfileActive),
            ("Invalid profile name", ErrorCode::InvalidInput),
            ("Failed to parse manifest", ErrorCode::ManifestError),
            ("Framework installation failed", ErrorCode::FrameworkError),
            ("Failed to write file", ErrorCode::FilesystemError),
            ("Something went wrong", ErrorCode::Unknown),
        ];

        for (message, expected_code) in test_cases {
            let anyhow_err = anyhow::anyhow!("{}", message);
            let ipc_err: IpcError = anyhow_err.into();
            assert_eq!(ipc_err.code, expected_code, "Failed for message: {}", message);
        }
    }
}
