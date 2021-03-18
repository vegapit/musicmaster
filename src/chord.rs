use crate::{Note, Interval};

#[derive(Debug, Copy, Clone)]
pub enum ChordQuality{
    Major,
    Minor,
    Diminished,
    Augmented,
    SuspendedTwo,
    SuspendedFour,
    FlatFive,
    SuspendedTwoFlatFive,
    DominantSeventh,
    MajorSeventh,
    MinorSeventh,
    DiminishedSeventh,
    AugmentedSeventh,
    MajorSeventhFlatFive,
    MinorSeventhFlatFive,
    DominantSeventhFlatFive,
    MinorMajorSeventh,
    AugmentedMajorSeventh,
    SuspendedTwoSuspendedFour
}

#[derive(Debug, Clone)]
pub struct Chord{
    pub root_note: Note,
    pub chord_quality: ChordQuality,
    pub intervals: Vec<Interval>
}

impl Chord{

    /// Creates a Scale based on names and intervals
    fn new(root_note: Note, chord_quality: ChordQuality, intervals: Vec<Interval> ) -> Chord {
        Chord{root_note: root_note, chord_quality: chord_quality, intervals: intervals }
    }

    /// Creates the corresponding Chord object
    pub fn factory(chord_root: Note, chord_quality: ChordQuality) -> Chord {
        let interval_names = match chord_quality {
            ChordQuality::Major => vec!["M3","m3","P4"],
            ChordQuality::Minor => vec!["m3","M3","P4"],
            ChordQuality::Diminished => vec!["m3","m3","d5"],
            ChordQuality::Augmented => vec!["M3","M3","M3"],
            ChordQuality::SuspendedTwo => vec!["M2","P4","P4"],
            ChordQuality::SuspendedFour => vec!["P4","M2","P4"],
            ChordQuality::FlatFive => vec!["M3","M2","d5"],
            ChordQuality::SuspendedTwoFlatFive => vec!["M2","M3","d5"],
            ChordQuality::DominantSeventh => vec!["M3","m3","m3","M2"],
            ChordQuality::MajorSeventh => vec!["M3","m3","M3","m2"],
            ChordQuality::MinorSeventh => vec!["m3","M3","m3","M2"],
            ChordQuality::MinorMajorSeventh => vec!["m3","M3","M3","m2"],
            ChordQuality::DiminishedSeventh => vec!["m3","m3","m3","m3"],
            ChordQuality::AugmentedSeventh => vec!["M3","M3","M2","M2"],
            ChordQuality::AugmentedMajorSeventh => vec!["M3","M3","m3","m2"],
            ChordQuality::MajorSeventhFlatFive => vec!["M3","M2","P4","m2"],
            ChordQuality::MinorSeventhFlatFive => vec!["m3","m3","M3","M2"],
            ChordQuality::DominantSeventhFlatFive => vec!["M3","M2","M3","M2"],
            ChordQuality::SuspendedTwoSuspendedFour => vec!["M2","m3","M2","P4"]

        };
        let intervals : Vec<Interval> = interval_names.iter()
            .map(|s| Interval::from_name(s).unwrap() )
            .collect();
        Chord::new(chord_root, chord_quality, intervals)
    }

    /// Get the notes of the Chord
    pub fn get_notes(&self, optimize: bool) -> Vec<Note> { 
        let mut res = vec![ self.root_note.clone() ];
        for i in 0..self.intervals.len()-1 {
            let mut note = self.intervals[i].apply( &res[i] );
            if res[i].letter == note.letter && optimize { 
                if let Some(obj) = note.equivalent() { 
                    note = obj;
                }
            }
            res.push( note );
        }
        res
    }

    /// Get chord Intervals from root note
    pub fn get_root_intervals(&self) -> Vec<Interval> {
        let mut root_intervals : Vec<Interval> = self.intervals.iter().scan(0, |state, x| {
            *state = *state + x.value;
            Some( Interval::new(*state) )
        }).collect();
        root_intervals.insert(0, Interval::new(0));
        root_intervals
    }

    /// Returns how many Chord notes are contained in the Notes provided
    pub fn match_rating(&self, notes: &Vec<Note>) -> usize {
        self.get_notes(false).iter()
            .filter(|&note| notes.iter().any(|elt| elt == note) )
            .count()
    }

    /// Checks if Chord matches all Notes provided
    pub fn matches_notes(&self, notes: &Vec<Note>) -> bool {
        notes.iter().all(|elt1| {
            self.get_notes(false).iter().any(|elt2| elt1 == elt2)
        })
    }

