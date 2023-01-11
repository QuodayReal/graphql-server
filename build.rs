type MyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> MyResult {
    dotenv::dotenv().ok();

    auto_compile()?;
    Ok(())
}

fn auto_compile() -> MyResult {
    let proto_path: String = std::env::var("PROTO_PATH").expect("PROTO_PATH must be set");
    let proto_files = std::fs::read_dir(&proto_path)?;
    for proto_file in proto_files {
        let proto_file = proto_file?;
        let proto_file_path = proto_file.path();
        let proto_file_name = proto_file_path.file_name().unwrap().to_str().unwrap();
        if proto_file_name.ends_with(".proto") {
            println!("cargo:warning=Compiling {}", proto_file_path.display());
            tonic_build::configure()
                .build_client(true)
                .out_dir("src/protos")
                .compile(&[proto_file_path.to_str().unwrap()], &[&proto_path])?;
        }
    }

    Ok(())
}

