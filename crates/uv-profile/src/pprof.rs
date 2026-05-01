//! Build a pprof [`Profile`] from sampled stacks and write it to disk
//! gzip-compressed.
//!
//! The pprof file format is the same one produced by Go's `runtime/pprof`
//! and consumed by `go tool pprof`, speedscope, and Pyroscope.

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use prost::Message;
use uv_pep768::Frame;

use crate::proto;

/// Builder that accumulates samples and emits a pprof [`Profile`].
#[derive(Default)]
pub struct ProfileBuilder {
    samples: Vec<proto::Sample>,
    locations: Vec<proto::Location>,
    functions: Vec<proto::Function>,
    /// Dedup key: `(filename_id, qualname_id)` → Function id.
    function_index: HashMap<(i64, i64), u64>,
    /// Dedup key: `(function_id, line)` → Location id.
    location_index: HashMap<(u64, i64), u64>,
    strings: StringTable,
    sample_type: Option<proto::ValueType>,
}

impl ProfileBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        // Sample type advertised in the profile: `alloc_space` measured
        // in bytes. This is what `go tool pprof` and friends interpret as
        // a memory profile.
        let type_id = builder.strings.intern("alloc_space");
        let unit_id = builder.strings.intern("bytes");
        builder.sample_type = Some(proto::ValueType {
            r#type: type_id,
            unit: unit_id,
        });
        builder
    }

    /// Append a sample attributing `bytes` of allocated space to the call
    /// stack `frames`. `frames[0]` is the innermost (top-of-stack) frame.
    pub fn add_sample(&mut self, frames: &[Frame], bytes: u64) {
        let bytes = i64::try_from(bytes).unwrap_or(i64::MAX);
        let mut location_ids = Vec::with_capacity(frames.len());
        for frame in frames {
            location_ids.push(self.intern_location(frame));
        }
        // pprof requires at least one location per sample. If we have no
        // attribution (the common case in v1), inject a sentinel
        // "<unattributed>" frame so the profile remains valid and tools
        // still render the bytes.
        if location_ids.is_empty() {
            location_ids.push(self.intern_unattributed());
        }
        self.samples.push(proto::Sample {
            location_id: location_ids,
            value: vec![bytes],
            label: Vec::new(),
        });
    }

    fn intern_location(&mut self, frame: &Frame) -> u64 {
        let filename_id = self.strings.intern(&frame.filename);
        let qualname_id = self.strings.intern(&frame.qualname);
        let function_id = self.intern_function(filename_id, qualname_id);
        let line = i64::from(frame.line);
        if let Some(id) = self.location_index.get(&(function_id, line)) {
            return *id;
        }
        let id = u64::try_from(self.locations.len() + 1).unwrap_or(u64::MAX);
        self.locations.push(proto::Location {
            id,
            mapping_id: 0,
            address: 0,
            line: vec![proto::Line {
                function_id,
                line,
                column: 0,
            }],
            is_folded: false,
        });
        self.location_index.insert((function_id, line), id);
        id
    }

    fn intern_function(&mut self, filename_id: i64, qualname_id: i64) -> u64 {
        if let Some(id) = self.function_index.get(&(filename_id, qualname_id)) {
            return *id;
        }
        let id = u64::try_from(self.functions.len() + 1).unwrap_or(u64::MAX);
        self.functions.push(proto::Function {
            id,
            name: qualname_id,
            system_name: qualname_id,
            filename: filename_id,
            start_line: 0,
        });
        self.function_index.insert((filename_id, qualname_id), id);
        id
    }

    fn intern_unattributed(&mut self) -> u64 {
        let frame = Frame::unknown();
        self.intern_location(&frame)
    }

    /// Encode the profile as gzipped pprof and write it to `path`.
    pub fn write_gzipped(self, path: &Path) -> std::io::Result<()> {
        let profile = proto::Profile {
            sample_type: self.sample_type.into_iter().collect(),
            sample: self.samples,
            mapping: Vec::new(),
            location: self.locations,
            function: self.functions,
            string_table: self.strings.into_vec(),
            drop_frames: 0,
            keep_frames: 0,
            time_nanos: 0,
            duration_nanos: 0,
            period_type: None,
            period: 0,
            comment: Vec::new(),
            default_sample_type: 0,
            doc_url: String::new(),
        };

        let mut bytes = Vec::with_capacity(profile.encoded_len());
        profile.encode(&mut bytes).map_err(std::io::Error::other)?;

        let mut encoder = flate2::write::GzEncoder::new(
            fs_err::File::create(path)?,
            flate2::Compression::default(),
        );
        encoder.write_all(&bytes)?;
        encoder.finish()?;
        Ok(())
    }
}

/// Interned string table. The empty string at index 0 is required by the
/// pprof spec so that consumers can reliably interpret "unset" string
/// indices.
struct StringTable {
    strings: Vec<String>,
    index: HashMap<String, i64>,
}

impl StringTable {
    fn intern(&mut self, value: &str) -> i64 {
        if let Some(id) = self.index.get(value) {
            return *id;
        }
        let id = i64::try_from(self.strings.len()).unwrap_or(i64::MAX);
        self.strings.push(value.to_owned());
        self.index.insert(value.to_owned(), id);
        id
    }

    fn into_vec(self) -> Vec<String> {
        self.strings
    }
}

impl Default for StringTable {
    fn default() -> Self {
        let mut me = Self {
            strings: Vec::new(),
            index: HashMap::new(),
        };
        // pprof requires string_table[0] to be the empty string.
        me.intern("");
        me
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use prost::Message;

    use super::*;

    fn frame(filename: &str, qualname: &str, line: u32) -> Frame {
        Frame {
            filename: Arc::from(filename),
            qualname: Arc::from(qualname),
            line,
        }
    }

    #[test]
    fn empty_stack_uses_unattributed_sentinel() {
        let mut builder = ProfileBuilder::new();
        builder.add_sample(&[], 4096);

        // Round-trip through encode/decode and check the structure.
        let path = tempfile::NamedTempFile::new().unwrap();
        builder.write_gzipped(path.path()).unwrap();

        let bytes = fs_err::read(path.path()).unwrap();
        let mut decoded = Vec::new();
        std::io::Read::read_to_end(&mut flate2::read::GzDecoder::new(&bytes[..]), &mut decoded)
            .unwrap();
        let profile = proto::Profile::decode(&decoded[..]).unwrap();

        assert_eq!(profile.sample.len(), 1);
        assert_eq!(profile.sample[0].value, vec![4096]);
        assert_eq!(profile.sample[0].location_id.len(), 1);
        assert!(
            profile.string_table.iter().any(|s| s == "<unknown>"),
            "expected the <unknown> sentinel in the string table"
        );
    }

    #[test]
    fn dedups_locations_and_functions() {
        let mut builder = ProfileBuilder::new();
        let f = frame("a.py", "fn", 10);
        builder.add_sample(std::slice::from_ref(&f), 100);
        builder.add_sample(std::slice::from_ref(&f), 200);

        assert_eq!(builder.locations.len(), 1);
        assert_eq!(builder.functions.len(), 1);
        assert_eq!(builder.samples.len(), 2);
    }
}
