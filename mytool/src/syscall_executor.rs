use anyhow::Result;
use async_trait::async_trait;
use log::info;
use chrono::Utc;

use crate::syscall_types::{Syscall, SyscallResult, SyscallMetadata};

/// A trait that defines how "Syscalls" (impure operations) are executed and managed.
///
/// Implementors of this trait are responsible for orchestrating the execution of a `Syscall`
/// instance, potentially adding cross-cutting concerns like logging, caching, monitoring,
/// and future persistence or verifiability features.
#[async_trait]
pub trait SyscallExecutor: Send + Sync + 'static {
    /// Executes a given `Syscall` instance.
    ///
    /// This method is the central point for running any impure operation defined
    /// by the `Syscall` trait. It wraps the core `Syscall::execute` logic with
    /// additional management functionalities.
    ///
    /// # Arguments
    /// * `syscall` - A `Box`ed instance of a type implementing the `Syscall` trait.
    ///
    /// # Returns
    /// A `Result` containing the `SyscallResult` of the operation, or an `anyhow::Error`
    /// if the execution management itself fails.
    async fn execute_syscall(&self, syscall: Box<dyn Syscall>) -> Result<SyscallResult>;
}

/// A default implementation of the `SyscallExecutor` trait.
///
/// This executor currently focuses on logging syscall execution details, including
/// start/end times, success status, and input/output hashes. In future iterations
/// for the MCP Tycoon Game, this would be extended to integrate caching, persistence
/// layers (like IPFS), and potentially Zero-Knowledge Proof (ZKP) generation.
pub struct DefaultSyscallExecutor {
    // In a more complete implementation, this would hold references to caches, persistence layers, etc.
}

impl DefaultSyscallExecutor {
    /// Creates a new `DefaultSyscallExecutor` instance.
    ///
    /// Currently, it does not require any parameters, but future extensions might
    /// include configurations for caching mechanisms, logging verbosity, etc.
    pub fn new() -> Self {
        DefaultSyscallExecutor {}
    }
}

#[async_trait]
impl SyscallExecutor for DefaultSyscallExecutor {
    /// Executes a given syscall, providing logging and basic metadata generation.
    ///
    /// The execution flow includes:
    /// 1. Recording the start time.
    /// 2. Logging the syscall name and its input hash.
    /// 3. Executing the core syscall logic (`syscall.execute().await`).
    /// 4. Recording the end time and calculating duration.
    /// 5. Handling potential errors during syscall execution.
    /// 6. Generating and logging `SyscallMetadata`, including output hash.
    /// 7. Returning the `SyscallResult`.
    ///
    /// # Arguments
    /// * `syscall` - A `Box`ed instance of a type implementing the `Syscall` trait.
    ///
    /// # Returns
    /// A `Result` containing the `SyscallResult` of the operation, or an `anyhow::Error`
    /// if there's an issue with internal management (e.g., hashing inputs/outputs).
    async fn execute_syscall(&self, syscall: Box<dyn Syscall>) -> Result<SyscallResult> {
        let start_time = Utc::now();
        let name = syscall.name();
        let inputs_hash = syscall.inputs_hash()?;

        info!("Executing syscall: {} with inputs hash: {}", name, inputs_hash);

        let result = syscall.execute().await;
        let end_time = Utc::now();
        let duration_ms = (end_time - start_time).num_milliseconds() as u64;

        let syscall_result = match result {
            Ok(s_result) => s_result,
            Err(e) => SyscallResult::Error(format!("Syscall execution failed: {}", e)),
        };

        let outputs_hash = syscall.outputs_hash(&syscall_result)?;
        // SyscallMetadata is used for comprehensive logging and future verifiability.
        // It's generated here but not explicitly returned by this method, as its primary
        // purpose is for internal record-keeping and observability within the executor.
        let metadata = syscall.metadata(&syscall_result, duration_ms);

        info!(
            "Syscall {} finished. Success: {}, Duration: {}ms, Outputs Hash: {}",
            name, metadata.successful, duration_ms, outputs_hash
        );

        // TODO: In future iterations for the MCP Tycoon Game, this is where caching,
        // persistence (e.g., to IPFS or a blockchain for verifiable game state),
        // and Zero-Knowledge Proof (ZKP) generation would be integrated.
        // For now, it primarily logs the operation.

        Ok(syscall_result)
    }
}
