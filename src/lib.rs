pub use svg;

use scof::Marking;
mod glyph;

const BAR_WIDTH: i32 = 3200;
const BAR_W: f32 = BAR_WIDTH as f32;

/// Default font (Bravura).
pub const DEFAULT: &'static str = include_str!("vfont/bravura.vfont");

// A half or whole step.
const STEP: i32 = 125;

use glyph::GlyphId::{self, *};

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
fn render_measure(out: &mut String, mut margin: i32, screen_width: i32, cursor_dims: Option<(f32, f32)>, scof: &scof::Scof, bar: usize, chan: usize, user_cursor: usize) {
//    stamp(out, NoteheadFill, margin + 2000, 1000 - STEP * 3);
//    stem_d(out, margin + 2000 + 15, -STEP * 3);

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
                        1 => {
                            let width = 1.0;
                            if curs == user_cursor {
                                cursor(out, margin + 32, 1000 -STEP * 4, (BAR_W * width) as i32);
                            }
                            stamp(out, Rest1, margin + 250, 1000)
                        },
                        2 => {
                            let width = 1.0 / 2.0;
                            if curs == user_cursor {
                                cursor(out, margin + 32, 1000 -STEP * 4, (BAR_W * width) as i32);
                            }
                            stamp(out, Rest2, margin + 250, 1000)
                        },
                        4 => {
                            let width = 1.0 / 4.0;
                            if curs == user_cursor {
                                cursor(out, margin + 32, 1000 -STEP * 4, (BAR_W * width) as i32);
                            }
                            stamp(out, Rest4, margin + 250, 1000)
                        },
                        8 => {
                            let width = 1.0 / 8.0;
                            if curs == user_cursor {
                                cursor(out, margin + 32, 1000 -STEP * 4, (BAR_W * width) as i32);
                            }
                            stamp(out, Rest8, margin + 250, 1000)
                        },
                        16 => {
                            let width = 1.0 / 16.0;
                            if curs == user_cursor {
                                cursor(out, margin + 32, 1000 -STEP * 4, (BAR_W * width) as i32);
                            }
                            stamp(out, Rest16, margin + 250, 1000)
                        },
                        32 => {
                            let width = 1.0 / 32.0;
                            if curs == user_cursor {
                                cursor(out, margin + 32, 1000 -STEP * 4, (BAR_W * width) as i32);
                            }
                            stamp(out, Rest32, margin + 250, 1000)
                        },
                        64 => {
                            let width = 1.0 / 64.0;
                            if curs == user_cursor {
                                cursor(out, margin + 32, 1000 -STEP * 4, (BAR_W * width) as i32);
                            }
                            stamp(out, Rest64, margin + 250, 1000)
                        },
                        128 => {
                            let width = 1.0 / 128.0;
                            if curs == user_cursor {
                                cursor(out, margin + 32, 1000 -STEP * 4, (BAR_W * width) as i32);
                            }
                            stamp(out, Rest128, margin + 250, 1000)
                        },
                        _ => { /*unknown, do nothing*/ }
                    }
                    margin += (BAR_W * note.duration.0 as f32 / note.duration.1 as f32) as i32;
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

fn render(out: &mut String, screen_width: i32, scof: &scof::Scof, chan: usize, user_cursor: usize) {
    let margin = 96;
    let screen_width = screen_width - margin;

/*    // Clef
    stamp(out, ClefC, 96, 1000);
    // Time Signature
    stamp(out, TimeSig3, 96 + 1000, 1000 - STEP * 2); // 421
    stamp(out, TimeSig4, 96 + 1000 - ((470 - 421) / 2), 1000 + STEP * 2); // 470*/

    // Print enough measures to fill the screen.
    let mut i = 0usize;
    'm: loop {
        let position = BAR_WIDTH * i as i32;

        if position > screen_width {
            break 'm;
        }

        // Render this measure
        let curs = if i == 0 {
            Some((0.0, 0.25))
        } else {
            None
        };
        render_measure(out, margin + position, screen_width, curs, scof, i, chan, user_cursor);
        // Barline
        stamp(out, Barline, margin + position + BAR_WIDTH, 1500);

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
pub fn test_svg(vfont: &str, screen_width: i32, scof: &scof::Scof, curs: usize) -> String {
    // Header: DEFS section.
    let mut out = "<svg viewBox=\"0 0 8192 2048\">\n".to_string();
    out.push_str(vfont);

    // Bodyer: Each Bar
    render(&mut out, screen_width, scof, 0, curs); // Channel 0

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
