use midi::{Octave, Pitch};

#[derive(Debug, Clone, PartialEq)]
pub struct Update {
    pub op: UpdateType,
    pub pitch: Option<Pitch>,

}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateType {
    NoteOn,
    NoteOff,
    Noop
}

impl Update {
    pub fn noop() -> Update {
        Update {
            op: UpdateType::Noop,
            pitch: None,
        }
    }
}
