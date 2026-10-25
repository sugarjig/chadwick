use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=CHADWICK_DIR");
    println!("cargo:rerun-if-env-changed=CHADWICK_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=CHADWICK_LIB_DIR");
    println!("cargo:rerun-if-env-changed=CHADWICK_STATIC");

    let is_static = env::var("CHADWICK_STATIC")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let mut include_dirs = Vec::new();
    let mut lib_dirs = Vec::new();

    // 1. Explicit environment variables
    if let Ok(dir) = env::var("CHADWICK_DIR") {
        let p = PathBuf::from(dir);
        include_dirs.push(p.join("include"));
        include_dirs.push(p.join("include").join("chadwick"));
        lib_dirs.push(p.join("lib"));
    }
    if let Ok(dir) = env::var("CHADWICK_INCLUDE_DIR") {
        let p = PathBuf::from(dir);
        include_dirs.push(p.join("chadwick"));
        include_dirs.push(p);
    }
    if let Ok(dir) = env::var("CHADWICK_LIB_DIR") {
        lib_dirs.push(PathBuf::from(dir));
    }

    // 2. Try pkg-config
    if include_dirs.is_empty() {
        if let Ok(library) = pkg_config::Config::new().probe("chadwick") {
            include_dirs.extend(library.include_paths);
            lib_dirs.extend(library.link_paths);
        }
    }

    // 3. Try vcpkg
    if include_dirs.is_empty() {
        if let Ok(library) = vcpkg::Config::new().find_package("chadwick") {
            include_dirs.extend(library.include_paths);
            lib_dirs.extend(library.link_paths);
        }
    }

    // 4. Try brew --prefix chadwick
    if include_dirs.is_empty() {
        if let Ok(output) = Command::new("brew").args(["--prefix", "chadwick"]).output() {
            if output.status.success() {
                let prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !prefix.is_empty() {
                    let p = PathBuf::from(prefix);
                    include_dirs.push(p.join("include"));
                    include_dirs.push(p.join("include").join("chadwick"));
                    lib_dirs.push(p.join("lib"));
                }
            }
        }
    }

    // 5. Fallback standard system paths
    let standard_prefixes = ["/opt/homebrew", "/usr/local", "/usr"];
    for prefix in standard_prefixes {
        let p = Path::new(prefix);
        let inc = p.join("include").join("chadwick");
        let inc_direct = p.join("include");
        let lib = p.join("lib");
        if inc.exists() || inc_direct.join("chadwick.h").exists() {
            include_dirs.push(inc_direct);
            include_dirs.push(inc);
            lib_dirs.push(lib);
            break;
        }
    }

    // Configure link paths and library
    for lib_dir in &lib_dirs {
        if lib_dir.exists() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
        }
    }

    let link_type = if is_static { "static=" } else { "" };
    println!("cargo:rustc-link-lib={}chadwick", link_type);

    // Build bindings with bindgen
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("^cw_.*")
        .allowlist_type("^CW.*")
        .allowlist_type("^cw_.*")
        .allowlist_var("^CW_.*")
        .allowlist_var("^cw_.*")
        .derive_default(true)
        .derive_debug(true);

    for include_dir in &include_dirs {
        if include_dir.exists() {
            builder = builder.clang_arg(format!("-I{}", include_dir.display()));
        }
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings for Chadwick C library");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
