#![feature(custom_attribute)]
#![feature(plugin)]
#![feature(exclusive_range_pattern)]

// Temporarly lints during rapid iteration..
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

#[macro_use] extern crate log;

// #[cfg(all(feature="winit", feature="glium"))]
#[macro_use] extern crate conrod;
#[macro_use] extern crate conrod_derive;

extern crate midir;
extern crate clap;
extern crate rayon;
extern crate env_logger;
#[macro_use] extern crate android_support;


use clap::App;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

mod midi;
mod surface;
mod utils;
mod widgets;
mod layouts;
mod theme;
mod backend;

use utils::{Environment};
use midi::{Reader, Message};
use surface::{Core as Surface, Canvas};
use backend as Backend;

