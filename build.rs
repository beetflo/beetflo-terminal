
#![feature(plugin)]
#![feature(rustc_private)]

#[macro_use] extern crate log;

extern crate pkg_config;
extern crate gcc;
extern crate cmake;

// use pkg_config::{Config, Error};

extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    let dst = cmake::Config::new("lib/beets")
        .out_dir("tmp")
        .build();

    println!("cargo:rustc-link-search=native=tmp/build");
    // println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=beets");
}

