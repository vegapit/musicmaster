extern crate musicmaster;

use musicmaster::{Note, NoteLetter, NoteAccidental, Scale, Chord, ScaleQuality, ChordQuality};

#[test]
fn scalequality_from() {
    let quality : ScaleQuality = "HarmonicMajor".parse().unwrap();
    assert_eq!( quality, ScaleQuality::HarmonicMajor );
}

#[test]
fn scale_triad() {
    let root_note = Note::new( NoteLetter::A, NoteAccidental::Natural);
    let natural_minor_scale = Scale::factory(root_note, ScaleQuality::Major, 5);
    let triads = natural_minor_scale.get_chords(true);

    assert_eq!(triads[0].print(), String::from("Am7"));
    assert_eq!(triads[1].print(), String::from("Bm7(b5)"));
    assert_eq!(triads[2].print(), String::from("CMaj7"));
    assert_eq!(triads[3].print(), String::from("Dm7"));
    assert_eq!(triads[4].print(), String::from("Em7"));
    assert_eq!(triads[5].print(), String::from("FMaj7"));
    assert_eq!(triads[6].print(), String::from("G7"));

    assert_eq!(triads[0].print_numeral(&root_note, 0), String::from("I"));
    assert_eq!(triads[1].print_numeral(&root_note, 1), String::from("II"));
    assert_eq!(triads[2].print_numeral(&root_note, 2), String::from("bIII"));
    assert_eq!(triads[3].print_numeral(&root_note, 3), String::from("IV"));
    assert_eq!(triads[4].print_numeral(&root_note, 4), String::from("V"));
    assert_eq!(triads[5].print_numeral(&root_note, 5), String::from("bVI"));
    assert_eq!(triads[6].print_numeral(&root_note, 6), String::from("bVII"));
}

#[test]
fn mode_notes() {
    let root_note = Note::new( NoteLetter::A, NoteAccidental::Natural );
    let phrygian_mode = Scale::factory( root_note, ScaleQuality::Major, 2 );
    let notes = phrygian_mode.get_notes(true);
    
    assert_eq!(notes[0].print(), String::from("A"));
    assert_eq!(notes[1].print(), String::from("Bb"));
    assert_eq!(notes[2].print(), String::from("C"));
    assert_eq!(notes[3].print(), String::from("D"));
    assert_eq!(notes[4].print(), String::from("E"));
    assert_eq!(notes[5].print(), String::from("F"));
    assert_eq!(notes[6].print(), String::from("G"));

    let intervals = phrygian_mode.get_root_intervals();
    assert_eq!(intervals[0].print(), String::from("P1"));
    assert_eq!(intervals[1].print(), String::from("m2"));
    assert_eq!(intervals[2].print(), String::from("m3"));
    assert_eq!(intervals[3].print(), String::from("P4"));
    assert_eq!(intervals[4].print(), String::from("P5"));
    assert_eq!(intervals[5].print(), String::from("m6"));
    assert_eq!(intervals[6].print(), String::from("m7"));
}

#[test]
fn scale_chord_contain() {
    let root_note = Note::new( NoteLetter::C, NoteAccidental::Natural );
    let phrygian_mode = Scale::factory(root_note, ScaleQuality::Major, 2);
    
    let cminor7 = Chord::factory( Note::new( NoteLetter::C, NoteAccidental::Natural), ChordQuality::MinorSeventh );
    assert_eq!(phrygian_mode.print_chord_numeral(&cminor7).unwrap(), String::from("I"));

    let fminor7 = Chord::factory( Note::new( NoteLetter::F, NoteAccidental::Natural), ChordQuality::MinorSeventh );
    assert_eq!(phrygian_mode.print_chord_numeral(&fminor7).unwrap(), String::from("IV"));
}

#[test]
fn scale_proximity() {
    let root_note = Note::new( NoteLetter::D, NoteAccidental::Natural );
    let initial_scale = Scale::factory(root_note, ScaleQuality::Major, 5);
    
    let proximity_scales = initial_scale.proximity_scales();
    for (key,val) in proximity_scales.iter() {
        for scale in val {
            if let Some( new_scale ) = scale.root_shift( &root_note ) {
                println!("NoteDifference: {} Scale: {} ({})", key, scale.print(), new_scale.print());
            } else {
                println!("NoteDifference: {} Scale: {}", key, scale.print());
            }
        }
    }
}