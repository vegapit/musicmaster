use std::collections::HashMap;
use std::str::FromStr;
use crate::{Note, Interval, NoteLetter, NoteAccidental, Chord};

#[derive(Debug)]
pub struct ParseScaleQualityError {
    details: String
}

impl ParseScaleQualityError {
    fn new(msg: &str) -> Self {
        ParseScaleQualityError{ details: format!("Could not convert {} to a ScaleQuality", msg) }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScaleQuality{
    Major,
    MelodicMinor,
    HarmonicMajor,
    HarmonicMinor,
    DoubleHarmonic,
    Diminished,
    WholeTone,
    Chromatic
}

impl FromStr for ScaleQuality {
    type Err = ParseScaleQualityError;

    fn from_str(item: &str) -> Result<Self, Self::Err> {
        match item {
            "Major" => Ok( Self::Major ),
            "MelodicMinor" => Ok( Self::MelodicMinor ),
            "HarmonicMajor" => Ok( Self::HarmonicMajor ),
            "HarmonicMinor" => Ok( Self::HarmonicMinor ),
            "DoubleHarmonic" => Ok( Self::DoubleHarmonic ),
            "Diminished" => Ok( Self::Diminished ),
            "WholeTone" => Ok( Self::WholeTone ),
            "Chromatic" => Ok( Self::Chromatic ),
            _ => Err( ParseScaleQualityError::new(item) )
        }
    }
}


#[derive(Debug, Clone)]
pub struct Scale{
    pub root_note: Note,
    pub scale_quality: ScaleQuality,
    pub mode_degree: usize,
    pub intervals: Vec<Interval>
}

/// Represents a musical Scale
impl Scale {
    
    /// Creates a Scale based on names and intervals
    fn new(root_note: Note, scale_quality: ScaleQuality, mode_degree: usize, intervals: Vec<Interval> ) -> Scale {
        Scale{ root_note: root_note, scale_quality: scale_quality, mode_degree: mode_degree, intervals: intervals }
    }

    /// Creates the corresponding Scale object
    pub fn factory(root_note: Note, scale_quality: ScaleQuality, mode_degree: usize) -> Scale {
        let interval_names : Vec<&str> = match scale_quality {
            ScaleQuality::Major => vec!["M2","M2","m2","M2","M2","M2","m2"],
            ScaleQuality::MelodicMinor => vec!["M2","m2","M2","M2","M2","M2","m2"],
            ScaleQuality::HarmonicMinor => vec!["M2","m2","M2","M2","m2","m3","m2"],
            ScaleQuality::Diminished => vec!["M2","m2","M2","m2","M2","m2","M2","m2"],
            ScaleQuality::WholeTone => std::iter::repeat("M2").take(6).collect(),
            ScaleQuality::HarmonicMajor => vec!["M2","M2","m2","M2","m2","m3","m2"],
            ScaleQuality::DoubleHarmonic => vec!["m2","m3","m2","M2","m2","m3","m2"],
            ScaleQuality::Chromatic => std::iter::repeat("m2").take(10).collect()
        };
        let mode_intervals = interval_names.iter()
            .cycle()
            .skip( mode_degree )
            .take( interval_names.len() )
            .map(|s| Interval::from_name(s).unwrap() )
            .collect();
        Scale::new(root_note, scale_quality, mode_degree, mode_intervals)
    }

    /// Find Scales containing all notes provided
    pub fn identify(notes: &Vec<Note>) -> Vec<Scale> {
        let qualities = vec![
            ScaleQuality::Major,
            ScaleQuality::HarmonicMinor,
            ScaleQuality::HarmonicMajor,
            ScaleQuality::MelodicMinor,
            ScaleQuality::DoubleHarmonic,
            ScaleQuality::Diminished
        ];
        let mut res : Vec<Scale> = Vec::new();
        for quality in &qualities {
            for note in Scale::factory( Note::new(NoteLetter::C, NoteAccidental::Natural),ScaleQuality::Chromatic, 0).get_notes(false) {
                let scale = Scale::factory(note, quality.clone(), 0);
                if scale.matches_notes( &notes ) {
                    res.push( scale );
                }
            }
        }
        res
    }

    /// Print the Scale identifier e.g. Db Phrygian
    pub fn print(&self) -> String {
        let mut res = self.root_note.print();
        res.push_str(" ");
        res.push_str( self.print_quality().as_str() );
        res
    }

    /// Print the Scale quality e.g. Phrygian
    pub fn print_quality(&self) -> String {
        let mode_names : Vec<&str> = match self.scale_quality {
            ScaleQuality::Major => vec!["Major","Dorian","Phrygian","Lydian","Mixolydian","Aeolian","Locrian"],
            ScaleQuality::MelodicMinor => vec!["Melodic Minor","Assyrian","Lydian Augmented","Overtone","Hindu","Half-Diminished","Altered"],
            ScaleQuality::HarmonicMinor => vec!["Harmonic Minor","Locrian #6","Ionian #5","Ukrainian Dorian","Phrygian Dominant","Lydian #2","Super Locrian bb7"],
            ScaleQuality::Diminished => vec!["Diminished","Dominant Diminished","Diminished","Dominant Diminished","Diminished","Dominant Diminished","Diminished","Dominant Diminished"],
            ScaleQuality::WholeTone => std::iter::repeat("Whole Tone").take(6).collect(),
            ScaleQuality::HarmonicMajor => vec!["Harmonic Major","Dorian b5","Phrygian b4","Lydian b3","Mixolydian b2","Lydian Aug #2","Locrian bb7"],
            ScaleQuality::DoubleHarmonic => vec!["Double Harmonic","Lydian #2 #6","UltraPhrygian","Hungarian Minor","Oriental","Ionian Aug #2","Locrian bb3 bb7"],
            ScaleQuality::Chromatic => std::iter::repeat("Chromatic").take(11).collect()
        };
        mode_names[self.mode_degree].to_string()
    }

    /// Prints the numeral of the Chord in the Scale if it exists
    pub fn print_chord_numeral(&self, chord: &Chord) -> Option<String> {
        if self.matches_notes( &chord.get_notes(false) ) {
            let index = self.get_note_index( &chord.root_note ).unwrap();
            return Some( chord.print_numeral( &self.root_note, index ) );
        }
        None
    }

    /// Get the notes of the scale
    pub fn get_notes(&self, optimize: bool) -> Vec<Note> { 
        let mut res = vec![ self.root_note.clone() ];
        for i in 0..self.intervals.len()-1 {
            let mut note = self.intervals[i].apply(&res[i]);
            if res[i].letter == note.letter && optimize { 
                if let Some(obj) = note.equivalent() { 
                    note = obj;
                }
            }
            res.push( note );
        }
        res
    }

    /// Get scale Intervals from root note
    pub fn get_root_intervals(&self) -> Vec<Interval> {
        let mut root_intervals : Vec<Interval> = self.intervals.iter().scan(0, |state, x| {
            *state = *state + x.value;
            Some( Interval::new(*state) )
        }).collect();
        root_intervals.insert(0, Interval::new(0));
        root_intervals
    }

    /// Get the chords of the scale
    pub fn get_chords(&self, extended: bool) -> Vec<Chord> {
        let notes = self.get_notes(true);
        let mut res = Vec::<Chord>::new();
        for i in 0..notes.len() {
            let first_note = notes[i].clone();

            let second_note = if i + 2 < notes.len() {
                notes[i + 2].clone()
            } else {
                notes[i + 2 - notes.len()].clone()
            };

            let third_note = if i + 4 < notes.len() {
                notes[i + 4].clone()
            } else {
                notes[i + 4 - notes.len()].clone()
            };

            if !extended {
                let triad = Chord::identify( &vec![first_note, second_note, third_note] )[0].clone();
                res.push( triad );
            } else {
                let fourth_note = if i + 6 < notes.len() {
                    notes[i + 6].clone()
                } else {
                    notes[i + 6 - notes.len()].clone()
                };
                let chord = Chord::identify( &vec![first_note, second_note, third_note, fourth_note] )[0].clone();
                res.push( chord );
            }
        }
        res
    }

    /// Returns how many Scale notes are not contained in the Notes provided
    pub fn match_rating(&self, notes: &Vec<Note>) -> usize {
        let mut res : usize = self.get_notes(false).len();
        for elt1 in &self.get_notes(false) {
            if notes.iter().any(|elt2| elt2 == elt1) {
                res -= 1;
            }
        }  
        res
    }

    /// Checks if Scale contains all Notes provided
    pub fn matches_notes(&self, notes: &Vec<Note>) -> bool {
        notes.iter().all(|elt1| {
            self.get_notes(false).iter().any(|elt2| elt2 == elt1)
        })
    }

    /// Get Scales at proximity of the scale
    pub fn proximity_scales(&self) -> HashMap<usize,Vec<Scale>> {
        let qualities = vec![
            ScaleQuality::Major,
            ScaleQuality::HarmonicMinor,
            ScaleQuality::HarmonicMajor,
            ScaleQuality::MelodicMinor,
            ScaleQuality::DoubleHarmonic,
            ScaleQuality::Diminished,
            ScaleQuality::WholeTone
        ];
        let mut res = HashMap::<usize,Vec<Scale>>::new();
        for quality in &qualities {
            for note in Scale::factory( Note::new(NoteLetter::C, NoteAccidental::Natural),ScaleQuality::Chromatic, 0).get_notes(false) {
                let scale = Scale::factory(note, quality.clone(), 0);
                let rating = self.match_rating( &scale.get_notes(false) );
                if rating < 4 && rating > 0  {
                    let scales = vec![ scale.clone() ];
                    res.entry( rating ).and_modify(|elt| {
                        elt.push( scale )
                    }).or_insert( scales );
                }
            }
        }

        res
    }

    /// Move the root of the Scale if possible
    pub fn root_shift(&self, root: &Note) -> Option<Scale> {
        if let Some(index) = self.get_note_index( root ) {
            let new_scale = Scale::factory( root.clone(), self.scale_quality.clone(), index);
            Some( new_scale )
        } else {
            None
        }
    }

    fn get_note_index(&self, note: &Note) -> Option<usize> {
        let scale_notes = self.get_notes(false);
        for elt in scale_notes.iter().enumerate() {
            if elt.1 == note {
                return Some( elt.0 );
            }
        }
        None
    }
}

