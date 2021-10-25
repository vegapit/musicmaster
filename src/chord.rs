use std::fmt;
use std::convert::TryFrom;

use crate::{Note, Interval};

#[derive(Debug, Copy, Clone)]
pub enum ChordPosition {
    Root,
    FirstInversion,
    SecondInversion,
    ThirdInversion
}

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

impl TryFrom<&Vec<Interval>> for ChordQuality {
    type Error = &'static str;
    fn try_from(value: &Vec<Interval>) -> Result<Self, Self::Error> {
        let intervals_by_value : Vec<u32> = value.into_iter()
            .map(|elt| elt.get_value() )
            .collect();
        match intervals_by_value.as_slice() {
            [4,3,5] => Ok(Self::Major),
            [3,4,5] => Ok(Self::Minor),
            [3,3,6] => Ok(Self::Diminished),
            [4,4,4] => Ok(Self::Augmented),
            [2,5,5] => Ok(Self::SuspendedTwo),
            [5,2,5] => Ok(Self::SuspendedFour),
            [4,2,6] => Ok(Self::FlatFive),
            [2,4,6] => Ok(Self::SuspendedTwoFlatFive),
            [4,3,3,2] => Ok(Self::DominantSeventh),
            [4,3,4,1] => Ok(Self::MajorSeventh),
            [3,4,3,2] => Ok(Self::MinorSeventh),
            [3,4,4,1] => Ok(Self::MinorMajorSeventh),
            [3,3,3,3] => Ok(Self::DiminishedSeventh),
            [4,4,2,2] => Ok(Self::AugmentedSeventh),
            [4,4,3,1] => Ok(Self::AugmentedMajorSeventh),
            [4,2,5,1] => Ok(Self::MajorSeventhFlatFive),
            [3,3,4,2] => Ok(Self::MinorSeventhFlatFive),
            [4,2,4,2] => Ok(Self::DominantSeventhFlatFive),
            [2,3,2,5] => Ok(Self::SuspendedTwoSuspendedFour), 
            _ => Err("No ChordQuality found for these intervals")
        }
    }
}

impl fmt::Display for ChordQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Major => write!(f, ""),
            Self::Minor => write!(f, "m"),
            Self::Diminished => write!(f, "º"),
            Self::Augmented => write!(f, "+"),
            Self::SuspendedTwo => write!(f, "sus2"),
            Self::SuspendedFour => write!(f, "sus4"),
            Self::FlatFive => write!(f, "(b5)"),
            Self::SuspendedTwoFlatFive => write!(f, "sus2(b5)"),
            Self::DominantSeventh => write!(f, "7"),
            Self::MajorSeventh => write!(f, "Maj7"),
            Self::MinorSeventh => write!(f, "m7"),
            Self::DiminishedSeventh => write!(f, "º7"),
            Self::AugmentedSeventh => write!(f, "+7"),
            Self::MajorSeventhFlatFive => write!(f, "Maj7(b5)"),
            Self::MinorSeventhFlatFive => write!(f, "m7(b5)"),
            Self::DominantSeventhFlatFive => write!(f, "7(b5)"),
            Self::MinorMajorSeventh => write!(f, "mMaj7"),
            Self::AugmentedMajorSeventh => write!(f, "+Maj7"),
            Self::SuspendedTwoSuspendedFour => write!(f, "sus2sus4"),   
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chord{
    root_note: Note,
    quality: ChordQuality,
    position: ChordPosition
}

impl Chord{

    /// Creates a Chord from notes
    pub fn new(root_note: Note, quality: ChordQuality, position: ChordPosition) -> Chord {
        Chord{root_note, quality, position}
    }

