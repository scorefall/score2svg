/// Different parts of the music that can be drawn.
///
/// The IDs match SMuFL.  
#[repr(u32)]
pub enum GlyphId {
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

    // Staff Lines
    Staff1 = 0xE010,
    Staff2 = 0xE011,
    Staff3 = 0xE012,
    Staff4 = 0xE013,
    Staff5 = 0xE014,
    Staff6 = 0xE015,
    // Note Modifiers on Staff.
    Raise1 = 0xEB90,
    Raise2 = 0xEB91,
    Raise3 = 0xEB92,
    Raise4 = 0xEB93,
    Raise5 = 0xEB94,
    Raise6 = 0xEB95,
    Raise7 = 0xEB96,
    Raise8 = 0xEB97,
    Lower1 = 0xEB98,
    Lower2 = 0xEB99,
    Lower3 = 0xEB9A,
    Lower4 = 0xEB9B,
    Lower5 = 0xEB9C,
    Lower6 = 0xEB9D,
    Lower7 = 0xEB9E,
    Lower8 = 0xEB9F,

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
    TimeSig0 = 0xE080,
    TimeSig1 = 0xE081,
    TimeSig2 = 0xE082,
    TimeSig3 = 0xE083, // 421
    TimeSig4 = 0xE084, // 470
    TimeSig5 = 0xE085,
    TimeSig6 = 0xE086,
    TimeSig7 = 0xE087,
    TimeSig8 = 0xE088,
    TimeSig9 = 0xE089,
    // TODO ???
    TimeSigNum0 = 0xF5B7, // 0x0030
    TimeSigNum1 = 0xF5B9, // 0x0031
    TimeSigNum2 = 0xF5BB, // 0x0032
    TimeSigNum3 = 0xF5BD, // 0x0033
    TimeSigNum4 = 0xF5BF, // 0x0034
    TimeSigNum5 = 0xF5C1, // 0x0035
    TimeSigNum6 = 0xF5C3, // 0x0036
    TimeSigNum7 = 0xF5C5, // 0x0037
    TimeSigNum8 = 0xF5C7, // 0x0038
    TimeSigNum9 = 0xF5C9, // 0x0039
    TimeSigDen0 = 0xF5B6,
    TimeSigDen1 = 0xF5B8,
    TimeSigDen2 = 0xF5BA,
    TimeSigDen3 = 0xF5BC,
    TimeSigDen4 = 0xF5BE,
    TimeSigDen5 = 0xF5C0,
    TimeSigDen6 = 0xF5C2,
    TimeSigDen7 = 0xF5C4,
    TimeSigDen8 = 0xF5C6,
    TimeSigDen9 = 0xF5C8,
    // 4/4
    TimeSigCommon = 0xE08A,
    // 2/2
    TimeSigCut = 0xE08B,
    TimeSigPlus = 0xE08C,
    TimeSigNumPlus = 0xE08D, // 0x002B

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
    out.push_str(&format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#{:x}\"/>", x, y, u as u32));
}

// Add a stem downwards.
fn stem_d(out: &mut String, x: i32, y: i32) {
    out.push_str(&format!("<path transform=\"matrix(1 0 0 -1 {} {})\" d=\"M15 0h-30v875l30 -50v-875z\"/>", x, y));
}

// Add a stem upwards.
fn stem_u(out: &mut String, x: i32, y: i32) {
    out.push_str(&format!("<path transform=\"matrix(1 0 0 -1 {} {})\" d=\"M16 -50l-30 50v875h30v-875z\"/>", x, y - 3));
}


fn staff(out: &mut String, x: i32, y: i32, w: i32) {
    out.push_str(&format!("<path transform=\"matrix(1 0 0 1 {} {})\" d=\"M0 -16h{}v32h-{}v-32z\"/>", x, y, w, w));
}

/// Generate some test score.
pub fn test_svg(vfont: &str) -> String {
    // Header
    let mut out = "<svg viewBox=\"0 0 8192 2048\">".to_string();
    out.push_str(vfont);

//    // Bodyer
//    for i in 0..16 {
//        stamp(&mut out, Staff5, i * 500 + 96, STEP * 12);
//    }
    for i in 0..5 {
        // 5 line staff
        staff(&mut out, 96, STEP * (4 + i * 2), 8000);
    }
    // Clef
    stamp(&mut out, ClefC, 96, STEP * 8);
    // Time Signature
    stamp(&mut out, TimeSig3, 96 + 899, STEP * 6); // 421
    stamp(&mut out, TimeSig4, 96 + 899 - ((470 - 421) / 2), STEP * 10); // 470
    // Draw 
    stamp(&mut out, NoteheadHalf, 96 + 1545, STEP * 7);
    stem_d(&mut out, 96 + 1560, 41 + STEP * 14);
    // Draw
    stamp(&mut out, NoteheadHalf, 96 + 1545 + 500, STEP * 9);
    stem_u(&mut out, 96 + (1560 + 264) + 500, 41 - 132 - 875 + STEP * 16);

    // Draw 
    stamp(&mut out, NoteheadFill, 96 + 1545 + 1000, STEP * 5);
    stem_d(&mut out, 96 + 1560 + 1000, 41 + STEP * 12);
    // Draw
    stamp(&mut out, NoteheadFill, 96 + 1545 + 1500, STEP * 11);
    stem_u(&mut out, 96 + (1560 + 264) + 1500, 41 - 132 - 875 + STEP * 18);

//    stamp(&mut out, ClefCChange, 96 + 699, 512 + (STEP * 4));

    // Footer
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
