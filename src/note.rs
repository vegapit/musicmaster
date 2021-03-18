use std::cmp::Ordering;
use std::str::FromStr;
use crate::Interval;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum NoteLetter{
    A,B,C,D,E,F,G
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum NoteAccidental{
    Natural,
    Flat,
    Sharp
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Note{
    pub letter: NoteLetter,
    pub accidental: NoteAccidental
}

#[derive(Debug)]
pub struct ParseNoteError {
    details: String
}

impl ParseNoteError {
    fn new(msg: &str) -> Self {
        ParseNoteError{ details: format!("Could not convert {} to a Note", msg) }
    }
}

/// Represents a music note
impl Note{

    /// Creates a new music note
    pub fn new(letter: NoteLetter, accidental: NoteAccidental) -> Note {
        Note{letter: letter, accidental: accidental}
    }

    /// Returns a note name e.g. Eb
    pub fn print(&self) -> String {
        let mut res = match self.letter {
            NoteLetter::C => String::from("C"),
            NoteLetter::D => String::from("D"),
            NoteLetter::E => String::from("E"),
            NoteLetter::F => String::from("F"),
            NoteLetter::G => String::from("G"),
            NoteLetter::A => String::from("A"),
            NoteLetter::B => String::from("B")
        };
        match self.accidental {
            NoteAccidental::Flat => {
                res.push_str("b".to_string().as_str());
                res
            },
            NoteAccidental::Sharp => {
                res.push_str("#".to_string().as_str());
                res
            },
            _ => res
        }
    }

    /// Returns the note's numeral based on a root note e.g. Eb from C is 3b
    pub fn print_numeral(&self, scale_root: &Note, index: usize) -> String {
        let interval = Interval::from_notes(scale_root, &self);
        let interval_name = interval.print();
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
    
    /// Returns the note's numerical id
    pub fn get_id(&self) -> i32 {
        let mut res : i32 = match self.letter {
            NoteLetter::C => 0,
            NoteLetter::D => 2,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::G => 7,
            NoteLetter::A => 9,
            NoteLetter::B => 11
        };

        res = match self.accidental {
            NoteAccidental::Flat => if res == 0 { 11 } else { res - 1 },
            NoteAccidental::Sharp => if res == 11 { 0 } else { res + 1 },
            _ => res
        };
        
        res
    }

    /// Returns the note's enharmonic equivalent
    pub fn equivalent(&self) -> Option<Note> {
        match self.accidental{
            NoteAccidental::Natural => {
                match self.letter{
                    NoteLetter::B => Some( Note::new(NoteLetter::C, NoteAccidental::Flat) ),
                    NoteLetter::C => Some( Note::new(NoteLetter::B, NoteAccidental::Sharp) ),
                    NoteLetter::E => Some( Note::new(NoteLetter::F, NoteAccidental::Flat) ),
                    NoteLetter::F => Some( Note::new(NoteLetter::E, NoteAccidental::Sharp) ),
                    _=> None
                }
            },
            NoteAccidental::Sharp => {
                match self.letter{
                    NoteLetter::A => Some( Note::new(NoteLetter::B, NoteAccidental::Flat) ),
                    NoteLetter::B => Some( Note::new(NoteLetter::C, NoteAccidental::Natural) ),
                    NoteLetter::C => Some( Note::new(NoteLetter::D, NoteAccidental::Flat) ),
                    NoteLetter::D => Some( Note::new(NoteLetter::E, NoteAccidental::Flat) ),
                    NoteLetter::E => Some( Note::new(NoteLetter::F, NoteAccidental::Natural) ),
                    NoteLetter::F => Some( Note::new(NoteLetter::G, NoteAccidental::Flat) ),
                    NoteLetter::G => Some( Note::new(NoteLetter::A, NoteAccidental::Flat) )
                }
            },
            NoteAccidental::Flat => {
                match self.letter{
                    NoteLetter::A => Some( Note::new(NoteLetter::G, NoteAccidental::Sharp) ),
                    NoteLetter::B => Some( Note::new(NoteLetter::A, NoteAccidental::Sharp) ),
                    NoteLetter::C => Some( Note::new(NoteLetter::B, NoteAccidental::Natural) ),
                    NoteLetter::D => Some( Note::new(NoteLetter::C, NoteAccidental::Sharp) ),
                    NoteLetter::E => Some( Note::new(NoteLetter::D, NoteAccidental::Sharp) ),
                    NoteLetter::F => Some( Note::new(NoteLetter::E, NoteAccidental::Natural) ),
                    NoteLetter::G => Some( Note::new(NoteLetter::F, NoteAccidental::Sharp) )
                }
            }
        }
    }

    /// Returns the note one semitone above
    pub fn next(&self) -> Note {
        match self.accidental {
            NoteAccidental::Natural => {
                match self.letter{
                    NoteLetter::B  => Note::new(NoteLetter::C, NoteAccidental::Natural),
                    NoteLetter::E  => Note::new(NoteLetter::F, NoteAccidental::Natural),
                    _ =>  Note::new(self.letter, NoteAccidental::Sharp)
                }
            },
            NoteAccidental::Flat => Note::new(self.letter, NoteAccidental::Natural),
            NoteAccidental::Sharp => {
                match self.letter {
                    NoteLetter::C => Note::new(NoteLetter::D, NoteAccidental::Natural),
                    NoteLetter::D => Note::new(NoteLetter::E, NoteAccidental::Natural),
                    NoteLetter::E => Note::new(NoteLetter::F, NoteAccidental::Sharp),
                    NoteLetter::F => Note::new(NoteLetter::G, NoteAccidental::Natural),
                    NoteLetter::G => Note::new(NoteLetter::A, NoteAccidental::Natural),
                    NoteLetter::A => Note::new(NoteLetter::B, NoteAccidental::Natural),
                    NoteLetter::B => Note::new(NoteLetter::C, NoteAccidental::Sharp)
                }
            },
        }
    }

    /// Returns the note one semitone below
    pub fn previous(&self) -> Note {
        match self.accidental {
            NoteAccidental::Natural => {
                match self.letter{
                    NoteLetter::C  => Note::new(NoteLetter::B, NoteAccidental::Natural),
                    NoteLetter::F  => Note::new(NoteLetter::E, NoteAccidental::Natural),
                    _ => Note::new(self.letter, NoteAccidental::Flat)
                }
            },
            NoteAccidental::Sharp => Note::new(self.letter, NoteAccidental::Natural),
            NoteAccidental::Flat => {
                match self.letter {
                    NoteLetter::C => Note::new(NoteLetter::B, NoteAccidental::Flat),
                    NoteLetter::D => Note::new(NoteLetter::C, NoteAccidental::Natural),
                    NoteLetter::E => Note::new(NoteLetter::D, NoteAccidental::Natural),
                    NoteLetter::F => Note::new(NoteLetter::E, NoteAccidental::Flat),
                    NoteLetter::G => Note::new(NoteLetter::F, NoteAccidental::Natural),
                    NoteLetter::A => Note::new(NoteLetter::G, NoteAccidental::Natural),
                    NoteLetter::B => Note::new(NoteLetter::A, NoteAccidental::Natural)
                }
            },
        }
    }
}

impl FromStr for Note {
    type Err = ParseNoteError;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        match item {
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
            "B" => Ok( Note::new(NoteLetter::B, NoteAccidental::Natural) ),
            _ => Err( ParseNoteError::new(item) )
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
        self.get_id().cmp(&other.get_id())
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Note) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Eq for Note {}