// DTOs - Data Transfer Objects
//
// Purpose: TypeScript interfaces for IPC communication between frontend and Tauri backend
//
// Keep in sync with Rust DTOs in backend

// ==================== COMMAND PAYLOADS ====================

// Project Commands
export interface ProjectNewPayload {
  name: string;
}

export interface ProjectSavePayload {
  path?: string; // If omitted, save to current path
}

export interface ProjectLoadPayload {
  path: string;
}

export interface ProjectExportPayload {
  path: string;
  format: "wav" | "ogg" | "mp3";
}

// Transport Commands
export interface TransportPlayPayload {
  position?: number; // Start position in beats
}

export interface TransportStopPayload {
  // No payload
}

export interface TransportPausePayload {
  // No payload
}

export interface TransportSeekPayload {
  position: number; // Position in beats
}

export interface TransportSetBpmPayload {
  bpm: number;
}

export interface TransportSetTimeSignaturePayload {
  numerator: number;
  denominator: number;
}

// Grid Commands
export interface GridSetCellPayload {
  channel: number;
  row: number;
  col: number;
  velocity: number;
  enabled: boolean;
}

export interface GridClearCellPayload {
  channel: number;
  row: number;
  col: number;
}

export interface GridClearChannelPayload {
  channel: number;
}

export interface GridClearAllPayload {
  // No payload
}

export interface GridCopyPayload {
  sourceChannel: number;
  sourceStartCol: number;
  sourceEndCol: number;
  destChannel: number;
  destCol: number;
}

export interface GridPastePayload {
  sourceChannel: number;
  sourceStartCol: number;
  destChannel: number;
  destCol: number;
  clearSource: boolean;
}

// Pattern Commands
export interface PatternDto {
  id: number;
  name: string;
  length: number;
  grid: NoteDto[][];
}

export interface NoteDto {
  pitch: number;
  velocity: number;
  start: number;
  duration: number;
}

export interface PatternUpdateRequest {
  channel: number;
  patternIndex: number;
  cellRow: number;
  cellCol: number;
  note: NoteDto | null;
}

// Channel Commands
export interface ChannelAddPayload {
  name?: string;
  instrumentId?: string;
}

export interface ChannelRemovePayload {
  channel: number;
}

export interface ChannelSetNamePayload {
  channel: number;
  name: string;
}

export interface ChannelSetVolumePayload {
  channel: number;
  volume: number; // 0.0 - 1.0
}

export interface ChannelSetPanPayload {
  channel: number;
  pan: number; // -1.0 (left) to 1.0 (right)
}

export interface ChannelSetMutePayload {
  channel: number;
  muted: boolean;
}

export interface ChannelSetSoloPayload {
  channel: number;
  solo: boolean;
}

// Instrument Commands
export interface InstrumentListPayload {
  // No payload - returns all available instruments
}

export interface InstrumentLoadPayload {
  path: string;
}

export interface InstrumentAssignPayload {
  channel: number;
  instrumentId: string;
}

// Preview Commands
export interface NotePreviewPayload {
  channel: number;
  note: number;
  velocity: number;
}

export interface NotePreviewResponse {
  success: boolean;
}

// ==================== SOURCE COMMANDS ====================

export interface ListSourcesResponse {
  sources: SourceInfo[];
}

export interface SourceInfo {
  id: string;
  name: string;
  category: string;
}

export interface ChannelSetSourcePayload {
  channel: number;
  source_id: string;
}

export interface ChannelSetSourceResponse {
  channel: number;
  source_id: string;
  success: boolean;
}

export interface GetSourceParamsPayload {
  source_id: string;
}

export interface ParamInfo {
  id: string;
  name: string;
  min: number;
  max: number;
  default: number;
  unit: string;
}

export interface GetSourceParamsResponse {
  source_id: string;
  params: ParamInfo[];
}

export interface SetSourceParamPayload {
  channel: number;
  param_id: string;
  value: number;
}

