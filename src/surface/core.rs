
use midi::{Message};

use std;
use std::thread::sleep;
use std::time::Duration;

use std::sync::mpsc::{Sender, Receiver};
// use std::sync::mpsc::{self, RecvTimeoutError};

use Canvas;
use surface::{Update, UpdateType};

pub struct Core {
    recv: Receiver<Message>,
    sender: Sender<Message>,
    controller: Option<CanvasController>
}

struct CanvasController {
    sender: Sender<Update>
}

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

impl Core {
    pub fn new(recv: Receiver<Message>, sender: Sender<Message>) -> Core {
        Core { recv: recv, sender: sender, controller: None }
    }

    pub fn render(&mut self) {
        let (cv_send, cv_recv) = std::sync::mpsc::channel::<Update>();
        std::thread::spawn(|| Canvas::new(cv_recv).init(WIDTH, HEIGHT));

        for msg in &self.recv {
            let up = translate_midi_into_surface_update(msg);
            cv_send.send(up).unwrap();
        }
    }
}

pub fn translate_midi_into_surface_update(msg: Message) -> Update {
    Update {
        op: UpdateType::NoteOn,
        pitch: Some(msg.pitch)
    }
}
