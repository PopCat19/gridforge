// grid-editor.tsx
//
// Purpose: Interactive note grid for pattern editing
//
// This module:
// - Renders rows as pitches (MIDI notes)
// - Renders columns as ticks (16 steps for 1 bar)
// - Allows click to toggle note cells with audio preview

import { Component, createSignal, For } from "solid-js";
import { notePreview } from "../ipc/bridge";
import {
  noteNumberToName,
  rowToNote,
  isBlackKey,
  DEFAULT_NOTE_RANGE_START,
  STEPS_PER_BAR,
} from "../utils/music";

const NUM_ROWS = 60;
const DEFAULT_VELOCITY = 100;

interface CellData {
  enabled: boolean;
}

const GridEditor: Component = () => {
  const [grid, setGrid] = createSignal<CellData[][]>(createEmptyGrid());
  const [selectedChannel, setSelectedChannel] = createSignal(0);
  const [selectedPattern, setSelectedPattern] = createSignal(0);

  function createEmptyGrid(): CellData[][] {
    const rows: CellData[][] = [];
    for (let row = 0; row < NUM_ROWS; row++) {
      const rowData: CellData[] = [];
      for (let col = 0; col < STEPS_PER_BAR; col++) {
        rowData.push({ enabled: false });
      }
      rows.push(rowData);
    }
    return rows;
  }

  async function handleCellClick(row: number, col: number) {
    const currentGrid = grid();
    const cell = currentGrid[row][col];
    const newEnabled = !cell.enabled;

    const newGrid = currentGrid.map((r, rIdx) =>
      r.map((c, cIdx) => {
        if (rIdx === row && cIdx === col) {
          return { ...c, enabled: newEnabled };
        }
        return c;
      })
    );
    setGrid(newGrid);

    if (newEnabled) {
      const note = rowToNote(NUM_ROWS - 1 - row, DEFAULT_NOTE_RANGE_START);
      try {
        await notePreview({
          channel: selectedChannel(),
          note,
          velocity: DEFAULT_VELOCITY,
        });
      } catch (e) {
        console.error("Failed to preview note:", e);
      }
    }
  }

  function handleChannelChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    setSelectedChannel(parseInt(target.value, 10));
  }

  function handlePatternChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    setSelectedPattern(parseInt(target.value, 10));
  }

  return (
    <div class="panel grid-editor">
      <div class="grid-header">
        <h3>Grid Editor</h3>
        <div class="grid-controls">
          <label>
            Ch:
            <select value={selectedChannel()} onChange={handleChannelChange}>
              <option value="0">1</option>
              <option value="1">2</option>
              <option value="2">3</option>
              <option value="3">4</option>
              <option value="4">5</option>
              <option value="5">6</option>
              <option value="6">7</option>
              <option value="7">8</option>
              <option value="8">9</option>
              <option value="9">10</option>
              <option value="10">11</option>
              <option value="11">12</option>
              <option value="12">13</option>
              <option value="13">14</option>
              <option value="14">15</option>
              <option value="15">16</option>
            </select>
          </label>
          <label>
            Pat:
            <select value={selectedPattern()} onChange={handlePatternChange}>
              <option value="0">1</option>
              <option value="1">2</option>
              <option value="2">3</option>
              <option value="3">4</option>
              <option value="4">5</option>
              <option value="5">6</option>
              <option value="6">7</option>
              <option value="7">8</option>
            </select>
          </label>
        </div>
      </div>
      <div class="grid-container">
        <div class="grid-pitch-labels">
          <For each={Array.from({ length: NUM_ROWS }, (_, i) => NUM_ROWS - 1 - i)}>
            {(row) => {
              const note = rowToNote(row, DEFAULT_NOTE_RANGE_START);
              const isBlack = isBlackKey(note);
              return (
                <div class="pitch-label" classList={{ "black-key": isBlack }}>
                  {noteNumberToName(note)}
                </div>
              );
            }}
          </For>
        </div>
        <div class="grid-cells-container">
          <div class="grid-ticks">
            <For each={Array.from({ length: STEPS_PER_BAR }, (_, i) => i)}>
              {(col) => (
                <div class="tick-label" classList={{ beat: col % 4 === 0 }}>
                  {col + 1}
                </div>
              )}
            </For>
          </div>
          <div class="grid-cells">
            <For each={grid()}>
              {(row, rowIdx) => (
                <div class="grid-row">
                  <For each={row}>
                    {(cell, colIdx) => {
                      const note = rowToNote(NUM_ROWS - 1 - rowIdx(), DEFAULT_NOTE_RANGE_START);
                      const isBlack = isBlackKey(note);
                      return (
                        <div
                          class="grid-cell"
                          classList={{
                            active: cell.enabled,
                            "black-key": isBlack,
                            beat: colIdx() % 4 === 0,
                          }}
                          onClick={() => handleCellClick(rowIdx(), colIdx())}
                        />
                      );
                    }}
                  </For>
                </div>
              )}
            </For>
          </div>
        </div>
      </div>
    </div>
  );
};

export default GridEditor;