    pub fn from_intervals(root_note: Note, intervals: &Vec<Interval>) -> Option<Chord> {
        let intervals_by_value : Vec<u32> = intervals.iter()
            .map(|elt| elt.get_value() )
            .collect();
        match intervals_by_value.as_slice() {
            // TRIADS
            // Major
            [4,3] => Some( Chord::new(root_note, ChordQuality::Major, ChordPosition::Root) ),
            [3,5] => Some( Chord::new(root_note, ChordQuality::Major, ChordPosition::FirstInversion) ),
            [5,4] => Some( Chord::new(root_note, ChordQuality::Major, ChordPosition::SecondInversion) ),
            // Minor
            [3,4] => Some( Chord::new(root_note, ChordQuality::Minor, ChordPosition::Root) ),
            [4,5] => Some( Chord::new(root_note, ChordQuality::Minor, ChordPosition::FirstInversion) ),
            [5,3] => Some( Chord::new(root_note, ChordQuality::Minor, ChordPosition::SecondInversion) ),
            // Diminished
            [3,3] => Some( Chord::new(root_note, ChordQuality::Diminished, ChordPosition::Root) ),
            [3,6] => Some( Chord::new(root_note, ChordQuality::Diminished, ChordPosition::FirstInversion) ),
            [6,3] => Some( Chord::new(root_note, ChordQuality::Diminished, ChordPosition::SecondInversion) ),
            // Augmented
            [4,4] => Some( Chord::new(root_note, ChordQuality::Augmented, ChordPosition::Root) ), 
            // Suspended2
            [2,5] => Some( Chord::new(root_note, ChordQuality::SuspendedTwo, ChordPosition::Root) ),
            [5,5] => Some( Chord::new(root_note, ChordQuality::SuspendedTwo, ChordPosition::FirstInversion) ),
            // Suspended4
            [5,2] => Some( Chord::new(root_note, ChordQuality::SuspendedFour, ChordPosition::Root) ),
            // Flat5
            [4,2] => Some( Chord::new(root_note, ChordQuality::FlatFive, ChordPosition::Root) ),
            [2,6] => Some( Chord::new(root_note, ChordQuality::FlatFive, ChordPosition::FirstInversion) ),
            [6,4] => Some( Chord::new(root_note, ChordQuality::FlatFive, ChordPosition::SecondInversion) ),
            // supended2Flat5
            [2,4] => Some( Chord::new(root_note, ChordQuality::SuspendedTwoFlatFive, ChordPosition::Root) ),
            [4,6] => Some( Chord::new(root_note, ChordQuality::SuspendedTwoFlatFive, ChordPosition::FirstInversion) ),
            [6,2] => Some( Chord::new(root_note, ChordQuality::SuspendedTwoFlatFive, ChordPosition::SecondInversion) ),
            // EXTENDED CHORDS
            // Dominant7
            [4,3,3] => Some( Chord::new(root_note, ChordQuality::DominantSeventh, ChordPosition::Root) ),
            [3,3,2] => Some( Chord::new(root_note, ChordQuality::DominantSeventh, ChordPosition::FirstInversion) ),
            [3,2,4] => Some( Chord::new(root_note, ChordQuality::DominantSeventh, ChordPosition::SecondInversion) ),
            [2,4,3] => Some( Chord::new(root_note, ChordQuality::DominantSeventh, ChordPosition::ThirdInversion) ),
            // Major7
            [4,3,4] => Some( Chord::new(root_note, ChordQuality::MajorSeventh, ChordPosition::Root) ),
            [3,4,1] => Some( Chord::new(root_note, ChordQuality::MajorSeventh, ChordPosition::FirstInversion) ),
            [4,1,4] => Some( Chord::new(root_note, ChordQuality::MajorSeventh, ChordPosition::SecondInversion) ),
            [1,4,3] => Some( Chord::new(root_note, ChordQuality::MajorSeventh, ChordPosition::ThirdInversion) ),
            // Minor7
            [3,4,3] => Some( Chord::new(root_note, ChordQuality::MinorSeventh, ChordPosition::Root) ),
            [4,3,2] => Some( Chord::new(root_note, ChordQuality::MinorSeventh, ChordPosition::FirstInversion) ),
            [3,2,3] => Some( Chord::new(root_note, ChordQuality::MinorSeventh, ChordPosition::SecondInversion) ),
            [2,3,4] => Some( Chord::new(root_note, ChordQuality::MinorSeventh, ChordPosition::ThirdInversion) ),
            // MinorMajor7
            [3,4,4] => Some( Chord::new(root_note, ChordQuality::MinorSeventh, ChordPosition::Root) ),
            [4,4,1] => Some( Chord::new(root_note, ChordQuality::MinorSeventh, ChordPosition::FirstInversion) ),
            [4,1,3] => Some( Chord::new(root_note, ChordQuality::MinorSeventh, ChordPosition::SecondInversion) ),
            [1,3,4] => Some( Chord::new(root_note, ChordQuality::MinorSeventh, ChordPosition::ThirdInversion) ),
            // Diminshed7
            [3,3,3] => Some( Chord::new(root_note, ChordQuality::DiminishedSeventh, ChordPosition::Root) ),
            // Augmented7
            [4,4,2] => Some( Chord::new(root_note, ChordQuality::AugmentedSeventh, ChordPosition::Root) ),
            [4,2,2] => Some( Chord::new(root_note, ChordQuality::AugmentedSeventh, ChordPosition::FirstInversion) ),
            [2,2,4] => Some( Chord::new(root_note, ChordQuality::AugmentedSeventh, ChordPosition::SecondInversion) ),
            [2,4,4] => Some( Chord::new(root_note, ChordQuality::AugmentedSeventh, ChordPosition::ThirdInversion) ),
            // AugmentedMajor7
            [4,4,3] => Some( Chord::new(root_note, ChordQuality::AugmentedMajorSeventh, ChordPosition::Root) ),
            [4,3,1] => Some( Chord::new(root_note, ChordQuality::AugmentedMajorSeventh, ChordPosition::FirstInversion) ),
            [3,1,4] => Some( Chord::new(root_note, ChordQuality::AugmentedMajorSeventh, ChordPosition::SecondInversion) ),
            [1,4,4] => Some( Chord::new(root_note, ChordQuality::AugmentedMajorSeventh, ChordPosition::ThirdInversion) ),
            // Major7Flat5
            [4,2,5] => Some( Chord::new(root_note, ChordQuality::MajorSeventhFlatFive, ChordPosition::Root) ),
            [2,5,1] => Some( Chord::new(root_note, ChordQuality::MajorSeventhFlatFive, ChordPosition::FirstInversion) ),
            [5,1,4] => Some( Chord::new(root_note, ChordQuality::MajorSeventhFlatFive, ChordPosition::SecondInversion) ),
            [1,4,2] => Some( Chord::new(root_note, ChordQuality::MajorSeventhFlatFive, ChordPosition::ThirdInversion) ),
            // Minor7Flat5
            [3,3,4] => Some( Chord::new(root_note, ChordQuality::MinorSeventhFlatFive, ChordPosition::Root) ),
            [3,4,2] => Some( Chord::new(root_note, ChordQuality::MinorSeventhFlatFive, ChordPosition::FirstInversion) ),
            [4,2,3] => Some( Chord::new(root_note, ChordQuality::MinorSeventhFlatFive, ChordPosition::SecondInversion) ),
            [2,3,3] => Some( Chord::new(root_note, ChordQuality::MinorSeventhFlatFive, ChordPosition::ThirdInversion) ),
            // Dominant7Flat5
            [4,2,4] => Some( Chord::new(root_note, ChordQuality::DominantSeventhFlatFive, ChordPosition::Root) ),
            [2,4,2] => Some( Chord::new(root_note, ChordQuality::DominantSeventhFlatFive, ChordPosition::FirstInversion) ),
            // Supended2Suspended4
            [2,3,2] => Some( Chord::new(root_note, ChordQuality::SuspendedTwoSuspendedFour, ChordPosition::Root) ),
            [3,2,5] => Some( Chord::new(root_note, ChordQuality::SuspendedTwoSuspendedFour, ChordPosition::FirstInversion) ),
            [2,5,2] => Some( Chord::new(root_note, ChordQuality::SuspendedTwoSuspendedFour, ChordPosition::SecondInversion) ),
            [5,2,3] => Some( Chord::new(root_note, ChordQuality::SuspendedTwoSuspendedFour, ChordPosition::ThirdInversion) ),
            _ => None
        }
    }

