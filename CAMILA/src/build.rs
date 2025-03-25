// This compiles the CAMILA.proto file
fn main(){
    tonic_build::compile_protos("proto/CAMILA.proto")
    .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}

