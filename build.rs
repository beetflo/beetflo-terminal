use std::process::Command;
use std::env;
use std::path::Path;

extern crate gcc;


fn main() {
    // gcc::compile_library("libbeets.a", &["src/beets.c"]);
    gcc::Config::new()
        .file("src/beets.c")
        .define("FOO", Some("bar"))
        .include("src")
        .compile("libbeets.a");
}
