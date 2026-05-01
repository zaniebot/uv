//! Walk the target's Python frame chain and emit [`Frame`]s.
//!
//! The CPython runtime layout for `_PyRuntime`, `_PyInterpreterFrame`,
//! `PyCodeObject`, and `PyUnicodeObject` evolves between minor releases.
//! Rather than hard-coding byte offsets here, the walk is driven by the
//! offsets advertised in the target's `_Py_DebugOffsets` header (read via
//! [`crate::layout::PyDebugOffsets`]).
//!
//! Only the stable header — cookie, packed version, free-threaded flag —
//! is mirrored in this crate today. Each version-specific sub-struct
//! (interpreter list head, threadstate cradle, frame `f_back` /
//! `f_executable`, code `co_filename` / `co_qualname` / `co_linetable`)
//! must be added before [`sample`] returns non-empty stacks. Until that
//! work lands, [`sample`] returns an empty list of stacks; consumers
//! should treat empty stacks as "attribution not available, attribute the
//! sample to `<unknown>`."

use std::sync::Arc;

use crate::SampleError;
use crate::handle::ProcessHandle;
use crate::layout::PyDebugOffsets;
use crate::string_cache::StringCache;

/// A single frame from the target's Python call stack.
#[derive(Clone, Debug)]
pub struct Frame {
    pub filename: Arc<str>,
    pub qualname: Arc<str>,
    pub line: u32,
}

impl Frame {
    /// Construct a frame representing "could not capture attribution",
    /// useful as a sentinel when the target's runtime layout is not yet
    /// understood by this crate.
    pub fn unknown() -> Self {
        Self {
            filename: Arc::from("<unknown>"),
            qualname: Arc::from("<unknown>"),
            line: 0,
        }
    }
}

/// Maximum number of frames captured per thread.
pub const MAX_STACK_DEPTH: usize = 1024;

#[expect(
    clippy::unnecessary_wraps,
    reason = "real frame walking will surface SampleError variants once implemented"
)]
pub(crate) fn sample(
    _handle: &ProcessHandle,
    _offsets: &PyDebugOffsets,
    _runtime_addr: u64,
    _strings: &mut StringCache,
) -> Result<Vec<Vec<Frame>>, SampleError> {
    // The full walk requires reading version-specific sub-structs from
    // `_Py_DebugOffsets`:
    //   1. RuntimeOffsets::interpreters_head -> linked list of PyInterpreterState
    //   2. InterpreterOffsets::threads_head  -> linked list of PyThreadState
    //   3. ThreadOffsets::current_frame      -> _PyInterpreterFrame*
    //   4. FrameOffsets::previous            -> walk back through the chain
    //   5. FrameOffsets::executable          -> PyCodeObject*
    //   6. CodeOffsets::filename, qualname   -> PyUnicodeObject* -> UTF-8
    //   7. CodeOffsets::linetable + previous_instr -> current line via
    //      PEP 626 line-table decoding
    //
    // These are tracked as a follow-up (see crates/uv-pep768/README later).
    // Returning an empty Vec is the correct behaviour today: the caller
    // ([`uv_profile`]) treats empty stacks as "unattributed" and still
    // emits useful RSS-over-time samples.
    Ok(Vec::new())
}
