pub use svg;

use scof::Marking;

/// Different parts of the music that can be drawn.
///
/// The IDs match SMuFL.  
#[repr(u32)]
pub enum GlyphId {
    // Barline
    Barline = 0xE030,

    // Stem For Notes
    Stem = 0xE210,
    StemBuzzRoll = 0xE217,
    StemDamp = 0xE218,
    StemHarpStringNoise = 0xE21F,
    StemRimShot = 0xE21E,
    StemBowBridge = 0xE215,
    StemBowTailpiece = 0xE216,

    // 8th-1024th Flags
    FlagUp8 = 0xE240, // LP: 0xE21D
    FlagDown8 = 0xE241, // LP: 0xE222
    FlagUp16 = 0xE242, // LP: 0xE21E
    FlagDown16 = 0xE243, // LP: 0xE223
    FlagUp32 = 0xE244, // LP: 0xE21F
    FlagDown32 = 0xE245, // LP: 0xE224
    FlagUp64 = 0xE246, // LP: 0xE220
    FlagDown64 = 0xE247, // LP: 0xE225
    FlagUp128 = 0xE248, // LP: 0xE221
    FlagDown128 = 0xE249, // LP: 0xE226
/*    FlagUp256 = 0xE24A,
    FlagDown256 = 0xE24B,
    FlagUp512 = 0xE24C,
    FlagDown512 = 0xE24D,
    FlagUp1024 = 0xE24E,
    FlagDown1024 = 0xE24F,*/

    // -- NOTEHEADS --
    // Double Whole Note Notehead (8 beats)
    NoteheadDoubleWhole = 0xE0A0, // 0xE191
    NoteheadDoubleWholeX = 0xE0A6,
    NoteheadDoubleWholeSquare = 0xE0A1,
    NoteheadDoubleWholeWithX = 0xE0B4,
    // Whole Note & Half Note Notehead (4 beats)
    NoteheadOutlineSquare = 0xE0B8,
    NoteheadOutlineLargeSquare = 0xE11B,
    // Whole Note
    NoteheadWhole = 0xE0A2, // 0xE192
    NoteheadWholeX = 0xE0A7, // 0xE1A0
    // Half Note
    NoteheadHalf = 0xE0A3, // 0xE193
    NoteheadHalfX = 0xE0A8, // 0xE1A1
    // Quarter And Smaller Note Nothead (< 2 beats)
    NoteheadFill = 0xE0A4, // 0xE194
    NoteheadFillX = 0xE0A9, // 0xE1A2
    NoteheadSquare = 0xE0B9,
    NoteheadLargeSquare = 0xE11A,

    // -- RESTS --
    // Whole Rest
    Rest1 = 0xE4E3, // LP: 0xE100
    // Half Rest
    Rest2 = 0xE4E4, // LP: 0xE101
    // Quarter Rest (2 Styles Default=Modern)
    Rest4 = 0xE4E5, // LP: 0xE108
    Rest4Old = 0xE4F2, // LP: 0x109
    // ...
    Rest8 = 0xE4E6, // LP: E10A
    Rest16 = 0xE4E7, // LP: E10B
    Rest32 = 0xE4E8, // LP: E10C
    Rest64 = 0xE4E9, // LP: E10D
    Rest128 = 0xE4EA, // LP: E10E
/*    Rest256 = 0xE4EB, // 
    Rest512 = 0xE4EC, // 
    Rest1024 = 0xE4ED, // */

    // Pitch Bends
    PitchPlop = 0xE5E0,
    PitchScoop = 0xE5D0,
    PitchSmear = 0xE5E2,

    // Coda
    Coda = 0xE048,
    CodaSquare = 0xE049,

