// Events - Typed event subscriptions
//
// Purpose: Define event name constants and helper types for event payloads
//
// Provides event listener setup helpers using @tauri-apps/api/event

import {
  listen,
  type UnlistenFn,
  type Event,
  type EventName,
} from "@tauri-apps/api/event";
import type {
  TransportStateEventPayload,
  GridChangeEventPayload,
  ChannelChangeEventPayload,
  ProjectChangedEventPayload,
  AudioEngineStateEventPayload,
  ErrorEventPayload,
  PlayheadEventPayload,
  LevelsEventPayload,
} from "./dto";

// ==================== EVENT NAME CONSTANTS ====================

export const EVENT_TRANSPORT_STATE = "transport-state";
export const EVENT_GRID_CHANGE = "grid-change";
export const EVENT_CHANNEL_CHANGE = "channel-change";
export const EVENT_PROJECT_CHANGED = "project-changed";
export const EVENT_AUDIO_ENGINE_STATE = "audio-engine-state";
export const EVENT_ERROR = "error";
export const EVENT_PLAYHEAD = "playhead";
export const EVENT_LEVELS = "levels";

// ==================== EVENT LISTENER TYPES ====================

export type TransportStateListener = (event: Event<TransportStateEventPayload>) => void;
export type GridChangeListener = (event: Event<GridChangeEventPayload>) => void;
export type ChannelChangeListener = (event: Event<ChannelChangeEventPayload>) => void;
export type ProjectChangedListener = (event: Event<ProjectChangedEventPayload>) => void;
export type AudioEngineStateListener = (event: Event<AudioEngineStateEventPayload>) => void;
export type ErrorListener = (event: Event<ErrorEventPayload>) => void;
export type PlayheadListener = (event: Event<PlayheadEventPayload>) => void;
export type LevelsListener = (event: Event<LevelsEventPayload>) => void;

// ==================== EVENT LISTENER HELPERS ====================

/**
 * Subscribe to transport state changes
 * Emitted when play/stop/pause/seek occurs or BPM/time signature changes
 */
export async function onTransportState(listener: TransportStateListener): Promise<UnlistenFn> {
  return listen<TransportStateEventPayload>(EVENT_TRANSPORT_STATE, listener);
}

/**
 * Subscribe to grid cell changes
 * Emitted when a cell is added, modified, or removed
 */
export async function onGridChange(listener: GridChangeListener): Promise<UnlistenFn> {
  return listen<GridChangeEventPayload>(EVENT_GRID_CHANGE, listener);
}

/**
 * Subscribe to channel state changes
 * Emitted when channel volume, pan, mute, solo, or name changes
 */
export async function onChannelChange(listener: ChannelChangeListener): Promise<UnlistenFn> {
  return listen<ChannelChangeEventPayload>(EVENT_CHANNEL_CHANGE, listener);
}

/**
 * Subscribe to project state changes
 * Emitted when project is created, loaded, saved, or modified
 */
export async function onProjectChanged(listener: ProjectChangedListener): Promise<UnlistenFn> {
  return listen<ProjectChangedEventPayload>(EVENT_PROJECT_CHANGED, listener);
}

/**
 * Subscribe to audio engine state changes
 * Emitted when audio engine initializes or encounters errors
 */
export async function onAudioEngineState(listener: AudioEngineStateListener): Promise<UnlistenFn> {
  return listen<AudioEngineStateEventPayload>(EVENT_AUDIO_ENGINE_STATE, listener);
}

/**
 * Subscribe to error events
 * Emitted when an error occurs in the backend
 */
export async function onError(listener: ErrorListener): Promise<UnlistenFn> {
  return listen<ErrorEventPayload>(EVENT_ERROR, listener);
}

/**
 * Subscribe to playhead position changes
 * Emitted on each step change during playback
 */
export async function onPlayhead(listener: PlayheadListener): Promise<UnlistenFn> {
  return listen<PlayheadEventPayload>(EVENT_PLAYHEAD, listener);
}

/**
 * Subscribe to audio level changes
 * Emitted at regular intervals with current channel and master levels
 */
export async function onLevels(listener: LevelsListener): Promise<UnlistenFn> {
  return listen<LevelsEventPayload>(EVENT_LEVELS, listener);
}

// ==================== EVENT PAYLOAD TYPE GUARDS ====================

export function isTransportStatePayload(payload: unknown): payload is TransportStateEventPayload {
  return (
    typeof payload === "object" &&
    payload !== null &&
    "playing" in payload &&
    "position" in payload &&
    "bpm" in payload
  );
}

export function isGridChangePayload(payload: unknown): payload is GridChangeEventPayload {
  return (
    typeof payload === "object" &&
    payload !== null &&
    "channel" in payload &&
    "row" in payload &&
    "col" in payload
  );
}

export function isChannelChangePayload(payload: unknown): payload is ChannelChangeEventPayload {
  return (
    typeof payload === "object" &&
    payload !== null &&
    "channel" in payload &&
    "name" in payload &&
    "volume" in payload
  );
}

export function isProjectChangedPayload(payload: unknown): payload is ProjectChangedEventPayload {
  return (
    typeof payload === "object" &&
    payload !== null &&
    "projectId" in payload &&
    "name" in payload
  );
}

export function isAudioEngineStatePayload(payload: unknown): payload is AudioEngineStateEventPayload {
  return (
    typeof payload === "object" &&
    payload !== null &&
    "initialized" in payload &&
    "sampleRate" in payload
  );
}

export function isErrorPayload(payload: unknown): payload is ErrorEventPayload {
  return (
    typeof payload === "object" &&
    payload !== null &&
    "code" in payload &&
    "message" in payload
  );
}

// ==================== EVENT EMITTER TYPE ====================

/**
 * Type for the event emitter object returned by createEventEmitter
 */
export interface EventEmitter {
  transportState: (listener: TransportStateListener) => Promise<UnlistenFn>;
  gridChange: (listener: GridChangeListener) => Promise<UnlistenFn>;
  channelChange: (listener: ChannelChangeListener) => Promise<UnlistenFn>;
  projectChanged: (listener: ProjectChangedListener) => Promise<UnlistenFn>;
  audioEngineState: (listener: AudioEngineStateListener) => Promise<UnlistenFn>;
  error: (listener: ErrorListener) => Promise<UnlistenFn>;
  playhead: (listener: PlayheadListener) => Promise<UnlistenFn>;
  levels: (listener: LevelsListener) => Promise<UnlistenFn>;
}

/**
 * Creates a unified event emitter object for convenient subscription
 */
export function createEventEmitter(): EventEmitter {
  return {
    transportState: onTransportState,
    gridChange: onGridChange,
    channelChange: onChannelChange,
    projectChanged: onProjectChanged,
    audioEngineState: onAudioEngineState,
    error: onError,
    playhead: onPlayhead,
    levels: onLevels,
  };
}

// ==================== EVENT NAMES ARRAY ====================

/**
 * All available event names for documentation/reference
 */
export const ALL_EVENTS: EventName[] = [
  EVENT_TRANSPORT_STATE,
  EVENT_GRID_CHANGE,
  EVENT_CHANNEL_CHANGE,
  EVENT_PROJECT_CHANGED,
  EVENT_AUDIO_ENGINE_STATE,
  EVENT_ERROR,
  EVENT_PLAYHEAD,
  EVENT_LEVELS,
];
