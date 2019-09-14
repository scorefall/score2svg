// ScoreFall Studio - Music Composition Software
//
// Copyright (C) 2019 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
// Copyright (C) 2019 Doug P. Lau
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod glyph;
mod svg;

pub use glyph::GlyphId;
pub use svg::{Element, Group, Path, Rect, Use};

use scof::{Cursor, Marking, Note, Scof};
use std::fmt;

/// Width of one bar (measure)
const BAR_WIDTH: i32 = 3200;
/// Width of the barline.
const BARLINE_WIDTH: i32 = 36;
/// Space before each note.
const NOTE_MARGIN: i32 = 250;
/// Color of cursor
const CURSOR_COLOR: u32 = 0xFF9AF0;
/// Width of a whole rest (in font units).
const WHOLE_REST_WIDTH: i32 = 230;

/// Get Bravura font paths
pub fn bravura() -> Vec<Path> {
    include!("vfont/bravura.vfont")
}

/// Staff lines
pub struct Staff {
    /// Number of lines on staff
    pub lines: i32,
    /// Number of steps from middle C to middle staff line
    pub steps: i32,
}

impl Staff {
    /// A half or whole step visual distance in the measure.
    const STEP_DY: i32 = 125;
    /// Margin X
    const MARGIN_X: i32 = 96;
    /// Margin Y
    const MARGIN_Y: i32 = Staff::STEP_DY * 6;
    /// Width of staff lines (looks best if it matches BARLINE_WIDTH).
    const LINE_WIDTH: i32 = BARLINE_WIDTH;

    /// Create a new staff
    pub fn new(lines: i32, steps: i32) -> Self {
        Staff { lines, steps }
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

    /// Get the height of the staff plus any related glyphs below or above.
    pub fn virtual_height(&self) -> i32 {
        self.height() + Self::MARGIN_Y * 2
    }

    /// Get the middle of the staff
    pub fn middle(&self) -> i32 {
        Staff::MARGIN_Y + self.height() / 2
    }

    /// Get the Y value of middle C relative to the staff
    pub fn middle_c(&self) -> i32 {
        self.middle() + Staff::STEP_DY * self.steps
    }

    /// Create a staff path
    pub fn path(&self, width: i32) -> Path {
        let mut d = String::new();
        for i in 0..self.lines {
            let x = BARLINE_WIDTH;
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
    /// Width of stems
    const STEM_WIDTH: u32 = 30;
    /// Length of stems
    const STEM_LENGTH: u32 = (7.0 * Staff::STEP_DY as f32) as u32;
    /// Width of note head
    const HEAD_WIDTH: i32 = 263;

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
                    let duration = note.duration;

                    self.add_mark(&note);
                    self.width += duration * BAR_WIDTH;
                },
                _ => unreachable!(),
            }
            curs.right_unchecked();
        }

        // Insert whole measure rest (different from whole rest).
        // whole measure rests are always 1 measure, so can be any number of
        // beats depending on the time signature.  They look like a whole rest,
        // but are centered.
        if is_empty {
            self.add_rest(None);
            self.width += BAR_WIDTH;
        }

