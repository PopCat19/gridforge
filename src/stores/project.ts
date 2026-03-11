// project.ts
//
// Purpose: Reactive project state management using SolidJS stores
//
// This module:
// - Holds local editable project state mirroring backend Project schema
// - Provides functions to initialize from backend responses and update state

import { createStore, produce } from "solid-js/store";
import { patternUpdate } from "../ipc/commands";

export interface TimeSignature {
  numerator: number;
  denominator: number;
}

export interface Note {
  pitch: number;
  velocity: number;
  start: number;
  duration: number;
}

export interface Pattern {
  id: number;
  name: string;
  length: number;
  grid: Note[][];
}

export interface Channel {
  id: number;
  name: string;
  volume: number;
  pan: number;
  mute: boolean;
  solo: boolean;
  instrument: string | null;
}

export interface Sequence {
  id: number;
  name: string;
  patternIndices: number[];
}

export interface ProjectState {
  version: string;
  name: string;
  tempo: number;
  timeSignature: TimeSignature;
  channels: Channel[];
  patterns: Pattern[];
  sequences: Sequence[];
}

export interface UIState {
  selectedChannel: number | null;
  selectedPattern: number | null;
  selectedSequence: number | null;
  selectedBar: number;
  hoveredCell: { channel: number; row: number; col: number } | null;
  isModified: boolean;
}

export interface ProjectStore {
  project: ProjectState;
  ui: UIState;
}

const defaultProject: ProjectState = {
  version: "1.0.0",
  name: "Untitled Project",
  tempo: 120,
  timeSignature: { numerator: 4, denominator: 4 },
  channels: [],
  patterns: [],
  sequences: [],
};

const defaultUI: UIState = {
  selectedChannel: null,
  selectedPattern: null,
  selectedSequence: null,
  selectedBar: 0,
  hoveredCell: null,
  isModified: false,
};

