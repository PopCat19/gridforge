// music.ts
//
// Purpose: Music utility functions for note/pitch conversion
//
// This module:
// - Converts MIDI note numbers to note names
// - Maps grid rows to MIDI note numbers
// - Provides note constants and ranges

const NOTE_NAME_SHARP = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const NOTE_NAME_FLAT = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

export const MIDI_NOTE_MIN = 0;
export const MIDI_NOTE_MAX = 127;

export const DEFAULT_NOTE_RANGE_START = 24;
export const DEFAULT_NOTE_RANGE_END = 84;

export const STEPS_PER_BAR = 16;

export function noteNumberToName(noteNumber: number, useFlats = false): string {
  if (noteNumber < MIDI_NOTE_MIN || noteNumber > MIDI_NOTE_MAX) {
    return "???";
  }

  const noteName = useFlats ? NOTE_NAME_FLAT : NOTE_NAME_SHARP;
  const octave = Math.floor(noteNumber / 12) - 1;
  const noteIndex = noteNumber % 12;

  return `${noteName[noteIndex]}${octave}`;
}

export function noteNameToNumber(noteName: string): number {
  const match = noteName.match(/^([A-Ga-g])([#b]?)(-?\d+)$/);
  if (!match) return 60;

  const [, letter, accidental, octaveStr] = match;
  const octave = parseInt(octaveStr, 10);

  const noteIndex = letter.toUpperCase().charCodeAt(0) - "A".charCodeAt(0);
  const noteOffset = [0, 2, 4, 5, 7, 9, 11][noteIndex];

  let adjustedOffset = noteOffset;
  if (accidental === "#") adjustedOffset += 1;
  if (accidental === "b") adjustedOffset -= 1;

  return (octave + 1) * 12 + adjustedOffset;
}

export function rowToNote(row: number, rangeStart = DEFAULT_NOTE_RANGE_START): number {
  return rangeStart + row;
}

export function noteToRow(note: number, rangeStart = DEFAULT_NOTE_RANGE_START): number {
  return note - rangeStart;
}

export function getNoteRange(startNote: number, numRows: number): number[] {
  const notes: number[] = [];
  for (let i = 0; i < numRows; i++) {
    notes.push(startNote + i);
  }
  return notes;
}

export function getNoteNamesForRange(startNote: number, numRows: number, useFlats = false): string[] {
  return getNoteRange(startNote, numRows).map((n) => noteNumberToName(n, useFlats));
}

export function isBlackKey(noteNumber: number): boolean {
  const noteInOctave = noteNumber % 12;
  return [1, 3, 6, 8, 10].includes(noteInOctave);
}

export function getOctave(noteNumber: number): number {
  return Math.floor(noteNumber / 12) - 1;
}
