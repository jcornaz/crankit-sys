use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;

fn main() {
    let c_api_path = PathBuf::from(env::var("PLAYDATE_SDK_PATH").unwrap()).join("C_API");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed={}", c_api_path.display());
    bindgen::Builder::default()
        .use_core()
        .header("wrapper.h")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .allowlist_type("PlaydateAPI")
        .allowlist_type("PDSystemEvent")
        .allowlist_type("LCDSolidColor")
        .allowlist_type("LCDColor")
        .allowlist_type("LCDPattern")
        .allowlist_var("LCD_COLUMNS")
        .allowlist_var("LCD_ROWS")
        .bitfield_enum("PDButtons")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{}", c_api_path.display()))
        .clang_arg("-DTARGET_EXTENSION")
        .generate()
        .unwrap()
        .write_to_file(out_path)
        .unwrap();
}
