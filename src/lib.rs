/// Different parts of the music that can be drawn.
///
/// The IDs match SMuFL.  
#[repr(u32)]
pub enum Glyph {
    // Stem For Notes
    Stem = 0xE210,
    StemBuzzRoll = 0xE217,
    StemDamp = 0xE218,
    StemHarpStringNoise = 0xE21F,
    StemRimShot = 0xE21E,
    StemBowBridge = 0xE215,
    StemBowTailpiece = 0xE216,

    // 8th-1024th Flags
    FlagUp8 = 0xE240,
    FlagDown8 = 0xE241,
    FlagUp16 = 0xE242,
    FlagDown16 = 0xE243,
    FlagUp32 = 0xE244,
    FlagDown32 = 0xE245,
    FlagUp64 = 0xE246,
    FlagDown64 = 0xE247,
    FlagUp128 = 0xE248,
    FlagDown128 = 0xE249,
    FlagUp256 = 0xE24A,
    FlagDown256 = 0xE24B,
    FlagUp512 = 0xE24C,
    FlagDown512 = 0xE24D,
    FlagUp1024 = 0xE24E,
    FlagDown1024 = 0xE24F,

    // -- NOTEHEADS --
    // Double Whole Note Notehead (8 beats)
    NoteheadDoubleWhole = 0xE0A0,
    NoteheadDoubleWholeX = 0xE0A6,
    NoteheadDoubleWholeSquare = 0xE0A1,
    NoteheadDoubleWholeWithX = 0xE0B4,
    // Whole Note & Half Note Notehead (4 beats)
    NoteheadOutlineSquare = 0xE0B8,
    NoteheadOutlineLargeSquare = 0xE11B,
    // Whole Note
    NoteheadWhole = 0xE0A2,
    NoteheadWholeX = 0xE0A7,
    // Half Note
    NoteheadHalf = 0xE0A3,
    NoteheadHalfX = 0xE0A8,
    // Quarter And Smaller Note Nothead (< 2 beats)
    NoteheadFill = 0xE0A4,
    NoteheadFillX = 0xE0A9,
    NoteheadSquare = 0xE0B9,
    NoteheadLargeSquare = 0xE11A,

    // -- RESTS --
    // Whole Rest
    Rest1 = 0xE4E3,
    // Half Rest
    Rest2 = 0xE4E4,
    // Quarter Rest (2 Styles Default=Modern)
    Rest4 = 0xE4E5,
    Rest4Old = 0xE4F2,
    // ...
    Rest8 = 0xE4E6,
    Rest16 = 0xE4E7,
    Rest32 = 0xE4E8,
    Rest64 = 0xE4E9,
    Rest128 = 0xE4EA,
    Rest256 = 0xE4EB,
    Rest512 = 0xE4EC,
    Rest1024 = 0xE4ED,

    // Pitch Bends
    PitchPlop = 0xE5E0,
    PitchScoop = 0xE5D0,
    PitchSmear = 0xE5E2,

    // Coda
    Coda = 0xE048,
    CodaSquare = 0xE049,

    // Segno
    Segno = 0xE047,

    // Measure Repeats
    MeasureRepeat1 = 0xE500,
    MeasureRepeat2 = 0xE501,
    MeasureRepeat4 = 0xE502,

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
    FlatDouble = 0xE264,
    FlatDoubleEqual = 0xE2F0,
    FlatDoubleFlatComma1 = 0xE2C0,
    FlatDoubleSharpComma1 = 0xE2C5,
    FlatDoubleFlatComma2 = 0xE2CA,
    FlatDoubleSharpComma2 = 0xE2CF,
    FlatDoubleFlatComma3 = 0xE2D4,
    FlatDoubleSharpComma3 = 0xE2D9,
    // Differently Tuned Flat
    Flat = 0xE260,
    FlatEqual = 0xE2F1,
    FlatFlatComma1 = 0xE2C1,
    FlatSharpComma1 = 0xE2C6,
    FlatFlatComma2 = 0xE2CB,
    FlatSharpComma2 = 0xE2D0,
    FlatFlatComma3 = 0xE2D5,
    FlatSharpComma3 = 0xE2DA,
    // Differently Tuned Natural
    Natural = 0xE261,
    NaturalEqual = 0xE2F2,
    NaturalFlatComma1 = 0xE2C2,
    NaturalSharpComma1 = 0xE2C7,
    NaturalFlatComma2 = 0xE2CC,
    NaturalSharpComma2 = 0xE2D1,
    NaturalFlatComma3 = 0xE2D6,
    NaturalSharpComma3 = 0xE2DB,
    // Differently Tuned Sharp
    Sharp = 0xE262,
    SharpEqual = 0xE2F3,
    SharpFlatComma1 = 0xE2C3,
    SharpSharpComma1 = 0xE2C8,
    SharpFlatComma2 = 0xE2CD,
    SharpSharpComma2 = 0xE2D2,
    SharpFlatComma3 = 0xE2D7,
    SharpSharpComma3 = 0xE2DC,
    // Differently Tuned Double Sharp
    SharpDouble = 0xE263,
    SharpDoubleEqual = 0xE2F4,
    SharpDoubleFlatComma1 = 0xE2C4,
    SharpDoubleSharpComma1 = 0xE2C9,
    SharpDoubleFlatComma2 = 0xE2CE,
    SharpDoubleSharpComma2 = 0xE2D3,
    SharpDoubleFlatComma3 = 0xE2D8,
    SharpDoubleSharpComma3 = 0xE2DD,
    // Quarter Tones (2 styles - default=Stein-Zimmerman)
    SharpQuarter3 = 0xE283,
    SharpQuarter3Busotti = 0xE474,
    FlatQuarter3 = 0xE281,
//    FlatQuarter3Busotti = ,
    FlatQuarter1 = 0xE280,
    FlatQuarter1Iranian = 0xE460, // Koron
    FlatQuarter1Numeric = 0xE47F,
    SharpQuarter1 = 0xE282,
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

    // Clef
    ClefTab4 = 0xE06E,
    ClefTab6 = 0xE06D,
    ClefC = 0xE05C,
    ClefCChange = 0xE07B,

    // Time Signature
    TimeSig0 = 0xE080,
    TimeSig1 = 0xE081,
    TimeSig2 = 0xE082,
    TimeSig3 = 0xE083,
    TimeSig4 = 0xE084,
    TimeSig5 = 0xE085,
    TimeSig6 = 0xE086,
    TimeSig7 = 0xE087,
    TimeSig8 = 0xE088,
    TimeSig9 = 0xE089,
    TimeSig44 = 0xE08A,
    TimeSig22 = 0xE08B,
    TimeSigPlus = 0xE08C,
    TimeSigNumPlus = 0xE08D,

    // Tremelo
    Tremelo1 = 0xE220,
    Tremelo2 = 0xE221,
    Tremelo3 = 0xE222,
    Tremelo4 = 0xE223,
    Tremelo5 = 0xE224,

    // Dynamics
    PPPPPP = 0xE527,
    PPPPP = 0xE528,
    PPPP = 0xE529,
    PPP = 0xE52A,
    PP = 0xE52B,
    P = 0xE520,
    MP = 0x,
    MF = 0x,
    F = 0x,
    FF = 0x,
    FFF = 0x,
    FFFF = 0x,
    FFFFF = 0x,
    FFFFFF = 0x,

    // Beam TODO: How do we do this?
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
