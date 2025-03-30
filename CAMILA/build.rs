use std::{env, error::Error, path::PathBuf};

// This compiles the CAMILA.proto file
fn main() -> Result<(),  Box<dyn Error>>{
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("CAMILA_descriptor.bin"))
        .compile(&["proto/CAMILA.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/CAMILA.proto")?;

    Ok(())
}

