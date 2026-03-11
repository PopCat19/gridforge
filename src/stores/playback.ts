// playback.ts
//
// Purpose: Reactive playback state management using SolidJS stores
//
// This module:
// - Holds UI-only playback state (not persisted)
// - Provides functions to control playback and update meters

import { createStore, produce } from "solid-js/store";
import { onPlayhead, onLevels, type PlayheadListener, type LevelsListener } from "../ipc/events";
import type { UnlistenFn } from "@tauri-apps/api/event";

export interface PlayheadPosition {
  bar: number;
  tick: number;
}

export interface ChannelLevel {
  peak: number;
  rms: number;
}

export interface PlaybackState {
  isPlaying: boolean;
  tempo: number;
  playhead: PlayheadPosition;
  levels: ChannelLevel[];
}

export interface PlaybackStore {
  playback: PlaybackState;
}

const defaultPlayback: PlaybackState = {
  isPlaying: false,
  tempo: 120,
  playhead: { bar: 0, tick: 0 },
  levels: [],
};

export function createPlaybackStore() {
  const [store, setStore] = createStore<PlaybackStore>({
    playback: { ...defaultPlayback },
  });

  function setPlaying(playing: boolean) {
    setStore("playback", "isPlaying", playing);
  }

  function play() {
    setStore("playback", "isPlaying", true);
  }

  function pause() {
    setStore("playback", "isPlaying", false);
  }

  function stop() {
    setStore(
      produce((s) => {
        s.playback.isPlaying = false;
        s.playback.playhead = { bar: 0, tick: 0 };
      })
    );
  }

  function setTempo(tempo: number) {
    setStore("playback", "tempo", Math.max(20, Math.min(300, tempo)));
  }

  function setPlayheadPosition(bar: number, tick: number) {
    setStore("playback", "playhead", { bar, tick });
  }

  function setPlayheadBeat(beat: number, ticksPerBeat: number = 480) {
    const bar = Math.floor(beat / 4);
    const tick = Math.floor((beat % 4) * ticksPerBeat);
    setStore("playback", "playhead", { bar, tick });
  }

  function getPlayheadBeat(ticksPerBeat: number = 480): number {
    return store.playback.playhead.bar * 4 + store.playback.playhead.tick / ticksPerBeat;
  }

  function updateLevels(levels: ChannelLevel[]) {
    setStore("playback", "levels", levels);
  }

  function setChannelLevel(channel: number, level: ChannelLevel) {
    setStore(
      produce((s) => {
        while (s.playback.levels.length <= channel) {
          s.playback.levels.push({ peak: 0, rms: 0 });
        }
        s.playback.levels[channel] = level;
      })
    );
  }

  function clearLevels() {
    setStore("playback", "levels", []);
  }

  async function subscribeToPlayhead(): Promise<UnlistenFn> {
    const listener: PlayheadListener = (event) => {
      const payload = event.payload;
      setStore("playback", "playhead", { bar: payload.bar, tick: payload.tick });
      setStore("playback", "isPlaying", payload.is_playing);
    };
    return onPlayhead(listener);
  }

  async function subscribeToLevels(): Promise<UnlistenFn> {
    const listener: LevelsListener = (event) => {
      const payload = event.payload;
      const levels = payload.channels.map((peak) => ({ peak, rms: peak * 0.7 }));
      setStore("playback", "levels", levels);
      // Store master level at the end
      setStore(
        produce((s) => {
          while (s.playback.levels.length < payload.channels.length) {
            s.playback.levels.push({ peak: 0, rms: 0 });
          }
          s.playback.levels[payload.channels.length] = { peak: payload.master, rms: payload.master * 0.7 };
        })
      );
    };
    return onLevels(listener);
  }

  return {
    store,
    setPlaying,
    play,
    pause,
    stop,
    setTempo,
    setPlayheadPosition,
    setPlayheadBeat,
    getPlayheadBeat,
    updateLevels,
    setChannelLevel,
    clearLevels,
    subscribeToPlayhead,
    subscribeToLevels,
  };
}

export type PlaybackStoreApi = ReturnType<typeof createPlaybackStore>;
