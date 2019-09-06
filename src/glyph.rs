#![allow(unused)] // FIXME: For now, until all of the glyphs are implemented.

/// Different parts of the music that can be drawn.
///
/// The IDs match SMuFL.  
#[repr(u32)]
#[derive(Clone, Copy)]
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
    FlagUp8 = 0xE240,             // LP: 0xE21D
    FlagDown8 = 0xE241,           // LP: 0xE222
    FlagUp16 = 0xE242,            // LP: 0xE21E
    FlagDown16 = 0xE243,          // LP: 0xE223
    FlagUp32 = 0xE244,            // LP: 0xE21F
    FlagDown32 = 0xE245,          // LP: 0xE224
    FlagUp64 = 0xE246,            // LP: 0xE220
    FlagDown64 = 0xE247,          // LP: 0xE225
    FlagUp128 = 0xE248,           // LP: 0xE221
    FlagDown128 = 0xE249,         // LP: 0xE226
    NoteheadDoubleWhole = 0xE0A0, // 0xE191
    NoteheadDoubleWholeX = 0xE0A6,
    NoteheadDoubleWholeSquare = 0xE0A1,
    NoteheadDoubleWholeWithX = 0xE0B4,
    // Whole Note & Half Note Notehead (4 beats)
    NoteheadOutlineSquare = 0xE0B8,
    NoteheadOutlineLargeSquare = 0xE11B,
    // Whole Note
    NoteheadWhole = 0xE0A2,  // 0xE192
    NoteheadWholeX = 0xE0A7, // 0xE1A0
    // Half Note
    NoteheadHalf = 0xE0A3,  // 0xE193
    NoteheadHalfX = 0xE0A8, // 0xE1A1
    // Quarter And Smaller Note Nothead (< 2 beats)
    NoteheadFill = 0xE0A4,  // 0xE194
    NoteheadFillX = 0xE0A9, // 0xE1A2
    NoteheadSquare = 0xE0B9,
    NoteheadLargeSquare = 0xE11A,

    // -- RESTS --
    // Whole Rest
    Rest1 = 0xE4E3, // LP: 0xE100
    // Half Rest
    Rest2 = 0xE4E4, // LP: 0xE101
    // Quarter Rest (2 Styles Default=Modern)
    Rest4 = 0xE4E5,    // LP: 0xE108
    Rest4Old = 0xE4F2, // LP: 0x109
    // ...
    Rest8 = 0xE4E6,   // LP: E10A
    Rest16 = 0xE4E7,  // LP: E10B
    Rest32 = 0xE4E8,  // LP: E10C
    Rest64 = 0xE4E9,  // LP: E10D
    Rest128 = 0xE4EA, // LP: E10E
    PitchPlop = 0xE5E0,
    PitchScoop = 0xE5D0,
    PitchSmear = 0xE5E2,

    // Coda
    Coda = 0xE048,
    CodaSquare = 0xE049,

    // Segno
    Segno = 0xE047,

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

impl GlyphId {
    /// Get the glyph for a rest with a specific duration
    pub(super) fn rest_duration(duration: u8) -> Self {
        use GlyphId::*;

        match duration {
            // 0 => Rest1, // FIXME: Double Whole Rest?
            1 => Rest1,
            2 => Rest2,
            4 => Rest4,
            8 => Rest8,
            16 => Rest16,
            32 => Rest32,
            64 => Rest64,
            128 => Rest128,
            _ => unreachable!(),
        }
    }

    /// Get the flag glyph for a note with a specific duration
    pub(super) fn flag_duration(duration: u8, up: bool) -> Option<Self> {
        use GlyphId::*;

        Some(match duration {
            1 | 2 | 4 => return None,
            8 => {
                if up {
                    FlagUp8
                } else {
                    FlagDown8
                }
            }
            16 => {
                if up {
                    FlagUp16
                } else {
                    FlagDown16
                }
            }
            32 => {
                if up {
                    FlagUp32
                } else {
                    FlagDown32
                }
            }
            64 => {
                if up {
                    FlagUp64
                } else {
                    FlagDown64
                }
            }
            128 => {
                if up {
                    FlagUp128
                } else {
                    FlagDown128
                }
            }
            _ => unreachable!(),
        })
    }

    /// Get the notehead glyph for a note with a specific duration
    pub(super) fn notehead_duration(duration: u8) -> Self {
        use GlyphId::*;
        Self::notehead_variants(
            NoteheadWhole,
            NoteheadHalf,
            NoteheadFill,
            duration,
        )
    }

    /// Get the notehead glyph for a note with a specific duration
    pub(super) fn x_notehead_duration(duration: u8) -> Self {
        use GlyphId::*;
        Self::notehead_variants(
            NoteheadWholeX,
            NoteheadHalfX,
            NoteheadFillX,
            duration,
        )
    }

    /// Get the square notehead glyph for a note with a specific duration
    pub(super) fn square_notehead_duration(duration: u8) -> Self {
        use GlyphId::*;
        Self::notehead_variants(
            NoteheadOutlineSquare,
            NoteheadOutlineSquare,
            NoteheadSquare,
            duration,
        )
    }

    /// Get the large square notehead glyph for a note with a specific duration
    pub(super) fn large_square_notehead_duration(duration: u8) -> Self {
        use GlyphId::*;
        Self::notehead_variants(
            NoteheadOutlineLargeSquare,
            NoteheadOutlineLargeSquare,
            NoteheadLargeSquare,
            duration,
        )
    }

    /// Given a duration and set of notehead glyphs, choose appropriate glyph
    fn notehead_variants(
        whole: GlyphId,
        half: GlyphId,
        fill: GlyphId,
        duration: u8,
    ) -> Self {
        if duration > 4 {
            fill
        } else {
            match duration {
                // 0 => NoteheadDoubleWhole,
                1 => whole,
                2 => half,
                4 => fill,
                _ => unreachable!(),
            }
        }
    }
}
