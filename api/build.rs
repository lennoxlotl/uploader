fn main() {
    println!("cargo:rerun-if-changed=migrations");
    build_info_build::build_script();
}
