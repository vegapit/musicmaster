extern crate musicmaster;

use musicmaster::{Interval, Note, NoteLetter, NoteAccidental};

#[test]
fn interval_string() {
    assert_eq!(Interval::new(4).to_string(), String::from("M3"));
    assert_eq!(Interval::new(7).to_string(), String::from("P5"));
    assert_eq!(Interval::new(10).to_string(), String::from("m7"));
    assert_eq!(Interval::new(2).to_string(), String::from("M2"));
}

#[test]
fn interval_from_notes() {
    let cnote = Note::new(NoteLetter::C, NoteAccidental::Natural);
    let fnote = Note::new(NoteLetter::F, NoteAccidental::Natural);
    assert_eq!(Interval::from_notes(&cnote, &fnote).get_value(), 5);
    assert_eq!(Interval::from_notes(&fnote, &cnote).get_value(), 7);
}