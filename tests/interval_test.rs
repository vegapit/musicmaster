extern crate musicmaster;

use musicmaster::{Interval, Note, NoteLetter, NoteAccidental};

#[test]
fn interval_print() {
    assert_eq!(Interval::new(4).print(), String::from("M3"));
    assert_eq!(Interval::new(7).print(), String::from("P5"));
    assert_eq!(Interval::new(10).print(), String::from("m7"));
    assert_eq!(Interval::new(2).print(), String::from("M2"));
}

#[test]
fn interval_from_notes() {
    let cnote = Note::new(NoteLetter::C, NoteAccidental::Natural);
    let fnote = Note::new(NoteLetter::F, NoteAccidental::Natural);
    assert_eq!(Interval::from_notes(&cnote, &fnote).value, 5);
    assert_eq!(Interval::from_notes(&fnote, &cnote).value, 7);
}