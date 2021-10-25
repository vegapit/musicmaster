use std::cmp::Ordering;
use std::fmt;
use crate::{Note, NoteLetter, NoteAccidental};

#[derive(Debug, Clone, Copy, Hash)]
pub struct MIDINote{
    note: Note,
    octave: i32
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
    
    /// Returns the note's numerical id
    pub fn get_index(&self) -> u32 {
        let mut res : u32 = match self.note.get_letter() {
            NoteLetter::C => 0,
            NoteLetter::D => 2,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::G => 7,
            NoteLetter::A => 9,
            NoteLetter::B => 11
        };

        res = match self.note.get_accidental() {
            NoteAccidental::Flat => res - 1,
            NoteAccidental::Sharp => res + 1,
            _ => res
        };

        res += 12 * (self.octave as u32 + 1); 
        
        res
    }

    /// Returns the note's MIDI representation 
    pub fn get_midi_number(&self) -> Option<u32> {
        let idx = self.get_index();
        if idx < 128 {
            Some( idx as u32 )
        } else {
            None
        }
    }

    /// Returns the note's enharmonic equivalent
    pub fn equivalents(&self) -> Vec<MIDINote> {
        self.note.equivalents().into_iter()
            .map(|note| MIDINote::from_note(note, self.octave))
            .collect()
    }

    /// Returns the note one semitone above
    pub fn next(&self) -> MIDINote {
        match self.note.get_accidental() {
            NoteAccidental::Natural => {
                match self.note.get_letter() {
                    NoteLetter::B  => MIDINote::new(NoteLetter::C, NoteAccidental::Natural, self.octave + 1),
                    NoteLetter::E  => MIDINote::new(NoteLetter::F, NoteAccidental::Natural, self.octave),
                    _ =>  MIDINote::new(self.note.get_letter(), NoteAccidental::Sharp, self.octave)
                }
            },
            NoteAccidental::Flat => {
                match self.note.get_letter() {
                    NoteLetter::C => MIDINote::new(NoteLetter::C, NoteAccidental::Natural, self.octave + 1),
                    _ => MIDINote::new(self.note.get_letter(), NoteAccidental::Natural, self.octave)
                }
            },
            NoteAccidental::Sharp => {
                match self.note.get_letter() {
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
        match self.note.get_accidental() {
            NoteAccidental::Natural => {
                match self.note.get_letter() {
                    NoteLetter::C  => MIDINote::new(NoteLetter::B, NoteAccidental::Natural, self.octave - 1),
                    NoteLetter::F  => MIDINote::new(NoteLetter::E, NoteAccidental::Natural, self.octave),
                    _ => MIDINote::new(self.note.get_letter(), NoteAccidental::Flat, self.octave)
                }
            },
            NoteAccidental::Sharp => {
                match self.note.get_letter() {
                    NoteLetter::B => MIDINote::new(NoteLetter::B, NoteAccidental::Natural, self.octave - 1),
                    _ => MIDINote::new(self.note.get_letter(), NoteAccidental::Natural, self.octave)
                }
            },
            NoteAccidental::Flat => {
                match self.note.get_letter() {
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

impl fmt::Display for MIDINote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.note.to_string(), self.octave)
    }
}

impl Ord for MIDINote {
    fn cmp(&self, other: &MIDINote) -> Ordering {
        self.get_index().cmp(&other.get_index())
    }
}

impl PartialOrd for MIDINote {
    fn partial_cmp(&self, other: &MIDINote) -> Option<Ordering> {
        Some( self.get_index().cmp(&other.get_index()) )
    }
}

impl PartialEq for MIDINote {
    fn eq(&self, other: &MIDINote) -> bool {
        self.get_index() == other.get_index()
    }
}

impl Eq for MIDINote {}