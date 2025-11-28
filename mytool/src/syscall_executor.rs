use anyhow::Result;
use async_trait::async_trait;
use log::info;
use chrono::Utc;

use crate::syscall_types::{Syscall, SyscallResult};

/// A trait that defines how syscalls are executed and managed (e.g., cached, logged, persisted).
#[async_trait]
pub trait SyscallExecutor: Send + Sync + 'static {
    /// Executes a given syscall, wrapping it with logging, caching, and potentially persistence.
    async fn execute_syscall(&self, syscall: Box<dyn Syscall>) -> Result<SyscallResult>;
}

pub struct DefaultSyscallExecutor {
    // In a more complete implementation, this would hold references to caches, persistence layers, etc.
}

impl DefaultSyscallExecutor {
    pub fn new() -> Self {
        DefaultSyscallExecutor {}
    }
}

#[async_trait]
impl SyscallExecutor for DefaultSyscallExecutor {
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
        let metadata = syscall.metadata(&syscall_result, duration_ms);

        info!(
            "Syscall {} finished. Success: {}, Duration: {}ms, Outputs Hash: {}",
            name, metadata.successful, duration_ms, outputs_hash
        );

        // TODO: In future iterations, this is where caching, persistence, ZKP generation would be integrated.
        // For now, it just logs.

        Ok(syscall_result)
    }
}
