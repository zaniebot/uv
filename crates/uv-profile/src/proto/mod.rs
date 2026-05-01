//! Generated pprof protobuf bindings.
//!
//! Schema is vendored at `src/proto/profile.proto` and compiled by
//! `build.rs` via `prost-build`. The generated module name follows the
//! protobuf package name (`perftools.profiles`).

#[allow(
    unreachable_pub,
    clippy::struct_field_names,
    reason = "prost-generated bindings emit `pub` items and follow the proto schema's field naming"
)]
pub(crate) mod perftools {
    #[allow(
        unreachable_pub,
        clippy::struct_field_names,
        reason = "prost-generated bindings emit `pub` items and follow the proto schema's field naming"
    )]
    pub(crate) mod profiles {
        include!(concat!(env!("OUT_DIR"), "/perftools.profiles.rs"));
    }
}

pub(crate) use perftools::profiles::*;
