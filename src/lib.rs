pub use svg;

use std::fmt;

use scof::{Marking, Note, Scof};
mod glyph;

use glyph::GlyphId::{self, *};

/// Default font (Bravura).
pub const DEFAULT: &str = include_str!("vfont/bravura.vfont");

const BAR_WIDTH: i32 = 3200;
const BAR_W: f32 = BAR_WIDTH as f32;

// Width of the barline.
const BARLINE_WIDTH: i32 = 32;
// Width of the staff lines (looks best if it matches BARLINE_WIDTH).
const STAFF_WIDTH: i32 = BARLINE_WIDTH;
// A half or whole step visual distance in the measure.
const STEP_DY: i32 = 125;
// To traverse the whole height of the staff, you need 8 steps (4 spaces).
const STAFF_DY: i32 = STEP_DY * 8;
// Margin X.
const STAFF_MARGIN_X: i32 = 96;
// Margin Y.
const STAFF_MARGIN_Y: i32 = STEP_DY * 4;

// Space before each note.
const NOTE_MARGIN: i32 = 250;

const CURSOR_COLOR: &str = "ff14e2";

pub struct Renderer<'a> {
    out: String,
    scof: &'a Scof,
    chan: usize,
    curs: usize,
    measure: usize,
    vfont: &'a str,
    screen_width: i32,
    /// - `offset_x`: Offset from X origin.
    offset_x: i32,
}

impl<'a> Renderer<'a> {
    /// - `scof`: The score.
    /// - `chan`: Which channel the bar is on.
    /// - `curs`: User cursor (within measure).
    /// - `measure`: User cursor (which measure).
    /// - `vfont`: The font we're using (currently must be bravura).
    /// - `screen_width`: How many units within the viewport width.
    pub fn new(
        scof: &'a Scof,
        chan: usize,
        curs: usize,
        measure: usize,
        vfont: &'a str,
        screen_width: i32,
    ) -> Self {
        let out = String::new();
        let offset_x = 0;
        Renderer {
            out,
            scof,
            chan,
            curs,
            measure,
            vfont,
            screen_width,
            offset_x,
        }
    }

    /// Generate an SVG string.
    pub fn render(mut self) -> String {
        self.out.push_str("<svg>\n");
        self.out.push_str(self.vfont);
        self.render_body();
        self.out.push_str("</svg>");
        self.out
    }

    /// Generate body of SVG string.
    fn render_body(&mut self) {
        let screen_width = self.screen_width - STAFF_MARGIN_X;

        for measure in 0..9 {
            let position = BAR_WIDTH * measure as i32;
            if position > screen_width {
                break;
            }

            self.offset_x = STAFF_MARGIN_X + position;
            let mut cursor = Cursor::new();
            cursor.chan = self.chan;
            cursor.measure = self.measure;
            cursor.marking = self.curs;
            let mut elem = MeasureElem::new(self.offset_x, 0);
            elem.add_markings(self.scof, self.chan, measure, &cursor);
            self.out.push_str(&format!("{}", elem));
        }

        // 5 line staff
        for i in 0..5 {
            self.staff(
                STAFF_MARGIN_X,
                STEP_DY * (i * 2) + STAFF_MARGIN_Y,
                screen_width,
            );
        }
    }

    /// Render staff lines at a specific position
    fn staff(&mut self, x: i32, y: i32, w: i32) {
        self.out.push_str(&format!(
            "<path d=\"M{} {}h{}v{}h-{}v-{}z\"/>\n",
            x,
            y - STAFF_WIDTH / 2,
            w,
            STAFF_WIDTH,
            w,
            STAFF_WIDTH
        ));
    }
}

pub struct Cursor {
    /// Channel number
    chan: usize,
    /// Measure number
    measure: usize,
    /// Marking number within measure
    marking: usize,
}

impl Cursor {
    /// Create a new cursor
    pub fn new() -> Self {
        let chan = 0;
        let measure = 0;
        let marking = 0;
        Cursor { chan, measure, marking }
    }
    fn is_at(&self, chan: usize, measure: usize, marking: usize) -> bool {
        self.chan == chan &&
        self.measure == measure &&
        self.marking == marking
    }
}

#[derive(Clone, Copy)]
struct Duration {
    num: u8,
    den: u8,
}

impl Duration {
    fn new(dur: (u8, u8)) -> Self {
        let num = dur.0;
        let den = dur.1;
        Duration { num, den }
    }
    fn width(&self) -> i32 {
        let num = f32::from(self.num);
        let den = f32::from(self.den);
        (BAR_W * num * den.recip()) as i32
    }
}

pub struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\"/>",
            self.x, self.y, self.width, self.height, CURSOR_COLOR)
    }
}

impl Rect {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rect { x, y, width, height }
    }
}

pub struct Use {
    x: i32,
    y: i32,
    glyph: GlyphId,
}

impl fmt::Display for Use {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<use x=\"{}\" y=\"{}\" xlink:href=\"#{:x}\"/>", self.x,
            self.y, self.glyph as u32)
    }
}

impl Use {
    fn new(x: i32, y: i32, glyph: GlyphId) -> Self {
        Use { x, y, glyph }
    }
}

pub struct Group {
    x: i32,
    y: i32,
    elements: Vec<Element>,
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<g transform=\"translate({} {})\">", self.x, self.y)?;
        for elem in &self.elements {
            write!(f, "{}", elem);
        }
        write!(f, "</g>")
    }
}

impl Group {
    fn new(x: i32, y: i32) -> Self {
        let elements = vec![];
        Group { x, y, elements }
    }
    fn push(&mut self, elem: Element) {
        self.elements.push(elem);
    }
}

pub struct Path {
    d: String,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<path d=\"{}\"/>", self.d)
    }
}

