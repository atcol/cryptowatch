use std::io::Result;

use prost_build::Config;
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/cryptowatch-rest.proto");
    println!("cargo:rerun-if-changed=Cargo.lock");
    let _config = Config::new()
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile_protos(&["src/cryptowatch-rest.proto"], &["src/"])?;
    Ok(())
}
