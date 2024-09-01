fn main() -> Result<(), Box<dyn std::error::Error>> {
 let out_dir = std::env::var("OUT_DIR")?;
    println!("OUT_DIR: {}", out_dir);

    tonic_build::configure()
        .out_dir(&out_dir)
        .compile(
            &["proto-definitions/messenger/messenger.proto", "proto-definitions/matcher/matching.proto"],
            &["proto"],
        )?;
    Ok(())
}