        self.add_rect(self.width, BARLINE_WIDTH as u32, None, false);
    }

    /// Add a cursor
    /// - `cursor`: Cursor position.
    pub fn add_cursor(&mut self, scof: &Scof, cursor: &Cursor) {
        let mut width = 0;
        let mut curs = cursor.first_marking();
        while let Some(Marking::Note(note)) = scof.marking(&curs) {
            let duration = note.duration;
            if *cursor == curs {
                let x = width + BARLINE_WIDTH;
                let w = duration * BAR_WIDTH - BARLINE_WIDTH;
                if w > 0 {
                    self.add_rect(x, w as u32, Some(CURSOR_COLOR), true);
                }
                break;
            }
            width += duration * BAR_WIDTH;
            curs.right_unchecked();
        }
    }

    /// Add a rectangle from top to bottom of staff
    fn add_rect(&mut self, x: i32, width: u32, fill: Option<u32>, long: bool) {
        let (y, height) = if long {
            (0, self.staff.virtual_height() as u32)
        } else {
            (Staff::MARGIN_Y, self.staff.height() as u32)
        };
        let rect = Rect::new(x, y, width, height, None, None, fill);
        self.elements.push(Element::Rect(rect));
    }

    /// Add mark node for either a note or a rest
    fn add_mark(&mut self, note: &Note) {
        match &note.pitch {
            Some(_pitch) => self.add_pitch(note),
            None => self.add_rest(Some(note)),
        }
    }

    /// Add elements for a note
    fn add_pitch(&mut self, note: &Note) {
        let duration = note.duration;
        let y = self.staff.middle_c() + Staff::STEP_DY * note.visual_distance();
        let cp = GlyphId::notehead_duration(duration.den);
        self.add_use(cp, NOTE_MARGIN + self.width, y);
        // Only draw stem if not a whole note.
        if duration.num != 1 || duration.den != 1 {
            self.add_stem(NOTE_MARGIN + self.width, y);
        }
    }

    /// Add a stem
    fn add_stem(&mut self, x: i32, y: i32) {
        if y > self.staff.middle() {
            self.add_stem_up(x, y);
        } else {
            self.add_stem_down(x, y);
        }
    }

    /// Add a stem downwards.
    fn add_stem_down(&mut self, x: i32, y: i32) {
        // FIXME: stem should always reach the center line of the staff
        let rx = Some(Self::STEM_WIDTH / 2);
        let ry = Some(Self::STEM_WIDTH);
        let rect = Rect::new(x, y, Self::STEM_WIDTH, Self::STEM_LENGTH, rx, ry,
            None);
        self.elements.push(Element::Rect(rect));
    }

    /// Add a stem upwards.
    fn add_stem_up(&mut self, x: i32, y: i32) {
        // FIXME: stem should always reach the center line of the staff
        let rx = Some(Self::STEM_WIDTH / 2);
        let ry = Some(Self::STEM_WIDTH);
        let rect = Rect::new(x + Self::HEAD_WIDTH, y - Self::STEM_LENGTH as i32,
            Self::STEM_WIDTH, Self::STEM_LENGTH, rx, ry, None);
        self.elements.push(Element::Rect(rect));
    }

    /// Add `use` element for a rest
    fn add_rest(&mut self, note: Option<&Note>) {
        let note = if let Some(note) = note {
            note
        } else {
            let x = (BAR_WIDTH - WHOLE_REST_WIDTH) / 2;
            let y = self.staff.middle() - Staff::STEP_DY * 2;
            self.add_use(GlyphId::Rest1, x, y);
            return;
        };
        let duration = note.duration;
        let glyph = GlyphId::rest_duration(duration.den);
        let x = NOTE_MARGIN + self.width;
        let mut y = self.staff.middle();
        // Position whole rest glyph up 2 steps.
        if duration.num == 1 && duration.den == 1 {
            y -= Staff::STEP_DY * 2;
        }
        self.add_use(glyph, x, y);
    }

    /// Add use element
    fn add_use(&mut self, glyph: GlyphId, x: i32, y: i32) {
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
    // use super::*;

    // stamp(out, NoteheadFill, offset_x + 2000, STAFF_DY - STEP_DY * 3);
    // Clef
    // stamp(out, ClefC, 96, STAFF_DY);
    // Time Signature
    // stamp(out, TimeSig3, 96 + STAFF_DY, STAFF_DY - STEP_DY * 2); // 421
    // stamp(out, TimeSig4, 96 + STAFF_DY - ((470 - 421) / 2), STAFF_DY + STEP_DY * 2); // 470
    // stamp(out, NoteheadHalf, 96 + 2000 + 2000, STAFF_DY);
    // stamp(out, NoteheadHalf, 96 + 2000 + 4400, STAFF_DY);
}
