// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::{env, path::PathBuf};

use fs_extra::{copy_items, dir::CopyOptions};

fn main() {
    println!("cargo:rerun-if-changed=assets/*");

    let mut options = CopyOptions::new();
    options.overwrite = true;

    let mut paths_to_copy = Vec::new();
    paths_to_copy.push(PathBuf::from("./assets"));

    let out_dir = {
        if let Ok(target) = env::var("CARGO_TARGET_DIR") {
            PathBuf::from(format!("{}/{}", target, env::var("PROFILE").unwrap()))
        } else {
            PathBuf::from(format!(
                "{}/target/{}",
                env::var("CARGO_MANIFEST_DIR").unwrap(),
                env::var("PROFILE").unwrap()
            ))
        }
    };
    // let out_dir = env::var("OUT_DIR").unwrap();
    dbg!(&paths_to_copy, &out_dir);

    copy_items(&paths_to_copy, &out_dir, &options).expect("Failed to copy items");
}
