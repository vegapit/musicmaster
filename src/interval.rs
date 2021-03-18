use crate::Note;

#[derive(Debug, Clone)]
pub struct Interval{
    pub value: u32
}

/// Represents a musical interval
impl Interval{

    /// Creates a music interval based on a semitone value
    pub fn new(value: u32) -> Interval {
        Interval{value: value}
    }

    /// Creates a music interval baed on two notes
    pub fn from_notes(from: &Note, to: &Note) -> Interval {
        let diff = to.get_id() - from.get_id();
        if  diff >= 0 {
            Interval::new(diff as u32)
        } else {
            Interval::new(12+diff as u32)
        }
    }

    /// Creates a music interval from a name
    pub fn from_name(s: &str) -> Option<Interval> {
        for u in 0..11 {
            let interval = Interval::new(u);
            if interval.print() == s.to_string() {
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

    /// Returns the name of the music interval without octave considerations
    pub fn print(&self) -> String {
        let mut res = match self.value % 12 {
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

        res.push_str( number.to_string().as_str() );

        res
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Interval) -> bool {
        self.value == other.value
    }
}

impl Eq for Interval {}