    // Segno
    Segno = 0xE047,

/*    MeasureRepeat1 = 0xE500,
    MeasureRepeat2 = 0xE501,
    MeasureRepeat4 = 0xE502,*/
    // Measure Repeat Begin Dot
    MeasureRepeatUpper = 0xE503,
    // Measure Repeat Slash: Use as many slashes as how many measures to repeat.
    MeasureRepeatSlash = 0xE504,
    // Measure Repeat End Dot
    MeasureRepeatLower = 0xE505,

    // Repeats
    RepeatOpen = 0xE040,
    RepeatClose = 0xE041,
    RepeatCloseOpen = 0xE042,

    // -- Flat & Sharp --
/*  FlatComma1 = 0xE454,
    SharpComma1 = 0xE450,
    FlatComma2 = 0xE455,
    SharpComma2 = 0xE451,
    FlatComma3 = 0xE456,
    SharpComma3 = 0xE452,*/
    // Semi (Half) Tones
    // Differently Tuned Double Flat
    FlatDouble = 0xE264, // 0xE124
    FlatDoubleEqual = 0xE2F0,
    FlatDoubleFlatComma1 = 0xE2C0,
    FlatDoubleSharpComma1 = 0xE2C5,
    FlatDoubleFlatComma2 = 0xE2CA,
    FlatDoubleSharpComma2 = 0xE2CF,
    FlatDoubleFlatComma3 = 0xE2D4,
    FlatDoubleSharpComma3 = 0xE2D9,
    // Differently Tuned Flat
    Flat = 0xE260, // 0xE11B
    FlatEqual = 0xE2F1,
    FlatFlatComma1 = 0xE2C1,
    FlatSharpComma1 = 0xE2C6,
    FlatFlatComma2 = 0xE2CB,
    FlatSharpComma2 = 0xE2D0,
    FlatFlatComma3 = 0xE2D5,
    FlatSharpComma3 = 0xE2DA,
    // Differently Tuned Natural
    Natural = 0xE261, // 0xE117
    NaturalEqual = 0xE2F2,
    NaturalFlatComma1 = 0xE2C2,
    NaturalSharpComma1 = 0xE2C7,
    NaturalFlatComma2 = 0xE2CC,
    NaturalSharpComma2 = 0xE2D1,
    NaturalFlatComma3 = 0xE2D6,
    NaturalSharpComma3 = 0xE2DB,
    // Differently Tuned Sharp
    Sharp = 0xE262, // 0xE10F
    SharpEqual = 0xE2F3,
    SharpFlatComma1 = 0xE2C3,
    SharpSharpComma1 = 0xE2C8,
    SharpFlatComma2 = 0xE2CD,
    SharpSharpComma2 = 0xE2D2,
    SharpFlatComma3 = 0xE2D7,
    SharpSharpComma3 = 0xE2DC,
    // Differently Tuned Double Sharp
    SharpDouble = 0xE263, // 0xE126
    SharpDoubleEqual = 0xE2F4,
    SharpDoubleFlatComma1 = 0xE2C4,
    SharpDoubleSharpComma1 = 0xE2C9,
    SharpDoubleFlatComma2 = 0xE2CE,
    SharpDoubleSharpComma2 = 0xE2D3,
    SharpDoubleFlatComma3 = 0xE2D8,
    SharpDoubleSharpComma3 = 0xE2DD,
    // Quarter Tones (2 styles - default=)
    SharpQuarter3 = 0xED37, // 0xE116
    SharpQuarter3SteinZimmerman = 0xE283,
    SharpQuarter3Busotti = 0xE474,
    FlatQuarter3 = 0xED31, // 0xE121
    FlatQuarter3SteinZimmerman = 0xE281,
    FlatQuarter1 = 0xED33, // 0xE122
    FlatQuarter1SteinZimmerman = 0xE280,
    FlatQuarter1Iranian = 0xE460, // Koron
    FlatQuarter1Numeric = 0xE47F,
    SharpQuarter1 = 0xED35, // 0xE133
    SharpQuarter1SteinZimmerman = 0xE282,
    SharpQuarter1Iranian = 0xE461, // Sori
    SharpQuarter1Numeric = 0xE47E,
    // Differently Tuned Quarter Tones
    FlatQuarter1Tridecimal = 0xE2E4,
    FlatQuarter1Undecimal = 0xE2E2,
    // Third Tones (2 styles - default=Xenakis)
    SharpThird1 = 0xE470,
    SharpThird1Ferneyhough = 0xE48A,
//    FlatThird1 = , // FIXME
    FlatThird1Ferneyhough = 0xE48B,
    SharpThird2 = 0xE471,
    SharpThird2Ferneyhough = 0xE48C,
//    FlatThird2 = , // FIXME
    FlatThird2Ferneyhough = 0xE48D,

