use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../frontend/");

    let _ = Command::new("npm")
        .args(["run", "build"])
        .current_dir("../frontend/")
        .output()
        .unwrap();
}
