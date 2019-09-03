pub use svg;

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

const CURSOR_COLOR: &str = "F0F";

// Add a stem downwards.
#[allow(unused)] // FIXME
fn stem_d(out: &mut String, x: i32, y: i32) {
    // FIXME: Calculated from constants.
    out.push_str(&format!("<path d=\"M{} {}c-1 14-29 14-30 0v-855l30 50v855z\"/>\n", x + 15, y + 1895));
}

// Add a stem upwards.
#[allow(unused)] // FIXME
fn stem_u(out: &mut String, x: i32, y: i32) {
    // FIXME: Calculated from constants. 910 (805+105)
    out.push_str(&format!("<path d=\"M{} {}c-1 -14-29-14-30 0v805l30 50v-805z\"/>\n", x + 15, y + 105));
}

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
    pub fn new(scof: &'a Scof, chan: usize, curs: usize, measure: usize, vfont: &'a str,
        screen_width: i32) -> Self
    {
        let out = String::new();
        let offset_x = 0;
        Renderer { out, scof, chan, curs, measure, vfont, screen_width, offset_x }
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
            self.render_measure(measure);
            self.stamp(Barline, STAFF_MARGIN_X + position + BAR_WIDTH, STAFF_MARGIN_Y + STAFF_DY);
        }

        // 5 line staff
        for i in 0..5 {
            self.staff(STAFF_MARGIN_X, STEP_DY * (i * 2) + STAFF_MARGIN_Y, screen_width);
        }
    }

    /// - `measure`: measure number.
    fn render_measure(&mut self, measure: usize) {
        let mut empty = true;
        let mut curs = 0;

        while let Some(marking) = self.scof.marking(measure, self.chan, curs) {
            if empty {
                empty = false;
            }
            match marking {
                Marking::Note(note) => self.render_note(&note, curs, measure),
                _ => unreachable!(),
            }
            curs += 1;
        }

        if empty {
            self.stamp(Rest1, self.offset_x + NOTE_MARGIN, STEP_DY * 6);
        }
    }

    /// Add `use` element for either a note or a rest to the DOM.
    fn render_note(&mut self, note: &Note, curs: usize, measure: usize) {
        let duration = (f32::from(note.duration.0), f32::from(note.duration.1));

        // Add cursor, if it's here
        self.render_cursor(note, curs, measure, duration);

        // Render either note or rest
        if let Some(_pitch) = &note.pitch {
            self.render_pitch(&note, note.duration.1, duration);
        } else {
            self.render_rest(&note, note.duration.1, duration);
        }
        self.offset_x += (BAR_W * duration.0 * duration.1.recip()) as i32;
    }

    /// Add `use` element for a note to the DOM.
    fn render_cursor(&mut self, note: &Note, curs: usize, measure: usize, duration: (f32, f32)) {
        let duration_denom = note.duration.1;
        let width = duration.0 * duration.1.recip(); // Fraction to float.
        if curs == self.curs && measure == self.measure {
            self.cursor(self.offset_x + BARLINE_WIDTH, STEP_DY * 4, (BAR_W * width) as i32);
        }
    }

    /// Add `use` element for a note to the DOM.
    fn render_pitch(&mut self, note: &Note, dur: u8, duration: (f32, f32)) {
        // Draw 
        self.stamp(GlyphId::notehead_duration(dur), self.offset_x + NOTE_MARGIN, STAFF_DY - STEP_DY * 3);
        stem_d(&mut self.out, self.offset_x + NOTE_MARGIN + 15, -STEP_DY * 3);
/*        // Draw
        stamp(out, NoteheadFill, 96 + 2000 + STAFF_DY, STAFF_DY + STEP_DY * 3);
        stem_u(out, 96 + (2000 + 265) + STAFF_DY + 15, STEP_DY * 3);*/

        // FIXME
    }

    /// Add `use` element for a rest to the DOM.
    fn render_rest(&mut self, note: &Note, dur: u8, duration: (f32, f32)) {
        self.stamp(GlyphId::rest_duration(dur), self.offset_x + NOTE_MARGIN, STAFF_DY);
    }

    /// Render cursor at a specific position & with a specific width
    fn cursor(&mut self, x: i32, y: i32, w: i32) {
        self.out.push_str(&format!("<path d=\"M{} {}h{}v{}h-{}v-{}z\" fill=\"#{}\"/>\n", x, y, w, STAFF_DY, w, STAFF_DY, CURSOR_COLOR));
    }

    /// Render a glyph at a specific position
    fn stamp(&mut self, u: GlyphId, x: i32, y: i32) {
        self.out.push_str(&format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#{:x}\"/>\n", x, y, u as u32));
    }

    /// Render staff lines at a specific position
    fn staff(&mut self, x: i32, y: i32, w: i32) {
        self.out.push_str(&format!("<path d=\"M{} {}h{}v{}h-{}v-{}z\"/>\n", x, y - STAFF_WIDTH / 2, w, STAFF_WIDTH, w, STAFF_WIDTH));
    }
}

#[cfg(test)]
mod tests {

//    stamp(out, NoteheadFill, offset_x + 2000, STAFF_DY - STEP_DY * 3);
//    stem_d(out, offset_x + 2000 + 15, -STEP_DY * 3);
/*    // Clef
    stamp(out, ClefC, 96, STAFF_DY);
    // Time Signature
    stamp(out, TimeSig3, 96 + STAFF_DY, STAFF_DY - STEP_DY * 2); // 421
    stamp(out, TimeSig4, 96 + STAFF_DY - ((470 - 421) / 2), STAFF_DY + STEP_DY * 2); // 470*/

/*    // Draw 
    stamp(out, NoteheadHalf, 96 + 2000 + 2000, STAFF_DY);
    stem_d(out, 96 + 2000 + 2000 + 15, 0);

    // Barline
    stamp(out, Barline, 96 + 2000 + 4000, 1500);

    // Draw
    stamp(out, NoteheadHalf, 96 + 2000 + 4400, STAFF_DY);
    stem_u(out, 96 + (2000 + 265) + 4400 + 15, 0);*/
}
