fn main() {
    println!("cargo:rerun-if-changed=app/");
    tauri_build::build()
}
