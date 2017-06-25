
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, MidiOutput, Ignore};
use midi::{Note, Message};

use utils::Environment;
use midi::message::State;
pub struct Reader {
    sender: Sender<Message>
}

impl Reader {
    pub fn new(sender: Sender<Message>) -> Reader {
        Reader {
            sender: sender
        }
    }

    pub fn stream(&self) {
        debug!("beginning stream");

        let env = Environment::global();
        let mut input = String::new();

        let midi_out = MidiOutput::new("Beetflo Midi Output").unwrap();
        let mut midi_in = MidiInput::new("Beetflo Midi Input").unwrap();
        midi_in.ignore(Ignore::None);

        println!("Available input ports:");
        for i in 0..midi_in.port_count() {
            println!("{}: {}", i, midi_in.port_name(i).unwrap());
        }

        println!("\nAvailable output ports:");
        for i in 0..midi_out.port_count() {
            println!("{}: {}", i, midi_out.port_name(i).unwrap());
        }

        let in_port: u32 = env.input.clone();
        let out_port: u32 = env.output.clone();

        let mut internal_sender = self.sender.clone();

        let conn_in = midi_in.connect(in_port, "beetflo-connect", move |stamp, message, _| {
            stdout().flush().unwrap();
            internal_sender.send(Message::from_raw(message, stamp)).unwrap();
        }, ()).map_err(|e| e.kind()).unwrap();

        let mut conn_out = midi_out
            .connect(out_port, "beetflo-connect")
            .map_err(|e| e.kind()).unwrap();


        loop {
            conn_out.send(&[144, 60, 1]).unwrap();
            sleep(Duration::from_millis(200));
            conn_out.send(&[144, 60, 0]).unwrap();
        }
    }
}
