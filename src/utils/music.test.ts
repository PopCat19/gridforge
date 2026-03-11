// music.test.ts
//
// Purpose: Unit tests for music utility functions
//
// Tests:
// - Note number to name conversion
// - Note name to number conversion
// - Row/note mapping
// - Note range generation
// - Black key detection
// - Octave calculation

import { describe, test, expect } from "bun:test";
import {
  noteNumberToName,
  noteNameToNumber,
  rowToNote,
  noteToRow,
  getNoteRange,
  getNoteNamesForRange,
  isBlackKey,
  getOctave,
  MIDI_NOTE_MIN,
  MIDI_NOTE_MAX,
  DEFAULT_NOTE_RANGE_START,
  DEFAULT_NOTE_RANGE_END,
  STEPS_PER_BAR,
} from "./music";

describe("noteNumberToName", () => {
  test("converts middle C (MIDI 60) to C4", () => {
    expect(noteNumberToName(60)).toBe("C4");
  });

  test("converts A4 (MIDI 69) to A4", () => {
    expect(noteNumberToName(69)).toBe("A4");
  });

  test("uses sharps by default", () => {
    expect(noteNumberToName(61)).toBe("C#4");
  });

  test("uses flats when requested", () => {
    expect(noteNumberToName(61, true)).toBe("Db4");
  });

  test("handles octave 0", () => {
    expect(noteNumberToName(0)).toBe("C-1");
    expect(noteNumberToName(11)).toBe("B-1");
  });

  test("handles high octave", () => {
    expect(noteNumberToName(127)).toBe("G9");
  });

  test("returns ??? for out of range below minimum", () => {
    expect(noteNumberToName(-1)).toBe("???");
  });

  test("returns ??? for out of range above maximum", () => {
    expect(noteNumberToName(128)).toBe("???");
  });
});

describe("noteNameToNumber", () => {
  test("parses C4 to 60", () => {
    expect(noteNameToNumber("C4")).toBe(60);
  });

  test("parses A4 to 69", () => {
    expect(noteNameToNumber("A4")).toBe(69);
  });

  test("parses sharp notes", () => {
    expect(noteNameToNumber("C#4")).toBe(61);
    expect(noteNameToNumber("F#4")).toBe(66);
  });

  test("parses flat notes", () => {
    expect(noteNameToNumber("Db4")).toBe(61);
    expect(noteNameToNumber("Bb4")).toBe(70);
  });

  test("handles negative octaves", () => {
    expect(noteNameToNumber("C-1")).toBe(0);
  });

  test("handles case insensitive input", () => {
    expect(noteNameToNumber("c4")).toBe(60);
    expect(noteNameToNumber("c#4")).toBe(61);
  });

  test("returns default for invalid input", () => {
    expect(noteNameToNumber("")).toBe(60);
    expect(noteNameToNumber("invalid")).toBe(60);
  });
});

describe("rowToNote", () => {
  test("converts row 0 to default range start", () => {
    expect(rowToNote(0)).toBe(DEFAULT_NOTE_RANGE_START);
  });

  test("converts row to note with custom range", () => {
    expect(rowToNote(0, 48)).toBe(48);
    expect(rowToNote(10, 48)).toBe(58);
  });
});

describe("noteToRow", () => {
  test("converts default range start to row 0", () => {
    expect(noteToRow(DEFAULT_NOTE_RANGE_START)).toBe(0);
  });

  test("converts note to row with custom range", () => {
    expect(noteToRow(48, 48)).toBe(0);
    expect(noteToRow(58, 48)).toBe(10);
  });
});

describe("getNoteRange", () => {
  test("generates correct number of notes", () => {
    const range = getNoteRange(60, 16);
    expect(range).toHaveLength(16);
  });

  test("generates sequential notes", () => {
    const range = getNoteRange(60, 4);
    expect(range).toEqual([60, 61, 62, 63]);
  });

  test("handles single note", () => {
    const range = getNoteRange(60, 1);
    expect(range).toEqual([60]);
  });
});

describe("getNoteNamesForRange", () => {
  test("generates note names for range", () => {
    const names = getNoteNamesForRange(60, 4);
    expect(names).toEqual(["C4", "C#4", "D4", "D#4"]);
  });

  test("uses flats when requested", () => {
    const names = getNoteNamesForRange(60, 4, true);
    expect(names).toEqual(["C4", "Db4", "D4", "Eb4"]);
  });
});

describe("isBlackKey", () => {
  test("identifies black keys (sharps/flats)", () => {
    expect(isBlackKey(1)).toBe(true);  // C# / Db
    expect(isBlackKey(3)).toBe(true);  // D# / Eb
    expect(isBlackKey(6)).toBe(true);  // F# / Gb
    expect(isBlackKey(8)).toBe(true);  // G# / Ab
    expect(isBlackKey(10)).toBe(true); // A# / Bb
  });

  test("identifies white keys", () => {
    expect(isBlackKey(0)).toBe(false);  // C
    expect(isBlackKey(2)).toBe(false);  // D
    expect(isBlackKey(4)).toBe(false);  // E
    expect(isBlackKey(5)).toBe(false);  // F
    expect(isBlackKey(7)).toBe(false);  // G
    expect(isBlackKey(9)).toBe(false);  // A
    expect(isBlackKey(11)).toBe(false); // B
  });
});

describe("getOctave", () => {
  test("returns correct octave for various notes", () => {
    expect(getOctave(0)).toBe(-1);   // C-1
    expect(getOctave(11)).toBe(-1);  // B-1
    expect(getOctave(12)).toBe(0);   // C0
    expect(getOctave(60)).toBe(4);   // C4 (middle C)
    expect(getOctave(69)).toBe(4);   // A4
    expect(getOctave(127)).toBe(9);  // G9
  });
});

describe("constants", () => {
  test("MIDI_NOTE_MIN is 0", () => {
    expect(MIDI_NOTE_MIN).toBe(0);
  });

  test("MIDI_NOTE_MAX is 127", () => {
    expect(MIDI_NOTE_MAX).toBe(127);
  });

  test("DEFAULT_NOTE_RANGE_START is 24", () => {
    expect(DEFAULT_NOTE_RANGE_START).toBe(24);
  });

  test("DEFAULT_NOTE_RANGE_END is 84", () => {
    expect(DEFAULT_NOTE_RANGE_END).toBe(84);
  });

  test("STEPS_PER_BAR is 16", () => {
    expect(STEPS_PER_BAR).toBe(16);
  });
});
