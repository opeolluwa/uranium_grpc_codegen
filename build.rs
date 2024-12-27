use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let proto_files = load_proto_file("proto").unwrap();

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .protoc_arg("--proto_path=./proto")
        .build_client(true) // don't compile the client code
        .build_server(false)
        .type_attribute(
            ".",
            "#[derive(serde::Deserialize, ts_rs::TS, serde::Serialize)]",
        )
        .type_attribute(".", "#[ts(export)]")
        .out_dir("./src/client")
        .compile_protos(&proto_files, &["../proto"])?;

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .protoc_arg("--proto_path=./proto")
        .build_client(false) // don't compile the client code
        .build_server(true)
        .out_dir("./src/server")
        .compile_protos(&proto_files, &["../proto"])?;
    Ok(())
}

fn load_proto_file(proto_dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let proto_files = std::fs::read_dir(proto_dir)?
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|dir_entry| {
            if dir_entry
                .extension()
                .map_or(false, |extension| extension == "proto")
            {
                Some(dir_entry)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(proto_files)
}
