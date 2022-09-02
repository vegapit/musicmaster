use std::fmt;
use std::convert::TryFrom;
use crate::{Note, Interval, Chord, next_note_letter};

#[derive(Debug, PartialEq, Clone)]
pub enum Scale{
    Major,
    MelodicMinor,
    HarmonicMajor,
    HarmonicMinor,
    DoubleHarmonic,
    Diminished,
    WholeTone,
    Chromatic
}

pub fn get_mode_names(scale: &Scale) -> Vec<&'static str> {
    match scale {
        Scale::Major => vec!["Major","Dorian","Phrygian","Lydian","Mixolydian","Aeolian","Locrian"],
        Scale::MelodicMinor => vec!["Melodic Minor","Assyrian","Lydian Augmented","Overtone","Hindu","Half-Diminished","Altered"],
        Scale::HarmonicMinor => vec!["Harmonic Minor","Locrian #6","Ionian #5","Ukrainian Dorian","Phrygian Dominant","Lydian #2","Super Locrian bb7"],
        Scale::Diminished => vec!["Diminished","Dominant Diminished"],
        Scale::WholeTone => vec!["Whole Tone"],
        Scale::HarmonicMajor => vec!["Harmonic Major","Dorian b5","Phrygian b4","Lydian b3","Mixolydian b2","Lydian Aug #2","Locrian bb7"],
        Scale::DoubleHarmonic => vec!["Double Harmonic","Lydian #2 #6","UltraPhrygian","Hungarian Minor","Oriental","Ionian Aug #2","Locrian bb3 bb7"],
        Scale::Chromatic => vec!["Chromatic"]
    }
}

pub fn can_be_optimised(scale: &Scale) -> bool {
    match scale {
        Scale::Major | Scale::MelodicMinor |  Scale::HarmonicMajor | Scale::HarmonicMinor => true,
        _ => false
    }
}

impl TryFrom<&str> for Scale {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Major" => Ok( Self::Major ),
            "MelodicMinor" => Ok( Self::MelodicMinor ),
            "HarmonicMajor" => Ok( Self::HarmonicMajor ),
            "HarmonicMinor" => Ok( Self::HarmonicMinor ),
            "DoubleHarmonic" => Ok( Self::DoubleHarmonic ),
            "Diminished" => Ok( Self::Diminished ),
            "WholeTone" => Ok( Self::WholeTone ),
            "Chromatic" => Ok( Self::Chromatic ),
            _ => Err("No Scale found by this name")
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Major => write!(f, "Major"),
            Self::MelodicMinor => write!(f, "MelodicMinor"),
            Self::HarmonicMajor => write!(f, "HarmonicMajor"),
            Self::HarmonicMinor => write!(f, "HarmonicMinor"),
            Self::DoubleHarmonic => write!(f, "DoubleHarmonic"),
            Self::Diminished => write!(f, "Diminished"),
            Self::WholeTone => write!(f, "WholeTone"),
            Self::Chromatic => write!(f, "Chromatic")
        }
    }
}

pub fn all_scales() -> Vec<Scale> {
    vec![
        Scale::Major,
        Scale::MelodicMinor,
        Scale::HarmonicMajor,
        Scale::HarmonicMinor,
        Scale::DoubleHarmonic,
        Scale::Diminished,
        Scale::WholeTone,
        Scale::Chromatic
    ]
}

#[derive(Debug, Clone)]
pub struct Mode {
    root_note: Note,
    scale: Scale,
    degree: usize
}

/// Represents a musical Scale
impl Mode {
    
    /// Creates a Scale based on names and intervals
    pub fn new(root_note: Note, scale: Scale, degree: usize ) -> Self {
        Mode{ root_note: root_note, scale: scale, degree: degree }
    }

    /// Get note intervals
    pub fn get_intervals(&self) -> Vec<Interval> {
        let interval_names : Vec<&str> = match self.scale {
            Scale::Major => vec!["M2","M2","m2","M2","M2","M2","m2"],
            Scale::MelodicMinor => vec!["M2","m2","M2","M2","M2","M2","m2"],
            Scale::HarmonicMinor => vec!["M2","m2","M2","M2","m2","m3","m2"],
            Scale::Diminished => vec!["M2","m2","M2","m2","M2","m2","M2","m2"],
            Scale::WholeTone => std::iter::repeat("M2").take(6).collect(),
            Scale::HarmonicMajor => vec!["M2","M2","m2","M2","m2","m3","m2"],
            Scale::DoubleHarmonic => vec!["m2","m3","m2","M2","m2","m3","m2"],
            Scale::Chromatic => std::iter::repeat("m2").take(10).collect()
        };
        let intervals : Vec<Interval> = interval_names.iter()
            .cycle()
            .skip( self.degree )
            .take( interval_names.len() )
            .map(|s| Interval::from_name(s).unwrap() )
            .collect();
        intervals
    }

