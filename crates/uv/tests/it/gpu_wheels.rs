use anyhow::Result;
use assert_fs::{fixture::PathChild, prelude::FileWriteStr};

use uv_test::uv_snapshot;

const EXCLUDE_NEWER: &str = "2026-06-18T00:00:00Z";

/// Resolve PyTorch3D and its matching PyTorch build from Astral's public GPU index.
#[test]
fn pytorch3d_cuda_128_public_index() -> Result<()> {
    let context = uv_test::test_context!("3.12").with_exclude_newer(EXCLUDE_NEWER);
    let requirements_in = context.temp_dir.child("requirements.in");
    requirements_in.write_str(
        "pytorch3d==0.7.9+cu.12.8.torch.2.10.cu12.8torch2.10.0cxx11abiTRUE\ntorch==2.10.0",
    )?;

    uv_snapshot!(context.filters(), context
        .pip_compile()
        .arg("requirements.in")
        .arg("--python-platform")
        .arg("x86_64-manylinux_2_28")
        .arg("--index")
        .arg("https://wheels.astralhosted.com/simple/cu128/")
        .arg("--torch-backend")
        .arg("cu128")
        .arg("--preview")
        .arg("--no-deps")
        .arg("--no-header")
        .arg("--no-annotate"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    pytorch3d==0.7.9+cu.12.8.torch.2.10.cu12.8torch2.10.0cxx11abitrue
    torch==2.10.0+cu128

    ----- stderr -----
    Resolved 2 packages in [TIME]
    ");

    Ok(())
}
