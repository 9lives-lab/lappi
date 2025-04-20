#[allow(dead_code)]
fn generate_proto() {
    protobuf_codegen::Codegen::new()
        .protoc()
        .includes(&["../proto"])
        .input("../proto/collection.proto")
        .out_dir("src/proto")
        .run_from_script();
}

fn main() {
    // Uncomment to regenerate protobuf code
    // generate_proto();
}
