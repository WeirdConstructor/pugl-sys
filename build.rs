extern crate bindgen;

use std::process::Command;
use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    Command::new("python")
        .current_dir("pugl")
        .env("CFLAGS", "-fPIC")
	.arg("-B")
        .arg("waf")
        .arg("configure")
        .arg(format!("--out={}", out_path.to_str().unwrap()))
        .status()
        .expect("waf configure failed");

    Command::new("python")
        .current_dir("pugl")
        .env("CFLAGS", "-fPIC")
	.arg("-B")
        .arg("waf")
        .arg("build")
        .status()
        .expect("waf build failed");

    println!("cargo:rustc-link-search=native={}", out_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=pugl_x11-0");
    println!("cargo:rustc-link-lib=static=pugl_x11_cairo-0");
    println!("cargo:rustc-flags=-l cairo -l GLU -l GL -lX11 -lXext -lXrandr -lXcursor");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("pugl/pugl.h")
        .header("pugl/cairo.h")
        .header("pugl/include/pugl/stub.h")
        .clang_arg("-Ipugl/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // FIXME: do this properly
    let _ = fs::remove_file("pugl/.lock-waf_linux_build");
    let _ = fs::remove_file("pugl/.lock-waf_linux2_build");
}
