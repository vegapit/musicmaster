extern crate musicmaster;

use musicmaster::{Note, NoteLetter, Chord, NoteAccidental, ChordQuality};

#[test]
fn chord_print() {
    let root_note = Note::new(NoteLetter::C, NoteAccidental::Natural);
    let chord = Chord::factory( root_note, ChordQuality::Minor );
    assert_eq!( chord.print(), String::from("Cm") );
}


#[test]
fn chord_identify() {
    let root_note = Note::new(NoteLetter::A, NoteAccidental::Natural);
    let notes = vec![
        root_note,
        Note::new(NoteLetter::C, NoteAccidental::Natural),
        Note::new(NoteLetter::E, NoteAccidental::Natural)
    ];
    let chords = Chord::identify( &notes );
    match chords[0].chord_quality {
        ChordQuality::Minor => assert!( true ),
        _ => assert!(false)
    }
    assert_eq!( chords[0].root_note, root_note);
}