    // Grace Note
    GraceNoteSlashStemUp = 0xE564,
    GraceNoteSlashStemDown = 0xE565,

    // -- Clefs --
    // Tabulature
    ClefTab4 = 0xE06E,
    ClefTab6 = 0xE06D,
    // Alto Clef
    ClefC = 0xE05C,
    ClefCChange = 0xE07B,
    // Treble Clef
    ClefG = 0xE050,
//    ClefGCombiningAlta = 0xE059,
//    ClefGCombiningBassa = 0xE058,
    ClefGChange = 0xE07A,
    // Bass Clef
    ClefF = 0xE062,
    ClefFChange = 0xE07C,
    // Octave Changes
    Clef8 = 0xE07D,
    Clef15 = 0xE07E,
    // TODO: Group measures transpose by some number of octaves
/*    Clef8 = 0xE510,
    Clef15 = 0xE514,
    Clef22 = 0xE517,*/
    ClefLParens = 0xE51A,
    ClefRParens = 0xE51B,
//    Clef8vaBassa = ,
//    Clef15maBassa = ,
//    Clef22maBassa = ,
//    Clef8vaAlta = ,
//    Clef15maAlta = ,
//    Clef22maAlta = ,

    // Time Signature
    TimeSig0 = 0xE080, // 0x0030
    TimeSig1 = 0xE081, // 0x0031
    TimeSig2 = 0xE082, // 0x0032
    TimeSig3 = 0xE083, // 0x0033
    TimeSig4 = 0xE084, // 0x0034
    TimeSig5 = 0xE085, // 0x0035
    TimeSig6 = 0xE086, // 0x0036
    TimeSig7 = 0xE087, // 0x0037
    TimeSig8 = 0xE088, // 0x0038
    TimeSig9 = 0xE089, // 0x0039
    // 4/4
    TimeSigCommon = 0xE08A,
    // 2/2
    TimeSigCut = 0xE08B,
    TimeSigPlus = 0xE08C,
    TimeSigNumPlus = 0xE08D, // 0x002B
    // TODO ???
/*    TimeSigNum0 = 0xF5B7, 
    TimeSigNum1 = 0xF5B9, 
    TimeSigNum2 = 0xF5BB, 
    TimeSigNum3 = 0xF5BD, 
    TimeSigNum4 = 0xF5BF, 
    TimeSigNum5 = 0xF5C1,
    TimeSigNum6 = 0xF5C3,
    TimeSigNum7 = 0xF5C5,
    TimeSigNum8 = 0xF5C7,
    TimeSigNum9 = 0xF5C9,
    TimeSigDen0 = 0xF5B6,
    TimeSigDen1 = 0xF5B8,
    TimeSigDen2 = 0xF5BA,
    TimeSigDen3 = 0xF5BC,
    TimeSigDen4 = 0xF5BE,
    TimeSigDen5 = 0xF5C0,
    TimeSigDen6 = 0xF5C2,
    TimeSigDen7 = 0xF5C4,
    TimeSigDen8 = 0xF5C6,
    TimeSigDen9 = 0xF5C8,*/

    // Tremelo
    Tremelo1 = 0xE220,
    Tremelo2 = 0xE221,
    Tremelo3 = 0xE222,
    Tremelo4 = 0xE223,
    Tremelo5 = 0xE224,

