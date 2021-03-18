use std::cmp::Ordering;
use crate::{Note, NoteLetter, NoteAccidental};

#[derive(Debug, Clone, Copy, Hash)]
pub struct MIDINote{
    pub note: Note,
    pub octave: i32
}

/// Represents a MIDI note
impl MIDINote{

    /// Creates a new music note
    pub fn new(letter: NoteLetter, accidental: NoteAccidental, octave: i32) -> MIDINote {
        let note = Note::new(letter, accidental);
        MIDINote{note: note, octave: octave}
    }

    /// Creates a new MIDI note
    pub fn from_note(note: Note, octave: i32) -> MIDINote {
        MIDINote{note: note, octave: octave}
    }

    // Creates a new music note based on a MIDI number
    pub fn from_midi_number(midi_num: u32) -> Option<MIDINote> {
        if midi_num < 128 {
            let mut midi_note = MIDINote::new(NoteLetter::C, NoteAccidental::Natural, -2);
            for _ in 0..midi_num {
                midi_note = midi_note.next();
            }
            Some( midi_note )
        } else {
            None
        }
    }

    /// Returns a MIDI note name e.g. Eb5
    pub fn print(&self) -> String {
        let mut res = match self.note.letter {
            NoteLetter::C => String::from("C"),
            NoteLetter::D => String::from("D"),
            NoteLetter::E => String::from("E"),
            NoteLetter::F => String::from("F"),
            NoteLetter::G => String::from("G"),
            NoteLetter::A => String::from("A"),
            NoteLetter::B => String::from("B")
        };

        match self.note.accidental {
            NoteAccidental::Flat => {
                res.push_str("b".to_string().as_str());
            },
            NoteAccidental::Sharp => {
                res.push_str("#".to_string().as_str());
            },
            _ => ()
        }

        res.push_str(self.octave.to_string().as_str());
        res
    }
    
    /// Returns the note's numerical id
    pub fn get_id(&self) -> i32 {
        let mut res : i32 = match self.note.letter {
            NoteLetter::C => 0,
            NoteLetter::D => 2,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::G => 7,
            NoteLetter::A => 9,
            NoteLetter::B => 11
        };

        res = match self.note.accidental {
            NoteAccidental::Flat => res - 1,
            NoteAccidental::Sharp => res + 1,
            _ => res
        };

        res += 12 * (self.octave as i32 + 1); 
        
        res
    }

    /// Returns the note's MIDI representation 
    pub fn get_midi_number(&self) -> Option<u32> {
        let id = self.get_id();
        if id < 128 && id >= 0 {
            Some( id as u32 )
        } else {
            None
        }
    }

    /// Returns the note's enharmonic equivalent
    pub fn equivalent(&self) -> Option<MIDINote> {
        match self.note.accidental{
            NoteAccidental::Natural => {
                match self.note.letter{
                    NoteLetter::B => Some( MIDINote::new(NoteLetter::C, NoteAccidental::Flat, self.octave) ),
                    NoteLetter::C => Some( MIDINote::new(NoteLetter::B, NoteAccidental::Sharp, self.octave) ),
                    NoteLetter::E => Some( MIDINote::new(NoteLetter::F, NoteAccidental::Flat, self.octave) ),
                    NoteLetter::F => Some( MIDINote::new(NoteLetter::E, NoteAccidental::Sharp, self.octave) ),
                    _=> None
                }
            },
            NoteAccidental::Sharp => {
                match self.note.letter{
                    NoteLetter::A => Some( MIDINote::new(NoteLetter::B, NoteAccidental::Flat, self.octave) ),
                    NoteLetter::B => Some( MIDINote::new(NoteLetter::C, NoteAccidental::Natural, self.octave) ),
                    NoteLetter::C => Some( MIDINote::new(NoteLetter::D, NoteAccidental::Flat, self.octave) ),
                    NoteLetter::D => Some( MIDINote::new(NoteLetter::E, NoteAccidental::Flat, self.octave) ),
                    NoteLetter::E => Some( MIDINote::new(NoteLetter::F, NoteAccidental::Natural, self.octave) ),
                    NoteLetter::F => Some( MIDINote::new(NoteLetter::G, NoteAccidental::Flat, self.octave) ),
                    NoteLetter::G => Some( MIDINote::new(NoteLetter::A, NoteAccidental::Flat, self.octave) )
                }
            },
            NoteAccidental::Flat => {
                match self.note.letter{
                    NoteLetter::A => Some( MIDINote::new(NoteLetter::G, NoteAccidental::Sharp, self.octave) ),
                    NoteLetter::B => Some( MIDINote::new(NoteLetter::A, NoteAccidental::Sharp, self.octave) ),
                    NoteLetter::C => Some( MIDINote::new(NoteLetter::B, NoteAccidental::Natural, self.octave) ),
                    NoteLetter::D => Some( MIDINote::new(NoteLetter::C, NoteAccidental::Sharp, self.octave) ),
                    NoteLetter::E => Some( MIDINote::new(NoteLetter::D, NoteAccidental::Sharp, self.octave) ),
                    NoteLetter::F => Some( MIDINote::new(NoteLetter::E, NoteAccidental::Natural, self.octave) ),
                    NoteLetter::G => Some( MIDINote::new(NoteLetter::F, NoteAccidental::Sharp, self.octave) )
                }
            }
        }
    }

