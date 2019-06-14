# Notes
Our aim is to create SVG files from a music score data, and generate beautiful music by default.

## Measure Types
Measures are aligned by the smallest duration.  The measures get a fraction of the line.

- Whole or Half Note Measure Run: 1/9
- Quarter Note Measure Run: 1/8
- 8th Note Measure Run: 1/5
- 16th Note Measure Run: 1/2
- 32nd, 64th, 128th Note Measure Run: 1

## Rules
- 32 notes per line maximum.
- 10 measures per line maximum (only way is with 100 points).
- Accidentals give measure more space, but less if first glyph in the measure.  Also less if accidentals aren't in the way of anything.