    pub fn get_root(&self) -> Note { self.root_note }
    pub fn get_quality(&self) -> ChordQuality { self.quality }

    pub fn get_intervals(&self) -> Vec<Interval> {
        let interval_names = match (self.quality,self.position) {
            // Major
            (ChordQuality::Major, ChordPosition::Root) => vec!["M3","m3"],
            (ChordQuality::Major, ChordPosition::FirstInversion) => vec!["m3","P4"],
            (ChordQuality::Major, ChordPosition::SecondInversion) => vec!["P4","M3"],
            // Minor
            (ChordQuality::Minor, ChordPosition::Root) => vec!["m3","M3"],
            (ChordQuality::Minor, ChordPosition::FirstInversion) => vec!["M3","P4"],
            (ChordQuality::Minor, ChordPosition::SecondInversion) => vec!["P4","m3"],
            // Diminished
            (ChordQuality::Diminished, ChordPosition::Root) => vec!["m3","m3"],
            (ChordQuality::Diminished, ChordPosition::FirstInversion) => vec!["m3","d5"],
            (ChordQuality::Diminished, ChordPosition::SecondInversion) => vec!["d5","m3"],
            // Augmented
            (ChordQuality::Augmented, ChordPosition::Root) => vec!["M3","M3"],
            // Suspended2
            (ChordQuality::SuspendedTwo, ChordPosition::Root) => vec!["M2","P4"],
            (ChordQuality::SuspendedTwo, ChordPosition::FirstInversion) => vec!["P4","P4"],
            // Suspended4
            (ChordQuality::SuspendedFour, ChordPosition::Root) => vec!["P4","M2"],
            // Flat5
            (ChordQuality::FlatFive, ChordPosition::Root) => vec!["M3","M2"],
            (ChordQuality::FlatFive, ChordPosition::FirstInversion) => vec!["M2","d5"],
            (ChordQuality::FlatFive, ChordPosition::SecondInversion) => vec!["d5","M3"],
            // Suspended2Flat5
            (ChordQuality::SuspendedTwoFlatFive, ChordPosition::Root) => vec!["M2","M3"],
            (ChordQuality::SuspendedTwoFlatFive, ChordPosition::FirstInversion) => vec!["M3","d5"],
            (ChordQuality::SuspendedTwoFlatFive, ChordPosition::SecondInversion) => vec!["d5","M2"],
            // Dominant7
            (ChordQuality::DominantSeventh, ChordPosition::Root) => vec!["M3","m3","m3"],
            (ChordQuality::DominantSeventh, ChordPosition::FirstInversion) => vec!["m3","m3","M2"],
            (ChordQuality::DominantSeventh, ChordPosition::SecondInversion) => vec!["m3","M2","M3"],
            (ChordQuality::DominantSeventh, ChordPosition::ThirdInversion) => vec!["M2","M3","m3"],
            // Major7
            (ChordQuality::MajorSeventh, ChordPosition::Root) => vec!["M3","m3","M3"],
            (ChordQuality::MajorSeventh, ChordPosition::FirstInversion) => vec!["m3","M3","m2"],
            (ChordQuality::MajorSeventh, ChordPosition::SecondInversion) => vec!["M3","m2","M3"],
            (ChordQuality::MajorSeventh, ChordPosition::ThirdInversion) => vec!["m2","M3","m3"],
            // Minor7
            (ChordQuality::MinorSeventh, ChordPosition::Root) => vec!["m3","M3","m3"],
            (ChordQuality::MinorSeventh, ChordPosition::FirstInversion) => vec!["M3","m3","M2"],
            (ChordQuality::MinorSeventh, ChordPosition::SecondInversion) => vec!["m3","M2","m3"],
            (ChordQuality::MinorSeventh, ChordPosition::ThirdInversion) => vec!["M2","m3","M3"],
            // MinorMajor7
            (ChordQuality::MinorMajorSeventh, ChordPosition::Root) => vec!["m3","M3","M3"],
            (ChordQuality::MinorMajorSeventh, ChordPosition::FirstInversion) => vec!["M3","M3","m2"],
            (ChordQuality::MinorMajorSeventh, ChordPosition::SecondInversion) => vec!["M3","m2","m3"],
            (ChordQuality::MinorMajorSeventh, ChordPosition::ThirdInversion) => vec!["m2","m3","M3"],
            // Diminished7
            (ChordQuality::DiminishedSeventh, ChordPosition::Root) => vec!["m3","m3","m3"],
            // Augmented7
            (ChordQuality::AugmentedSeventh, ChordPosition::Root) => vec!["M3","M3","M2"],
            (ChordQuality::AugmentedSeventh, ChordPosition::FirstInversion) => vec!["M3","M2","M2"],
            (ChordQuality::AugmentedSeventh, ChordPosition::SecondInversion) => vec!["M2","M2","M3"],
            (ChordQuality::AugmentedSeventh, ChordPosition::ThirdInversion) => vec!["M2","M3","M3"],
            // AugmentedMajor7
            (ChordQuality::AugmentedMajorSeventh, ChordPosition::Root) => vec!["M3","M3","m3"],
            (ChordQuality::AugmentedMajorSeventh, ChordPosition::FirstInversion) => vec!["M3","m3","m2"],
            (ChordQuality::AugmentedMajorSeventh, ChordPosition::SecondInversion) => vec!["m3","m2","M3"],
            (ChordQuality::AugmentedMajorSeventh, ChordPosition::ThirdInversion) => vec!["m2","M3","M3"],
            // Major7Flat5
            (ChordQuality::MajorSeventhFlatFive, ChordPosition::Root) => vec!["M3","M2","P4"],
            (ChordQuality::MajorSeventhFlatFive, ChordPosition::FirstInversion) => vec!["M2","P4","m2"],
            (ChordQuality::MajorSeventhFlatFive, ChordPosition::SecondInversion) => vec!["P4","m2","M3"],
            (ChordQuality::MajorSeventhFlatFive, ChordPosition::ThirdInversion) => vec!["m2","M3","M2"],
            // Minor7Flat5
            (ChordQuality::MinorSeventhFlatFive, ChordPosition::Root) => vec!["m3","m3","M3"],
            (ChordQuality::MinorSeventhFlatFive, ChordPosition::FirstInversion) => vec!["m3","M3","M2"],
            (ChordQuality::MinorSeventhFlatFive, ChordPosition::SecondInversion) => vec!["M3","M2","m3"],
            (ChordQuality::MinorSeventhFlatFive, ChordPosition::ThirdInversion) => vec!["M2","m3","m3"],
            // Dominant7Flat5
            (ChordQuality::DominantSeventhFlatFive, ChordPosition::Root) => vec!["M3","M2","M3"],
            (ChordQuality::DominantSeventhFlatFive, ChordPosition::FirstInversion) => vec!["M2","M3","M2"],
            // Suspended2Suspended4
            (ChordQuality::SuspendedTwoSuspendedFour, ChordPosition::Root) => vec!["M2","m3","M2"],
            (ChordQuality::SuspendedTwoSuspendedFour, ChordPosition::FirstInversion) => vec!["m3","M2","P4"],
            (ChordQuality::SuspendedTwoSuspendedFour, ChordPosition::SecondInversion) => vec!["M2","P4","M2"],
            (ChordQuality::SuspendedTwoSuspendedFour, ChordPosition::ThirdInversion) => vec!["P4","M2","m3"],
            _ => vec![]

        };
        let intervals : Vec<Interval> = interval_names.iter()
            .map(|s| Interval::from_name(s).unwrap() )
            .collect();
        intervals
    }