    // Dynamics
/*    PPPPPP = 0xE527,
    PPPPP = 0xE528,
    PPPP = 0xE529,
    PPP = 0xE52A,
    PP = 0xE52B,*/
    P = 0xE520, // 0x0070
    M = 0xE521, // 0x006D
    F = 0xE522, // 0x0066
    R = 0xE523, // 0x0072
    S = 0xE524, // 0x0073
    Z = 0xE525, // 0x007A
    N = 0xE526,
/*    MP = 0xE52C,
    MF = 0xE52D,
    F = 0x,
    FF = 0x,
    FFF = 0x,
    FFFF = 0x,
    FFFFF = 0x,
    FFFFFF = 0x,*/

    // Beam TODO: How do we do this?

    // Glissandos
    GlissUpShort = 0xE5D1,
    GlissUpMedium = 0xE5D2,
    GlissUpLong = 0xE5D3,
    GlissDownShort = 0xE5DD,
    GlissDownMedium = 0xE5DE,
    GlissDownLong = 0xE5DF,
    GlissUpShortStyleB = 0xE5EC,
    GlissUpMediumStyleB = 0xE5ED,
    GlissUpLongStyleB = 0xE5EF,
    GlissDownShortStyleB = 0xE5DA,
    GlissDownMediumStyleB = 0xE5DB,
    GlissDownLongStyleB = 0xE5DC,

    // Tuplet
    Tuplet0 = 0xE880,
    Tuplet1 = 0xE881,
    Tuplet2 = 0xE882,
    Tuplet3 = 0xE883,
    Tuplet4 = 0xE884,
    Tuplet5 = 0xE885,
    Tuplet6 = 0xE886,
    Tuplet7 = 0xE887,
    Tuplet8 = 0xE888,
    Tuplet9 = 0xE889,
    TupletColon = 0xE88A,
}

/// Default font (Bravura).
pub const DEFAULT: &'static str = include_str!("vfont/bravura.vfont");

// A half or whole step.
const STEP: i32 = 125;

use GlyphId::*;

