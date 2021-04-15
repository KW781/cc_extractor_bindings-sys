//currently, there is no accounting for whitelisting/blacklisting of particular functions/variables, except blacklisting print_usage()
//this is likely to be implemented later on during GSoC

extern crate cmake;
extern crate bindgen;
use std::path::PathBuf;
use std::env;

fn main() {
    let dst = cmake::build("ccextractor/src"); //build lib_ccx using cmake

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=ccx");
    println!("cargo:return-if-changed=wrapper.h"); //re-generate the bindings if there are any changes made to the wrapper file

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .blacklist_function("print_usage");
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()); //write the bindings for the headers to the bindings.rs file
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Error with writing bindings");

}
