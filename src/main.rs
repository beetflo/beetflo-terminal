#![feature(custom_attribute)]
#![feature(plugin)]
#![feature(exclusive_range_pattern)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use] extern crate log;


extern crate midir;
extern crate clap;
extern crate rayon;
extern crate env_logger;

use clap::App;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

mod midi;
mod surface;
mod utils;

use utils::{Environment};
use midi::{Reader, Message};
use surface::{Surface};

fn main() {
    env_logger::init().unwrap();
    Environment::init();

    let mut handles = vec![];
    let (send, recv) = mpsc::channel::<Message>();
    let sender2 = send.clone();

    handles.push(thread::spawn(move || {
        debug!("Starting Midi Reader thread");
        Reader::new(send).stream()
    }));

    handles.push(thread::spawn(move || {
        debug!("Starting Surface rendering thread");
        Surface::new(recv, sender2).render()
    }));


    for handle in handles {
        handle.join().unwrap();
    }

}
