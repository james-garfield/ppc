fn main() {
    // panic!("OUT_DIR: {:?}", std::env::var("OUT_DIR"));

    tonic_build::compile_protos("proto/ppc.proto").expect("Error at proto compilation  ");

    // prost_build::compile_protos(&["proto/ppc.proto"], &["."]).unwrap();
}

 