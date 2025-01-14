extern crate musicmaster;

use musicmaster::{Note, NoteLetter, NoteAccidental, Scale, Chord, Mode, ChordQuality, ChordPosition};
use musicmaster::{all_note_letters, all_note_accidentals};
use std::convert::TryFrom;

#[test]
fn scale_from() {
    let scale = Scale::try_from("HarmonicMajor").unwrap();
    assert_eq!( scale, Scale::HarmonicMajor );
}

#[test]
fn mode_major() {
    let root_note = Note::new( NoteLetter::A, NoteAccidental::Natural);
    let mode = Mode::new(root_note, Scale::Major, 5);
    let triads : Vec<Chord> = mode.get_chords(true, true).into_iter()
        .map(|opt| opt.unwrap())
        .collect();

    assert_eq!(triads[0].to_string(), String::from("Am7"));
    assert_eq!(triads[1].to_string(), String::from("Bm7(b5)"));
    assert_eq!(triads[2].to_string(), String::from("CMaj7"));
    assert_eq!(triads[3].to_string(), String::from("Dm7"));
    assert_eq!(triads[4].to_string(), String::from("Em7"));
    assert_eq!(triads[5].to_string(), String::from("FMaj7"));
    assert_eq!(triads[6].to_string(), String::from("G7"));

    assert_eq!(triads[0].as_numeral(&root_note, 0), String::from("I"));
    assert_eq!(triads[1].as_numeral(&root_note, 1), String::from("II"));
    assert_eq!(triads[2].as_numeral(&root_note, 2), String::from("bIII"));
    assert_eq!(triads[3].as_numeral(&root_note, 3), String::from("IV"));
    assert_eq!(triads[4].as_numeral(&root_note, 4), String::from("V"));
    assert_eq!(triads[5].as_numeral(&root_note, 5), String::from("bVI"));
    assert_eq!(triads[6].as_numeral(&root_note, 6), String::from("bVII"));
}

#[test]
fn mode_notes() {
    let root_note = Note::new( NoteLetter::A, NoteAccidental::Natural );
    let phrygian_mode = Mode::new( root_note, Scale::Major, 2 );
    
    let notes = phrygian_mode.get_notes(true);
    assert_eq!(notes[0].to_string(), String::from("A"));
    assert_eq!(notes[1].to_string(), String::from("Bb"));
    assert_eq!(notes[2].to_string(), String::from("C"));
    assert_eq!(notes[3].to_string(), String::from("D"));
    assert_eq!(notes[4].to_string(), String::from("E"));
    assert_eq!(notes[5].to_string(), String::from("F"));
    assert_eq!(notes[6].to_string(), String::from("G"));

    let intervals = phrygian_mode.get_root_intervals();
    assert_eq!(intervals[0].to_string(), String::from("P1"));
    assert_eq!(intervals[1].to_string(), String::from("m2"));
    assert_eq!(intervals[2].to_string(), String::from("m3"));
    assert_eq!(intervals[3].to_string(), String::from("P4"));
    assert_eq!(intervals[4].to_string(), String::from("P5"));
    assert_eq!(intervals[5].to_string(), String::from("m6"));
    assert_eq!(intervals[6].to_string(), String::from("m7"));
}

#[test]
fn mode_chord_contain() {
    let root_note = Note::new( NoteLetter::C, NoteAccidental::Natural );
    let phrygian_mode = Mode::new(root_note, Scale::Major, 2);
    
    let cminor7 = Chord::new( Note::new( NoteLetter::C, NoteAccidental::Natural), ChordQuality::MinorSeventh, ChordPosition::Root );
    assert_eq!(phrygian_mode.chord_numeral(&cminor7).unwrap(), String::from("I"));

    let fminor7 = Chord::new( Note::new( NoteLetter::F, NoteAccidental::Natural), ChordQuality::MinorSeventh, ChordPosition::Root );
    assert_eq!(phrygian_mode.chord_numeral(&fminor7).unwrap(), String::from("IV"));
}

#[test]
fn all_major_modes() {
    for letter in all_note_letters() {
        for accidental in all_note_accidentals() {
            for degree in 1..8 {
                let root_note = Note::new( letter, accidental );
                let phrygian_mode = Mode::new( root_note, Scale::Major, degree );
                let intervals = phrygian_mode.get_root_intervals();
                assert_eq!(intervals.len(), 7);
                let notes = phrygian_mode.get_notes(false);
                assert_eq!(notes.len(), 7);
            }
        }
    }
}