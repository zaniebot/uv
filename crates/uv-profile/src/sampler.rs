//! Sampler task. Polls RSS, attempts PEP 768 stack capture, and feeds a
//! [`crate::pprof::ProfileBuilder`].

use tokio::sync::oneshot;
use tracing::{debug, warn};
use uv_pep768::Target;

use crate::ProfilerOptions;
use crate::pprof::ProfileBuilder;
use crate::rss;

/// Result of a sampling session.
pub(crate) struct Capture {
    pub(crate) builder: ProfileBuilder,
    pub(crate) samples_taken: u64,
    pub(crate) samples_with_stack: u64,
}

pub(crate) async fn run(
    pid: u32,
    options: ProfilerOptions,
    mut shutdown: oneshot::Receiver<()>,
) -> anyhow::Result<Capture> {
    let mut target = match Target::attach(pid) {
        Ok(target) => {
            debug!(pid, version = %target.version(), "uv-pep768 attached");
            Some(target)
        }
        Err(err) => {
            warn!(
                pid,
                "uv-pep768 attach failed: {err}; falling back to RSS-only sampling"
            );
            None
        }
    };

    let mut builder = ProfileBuilder::new();
    let mut last_rss = rss::read(pid).unwrap_or(0);
    let mut samples_taken: u64 = 0;
    let mut samples_with_stack: u64 = 0;
    let mut interval = tokio::time::interval(options.sample_interval);
    // First tick fires immediately; skip it so we have a real baseline.
    interval.tick().await;

    loop {
        tokio::select! {
            _ = &mut shutdown => break,
            _ = interval.tick() => {
                let Some(rss_now) = rss::read(pid) else {
                    debug!(pid, "RSS read failed; assuming target exited");
                    break;
                };
                samples_taken = samples_taken.saturating_add(1);
                let delta = rss_now.saturating_sub(last_rss);
                last_rss = rss_now;
                if delta == 0 {
                    continue;
                }

                let stacks = target
                    .as_mut()
                    .and_then(|t| match t.sample_stacks() {
                        Ok(stacks) => Some(stacks),
                        Err(err) => {
                            debug!(pid, "stack sample failed: {err}");
                            None
                        }
                    })
                    .unwrap_or_default();

                let primary = stacks.into_iter().next().unwrap_or_default();
                if !primary.is_empty() {
                    samples_with_stack = samples_with_stack.saturating_add(1);
                }
                let depth = options.max_stack_depth as usize;
                let trimmed: &[uv_pep768::Frame] = if primary.len() > depth {
                    &primary[..depth]
                } else {
                    &primary[..]
                };
                builder.add_sample(trimmed, delta);
            }
        }
    }

    Ok(Capture {
        builder,
        samples_taken,
        samples_with_stack,
    })
}