export function createProjectStore() {
  const [store, setStore] = createStore<ProjectStore>({
    project: { ...defaultProject },
    ui: { ...defaultUI },
  });

  function initializeFromNew(name: string) {
    setStore(
      produce((s) => {
        s.project = {
          version: "1.0.0",
          name,
          tempo: 120,
          timeSignature: { numerator: 4, denominator: 4 },
          channels: [],
          patterns: [],
          sequences: [],
        };
        s.ui = { ...defaultUI };
      })
    );
  }

  function setName(name: string) {
    setStore("project", "name", name);
    setStore("ui", "isModified", true);
  }

  function setTempo(tempo: number) {
    setStore("project", "tempo", Math.max(20, Math.min(300, tempo)));
    setStore("ui", "isModified", true);
  }

  function setTimeSignature(numerator: number, denominator: number) {
    setStore("project", "timeSignature", { numerator, denominator });
    setStore("ui", "isModified", true);
  }

  function addChannel(name?: string): number {
    const id = store.project.channels.length;
    const newChannel: Channel = {
      id,
      name: name ?? `Channel ${id + 1}`,
      volume: 1.0,
      pan: 0.0,
      mute: false,
      solo: false,
      instrument: null,
    };
    setStore(
      produce((s) => {
        s.project.channels.push(newChannel);
        s.ui.isModified = true;
      })
    );
    return id;
  }

  function removeChannel(channelId: number) {
    setStore(
      produce((s) => {
        s.project.channels = s.project.channels.filter((c) => c.id !== channelId);
        s.ui.isModified = true;
      })
    );
  }

  function updateChannel(channelId: number, updates: Partial<Omit<Channel, "id">>) {
    setStore(
      produce((s) => {
        const idx = s.project.channels.findIndex((c) => c.id === channelId);
        if (idx !== -1) {
          Object.assign(s.project.channels[idx], updates);
          s.ui.isModified = true;
        }
      })
    );
  }

  function addPattern(name?: string): number {
    const id = store.project.patterns.length;
    const newPattern: Pattern = {
      id,
      name: name ?? `Pattern ${id + 1}`,
      length: 16,
      grid: [],
    };
    setStore(
      produce((s) => {
        s.project.patterns.push(newPattern);
        s.ui.isModified = true;
      })
    );
    return id;
  }

  function removePattern(patternId: number) {
    setStore(
      produce((s) => {
        s.project.patterns = s.project.patterns.filter((p) => p.id !== patternId);
        s.ui.isModified = true;
      })
    );
  }

  function updatePattern(patternId: number, updates: Partial<Omit<Pattern, "id">>) {
    setStore(
      produce((s) => {
        const idx = s.project.patterns.findIndex((p) => p.id === patternId);
        if (idx !== -1) {
          Object.assign(s.project.patterns[idx], updates);
          s.ui.isModified = true;
        }
      })
    );
  }

  function setPatternNote(patternId: number, channel: number, row: number, note: Note | null) {
    setStore(
      produce((s) => {
        const pattern = s.project.patterns.find((p) => p.id === patternId);
        if (!pattern) return;

        while (pattern.grid.length <= channel) {
          pattern.grid.push([]);
        }
        const channelGrid = pattern.grid[channel];

        while (channelGrid.length <= row) {
          channelGrid.push({ pitch: 60, velocity: 0, start: 0, duration: 0 });
        }

        if (note) {
          channelGrid[row] = note;
        } else {
          channelGrid[row] = { pitch: 60, velocity: 0, start: 0, duration: 0 };
        }
        s.ui.isModified = true;
      })
    );

    // Sync to backend - fire and forget, log errors only
    const patternIndex = store.project.patterns.findIndex((p) => p.id === patternId);
    if (patternIndex !== -1) {
      patternUpdate({
        channel,
        patternIndex,
        cellRow: row,
        cellCol: 0, // Using row as the cell position
        note: note
          ? {
              pitch: note.pitch,
              velocity: note.velocity,
              start: note.start,
              duration: note.duration,
            }
          : null,
      }).catch((err) => {
        console.error("Failed to sync pattern to backend:", err);
      });
    }
  }

  function addSequence(name?: string): number {
    const id = store.project.sequences.length;
    const newSequence: Sequence = {
      id,
      name: name ?? `Sequence ${id + 1}`,
      patternIndices: [],
    };
    setStore(
      produce((s) => {
        s.project.sequences.push(newSequence);
        s.ui.isModified = true;
      })
    );
    return id;
  }

  function removeSequence(sequenceId: number) {
    setStore(
      produce((s) => {
        s.project.sequences = s.project.sequences.filter((seq) => seq.id !== sequenceId);
        s.ui.isModified = true;
      })
    );
  }

  function updateSequence(sequenceId: number, updates: Partial<Omit<Sequence, "id">>) {
    setStore(
      produce((s) => {
        const idx = s.project.sequences.findIndex((seq) => seq.id === sequenceId);
        if (idx !== -1) {
          Object.assign(s.project.sequences[idx], updates);
          s.ui.isModified = true;
        }
      })
    );
  }

  function selectChannel(channel: number | null) {
    setStore("ui", "selectedChannel", channel);
  }

  function selectPattern(pattern: number | null) {
    setStore("ui", "selectedPattern", pattern);
  }

  function selectSequence(sequence: number | null) {
    setStore("ui", "selectedSequence", sequence);
  }

  function selectBar(bar: number) {
    setStore("ui", "selectedBar", bar);
  }

  function setSequenceBarPattern(sequenceId: number, bar: number, patternIndex: number) {
    setStore(
      produce((s) => {
        const seq = s.project.sequences.find((seq) => seq.id === sequenceId);
        if (!seq) return;

        // Extend patternIndices array if needed
        while (seq.patternIndices.length <= bar) {
          seq.patternIndices.push(0);
        }
        seq.patternIndices[bar] = patternIndex;
        s.ui.isModified = true;
      })
    );
  }

  function setHoveredCell(cell: { channel: number; row: number; col: number } | null) {
    setStore("ui", "hoveredCell", cell);
  }

  function markSaved() {
    setStore("ui", "isModified", false);
  }

  return {
    store,
    initializeFromNew,
    setName,
    setTempo,
    setTimeSignature,
    addChannel,
    removeChannel,
    updateChannel,
    addPattern,
    removePattern,
    updatePattern,
    setPatternNote,
    addSequence,
    removeSequence,
    updateSequence,
    selectChannel,
    selectPattern,
    selectSequence,
    selectBar,
    setSequenceBarPattern,
    setHoveredCell,
    markSaved,
  };
}

export type ProjectStoreApi = ReturnType<typeof createProjectStore>;
