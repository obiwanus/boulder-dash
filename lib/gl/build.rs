extern crate gl_generator;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    Registry::new(
        Api::Gl,
        (4, 1),
        Profile::Core,
        Fallbacks::All,
        ["GL_NV_command_list"],
    )
    .write_bindings(GlobalGenerator, &mut file_gl)
    .unwrap();
}
