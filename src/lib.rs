mod glyph;
pub use glyph::GlyphId;

use scof::{Cursor, Marking, Note, Scof};
use std::fmt;

/// Width of one bar (measure)
const BAR_WIDTH: i32 = 3200;
/// Width of the barline.
const BARLINE_WIDTH: i32 = 36;
/// Space before each note.
const NOTE_MARGIN: i32 = 250;
/// Color of cursor
const CURSOR_COLOR: u32 = 0xff14e2;

/// Get Bravura font paths
pub fn bravura() -> Vec<Path> {
    include!("vfont/bravura.vfont")
}

/// Note duration within a measure
#[derive(Clone, Copy)]
struct Duration {
    num: u8,
    den: u8,
}

impl Duration {
    /// Create a new note duration
    fn new(dur: (u8, u8)) -> Self {
        let num = dur.0;
        let den = dur.1;
        Duration { num, den }
    }
    /// Get the width
    fn width(&self) -> i32 {
        let num = f32::from(self.num);
        let den = f32::from(self.den);
        (BAR_WIDTH as f32 * num * den.recip()) as i32
    }
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub fill: Option<String>,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"",
            self.x, self.y, self.width, self.height)?;
        if let Some(ref fill) = self.fill {
            write!(f, " fill=\"{}\"", fill)?;
        }
        write!(f, "/>")
    }
}

impl Rect {
    fn new(x: i32, y: i32, width: u32, height: u32, fill: Option<u32>) -> Self {
        let fill = match fill {
            Some(f) => Some(format!("#{:x}", f)),
            None => None,
        };
        Rect { x, y, width, height, fill }
    }
}

pub struct Use {
    pub x: i32,
    pub y: i32,
    pub glyph: GlyphId,
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
    pub x: i32,
    pub y: i32,
    pub elements: Vec<Element>,
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
    pub id: Option<String>,
    pub d: String,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref id) = self.id {
            write!(f, "<path id=\"{}\" d=\"{}\"/>", id, self.d)
        } else {
            write!(f, "<path d=\"{}\"/>", self.d)
        }
    }
}

impl Path {
    fn new<T: Into<String>>(id: Option<T>, d: T) -> Self {
        let id = match id {
            Some(id) => Some(id.into()),
            None => None,
        };
        let d = d.into();
        Path { id, d }
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

/// Staff lines
pub struct Staff {
    pub lines: i32,
}

impl Staff {
    /// A half or whole step visual distance in the measure.
    const STEP_DY: i32 = 125;
    /// Margin X
    const MARGIN_X: i32 = 96;
    /// Margin Y
    const MARGIN_Y: i32 = Staff::STEP_DY * 4;
    /// Width of staff lines (looks best if it matches BARLINE_WIDTH).
    const LINE_WIDTH: i32 = BARLINE_WIDTH;

    /// Create a new staff
    pub fn new(lines: i32) -> Self {
        Staff { lines }
    }

    /// Get the height of the staff
    pub fn height(&self) -> i32 {
        if self.lines > 0 {
            let spaces = self.lines - 1;
            Staff::STEP_DY * spaces * 2
        } else {
            0
        }
    }

    /// Get the middle of the staff
    pub fn middle(&self) -> i32 {
        self.height() / 2
    }

    /// Create a staff path
    pub fn path(&self, width: i32) -> Path {
        let mut d = String::new();
        for i in 0..self.lines {
            let x = 0;
            let y = Staff::MARGIN_Y + Staff::STEP_DY * (i * 2)
                - Staff::LINE_WIDTH / 2;
            let line = &format!("M{} {}h{}v{}h-{}v-{}z", x, y, width,
                Staff::LINE_WIDTH, width, Staff::LINE_WIDTH);
            d.push_str(line);
        }
        Path::new(None, d)
    }
}

pub struct MeasureElem {
    pub staff: Staff,
    pub elements: Vec<Element>,
    pub width: i32,
}

impl fmt::Display for MeasureElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.elements {
            write!(f, "{}", elem)?;
        }
        Ok(())
    }
}

impl MeasureElem {
    /// Create a new measure element
    pub fn new(staff: Staff) -> Self {
        let elements = vec![];
        let width = 0;
        MeasureElem { staff, elements, width }
    }
    /// Add markings
    ///
    /// - `scof`: The score.
    /// - `curs`: Cursor of measure.
    pub fn add_markings(&mut self, scof: &Scof, curs: &mut Cursor) {
        let mut is_empty = true;
        while let Some(marking) = scof.marking(&curs) {
            is_empty = false;
            match marking {
                Marking::Note(note) => {
                    let duration = Duration::new(note.duration);
                    self.add_mark(&note);
                    self.width += duration.width();
                },
                _ => unreachable!(),
            }
            curs.right_unchecked();
        }

        if is_empty {
            if let Ok(note) = "1R".parse::<Note>() {
                let duration = Duration::new(note.duration);
                self.add_rest(&note);
                self.width += duration.width();
            }
        }

        self.add_rect(self.width, BARLINE_WIDTH as u32, None);
    }

