use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=gui/mainview.fl");
    println!("cargo:rerun-if-changed=gui/mydialog.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("gui/mainview.fl", out_path.join("mainview.rs").to_str().unwrap()).expect("Failed to generate rust from fl file!");
    g.in_out("gui/mydialog.fl", out_path.join("mydialog.rs").to_str().unwrap()).expect("Failed to generate rust from fl file!");
}