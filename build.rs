use std::fs;

const CARGO_TOML_1: &str = r#"
[package]
name = "femto"
version = "0.0.1"
authors = ["Dariusz Depta <dariusz@confio.gmbh>"]
description = "Test suites for cw-multi-test"
repository = "https://github.com/DariuszDepta/femto"
license = "Apache-2.0"
edition = "2021"

[features]
default = ["v1"]
v1 = []
v2 = []

[dependencies]
cosmwasm-std = "1.5.3"
cw-multi-test = { version = "1.0.0-rc.0", features = ["cosmwasm_1_4"] }
cw-storage-plus = "1.2.0"
cw-utils = "1.0.3"
schemars = "0.8.16"
serde = "1.0.197"
"#;

const CARGO_TOML_2: &str = r#"
[package]
name = "femto"
version = "0.0.1"
authors = ["Dariusz Depta <dariusz@confio.gmbh>"]
description = "Test suites for cw-multi-test"
repository = "https://github.com/DariuszDepta/femto"
license = "Apache-2.0"
edition = "2021"

[features]
default = ["v2"]
v1 = []
v2 = []

[dependencies]
cosmwasm-std = "2.0.0"
cw-multi-test = "2.0.0-rc.0"
cw-storage-plus = "2.0.0-rc.0"
cw-utils = "2.0.0-rc.0"
schemars = "0.8.16"
serde = "1.0.197"
"#;


fn main() {
  if std::env::var("FEMTO_V1").is_ok() {
    fs::write("Cargo.toml", CARGO_TOML_1.trim()).expect("writing failed")
  }
  if std::env::var("FEMTO_V2").is_ok() {
    fs::write("Cargo.toml", CARGO_TOML_2.trim()).expect("writing failed")
  }
}
