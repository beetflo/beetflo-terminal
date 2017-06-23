#[derive(Debug, Clone)]
pub struct Message {
    state: State,
    stamp: f64,
    velocity: u8,
    pitch: Pitch,
}

#[derive(Debug, Clone)]
pub struct Pitch {
    oct: Octave, note: Name, int: u8
}

#[derive(Debug, Clone)]
pub enum Acid { Sharp, Flat, Natural }

#[derive(Debug, Clone)]
pub enum State { On, Off }

#[derive(Clone, Debug)]
pub enum Name { A, ASharp, B, C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, Unknown }

#[derive(Debug, Clone)]
pub enum Octave {
    Oct0, Oct1, Oct2, Oct3, Oct4, Oct5, Oct6, Oct7, Oct8, Oct9, Oct10
}

use std::ops::Mul;
use std::fmt;

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}


impl Name {
    pub fn str(&self) -> &str {
        // debug!("{:?}", self);
        match self {
            A => "A",
            ASharp => "A#",
            B => "B",
            C => "C",
            CSharp => "C#",
            D => "D",
            DSharp => "D#",
            E => "E",
            F => "F",
            FSharp => "F#",
            G => "G",
            GSharp => "GSharp",
            Unknown => "Unknown"
        }
    }

    pub fn from_interval(interval: u8) -> Name {
        let int = Octave::from_interval(interval).i();
        let defraction = int.wrapping_mul(12);

        let mut i = 0;
        if interval > defraction {
            i =  (interval - defraction);
        }

        if i > 12 { i = i - 12 }

        // debug!("i:{}, interval:{} defraction: {}",i, interval, defraction);
        match i {
            0  => Name::C,
            1  => Name::CSharp,
            2  => Name::D,
            3  => Name::DSharp,
            4  => Name::E,
            5  => Name::F,
            6  => Name::FSharp,
            7  => Name::G,
            8  => Name::GSharp,
            9  => Name::A,
            10 => Name::ASharp,
            11 => Name::B,
            12 => Name::C,
            _ => Name::Unknown
        }
    }
}

impl Octave {
    pub fn i(&self) -> u8 {
        // debug!("self: {:?}", self);
        let s = &format!("{:?}", self)[..];
        let m = match s {
           "Oct0"=> 0,
           "Oct1"=> 1,
           "Oct2"=> 2,
           "Oct3"=> 3,
           "Oct4"=> 4,
           "Oct5" => 5,
           "Oct6"=> 6,
           "Oct7"=> 7,
           "Oct8"=> 8,
           "Oct9"=> 9,
            "Oct10" => 10,
            &_ => 0,
        };

        // debug!("found: {}", m);
        m
    }

    pub fn from_interval(i: u8)-> Octave {
        match i {
            0..12  => Octave::Oct0,
            12..24  => Octave::Oct1,
            24..48  => Octave::Oct2,
            48..60  => Octave::Oct3,
            60..72  => Octave::Oct4,
            72..84  => Octave::Oct5,
            84..96  => Octave::Oct6,
            96..108 => Octave::Oct7,
            108..120 => Octave::Oct8,
            120..132 => Octave::Oct9,
            _=> Octave::Oct0
        }
    }
}


impl Message {
    pub fn from_raw(raw: &[u8], stamp: f64) -> Message {
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

    pub fn blank() -> Message {
        Message {
            state: State::Off,
            pitch: Pitch { oct: Octave::Oct4, note: Name::C, int: 0 },
            velocity: 0,
            stamp: 0.0
        }
    }

    pub fn update_state(&mut self, state: State) -> &mut Message {
        self.state = state;
        self
    }

    pub fn update_pitch(&mut self, pitch: Pitch) -> &mut Message {
        self.pitch = pitch;
        self
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
        note: Name::from_interval(p),
        oct: Octave::from_interval(p),
        int: p
    }
}
