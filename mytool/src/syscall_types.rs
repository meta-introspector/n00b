use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use async_trait::async_trait;
use uuid::Uuid;
use sha2::Digest;

// Enum to categorize different types of syscalls
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyscallCategory {
    GitHubApi,
    RocksDB,
    Filesystem,
    Git,
    Nix,
    // Add other categories as needed
}

// Struct to hold metadata about each syscall execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyscallMetadata {
    pub id: Uuid,
    pub category: SyscallCategory,
    pub timestamp: DateTime<Utc>,
    pub caller_info: Option<String>, // e.g., originating HTTP handler, user ID
    pub duration_ms: u64,
    pub successful: bool,
    pub error_message: Option<String>,
    pub inputs_hash: String, // Hash of serializable inputs for reproducibility
    pub outputs_hash: String, // Hash of serializable outputs for reproducibility
    pub persistence_id: Option<String>, // ID for IPFS, Git, Blockchain
}

// Standardized result type for syscalls
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyscallResult {
    Success(serde_json::Value), // Json value of the successful output
    Error(String),
}

/// A trait representing an impure operation that interacts with data outside the pure database.
/// This is our definition of a "syscall".
#[async_trait]
pub trait Syscall: Send + Sync + 'static {
    /// A unique name for this type of syscall (e.g., "GitHubApi_GetUser").
    fn name(&self) -> &'static str;

    /// Executes the impure operation.
    async fn execute(&self) -> Result<SyscallResult>;

    /// Returns serializable inputs to the syscall.
    /// Used for reproducibility and hashing.
    fn inputs(&self) -> Result<serde_json::Value>;

    /// Returns serializable outputs of the syscall.
    /// Used for reproducibility and hashing. Should be called after execute.
    fn outputs(&self, result: &SyscallResult) -> Result<serde_json::Value>;

    /// Returns metadata about the syscall.
    fn metadata(&self, result: &SyscallResult, duration_ms: u64) -> SyscallMetadata;

    /// Generates a reproducible hash of the syscall's inputs.
    fn inputs_hash(&self) -> Result<String> {
        let inputs_json = self.inputs()?;
        let hash = sha2::Sha256::digest(serde_json::to_string(&inputs_json)?.as_bytes());
        Ok(format!("{:x}", hash))
    }

    /// Generates a reproducible hash of the syscall's outputs.
    fn outputs_hash(&self, result: &SyscallResult) -> Result<String> {
        let outputs_json = self.outputs(result)?;
        let hash = sha2::Sha256::digest(serde_json::to_string(&outputs_json)?.as_bytes());
        Ok(format!("{:x}", hash))
    }
}
