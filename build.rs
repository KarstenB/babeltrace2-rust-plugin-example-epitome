// Copyright 2023 - 2023, Karsten Becker and the rust babeltrace2-plugin contributors
// SPDX-License-Identifier: GPL-2.0-or-later
use cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::generate(crate_dir)
        .unwrap()
        .write_to_file("target/epitome.h");

    /*cc::Build::new()
        .include("target")
        .file("src/wrapper.c")
        .compile("inline_wrapper.a");*/
}