    /// Add a cursor
    /// - `cursor`: Cursor position.
    pub fn add_cursor(&mut self, scof: &Scof, cursor: &Cursor) {
        let mut width = 0;
        let mut curs = cursor.first_marking();
        while let Some(Marking::Note(note)) = scof.marking(&curs) {
            let duration = Duration::new(note.duration);
            if *cursor == curs {
                let x = width + BARLINE_WIDTH / 2;
                let w = duration.width() - BARLINE_WIDTH / 2;
                if w > 0 {
                    self.add_rect(x, w as u32, Some(CURSOR_COLOR));
                }
                break;
            }
            width += duration.width();
            curs.right_unchecked();
        }
    }

    /// Add a rectangle from top to bottom of staff
    fn add_rect(&mut self, x: i32, width: u32, fill: Option<u32>) {
        let rect = Rect::new(x, Staff::MARGIN_Y, width,
            self.staff.height() as u32, fill);
        self.elements.push(Element::Rect(rect));
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
        let y = Staff::STEP_DY * note.visual_distance();
        let cp = GlyphId::notehead_duration(duration.den);
        self.add_use(cp, NOTE_MARGIN + self.width, y);
        // Only draw stem if not a whole note.
        if duration.num != 1 || duration.den != 1 {
            self.add_stem(NOTE_MARGIN + self.width, y);
        }
    }

    /// Add a stem
    fn add_stem(&mut self, x: i32, y: i32) {
        let y = y + self.staff.height();
        if y < Staff::MARGIN_Y + self.staff.middle() {
            self.add_stem_down(x, y);
        } else {
            self.add_stem_up(x, y);
        }
    }

    /// Add a stem downwards.
    fn add_stem_down(&mut self, x: i32, y: i32) {
        let len = (7.3 * Staff::STEP_DY as f32) as i32;
        let half = 15;
        let whole = 30;
        let d = format!("M{} {} v{} c0 {}-{} {}-{} 0v-{}l{} {}z", x + whole, y,
            len, half, whole, half, whole, len - whole, whole, whole);
        self.elements.push(Element::Path(Path::new(None, d)));
    }

    /// Add a stem upwards.
    fn add_stem_up(&mut self, x: i32, y: i32) {
        let len = (7.3 * Staff::STEP_DY as f32) as i32;
        let half = 15;
        let whole = 30;
        let head = 292;
        let d = format!("M{} {}v-{}c0-{}-{}-{}-{} 0v{}l{}-{}z", x + head, y,
            len, half, whole, half, whole, len, whole, whole);
        self.elements.push(Element::Path(Path::new(None, d)));
    }

    /// Add `use` element for a rest
    fn add_rest(&mut self, note: &Note) {
        let duration = Duration::new(note.duration);
        let glyph = GlyphId::rest_duration(duration.den);
        let x = NOTE_MARGIN + self.width;
        let mut y = 0;
        // Position whole rest glyph up 2 steps.
        if duration.num == 1 && duration.den == 1 {
            y -= Staff::STEP_DY * 2;
        }
        self.add_use(glyph, x, y);
    }

    /// Add use element
    fn add_use(&mut self, glyph: GlyphId, x: i32, y: i32) {
        let y = self.staff.height() + y;
        self.elements.push(Element::Use(Use::new(x, y, glyph)));
    }

    /// Add staff
    pub fn add_staff(&mut self) {
        let path = self.staff.path(self.width);
        self.elements.push(Element::Path(path))
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

    // stamp(out, NoteheadFill, offset_x + 2000, STAFF_DY - STEP_DY * 3);
    // Clef
    // stamp(out, ClefC, 96, STAFF_DY);
    // Time Signature
    // stamp(out, TimeSig3, 96 + STAFF_DY, STAFF_DY - STEP_DY * 2); // 421
    // stamp(out, TimeSig4, 96 + STAFF_DY - ((470 - 421) / 2), STAFF_DY + STEP_DY * 2); // 470
    // stamp(out, NoteheadHalf, 96 + 2000 + 2000, STAFF_DY);
    // stamp(out, NoteheadHalf, 96 + 2000 + 4400, STAFF_DY);
}
