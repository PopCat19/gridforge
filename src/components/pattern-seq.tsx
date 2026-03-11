// pattern-seq.tsx
//
// Purpose: Pattern sequencer and bar matrix for arranging patterns in a sequence
//
// This module:
// - Displays tracker-style sequence matrix (bars as columns)
// - Allows assigning pattern indices to bars
// - Shows currently playing bar and selected bar

import { Component, For, Show, createMemo } from "solid-js";
import { ProjectStoreApi } from "../stores/project";
import { PlaybackStoreApi } from "../stores/playback";

interface PatternSeqProps {
  store: ProjectStoreApi;
  playbackStore: PlaybackStoreApi;
}

const NUM_BARS = 16;
const MAX_PATTERNS = 16;

const PatternSeq: Component<PatternSeqProps> = (props) => {
  const sequences = () => props.store.store.project.sequences;
  const selectedSequence = () => props.store.store.ui.selectedSequence;
  const selectedBar = () => props.store.store.ui.selectedBar;
  const patterns = () => props.store.store.project.patterns;

  // Get current sequence data
  const currentSequence = createMemo(() => {
    const seqId = selectedSequence();
    if (seqId === null) return null;
    return sequences().find((s) => s.id === seqId) || null;
  });

  // Get pattern for a specific bar
  const getPatternForBar = (bar: number): number => {
    const seq = currentSequence();
    if (!seq) return 0;
    return seq.patternIndices[bar] ?? 0;
  };

  // Handle clicking on a bar cell to change pattern
  const handleBarClick = (bar: number) => {
    const seqId = selectedSequence();
    if (seqId === null) return;

    const currentPattern = getPatternForBar(bar);
    // Cycle through available patterns
    const nextPattern = (currentPattern + 1) % Math.min(patterns().length || 1, MAX_PATTERNS);
    props.store.setSequenceBarPattern(seqId, bar, nextPattern);
  };

  // Handle selecting a bar for editing
  const handleBarSelect = (bar: number) => {
    props.store.selectBar(bar);
  };

  // Create a new sequence if none exists
  const handleAddSequence = () => {
    props.store.addSequence("Main Sequence");
    // Select the newly created sequence
    props.store.selectSequence(sequences().length - 1);
  };

  // Get current playhead position
  const currentBar = () => props.playbackStore.store.playback.playhead.bar;
  const isPlaying = () => props.playbackStore.store.playback.isPlaying;

  // Ensure there's always a sequence selected
  const ensureSequenceSelected = () => {
    if (selectedSequence() === null && sequences().length > 0) {
      props.store.selectSequence(0);
    }
  };

  // Initialize on mount
  if (sequences().length === 0) {
    handleAddSequence();
  } else {
    ensureSequenceSelected();
  }

  return (
    <div class="panel pattern-seq">
      <div class="pattern-seq-header">
        <h3>Pattern Sequencer</h3>
        <div class="sequence-controls">
          <select
            class="sequence-select"
            value={selectedSequence() ?? ""}
            onChange={(e) => {
              const val = e.target.value;
              props.store.selectSequence(val === "" ? null : parseInt(val, 10));
            }}
          >
            <For each={sequences()}>
              {(seq) => <option value={seq.id}>{seq.name}</option>}
            </For>
          </select>
          <button class="add-sequence-btn" onClick={handleAddSequence}>
            + Seq
          </button>
        </div>
      </div>

      <Show
        when={selectedSequence() !== null}
        fallback={
          <div class="no-sequence">
            <p>No sequence selected. Create one to get started.</p>
            <button onClick={handleAddSequence}>Create Sequence</button>
          </div>
        }
      >
        <div class="pattern-matrix">
          {/* Bar numbers header */}
          <div class="matrix-header">
            <div class="bar-num-header">Bar</div>
            <For each={Array.from({ length: NUM_BARS }, (_, i) => i)}>
            {(bar) => (
            <div
            class="bar-number"
            classList={{
            selected: selectedBar() === bar,
            playing: isPlaying() && currentBar() === bar,
            }}
            onClick={() => handleBarSelect(bar)}
            >
            {bar + 1}
            </div>
            )}
            </For>
          </div>

          {/* Pattern row - shows which pattern is assigned to each bar */}
          <div class="pattern-row">
            <div class="pattern-label">Pattern</div>
            <For each={Array.from({ length: NUM_BARS }, (_, i) => i)}>
              {(bar) => {
                const patternIdx = () => getPatternForBar(bar);
                const hasPattern = () => patterns().length > patternIdx();

                return (
                  <div
                    class="pattern-cell"
                    classList={{
                      selected: selectedBar() === bar,
                      playing: isPlaying() && currentBar() === bar,
                      empty: !hasPattern(),
                    }}
                    onClick={() => handleBarClick(bar)}
                    title={`Bar ${bar + 1}: Click to change pattern`}
                  >
                    <Show when={hasPattern()} fallback="-">
                      {patternIdx()}
                    </Show>
                  </div>
                );
              }}
            </For>
          </div>
        </div>

        {/* Pattern legend */}
        <div class="pattern-legend">
          <span class="legend-title">Available Patterns:</span>
          <Show
            when={patterns().length > 0}
            fallback={<span class="no-patterns">No patterns yet</span>}
          >
            <For each={patterns().slice(0, MAX_PATTERNS)}>
              {(pattern, idx) => (
                <span class="legend-item" title={pattern.name}>
                  [{idx()}] {pattern.name}
                </span>
              )}
            </For>
          </Show>
        </div>
      </Show>
    </div>
  );
};

export default PatternSeq;
