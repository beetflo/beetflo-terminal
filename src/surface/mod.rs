mod canvas;
mod core;

use std::thread::sleep;
use std::time::Duration;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::{self, RecvTimeoutError};


use cursive::Cursive;
use cursive::views::{Dialog, TextView};

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

        // let mut siv = Cursive::new();
        // Creates a dialog with a single "Quit" button
        // siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
        //               .title("Cursive")
        //               .button("Quit", |s| s.quit()));
        
        loop {
            for msg in self.recv.iter() {
                debug!("{:#?}\n", msg);
                // siv.step();
            }

            sleep(Duration::from_millis(60));
        }
    }
}
