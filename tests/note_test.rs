extern crate musicmaster;

use musicmaster::{Note, NoteAccidental, NoteLetter};

#[test]
fn note_print() {
    assert_eq!(Note::new(NoteLetter::C, NoteAccidental::Natural).print(), String::from("C"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Sharp).print(), String::from("G#"));
    assert_eq!(Note::new(NoteLetter::E, NoteAccidental::Natural).print(), String::from("E"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Flat).print(), String::from("Gb"));
}

#[test]
fn note_nextprevious() {
    assert_eq!(Note::new(NoteLetter::C, NoteAccidental::Natural).previous().print(), String::from("B"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Sharp).previous().print(), String::from("G"));
    assert_eq!(Note::new(NoteLetter::E, NoteAccidental::Natural).next().print(), String::from("F"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Flat).next().print(), String::from("G"));
}

#[test]
fn note_equivalent() {
    assert_eq!(Note::new(NoteLetter::C, NoteAccidental::Sharp).equivalent().unwrap().print(), String::from("Db"));
    assert_eq!(Note::new(NoteLetter::F, NoteAccidental::Flat).equivalent().unwrap().print(), String::from("E"));
    assert_eq!(Note::new(NoteLetter::E, NoteAccidental::Natural).equivalent().unwrap().print(), String::from("Fb"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Flat).equivalent().unwrap().print(), String::from("F#"));
}

#[test]
fn note_numeral() {
    let root_note = Note::new(NoteLetter::C, NoteAccidental::Natural);
    assert_eq!(Note::new(NoteLetter::C, NoteAccidental::Sharp).print_numeral(&root_note, 1), String::from("2b"));
    assert_eq!(Note::new(NoteLetter::F, NoteAccidental::Flat).print_numeral(&root_note, 2), String::from("3"));
    assert_eq!(Note::new(NoteLetter::E, NoteAccidental::Natural).print_numeral(&root_note, 2), String::from("3"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Flat).print_numeral(&root_note, 4), String::from("5b"));
}

#[test]
fn note_from() {
    let note : Note = "Ab".parse().unwrap();
    assert_eq!(note, Note::new(NoteLetter::A, NoteAccidental::Flat));
}