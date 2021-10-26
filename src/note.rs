use std::cmp::Ordering;
use std::fmt;
use std::convert::TryFrom;
use itertools::Itertools;
use crate::Interval;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum NoteLetter{ A,B,C,D,E,F,G }

impl TryFrom<&str> for NoteLetter {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            _ => Err("Input string does not correspond to a known NoteLetter")
        }
    }
}

pub fn all_note_letters() -> Vec<NoteLetter> {
    ["A","B","C","D","E","F","G"].iter()
        .map(|&s| NoteLetter::try_from(s).unwrap())
        .collect()
}

pub fn next_note_letter(note_letter: &NoteLetter) -> NoteLetter {
    match note_letter {
        NoteLetter::A => NoteLetter::B,
        NoteLetter::B => NoteLetter::C,
        NoteLetter::C => NoteLetter::D,
        NoteLetter::D => NoteLetter::E,
        NoteLetter::E => NoteLetter::F,
        NoteLetter::F => NoteLetter::G,
        NoteLetter::G => NoteLetter::A
    }
}

pub fn previous_note_letter(note_letter: &NoteLetter) -> NoteLetter {
    match note_letter {
        NoteLetter::A => NoteLetter::G,
        NoteLetter::B => NoteLetter::A,
        NoteLetter::C => NoteLetter::B,
        NoteLetter::D => NoteLetter::C,
        NoteLetter::E => NoteLetter::D,
        NoteLetter::F => NoteLetter::E,
        NoteLetter::G => NoteLetter::F
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NoteAccidental{
    Natural,
    Flat,
    Sharp
}

pub fn all_note_accidentals() -> Vec<NoteAccidental> {
    vec![ NoteAccidental::Natural, NoteAccidental::Flat, NoteAccidental::Sharp ]
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Note{
    letter: NoteLetter,
    accidental: NoteAccidental
}

/// Represents a music note
impl Note{

    /// Creates a new music note
    pub fn new(letter: NoteLetter, accidental: NoteAccidental) -> Note {
        Note{letter: letter, accidental: accidental}
    }

    pub fn get_letter(&self) -> NoteLetter { self.letter }
    pub fn get_accidental(&self) -> NoteAccidental { self.accidental }

    /// Returns the note's numeral based on a root note e.g. Eb from C is 3b
    pub fn as_numeral(&self, scale_root: &Note, index: usize) -> String {
        let interval_name = Interval::from_notes(scale_root, &self).to_string();
        match interval_name.as_str() {
            "P1" => String::from("R"),
            "m2" => String::from("2b"),
            "M2" => {
                match index {
                    2 => String::from("3bb"),
                    _ => String::from("2")
                }
            },
            "m3" => {
                match index {
                    2 => String::from("3b"),
                    _ => String::from("2#")
                }
            },
            "M3" => {
                match index {
                    3 => String::from("4b"),
                    _ => String::from("3")
                }
            },
            "P4" => {
                match index {
                    3 => String::from("4"),
                    _ => String::from("3#")
                }
            },
            "d5" => {
                match index { 
                    3 => String::from("4#"),
                    _ => String::from("5b")
                }
            },
            "P5" => { 
                match index {
                    5 => String::from("6bb"),
                    _ => String::from("5")
                }
            },
            "m6" => {
                match index {
                    4 => String::from("5#"),
                    _ => String::from("6b")
                }
            },
            "M6" => {
                match index {
                    6 => String::from("7bb"),
                    _ => String::from("6")
                }
            },
            "m7" => {
                match index {
                    5 => String::from("6#"),
                    _ => String::from("7b")
                }
            },
            "M7" => String::from("7"),
            _ => String::from("")
        }
    }
    
    /// Returns the note's numerical index
    pub fn get_index(&self) -> u32 {
        let res : u32 = match self.letter {
            NoteLetter::C => 0,
            NoteLetter::D => 2,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::G => 7,
            NoteLetter::A => 9,
            NoteLetter::B => 11
        };
        match self.accidental {
            NoteAccidental::Flat => if res == 0 { 11 } else { res - 1 },
            NoteAccidental::Sharp => if res == 11 { 0 } else { res + 1 },
            _ => res
        }
    }

    /// Returns the note's enharmonic equivalents
    pub fn equivalents(&self) -> Vec<Note> {
        let mut res : Vec<Note> = Vec::new();
        let all_other_letters : Vec<NoteLetter> = all_note_letters().into_iter()
            .filter(|&elt| elt != self.get_letter() )
            .collect();
        for note_letter in all_other_letters.into_iter() {
            for accidental in all_note_accidentals().iter() {
                let note = Note::new( note_letter.clone(), accidental.clone() );
                if note == *self { res.push(note); }
            }
            if res.len() == 2 { break; }
        }
        res
    }

    /// Returns the note one semitone above
    pub fn next(&self) -> Note {
        let target_idx = if self.get_index() == 11 { 0 } else { self.get_index() + 1 };
        let candidates : Vec<Note> = all_note_letters().into_iter()
            .cartesian_product( all_note_accidentals().into_iter() )
            .map(|elt| Note::new(elt.0.clone(), elt.1.clone()) )
            .filter(|&note| note.get_index() == target_idx )
            .collect();
        match candidates.iter().find(|&&note| note.get_accidental() == NoteAccidental::Natural) {
            Some(note) => { return *note; }
            _ => {}
        }
        match candidates.iter().find(|&&note| note.get_accidental() == NoteAccidental::Sharp) {
            Some(note) => { return *note; }
            _ => {}
        }
        candidates[0]
    }

    /// Returns the note one semitone below
    pub fn previous(&self) -> Note {
        let target_idx = if self.get_index() == 0 { 11 } else { self.get_index() - 1 };
        let candidates : Vec<Note> = all_note_letters().into_iter()
            .cartesian_product( all_note_accidentals().into_iter() )
            .map(|elt| Note::new(elt.0.clone(), elt.1.clone()) )
            .filter(|&note| note.get_index() == target_idx )
            .collect();
        match candidates.iter().find(|&&note| note.get_accidental() == NoteAccidental::Natural) {
            Some(note) => { return *note; }
            _ => {}
        }
        match candidates.iter().find(|&&note| note.get_accidental() == NoteAccidental::Flat) {
            Some(note) => { return *note; }
            _ => {}
        }
        candidates[0]
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self.letter {
            NoteLetter::C => String::from("C"),
            NoteLetter::D => String::from("D"),
            NoteLetter::E => String::from("E"),
            NoteLetter::F => String::from("F"),
            NoteLetter::G => String::from("G"),
            NoteLetter::A => String::from("A"),
            NoteLetter::B => String::from("B")
        };
        match self.accidental {
            NoteAccidental::Flat => write!(f, "{}b", res),
            NoteAccidental::Sharp => write!(f, "{}#", res),
            _ => write!(f, "{}", res)
        }
    }
}

impl TryFrom<&str> for Note {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" => Ok( Note::new(NoteLetter::C, NoteAccidental::Natural) ),
            "C#" => Ok( Note::new(NoteLetter::C, NoteAccidental::Sharp) ),
            "Db" => Ok( Note::new(NoteLetter::D, NoteAccidental::Flat) ),
            "D" => Ok( Note::new(NoteLetter::D, NoteAccidental::Natural) ),
            "D#" => Ok( Note::new(NoteLetter::D, NoteAccidental::Sharp) ),
            "Eb" => Ok( Note::new(NoteLetter::E, NoteAccidental::Flat) ),
            "E" => Ok( Note::new(NoteLetter::E, NoteAccidental::Natural) ),
            "F" => Ok( Note::new(NoteLetter::F, NoteAccidental::Natural) ),
            "F#" => Ok( Note::new(NoteLetter::F, NoteAccidental::Sharp) ),
            "Gb" => Ok( Note::new(NoteLetter::G, NoteAccidental::Flat) ),
            "G" => Ok( Note::new(NoteLetter::G, NoteAccidental::Natural) ),
            "G#" => Ok( Note::new(NoteLetter::G, NoteAccidental::Sharp) ),
            "Ab" => Ok( Note::new(NoteLetter::A, NoteAccidental::Flat) ),
            "A" => Ok( Note::new(NoteLetter::A, NoteAccidental::Natural) ),
            "A#" => Ok( Note::new(NoteLetter::A, NoteAccidental::Sharp) ),
            "Bb" => Ok( Note::new(NoteLetter::B, NoteAccidental::Flat) ),
            "B" => Ok( Note::new(NoteLetter::B, NoteAccidental::Natural) ),
            _ => Err("Failed to generate Note from string")
        }
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Note) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Note) -> Ordering {
        self.get_index().cmp(&other.get_index())
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Note) -> bool {
        self.get_index() == other.get_index()
    }
}

impl Eq for Note {}