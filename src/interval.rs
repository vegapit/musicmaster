use std::fmt;
use crate::Note;

#[derive(Debug, Clone)]
pub struct Interval{
    value: u32
}

/// Represents a musical interval
impl Interval{

    /// Creates a music interval based on a semitone value
    pub fn new(value: u32) -> Interval {
        Interval{value: value}
    }

    pub fn get_value(&self) -> u32 { self.value }

    /// Creates a music interval based on two notes
    pub fn from_notes(from: &Note, to: &Note) -> Interval {
        if  to.get_index() >= from.get_index() {
            Interval::new(to.get_index() - from.get_index())
        } else {
            Interval::new(12 + to.get_index() - from.get_index())
        }
    }

    /// Creates a music interval from a name
    pub fn from_name(s: &str) -> Option<Interval> {
        for u in 0..11 {
            let interval = Interval::new(u);
            if interval.to_string() == s.to_string() {
                return Some( interval );
            }
        }
        None
    }

    /// Apply a music interval to a note and return note
    pub fn apply(&self, obj: &Note) -> Note {
        let mut res = obj.clone();
        for _ in 0..self.value {
            res = res.next();
        }
        res
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self.value % 12 {
            0 => String::from("P"),
            1 => String::from("m"),
            2 => String::from("M"),
            3 => String::from("m"),
            4 => String::from("M"),
            5 => String::from("P"),
            6 => String::from("d"),
            7 => String::from("P"),
            8 => String::from("m"),
            9 => String::from("M"),
            10 => String::from("m"),
            _ => String::from("M")
        };
        let number = match self.value % 12 {
            0 => 1,
            1 => 2,
            2 => 2,
            3 => 3,
            4 => 3,
            5 => 4,
            6 => 5,
            7 => 5,
            8 => 6,
            9 => 6,
            10 => 7,
            _ => 7
        };
        write!(f, "{}{}", res, number)
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Interval) -> bool {
        self.value == other.value
    }
}

impl Eq for Interval {}