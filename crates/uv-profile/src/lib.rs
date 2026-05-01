//! Memory profiler for uv.
//!
//! Spawned by [`uv run`](../uv/index.html) when the user passes
//! `--profile-path`. Polls the child process's RSS at a configurable
//! interval, attempts to attach via [`uv_pep768`] for Python call-stack
//! attribution on CPython 3.14+, and writes a gzipped pprof profile to
//! disk when sampling completes.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use tokio::sync::oneshot;
use tracing::{debug, info};

mod pprof;
mod proto;
mod rss;
mod sampler;

pub use pprof::ProfileBuilder;

/// Options for a [`Profiler`] session.
#[derive(Debug, Clone)]
pub struct ProfilerOptions {
    /// Time between samples. The default is 10ms (100Hz), matching
    /// typical sampling profilers.
    pub sample_interval: Duration,
    /// Maximum captured frames per stack. Bounded to avoid runaway in
    /// pathological cases.
    pub max_stack_depth: u32,
}

impl Default for ProfilerOptions {
    fn default() -> Self {
        Self {
            sample_interval: Duration::from_millis(10),
            max_stack_depth: 256,
        }
    }
}

/// A handle to a running profiler session.
pub struct Profiler {
    handle: tokio::task::JoinHandle<anyhow::Result<sampler::Capture>>,
    shutdown: Option<oneshot::Sender<()>>,
    output_path: PathBuf,
}

impl Profiler {
    /// Spawn a sampler task targeting `pid`. Returns immediately. The task
    /// runs until [`Self::finish`] is called or the child exits.
    pub fn start(pid: u32, output_path: PathBuf, options: ProfilerOptions) -> Self {
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(sampler::run(pid, options, rx));
        Self {
            handle,
            shutdown: Some(tx),
            output_path,
        }
    }

    /// Stop sampling and write the accumulated samples as a gzipped pprof
    /// profile to the configured output path. Idempotent.
    pub async fn finish(mut self) -> anyhow::Result<()> {
        if let Some(tx) = self.shutdown.take() {
            // If the receiver has already dropped (e.g. the target exited
            // and the sampler returned early), the send fails silently —
            // that's fine.
            let _ = tx.send(());
        }

        let capture = self
            .handle
            .await
            .context("memory profiler task panicked")??;

        debug!(
            samples = capture.samples_taken,
            attributed = capture.samples_with_stack,
            "writing memory profile"
        );

        if capture.samples_taken == 0 {
            info!(
                "memory profiler captured no samples (target exited before \
                 first tick); not writing profile file"
            );
            return Ok(());
        }

        if capture.samples_with_stack == 0 {
            info!(
                "memory profile lacks per-frame attribution; this is \
                 expected on CPython < 3.14 or when uv-pep768 attach is \
                 denied"
            );
        }

        capture
            .builder
            .write_gzipped(&self.output_path)
            .context("failed to write pprof profile")?;
        Ok(())
    }
}

/// Errors that can occur while constructing or finishing a profiler.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ProfilerError {
    #[error("profiler I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("profiler join: {0}")]
    Join(#[from] tokio::task::JoinError),
}