impl Path {
    fn new(d: String) -> Self {
        Path { d }
    }
}

pub enum Element {
    Group(Group),
    Rect(Rect),
    Use(Use),
    Path(Path),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Group(g) => g.fmt(f),
            Element::Rect(r) => r.fmt(f),
            Element::Use(u) => u.fmt(f),
            Element::Path(p) => p.fmt(f),
        }
    }
}

pub struct MeasureElem {
    group: Group,
    x: i32, // FIXME: remove these when groups are supported
    y: i32, // FIXME: remove these when groups are supported
    width: i32,
}

impl fmt::Display for MeasureElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.group)
    }
}

impl MeasureElem {
    /// Create a new measure element
    pub fn new(x: i32, y: i32) -> Self {
        let group = Group::new(x, y);
        let width = 0;
        MeasureElem { group, x, y, width }
    }
    /// Add markings
    ///
    /// - `scof`: The score.
    /// - `chan`: Channel number.
    /// - `measure`: Measure number.
    /// - `cursor`: Current cursor position.
    pub fn add_markings(&mut self, scof: &Scof, chan: usize, measure: usize,
        cursor: &Cursor)
    {
        let mut curs = 0;

        while let Some(marking) = scof.marking(measure, chan, curs) {
            match marking {
                Marking::Note(note) => {
                    let duration = Duration::new(note.duration);
                    if cursor.is_at(chan, measure, curs) {
                        self.add_cursor(duration);
                    }
                    self.add_mark(&note);
                    self.width += duration.width();
                },
                _ => unreachable!(),
            }
            curs += 1;
        }

        if curs == 0 {
            self.add_use(Rest1, NOTE_MARGIN, STEP_DY * 6);
        }

        self.add_use(Barline, BAR_WIDTH, STAFF_MARGIN_Y + STAFF_DY);
    }

    fn add_cursor(&mut self, duration: Duration) {
        self.group.push(Element::Rect(Rect::new(
            self.x + self.width + BARLINE_WIDTH,
            STEP_DY * 4,
            duration.width() as u32,
            STAFF_DY as u32)));
    }

    /// Add mark node for either a note or a rest
    fn add_mark(&mut self, note: &Note) {
        match &note.pitch {
            Some(_pitch) => self.add_pitch(note),
            None => self.add_rest(note),
        }
    }

    /// Add elements for a note
    fn add_pitch(&mut self, note: &Note) {
        let duration = Duration::new(note.duration);
        let offset_y = STEP_DY * note.visual_distance();

        let cp = GlyphId::notehead_duration(duration.den);
        self.add_use(cp, self.width + NOTE_MARGIN, STAFF_DY + offset_y);
        self.add_stem_down(self.width + NOTE_MARGIN + 15, offset_y);
    }

    /// Add a stem downwards.
    fn add_stem_down(&mut self, x: i32, y: i32) {
        // FIXME: Calculated from constants.
        let d = format!("M{} {}c-1 14-29 14-30 0v-855l30 50v855z",
            self.x + x + 15,
            y + 1895);
        self.group.push(Element::Path(Path::new(d)));
    }

    /// Add a stem upwards.
    fn add_stem_up(&mut self, x: i32, y: i32) {
        // FIXME: Calculated from constants. 910 (805+105)
        let d = format!("M{} {}c-1 -14-29-14-30 0v805l30 50v-805z",
            self.x + x + 15,
            y + 105);
        self.group.push(Element::Path(Path::new(d)));
    }

    /// Add `use` element for a rest
    fn add_rest(&mut self, note: &Note) {
        let duration = Duration::new(note.duration);
        let glyph = GlyphId::rest_duration(duration.den);
        let x = self.width + NOTE_MARGIN;
        let y = STAFF_DY;
        self.add_use(glyph, x, y);
    }

    /// Add use element
    fn add_use(&mut self, glyph: GlyphId, x: i32, y: i32) {
        self.group.push(Element::Use(Use::new(self.x + x, y, glyph)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect() {
        assert_eq!(Rect::new(10, 12, 25, 20).to_string(),
        "<rect x=\"10\" y=\"12\" width=\"25\" height=\"20\" fill=\"#ff14e2\"/>");
    }

    #[test]
    fn glyph() {
        assert_eq!(Use::new(37, 21, GlyphId::StemHarpStringNoise).to_string(),
        "<use x=\"37\" y=\"21\" xlink:href=\"#e21f\"/>");
    }

    #[test]
    fn group() {
        let mut group = Group::new();
        group.push(Element::Use(Use::new(2, 3, GlyphId::NoteheadWhole)));
        assert_eq!(group.to_string(),
        "<g><use x=\"2\" y=\"3\" xlink:href=\"#e0a2\"/></g>");
    }

    //    stamp(out, NoteheadFill, offset_x + 2000, STAFF_DY - STEP_DY * 3);
    //    stem_down(out, offset_x + 2000 + 15, -STEP_DY * 3);
    /*    // Clef
    stamp(out, ClefC, 96, STAFF_DY);
    // Time Signature
    stamp(out, TimeSig3, 96 + STAFF_DY, STAFF_DY - STEP_DY * 2); // 421
    stamp(out, TimeSig4, 96 + STAFF_DY - ((470 - 421) / 2), STAFF_DY + STEP_DY * 2); // 470*/

    /*    // Draw
    stamp(out, NoteheadHalf, 96 + 2000 + 2000, STAFF_DY);
    stem_down(out, 96 + 2000 + 2000 + 15, 0);

    // Barline
    stamp(out, Barline, 96 + 2000 + 4000, 1500);

    // Draw
    stamp(out, NoteheadHalf, 96 + 2000 + 4400, STAFF_DY);
    stem_up(out, 96 + (2000 + 265) + 4400 + 15, 0);*/
}
