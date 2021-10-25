extern crate musicmaster;

use musicmaster::{Note, NoteAccidental, NoteLetter};
use std::convert::TryFrom;


#[test]
fn note_from() {
    let note : Note = Note::try_from("Ab").unwrap();
    assert_eq!(note, Note::new(NoteLetter::G, NoteAccidental::Sharp));
}

#[test]
fn note_string() {
    assert_eq!(Note::new(NoteLetter::C, NoteAccidental::Natural).to_string(), String::from("C"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Sharp).to_string(), String::from("G#"));
    assert_eq!(Note::new(NoteLetter::E, NoteAccidental::Natural).to_string(), String::from("E"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Flat).to_string(), String::from("Gb"));
}

#[test]
fn note_previous() {
    assert_eq!(Note::new(NoteLetter::C, NoteAccidental::Natural).previous(), Note::try_from("B").unwrap());
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Sharp).previous(), Note::try_from("G").unwrap());
}

#[test]
fn note_next() {
    assert_eq!(Note::new(NoteLetter::E, NoteAccidental::Natural).next(), Note::try_from("F").unwrap());
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Flat).next(), Note::try_from("G").unwrap());
}

#[test]
fn note_equivalent() {
    let mut ref_note = Note::new(NoteLetter::C, NoteAccidental::Sharp); 
    for note in ref_note.equivalents().into_iter() {
        assert_eq!(ref_note.get_index(), note.get_index());    
    }
    ref_note = Note::new(NoteLetter::F, NoteAccidental::Flat); 
    for note in ref_note.equivalents().into_iter() {
        assert_eq!(ref_note.get_index(), note.get_index());    
    }
    ref_note = Note::new(NoteLetter::E, NoteAccidental::Natural); 
    for note in ref_note.equivalents().into_iter() {
        assert_eq!(ref_note.get_index(), note.get_index());    
    }
    ref_note = Note::new(NoteLetter::G, NoteAccidental::Flat); 
    for note in ref_note.equivalents().into_iter() {
        assert_eq!(ref_note.get_index(), note.get_index());    
    }
}

#[test]
fn note_index() {
    assert_eq!( Note::try_from("E").unwrap().get_index(), 4 );
    assert_eq!( Note::try_from("F#").unwrap().get_index(), 6 );
    assert_eq!( Note::try_from("Bb").unwrap().get_index(), 10 );
}

#[test]
fn note_numeral() {
    let root_note = Note::new(NoteLetter::C, NoteAccidental::Natural);
    assert_eq!(Note::new(NoteLetter::C, NoteAccidental::Sharp).as_numeral(&root_note, 1), String::from("2b"));
    assert_eq!(Note::new(NoteLetter::F, NoteAccidental::Flat).as_numeral(&root_note, 2), String::from("3"));
    assert_eq!(Note::new(NoteLetter::E, NoteAccidental::Natural).as_numeral(&root_note, 2), String::from("3"));
    assert_eq!(Note::new(NoteLetter::G, NoteAccidental::Flat).as_numeral(&root_note, 4), String::from("5b"));
}