    /// Get the notes of the Chord
    pub fn get_notes(&self) -> Vec<Note> { 
        let mut res = vec![ self.root_note.clone() ];
        let intervals = Self::new(self.root_note.clone(), self.quality.clone(), ChordPosition::Root ).get_intervals();
        for i in 0..intervals.len() {
            let note = intervals[i].apply( &res[i] );
            res.push( note );
        }
        let n = res.len();
        match self.position {
            ChordPosition::Root => res,
            ChordPosition::FirstInversion => res.into_iter().cycle().skip(1).take(n).collect::<Vec<Note>>(),
            ChordPosition::SecondInversion => res.into_iter().cycle().skip(2).take(n).collect::<Vec<Note>>(),
            ChordPosition::ThirdInversion => res.into_iter().cycle().skip(3).take(n).collect::<Vec<Note>>()
        }
    }

    /// Get chord Intervals from root note
    pub fn get_intervals_from_root(&self) -> Vec<Interval> {
        let mut root_intervals : Vec<Interval> = self.get_intervals().iter()
            .scan(0, |state, x| {
                *state = *state + x.get_value();
                Some( Interval::new(*state) )
            }).collect();
        root_intervals.insert(0, Interval::new(0));
        root_intervals
    }

    /// Prints a chord numeral based on a scale root note e.g. Eb from C is IIIb
    pub fn as_numeral(&self, scale_root: &Note, index: usize) -> String {
        let interval_name = Interval::from_notes(scale_root, &self.root_note).to_string();
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
        let intervals : Vec<Interval> = notes.as_slice()
            .windows(2)
            .map(|w| Interval::from_notes(&w[0], &w[1]) )
            .collect();
        let mut res : Vec<Chord> = Vec::new();
        for note in notes.iter() {
            let chord = Chord::from_intervals( note.clone(), &intervals ).unwrap();
            if chord.get_intervals().iter().zip( intervals.iter() ).all(|elt| elt.0 == elt.1) {
                res.push( chord );
            }
        }
        res
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root_note, self.quality.to_string())
    }
}

impl PartialEq for Chord {
    fn eq(&self, other: &Chord) -> bool {
        other.get_intervals().iter()
            .zip(self.get_intervals().iter())
            .all(|elt| elt.0 == elt.1 )
    }
}

impl Eq for Chord {}