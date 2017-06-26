
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
    error!("entry!");

    let dst = cmake::build("lib/beets");
    error!("{}", dst.display());
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=beets_static");
}