    /// Find Scales containing all notes provided
    pub fn identify(notes: &Vec<Note>) -> Vec<Self> {
        let mut res : Vec<Self> = Vec::new();
        for scale in all_scales().into_iter() {
            let num_modes = get_mode_names( &scale ).len();
            for degree in 0..num_modes {
                for note in notes.iter().skip(1) {
                    let mode = Self::new(note.clone(), scale.clone(), degree);
                    if mode.get_notes().iter().zip( notes.iter() ).all(|elt| elt.0 == elt.1) {
                        res.push( mode );
                    }
                }
            }
        }
        res
    }

    /// Prints the numeral of the Chord in the Scale if it exists
    pub fn chord_numeral(&self, chord: &Chord) -> Option<String> {
        if self.contains_notes( &chord.get_notes() ) {
            let index = self.get_note_index( &chord.get_root() ).unwrap();
            return Some( chord.as_numeral( &self.root_note, index ) );
        }
        None
    }

    /// Get the notes of the scale
    pub fn get_notes(&self) -> Vec<Note> {
        let intervals = self.get_intervals(); 
        let mut res = vec![ self.root_note.clone() ];
        if can_be_optimised(&self.scale) {
            let mut target_note_letter = next_note_letter( &self.root_note.get_letter() );
            for i in 0..intervals.len()-1 {
                let note = intervals[i].apply( &res[i] );
                if target_note_letter == note.get_letter() {
                    res.push( note );
                    target_note_letter = next_note_letter( &note.get_letter() );
                } else {
                    for eq_note in note.equivalents().into_iter() {
                        if target_note_letter == eq_note.get_letter() {
                            res.push( eq_note );
                            target_note_letter = next_note_letter( &eq_note.get_letter() );
                            break;
                        }
                    }
                }
            }
        } else {
            for i in 0..intervals.len()-1 {
                let note = intervals[i].apply( &res[i] );
                res.push( note );
            }
        }
        res
    }

    /// Get scale Intervals from root note
    pub fn get_root_intervals(&self) -> Vec<Interval> {
        let mut root_intervals : Vec<Interval> = self.get_intervals().iter()
            .scan(0, |state, x| {
                *state = *state + x.get_value();
                Some( Interval::new(*state) )
            }).collect();
        root_intervals.insert(0, Interval::new(0));
        root_intervals
    }

    /// Get the chords of the scale
    pub fn get_chords(&self, extended: bool) -> Vec<Option<Chord>> {
        let notes = self.get_notes();
        let notes_cycle : Vec<Note> = notes.iter()
            .cycle()
            .take(50)
            .cloned()
            .collect();
        let mut res = Vec::<Option<Chord>>::new();
        for i in 0..notes.len() {
            let first_note = notes_cycle[i].clone();
            let second_note = notes_cycle[i + 2].clone();
            let third_note = notes_cycle[i + 4].clone();
            if !extended {
                let triads = Chord::identify( &vec![first_note, second_note, third_note] );
                if triads.len() > 0 {
                    res.push( Some(triads[0].clone()) );
                } else {
                    res.push( None );
                }
            } else {
                let fourth_note = notes_cycle[i + 6].clone();

                let chords = if vec![first_note, second_note, third_note].iter().find(|&&note| note == fourth_note).is_none() {
                    Chord::identify( &vec![first_note, second_note, third_note, fourth_note] )
                } else {
                    Chord::identify( &vec![first_note, second_note, third_note] )
                };
                if chords.len() > 0 {
                    res.push( Some(chords[0].clone()) );
                } else {
                    res.push( None );
                }
            }
        }
        res
    }

    /// Checks if Scale contains all Notes provided
    pub fn contains_notes(&self, notes: &Vec<Note>) -> bool {
        notes.iter().all(|elt1| {
            self.get_notes().iter().any(|elt2| elt2 == elt1)
        })
    }

    /// Get position of note in the mode
    fn get_note_index(&self, note: &Note) -> Option<usize> {
        let scale_notes = self.get_notes();
        for elt in scale_notes.iter().enumerate() {
            if elt.1 == note {
                return Some( elt.0 );
            }
        }
        None
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.root_note, get_mode_names(&self.scale)[self.degree])
    }
}