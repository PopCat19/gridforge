// transport.tsx
//
// Purpose: Transport controls - play, stop, tempo controls

import { Component, createSignal } from "solid-js";
import { transportPlay, transportStop, transportSetBpm } from "../ipc/commands";

const Transport: Component = () => {
  const [isPlaying, setIsPlaying] = createSignal(false);
  const [bpm, setBpm] = createSignal(120);

  const handlePlay = async () => {
    try {
      await transportPlay({});
      setIsPlaying(true);
    } catch (err) {
      console.error("Failed to start playback:", err);
    }
  };

  const handleStop = async () => {
    try {
      await transportStop();
      setIsPlaying(false);
    } catch (err) {
      console.error("Failed to stop playback:", err);
    }
  };

  const handleBpmChange = async (e: Event) => {
    const target = e.target as HTMLInputElement;
    const newBpm = parseInt(target.value, 10);
    setBpm(newBpm);

    try {
      await transportSetBpm({ bpm: newBpm });
    } catch (err) {
      console.error("Failed to set tempo:", err);
    }
  };

  return (
    <div class="panel transport">
      <h3>Transport</h3>
      <div class="transport-controls">
        <button class="transport-btn">⏮</button>
        <button
          class={`transport-btn play ${isPlaying() ? "active" : ""}`}
          onClick={handlePlay}
        >
          ▶
        </button>
        <button
          class={`transport-btn stop ${!isPlaying() ? "active" : ""}`}
          onClick={handleStop}
        >
          ⏹
        </button>
        <button class="transport-btn">⏭</button>
      </div>
      <div class="tempo-control">
        <label>BPM: <span>{bpm()}</span></label>
        <input
          type="range"
          min="40"
          max="240"
          value={bpm()}
          onInput={handleBpmChange}
        />
      </div>
    </div>
  );
};

export default Transport;
