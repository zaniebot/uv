fn main() {
    println!("cargo:rerun-if-changed=src/proto/profile.proto");
    let mut config = prost_build::Config::new();
    config.bytes(["."]);
    config
        .compile_protos(&["src/proto/profile.proto"], &["src/proto"])
        .expect("failed to compile profile.proto");
}
