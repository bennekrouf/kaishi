fn main() -> Result<(), Box<dyn std::error::Error>> {
 let out_dir = std::env::var("OUT_DIR")?;
    println!("OUT_DIR: {}", out_dir);

    tonic_build::configure()
        .out_dir(&out_dir)
        .compile(
            &["proto-definitions/proto/log/log.proto", "proto-definitions/proto/matcher/matching.proto"], // Path to your .proto files
            &["proto"],           // Include path
        )?;
    Ok(())
}