    /// Returns the note one semitone above
    pub fn next(&self) -> MIDINote {
        match self.note.accidental {
            NoteAccidental::Natural => {
                match self.note.letter{
                    NoteLetter::B  => MIDINote::new(NoteLetter::C, NoteAccidental::Natural, self.octave + 1),
                    NoteLetter::E  => MIDINote::new(NoteLetter::F, NoteAccidental::Natural, self.octave),
                    _ =>  MIDINote::new(self.note.letter, NoteAccidental::Sharp, self.octave)
                }
            },
            NoteAccidental::Flat => {
                match self.note.letter {
                    NoteLetter::C => MIDINote::new(NoteLetter::C, NoteAccidental::Natural, self.octave + 1),
                    _ => MIDINote::new(self.note.letter, NoteAccidental::Natural, self.octave)
                }
            },
            NoteAccidental::Sharp => {
                match self.note.letter {
                    NoteLetter::C => MIDINote::new(NoteLetter::D, NoteAccidental::Natural, self.octave),
                    NoteLetter::D => MIDINote::new(NoteLetter::E, NoteAccidental::Natural, self.octave),
                    NoteLetter::E => MIDINote::new(NoteLetter::F, NoteAccidental::Sharp, self.octave),
                    NoteLetter::F => MIDINote::new(NoteLetter::G, NoteAccidental::Natural, self.octave),
                    NoteLetter::G => MIDINote::new(NoteLetter::A, NoteAccidental::Natural, self.octave),
                    NoteLetter::A => MIDINote::new(NoteLetter::B, NoteAccidental::Natural, self.octave),
                    NoteLetter::B => MIDINote::new(NoteLetter::C, NoteAccidental::Sharp, self.octave)
                }
            },
        }
    }

    /// Returns the note one semitone below
    pub fn previous(&self) -> MIDINote {
        match self.note.accidental {
            NoteAccidental::Natural => {
                match self.note.letter{
                    NoteLetter::C  => MIDINote::new(NoteLetter::B, NoteAccidental::Natural, self.octave - 1),
                    NoteLetter::F  => MIDINote::new(NoteLetter::E, NoteAccidental::Natural, self.octave),
                    _ => MIDINote::new(self.note.letter, NoteAccidental::Flat, self.octave)
                }
            },
            NoteAccidental::Sharp => {
                match self.note.letter {
                    NoteLetter::B => MIDINote::new(NoteLetter::B, NoteAccidental::Natural, self.octave - 1),
                    _ => MIDINote::new(self.note.letter, NoteAccidental::Natural, self.octave)
                }
            },
            NoteAccidental::Flat => {
                match self.note.letter {
                    NoteLetter::C => MIDINote::new(NoteLetter::B, NoteAccidental::Flat, self.octave),
                    NoteLetter::D => MIDINote::new(NoteLetter::C, NoteAccidental::Natural, self.octave),
                    NoteLetter::E => MIDINote::new(NoteLetter::D, NoteAccidental::Natural, self.octave),
                    NoteLetter::F => MIDINote::new(NoteLetter::E, NoteAccidental::Flat, self.octave),
                    NoteLetter::G => MIDINote::new(NoteLetter::F, NoteAccidental::Natural, self.octave),
                    NoteLetter::A => MIDINote::new(NoteLetter::G, NoteAccidental::Natural, self.octave),
                    NoteLetter::B => MIDINote::new(NoteLetter::A, NoteAccidental::Natural, self.octave)
                }
            },
        }
    }
}

impl Ord for MIDINote {
    fn cmp(&self, other: &MIDINote) -> Ordering {
        self.get_id().cmp(&other.get_id())
    }
}

impl PartialOrd for MIDINote {
    fn partial_cmp(&self, other: &MIDINote) -> Option<Ordering> {
        Some( self.get_id().cmp(&other.get_id()) )
    }
}

impl PartialEq for MIDINote {
    fn eq(&self, other: &MIDINote) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Eq for MIDINote {}