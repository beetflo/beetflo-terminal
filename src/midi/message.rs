#[derive(Debug, Clone)]
pub struct Message {
    state: State,
    pitch: Pitch,
    velocity: u8,
    stamp: f64
}

#[derive(Debug, Clone)]
pub struct Pitch {
    oct: Octave, note: Name
}

#[derive(Debug, Clone)]
pub enum Acid { Sharp, Flat, Natural }
#[derive(Debug, Clone)]
pub enum State { On, Off }
#[derive(Debug, Clone)]
pub enum Name { A,B,C,D,E,F,G}
#[derive(Debug, Clone)]
pub enum Octave {
    O1, O2, O3, O4, O5, O6, O7, O8, O9, O10
}



impl Message {
    pub fn from_raw(raw: &[u8], stamp: f64) -> Message {
        print!("\nraw: {:?}", raw);

        Message {
            state: translate_note_state(raw[0]),
            pitch: translate_to_pitch(raw[1]),
            velocity: raw[2],
            stamp: stamp
        }
    }

    pub fn describe(&self) {
        print!("⸳⸳⸳⸳⸳..⸳⸳⸳ {:?}, {:?}, Velocity: {:?}", self.state, self.pitch, self.velocity);
    }
}


fn translate_note_state(note: u8) -> State {
    if note == 144 {
        State::On
    } else {
        State::Off
    }
}

fn translate_to_pitch(p: u8) -> Pitch {
    Pitch {
        note: Name::C,
        oct: Octave::O4
    }
}
