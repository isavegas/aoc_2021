pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/day");
    println!("cargo:rerun-if-changed=src/input");
    aoc_core::generate_get_days();
    aoc_core::generate_get_inputs();
}
