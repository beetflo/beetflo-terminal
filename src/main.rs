extern crate novation_midi;
extern crate midir;

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, MidiOutput, Ignore};

mod midi;
use midi::{Note, Message};

fn main() {
    let mut input = String::new();
    let mut midi_out = MidiOutput::new("Beetflo out").unwrap();
    let mut midi_in = MidiInput::new("Beetflo in").unwrap();
    midi_in.ignore(Ignore::None);


    print!("Available Inputs: ");
    for i in 0..midi_in.port_count() {
        println!("{}: {}", i, midi_in.port_name(i).unwrap());
    }

    print!("Please select input port: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let in_port: u32 = input.trim().parse().unwrap();

    println!("\nAvailable output ports:");
    for i in 0..midi_out.port_count() {
        println!("{}: {}", i, midi_out.port_name(i).unwrap());
    }
    print!("Please select output port: ");
    stdout().flush().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let out_port: u32 = input.trim().parse().unwrap();

    println!("\nOpening connections");
    let conn_in = midi_in.connect(in_port, "midir-test", |stamp, message, _| {
        let msg = Message::from_raw(message, stamp);
        println!("");
        msg.describe();

    }, ()).map_err(|e| e.kind()).unwrap();

    let mut conn_out = midi_out.connect(out_port, "midir-test").map_err(|e| e.kind()).unwrap();

    println!("Connections open, enter `q` to exit ...");

    loop {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "q" {
            break;
        } else {
            // conn_out.send(&[144, 60, 1]);
            sleep(Duration::from_millis(200));
            // conn_out.send(&[144, 60, 0]);
        }
    }
    println!("Closing connections");
    // This is optional, the connections would automatically be closed
    // as soon as they go out of scope
    conn_in.close();
    conn_out.close();
    println!("Connections closed");
}
