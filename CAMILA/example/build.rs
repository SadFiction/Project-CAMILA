use std::{env, error::Error, path::PathBuf};

// This compiles the CAMILA.proto file
fn main(){
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("CAMILA_descriptor.bin"))
        .compile(&["proto/CAMILA.proto"], &["proto"]).unwrap();

    tonic_build::compile_protos("proto/CAMILA.proto")
    .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

}

