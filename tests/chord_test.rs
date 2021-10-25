extern crate musicmaster;

use std::convert::TryFrom;
use musicmaster::{Note, NoteLetter, Chord, NoteAccidental, ChordQuality, ChordPosition};

#[test]
fn chord_string() {
    let root_note = Note::new(NoteLetter::C, NoteAccidental::Natural);
    let chord = Chord::new( root_note, ChordQuality::Minor, ChordPosition::Root );
    assert_eq!( chord.to_string(), String::from("Cm") );
}


#[test]
fn chord_notes() {
    let mut chord = Chord::new( Note::try_from("E").unwrap(), ChordQuality::Minor, ChordPosition::SecondInversion);
    assert!( chord.get_notes().into_iter().zip( ["B","E","G"].iter().map(|&s| Note::try_from(s).unwrap()) ).all(|elt| elt.0 == elt.1) );
    chord = Chord::new( Note::try_from("C").unwrap(), ChordQuality::Diminished, ChordPosition::Root);
    assert!( chord.get_notes().into_iter().zip( ["C","Eb","Gb"].iter().map(|&s| Note::try_from(s).unwrap()) ).all(|elt| elt.0 == elt.1) );
}

#[test]
fn chord_identify() {
    let mut notes : Vec<Note> = [ "C", "E", "A" ].iter().map(|s| Note::try_from(*s).unwrap()).collect();
    for chord in Chord::identify( &notes ).into_iter() {
        assert_eq!( chord, Chord::new( Note::new(NoteLetter::A, NoteAccidental::Natural), ChordQuality::Minor, ChordPosition::FirstInversion ) );
    }
    notes = [ "F", "Ab", "C" ].iter().map(|s| Note::try_from(*s).unwrap()).collect();
    for chord in Chord::identify( &notes ).into_iter() {
        assert_eq!( chord, Chord::new( Note::new(NoteLetter::F, NoteAccidental::Natural), ChordQuality::Minor, ChordPosition::Root ) );
    }
    notes = [ "C", "Eb", "Gb", "A" ].iter().map(|s| Note::try_from(*s).unwrap()).collect();
    for chord in Chord::identify( &notes ).into_iter() {
        assert_eq!( chord, Chord::new( Note::new(NoteLetter::C, NoteAccidental::Natural), ChordQuality::DiminishedSeventh, ChordPosition::Root ) );
    }
}