use anyhow::Result;
use std::{fs, path::Path};
const ALLOWED_FUNC_PREFIX: &[&str] = &[
    "arg", "header", "rpm", "F", "fd", "pgp", "ri", "rs", "rr", "rc", "rm", "url",
];
const ALLOWED_TYPE_PREFIX: &[&str] = &[
    "ARG",
    "Header",
    "HEADER",
    "header",
    "rpm",
    "poptContext",
    "FD",
    "off",
    "pgp",
    "DIGEST",
    "fnpyKey",
    "url",
];
fn main() -> Result<()> {
    let conf = pkg_config::Config::new();
    let lib = conf.probe("rpm")?;
    for inc in lib.include_paths {
        // println!("{:?}",inc);
        if inc.join("rpm").is_dir() {
            let include_path = inc.join("rpm");
            let output = std::env::var("OUT_DIR")?;
            let mut builder = bindgen::Builder::default()
                .header(include_path.join("argv.h").to_str().unwrap())
                .header(include_path.join("header.h").to_str().unwrap())
                .header(include_path.join("rpmtypes.h").to_str().unwrap());
            for inc in fs::read_dir(include_path)? {
                let inc = inc?;
                let name = inc.file_name();
                let name = name.to_string_lossy();
                if name.starts_with("rpm") && name.ends_with(".h") {
                    builder = builder.header(inc.path().to_str().unwrap());
                }
            }
            builder
                .allowlist_type(format!("({}).*", ALLOWED_TYPE_PREFIX.join("|")))
                .allowlist_var(".*")
                .allowlist_function(format!("({}).*", ALLOWED_FUNC_PREFIX.join("|")))
                .generate()
                .unwrap()
                .write_to_file(Path::new(&output).join("bindings.rs"))?;
        }
    }
    Ok(())
}