    /// Prints the chord quality
    pub fn print_quality(&self) -> String {
        match self.chord_quality {
            ChordQuality::Major => String::from(""),
            ChordQuality::Minor => String::from("m"),
            ChordQuality::Diminished => String::from("º"),
            ChordQuality::Augmented => String::from("+"),
            ChordQuality::SuspendedTwo => String::from("sus2"),
            ChordQuality::SuspendedFour => String::from("sus4"),
            ChordQuality::FlatFive => String::from("(b5)"),
            ChordQuality::SuspendedTwoFlatFive => String::from("sus2(b5)"),
            ChordQuality::DominantSeventh => String::from("7"),
            ChordQuality::MajorSeventh => String::from("Maj7"),
            ChordQuality::MinorSeventh => String::from("m7"),
            ChordQuality::DiminishedSeventh => String::from("º7"),
            ChordQuality::AugmentedSeventh => String::from("+7"),
            ChordQuality::MajorSeventhFlatFive => String::from("Maj7(b5)"),
            ChordQuality::MinorSeventhFlatFive => String::from("m7(b5)"),
            ChordQuality::DominantSeventhFlatFive => String::from("7(b5)"),
            ChordQuality::MinorMajorSeventh => String::from("mMaj7"),
            ChordQuality::AugmentedMajorSeventh => String::from("+Maj7"),
            ChordQuality::SuspendedTwoSuspendedFour => String::from("sus2sus4")
        }
    }

    /// Prints the chord name e.g. Cm
    pub fn print(&self) -> String {
        let mut res = self.root_note.print();
        res.push_str( self.print_quality().as_str() );
        res
    }

    /// Prints a chord numeral based on a scale root note e.g. Eb from C is IIIb
    pub fn print_numeral(&self, scale_root: &Note, index: usize) -> String {
        let interval_name = Interval::from_notes(scale_root, &self.root_note).print();
        let numeral = match interval_name.as_str() {
            "P1" => "I",
            "m2" => "bII",
            "M2" => match index {
                2 => "bbIII",
                _ => "II"
            },
            "m3" => match index {
                2 => "bIII",
                _ => "#II"
            },
            "M3" => match index {
                3 => "bIV",
                _ => "III"
            },
            "P4" => match index {
                3 => "IV",
                _ => "#III"
            },
            "d5" => match index { 
                3 => "#IV",
                _ => "bV"
            },
            "P5" => match index {
                5 => "bbVI",
                _ => "V"
            },
            "m6" => match index {
                4 => "#V",
                _ => "bVI"
            },
            "M6" => match index {
                6 => "bbVII",
                _ => "VI"
            },
            "m7" => match index {
                5 => "#VI",
                _ => "bVII"
            },
            "M7" => "VII",
            _ => ""
        };
        numeral.to_string()
    }

    /// Find Chord corresponding to vector of Note
    pub fn identify(notes: &Vec<Note>) -> Vec<Chord> {
        let qualities = if notes.len() == 4 {
            vec![
                ChordQuality::DominantSeventh,
                ChordQuality::MajorSeventh,
                ChordQuality::MinorSeventh,
                ChordQuality::DiminishedSeventh,
                ChordQuality::AugmentedSeventh,
                ChordQuality::MajorSeventhFlatFive,
                ChordQuality::MinorSeventhFlatFive,
                ChordQuality::DominantSeventhFlatFive,
                ChordQuality::MinorMajorSeventh,
                ChordQuality::AugmentedMajorSeventh,
                ChordQuality::SuspendedTwoSuspendedFour
            ]
        } else if notes.len() == 3 {
            vec![
                ChordQuality::Major,
                ChordQuality::Minor,
                ChordQuality::Diminished,
                ChordQuality::Augmented,
                ChordQuality::SuspendedTwo,
                ChordQuality::SuspendedFour,
                ChordQuality::FlatFive,
                ChordQuality::SuspendedTwoFlatFive
            ]
        } else {
            return vec![]
        };
        let mut res : Vec<Chord> = Vec::new();
        for quality in qualities {
            for note in notes {
                let chord = Chord::factory( note.clone(), quality.clone() );
                if chord.matches_notes( notes ) {
                    res.push( chord );
                }
            }
        }
        res
    }
}

impl PartialEq for Chord {
    fn eq(&self, other: &Chord) -> bool {
        other.get_notes(false).iter()
            .zip(self.get_notes(false).iter())
            .all(|elt| elt.0 == elt.1 )
    }
}

impl Eq for Chord {}