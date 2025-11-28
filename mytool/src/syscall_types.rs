use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use async_trait::async_trait;
use uuid::Uuid;
use sha2::Digest;

/// Represents different categories of "Syscalls" (impure operations) that the system can perform.
///
/// This enum helps categorize external interactions, which is crucial for managing,
/// logging, and eventually verifying these operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyscallCategory {
    /// Interaction with the GitHub API.
    GitHubApi,
    /// Operations involving the RocksDB database.
    RocksDB,
    /// Direct filesystem interactions.
    Filesystem,
    /// Git repository operations.
    Git,
    /// Interactions with the Nix package manager or Nix store.
    Nix,
    // Add other categories as needed
}

/// Metadata recorded for each execution of a "Syscall".
///
/// This struct captures essential information about a syscall's execution,
/// including its unique identifier, category, timing, success status,
/// and hashes of its inputs and outputs for reproducibility.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyscallMetadata {
    /// A unique identifier for this syscall execution.
    pub id: Uuid,
    /// The category of the syscall (e.g., GitHub API, RocksDB).
    pub category: SyscallCategory,
    /// The timestamp when the syscall was executed (UTC).
    pub timestamp: DateTime<Utc>,
    /// Optional information about the caller (e.g., originating HTTP handler, user ID).
    pub caller_info: Option<String>,
    /// The duration of the syscall execution in milliseconds.
    pub duration_ms: u64,
    /// Indicates whether the syscall executed successfully.
    pub successful: bool,
    /// If an error occurred, a description of the error.
    pub error_message: Option<String>,
    /// SHA256 hash of the serializable inputs used for reproducibility.
    pub inputs_hash: String,
    /// SHA256 hash of the serializable outputs for reproducibility.
    pub outputs_hash: String,
    /// Optional ID for external persistence mechanisms (e.g., IPFS, Git, Blockchain).
    pub persistence_id: Option<String>,
}

/// Standardized result type for the execution of a "Syscall".
///
/// Encapsulates either a successful outcome with a JSON value or an error message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyscallResult {
    /// Indicates successful execution, carrying the JSON value of the output.
    Success(serde_json::Value),
    /// Indicates a failed execution, carrying an error message.
    Error(String),
}

/// A trait representing an impure operation that interacts with data outside the pure database.
/// This is our definition of a "syscall" within the MCP Tycoon Game backend.
///
/// Implementations of this trait encapsulate specific external interactions, making them
/// manageable, testable, and preparing them for verifiability and Zero-Knowledge Proof (ZKP)
/// integration.
#[async_trait]
pub trait Syscall: Send + Sync + 'static {
    /// Returns a unique, static name for this specific type of syscall.
    ///
    /// # Examples
    /// - "GitHubApi_GetUser"
    /// - "RocksDB_PutCode"
    fn name(&self) -> &'static str;

    /// Executes the impure operation defined by this syscall.
    ///
    /// This is the core logic that performs the external interaction.
    ///
    /// # Returns
    /// A `SyscallResult` indicating success with its output or failure with an error message.
    async fn execute(&self) -> Result<SyscallResult>;

    /// Returns a serializable JSON representation of the inputs to this syscall.
    ///
    /// This is crucial for reproducibility, logging, and generating input hashes.
    ///
    /// # Returns
    /// A `serde_json::Value` representing the syscall's inputs.
    fn inputs(&self) -> Result<serde_json::Value>;

    /// Returns a serializable JSON representation of the outputs from this syscall.
    ///
    /// This should be called after `execute` and is used for reproducibility,
    /// logging, and generating output hashes.
    ///
    /// # Arguments
    /// * `result` - The `SyscallResult` obtained from the `execute` method.
    ///
    /// # Returns
    /// A `serde_json::Value` representing the syscall's outputs.
    fn outputs(&self, result: &SyscallResult) -> Result<serde_json::Value>;

    /// Generates `SyscallMetadata` for the execution of this syscall.
    ///
    /// This method gathers all relevant metadata after execution, including
    /// success status, duration, and hashes.
    ///
    /// # Arguments
    /// * `result` - The `SyscallResult` obtained from the `execute` method.
    /// * `duration_ms` - The duration of the syscall execution in milliseconds.
    ///
    /// # Returns
    /// A `SyscallMetadata` struct populated with execution details.
    fn metadata(&self, result: &SyscallResult, duration_ms: u64) -> SyscallMetadata;

    /// Generates a reproducible SHA256 hash of the syscall's inputs.
    ///
    /// The hash is derived from the serializable JSON representation of the inputs.
    ///
    /// # Returns
    /// A hexadecimal string representing the SHA256 hash of the inputs.
    fn inputs_hash(&self) -> Result<String> {
        let inputs_json = self.inputs()?;
        let hash = sha2::Sha256::digest(serde_json::to_string(&inputs_json)?.as_bytes());
        Ok(format!("{:x}", hash))
    }

    /// Generates a reproducible SHA256 hash of the syscall's outputs.
    ///
    /// The hash is derived from the serializable JSON representation of the outputs.
    ///
    /// # Arguments
    /// * `result` - The `SyscallResult` obtained from the `execute` method.
    ///
    /// # Returns
    /// A hexadecimal string representing the SHA256 hash of the outputs.
    fn outputs_hash(&self, result: &SyscallResult) -> Result<String> {
        let outputs_json = self.outputs(result)?;
        let hash = sha2::Sha256::digest(serde_json::to_string(&outputs_json)?.as_bytes());
        Ok(format!("{:x}", hash))
    }
}
