extern crate cbindgen;

use std::path::Path;

fn main() {
    let header_file = Path::new("examples")
        .join("c")
        .join("include")
        .join("client.h");

    cbindgen::generate(".")
        .expect("Unable to generate bindings")
        .write_to_file(header_file);
}
