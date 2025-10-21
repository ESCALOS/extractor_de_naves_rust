fn main() {
    // Instrucción para que Cargo pase el .res al linker
    println!("cargo:rustc-link-arg-bins=icon.res");
}