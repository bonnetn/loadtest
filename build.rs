fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.type_attribute(
        ".loadtest.RequestResult",
        r#"#[expect(dead_code, reason = "prost-generated enum variants used by protocol")]"#,
    );
    config.compile_protos(&["proto/loadtest.proto"], &["proto/"])?;
    Ok(())
}