export interface SetSourceParamResponse {
  channel: number;
  param_id: string;
  value: number;
  success: boolean;
}

// ==================== COMMAND RESPONSE TYPES ====================

export interface ProjectNewResponse {
  projectId: string;
  name: string;
  createdAt: string;
}

export interface ProjectSaveResponse {
  path: string;
  savedAt: string;
}

export interface ProjectLoadResponse {
  projectId: string;
  name: string;
  path: string;
  bpm: number;
  timeSignature: { numerator: number; denominator: number };
  channels: ChannelState[];
}

export interface ProjectExportResponse {
  path: string;
  exportedAt: string;
  duration: number;
}

export interface TransportPlayResponse {
  playing: boolean;
  position: number;
}

export interface TransportStopResponse {
  playing: boolean;
  position: number;
}

export interface TransportPauseResponse {
  playing: boolean;
}

export interface TransportSeekResponse {
  position: number;
}

export interface TransportSetBpmResponse {
  bpm: number;
}

export interface TransportSetTimeSignatureResponse {
  numerator: number;
  denominator: number;
}

export interface GridSetCellResponse {
  success: boolean;
}

export interface GridClearCellResponse {
  success: boolean;
}

export interface GridClearChannelResponse {
  success: boolean;
}

export interface GridClearAllResponse {
  success: boolean;
}

export interface GridCopyResponse {
  success: boolean;
  cellsCopied: number;
}

export interface GridPasteResponse {
  success: boolean;
  cellsPasted: number;
}

export interface PatternUpdateResponse {
  success: boolean;
}

export interface ChannelAddResponse {
  channel: number;
  name: string;
}

export interface ChannelRemoveResponse {
  success: boolean;
}

export interface ChannelSetNameResponse {
  channel: number;
  name: string;
}

export interface ChannelSetVolumeResponse {
  channel: number;
  volume: number;
}

export interface ChannelSetPanResponse {
  channel: number;
  pan: number;
}

export interface ChannelSetMuteResponse {
  channel: number;
  muted: boolean;
}

export interface ChannelSetSoloResponse {
  channel: number;
  solo: boolean;
}

export interface InstrumentListResponse {
  instruments: InstrumentInfo[];
}

export interface InstrumentLoadResponse {
  instrumentId: string;
  name: string;
}

export interface InstrumentAssignResponse {
  channel: number;
  instrumentId: string;
}

// ==================== EVENT PAYLOADS ====================

export interface TransportStateEventPayload {
  playing: boolean;
  position: number; // Current position in beats
  bpm: number;
  timeSignature: { numerator: number; denominator: number };
}

export interface GridChangeEventPayload {
  channel: number;
  row: number;
  col: number;
  velocity: number;
  enabled: boolean;
}

export interface ChannelChangeEventPayload {
  channel: number;
  name: string;
  volume: number;
  pan: number;
  muted: boolean;
  solo: boolean;
}

export interface ProjectChangedEventPayload {
  projectId: string;
  name: string;
  path: string | null;
  modified: boolean;
}

export interface AudioEngineStateEventPayload {
  initialized: boolean;
  sampleRate: number;
  bufferSize: number;
}

export interface ErrorEventPayload {
  code: string;
  message: string;
  details?: Record<string, unknown>;
}

export interface PlayheadEventPayload {
  bar: number;
  beat: number;
  step: number;
  tick: number;
  is_playing: boolean;
}

export interface LevelsEventPayload {
  channels: number[];
  master: number;
}

// ==================== SHARED TYPES ====================

export interface ChannelState {
  channel: number;
  name: string;
  volume: number;
  pan: number;
  muted: boolean;
  solo: boolean;
  instrumentId: string | null;
}

export interface InstrumentInfo {
  id: string;
  name: string;
  category: string;
}

export interface GridCell {
  channel: number;
  row: number;
  col: number;
  velocity: number;
  enabled: boolean;
}

export interface GridState {
  channels: number;
  rows: number;
  cols: number;
  cells: GridCell[];
}