fn stamp(out: &mut String, u: GlyphId, x: i32, y: i32) {
    out.push_str(&format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#{:x}\"/>\n", x, y, u as u32));
}

// Add a stem downwards.
fn stem_d(out: &mut String, x: i32, y: i32) {
    out.push_str(&format!("<path d=\"M{} {}c-1 14-29 14-30 0v-855l30 50v855z\"/>\n", x + 15, y + 1895));
}

// Add a stem upwards. 910
fn stem_u(out: &mut String, x: i32, y: i32) {
    out.push_str(&format!("<path d=\"M{} {}c-1 -14-29-14-30 0v805l30 50v-805z\"/>\n", x + 15, y + 105));
}


fn staff(out: &mut String, x: i32, y: i32, w: i32) {
    out.push_str(&format!("<path d=\"M{} {}h{}v32h-{}v-32z\"/>\n", x, y - 16, w, w));
}

fn cursor(out: &mut String, x: i32, y: i32, w: i32) {
    out.push_str(&format!("<path d=\"M{} {}h{}v1000h-{}v-1000z\" fill=\"#AAF\"/>\n", x, y, w, w));
}

/// - `margin`: Offset from X origin.
/// - `screen_width`: Width of screen.
/// - `cursor_dims`: If cursor should be drawn, % position and % width.
/// - `scof`: The score.
/// - `bar`: measure number.
/// - `chan`: Which channel the bar is on.
fn render_measure(out: &mut String, mut margin: i32, screen_width: i32, cursor_dims: Option<(f32, f32)>, scof: &scof::Scof, bar: usize, chan: usize) {
//    stamp(out, NoteheadFill, margin + 2000, 1000 - STEP * 3);
//    stem_d(out, margin + 2000 + 15, -STEP * 3);

    if let Some((position, width)) = cursor_dims {
        cursor(out, margin + 32 + (4000f32 * position) as i32, 1000 -STEP * 4, (4000f32 * width) as i32);
    }

    let mut empty = true;
    let mut curs = 0;
    while let Some(marking) = scof.get(bar, chan, curs) {
        if empty {
            empty = false;
        }
        match marking {
            Marking::Note(note) => {
                if let Some(pitch) = note.pitch {
                    
                } else {
                    // Rest
                    match note.duration.1 {
                        1 => stamp(out, Rest1, margin + 250, 1000),
                        2 => stamp(out, Rest2, margin + 250, 1000),
                        4 => stamp(out, Rest4, margin + 250, 1000),
                        8 => stamp(out, Rest8, margin + 250, 1000),
                        16 => stamp(out, Rest16, margin + 250, 1000),
                        32 => stamp(out, Rest32, margin + 250, 1000),
                        64 => stamp(out, Rest64, margin + 250, 1000),
                        128 => stamp(out, Rest128, margin + 250, 1000),
                        _ => { /*unknown, do nothing*/ }
                    }
                    margin += (4000f32 * note.duration.0 as f32 / note.duration.1 as f32) as i32;
                }
            }
            _ => {/*unknown, do nothing*/}
        }
        curs += 1;
    }

    if empty {
        stamp(out, Rest1, margin + 250, 1000 -STEP * 2);
    }
}

fn render(out: &mut String, screen_width: i32, scof: &scof::Scof, chan: usize) {
    let margin = 96;
    let screen_width = screen_width - margin * 2;

/*    // Clef
    stamp(out, ClefC, 96, 1000);
    // Time Signature
    stamp(out, TimeSig3, 96 + 1000, 1000 - STEP * 2); // 421
    stamp(out, TimeSig4, 96 + 1000 - ((470 - 421) / 2), 1000 + STEP * 2); // 470*/

    // Print enough measures to fill the screen.
    let mut i = 0usize;
    'm: loop {
        let position = 4000 * i as i32;

        if position > screen_width {
            break 'm;
        }

        // Render this measure
        let curs = if i == 0 {
            Some((0.0, 0.25))
        } else {
            None
        };
        render_measure(out, margin + position, screen_width, curs, scof, i, chan);
        // Barline
        stamp(out, Barline, margin + position + 4000, 1500);

        i += 1;
    }

    // 5 line staff
    for i in 0..5 {
        staff(out, margin, STEP * (4 + i * 2), screen_width);
    }

/*    // Draw 
    stamp(out, NoteheadFill, 96 + 2000, 1000 - STEP * 3);
    stem_d(out, 96 + 2000 + 15, -STEP * 3);
    // Draw
    stamp(out, NoteheadFill, 96 + 2000 + 1000, 1000 + STEP * 3);
    stem_u(out, 96 + (2000 + 265) + 1000 + 15, STEP * 3);

    // Draw 
    stamp(out, NoteheadHalf, 96 + 2000 + 2000, 1000);
    stem_d(out, 96 + 2000 + 2000 + 15, 0);

    // Barline
    stamp(out, Barline, 96 + 2000 + 4000, 1500);

    // Draw
    stamp(out, NoteheadHalf, 96 + 2000 + 4400, 1000);
    stem_u(out, 96 + (2000 + 265) + 4400 + 15, 0);*/
}

/// Generate some test score.
pub fn test_svg(vfont: &str, screen_width: i32, scof: &scof::Scof) -> String {
    // Header: DEFS section.
    let mut out = "<svg viewBox=\"0 0 8192 2048\">\n".to_string();
    out.push_str(vfont);

    // Bodyer: Each Bar
    render(&mut out, screen_width, scof, 0); // Channel 0

//    stamp(&mut out, ClefCChange, 96 + 699, 512 + (STEP * 4));

    // Footer: Close File
    out.push_str("</svg>");
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
