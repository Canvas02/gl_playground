// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::{env::var, fs::File, path::Path};

use gl_generator::{DebugStructGenerator, Registry, StructGenerator};

extern crate gl_generator;

fn main() {
    let dist = var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dist).join("gl_bindings.rs")).unwrap();

    let reg = Registry::new(
        gl_generator::Api::Gl,
        (4, 5),
        gl_generator::Profile::Core,
        gl_generator::Fallbacks::All,
        [],
    );

    if let Ok(_) = var("CARGO_FEATURE_DEBUG") {
        reg.write_bindings(DebugStructGenerator, &mut file).unwrap();
    } else {
        reg.write_bindings(StructGenerator, &mut file).unwrap();
    }
}
