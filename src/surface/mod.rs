mod canvas;
mod core;

use std::thread::sleep;
use std::time::Duration;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::{self, RecvTimeoutError};


use midi::{Message};

pub use self::canvas::{Canvas};
pub use self::core::{Core};

pub struct Surface {
    recv: Receiver<Message>,
    sender: Sender<Message>
}

impl Surface {
    pub fn new(recv: Receiver<Message>, sender: Sender<Message>) -> Surface {
        Surface { recv: recv, sender: sender }
    }

    pub fn render(&self) {
        loop {
            for msg in self.recv.iter() {
                debug!("received: {:#?}", msg);
            }


            sleep(Duration::from_millis(30));
        }
    }
}
