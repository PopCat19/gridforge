// Commands - Typed Tauri command invocations
//
// Purpose: Wrap tauri.invoke calls with fully typed functions
//
// Each function is typed with input and output from dto.ts

import { invoke } from "@tauri-apps/api/core";
import type {
  ProjectNewPayload,
  ProjectSavePayload,
  ProjectLoadPayload,
  ProjectExportPayload,
  TransportPlayPayload,
  TransportSeekPayload,
  TransportSetBpmPayload,
  TransportSetTimeSignaturePayload,
  GridSetCellPayload,
  GridClearCellPayload,
  GridClearChannelPayload,
  GridCopyPayload,
  GridPastePayload,
  ChannelAddPayload,
  ChannelRemovePayload,
  ChannelSetNamePayload,
  ChannelSetVolumePayload,
  ChannelSetPanPayload,
  ChannelSetMutePayload,
  ChannelSetSoloPayload,
  InstrumentListPayload,
  InstrumentLoadPayload,
  InstrumentAssignPayload,
  NotePreviewPayload,
  NotePreviewResponse,
  ProjectNewResponse,
  ProjectSaveResponse,
  ProjectLoadResponse,
  ProjectExportResponse,
  TransportPlayResponse,
  TransportStopResponse,
  TransportPauseResponse,
  TransportSeekResponse,
  TransportSetBpmResponse,
  TransportSetTimeSignatureResponse,
  GridSetCellResponse,
  GridClearCellResponse,
  GridClearChannelResponse,
  GridClearAllResponse,
  GridCopyResponse,
  GridPasteResponse,
  PatternUpdateRequest,
  PatternUpdateResponse,
  ChannelAddResponse,
  ChannelRemoveResponse,
  ChannelSetNameResponse,
  ChannelSetVolumeResponse,
  ChannelSetPanResponse,
  ChannelSetMuteResponse,
  ChannelSetSoloResponse,
  InstrumentListResponse,
  InstrumentLoadResponse,
  InstrumentAssignResponse,
  ListSourcesResponse,
  ChannelSetSourcePayload,
  ChannelSetSourceResponse,
  GetSourceParamsPayload,
  GetSourceParamsResponse,
  SetSourceParamPayload,
  SetSourceParamResponse,
} from "./dto";

// ==================== PROJECT COMMANDS ====================

export async function projectNew(payload: ProjectNewPayload): Promise<ProjectNewResponse> {
  return invoke<ProjectNewResponse>("project_new", { payload });
}

export async function projectSave(payload: ProjectSavePayload = {}): Promise<ProjectSaveResponse> {
  return invoke<ProjectSaveResponse>("project_save", { payload });
}

export async function projectLoad(payload: ProjectLoadPayload): Promise<ProjectLoadResponse> {
  return invoke<ProjectLoadResponse>("project_load", { payload });
}

export async function projectExport(payload: ProjectExportPayload): Promise<ProjectExportResponse> {
  return invoke<ProjectExportResponse>("project_export", { payload });
}

// ==================== TRANSPORT COMMANDS ====================

export async function transportPlay(payload: TransportPlayPayload = {}): Promise<TransportPlayResponse> {
  return invoke<TransportPlayResponse>("transport_play", { payload });
}

export async function transportStop(): Promise<TransportStopResponse> {
  return invoke<TransportStopResponse>("transport_stop", {});
}

export async function transportPause(): Promise<TransportPauseResponse> {
  return invoke<TransportPauseResponse>("transport_pause", {});
}

export async function transportSeek(payload: TransportSeekPayload): Promise<TransportSeekResponse> {
  return invoke<TransportSeekResponse>("transport_seek", { payload });
}

export async function transportSetBpm(payload: TransportSetBpmPayload): Promise<TransportSetBpmResponse> {
  return invoke<TransportSetBpmResponse>("transport_set_bpm", { payload });
}

export async function transportSetTimeSignature(
  payload: TransportSetTimeSignaturePayload,
): Promise<TransportSetTimeSignatureResponse> {
  return invoke<TransportSetTimeSignatureResponse>("transport_set_time_signature", { payload });
}

// ==================== GRID COMMANDS ====================

export async function gridSetCell(payload: GridSetCellPayload): Promise<GridSetCellResponse> {
  return invoke<GridSetCellResponse>("grid_set_cell", { payload });
}

export async function gridClearCell(payload: GridClearCellPayload): Promise<GridClearCellResponse> {
  return invoke<GridClearCellResponse>("grid_clear_cell", { payload });
}

export async function gridClearChannel(payload: GridClearChannelPayload): Promise<GridClearChannelResponse> {
  return invoke<GridClearChannelResponse>("grid_clear_channel", { payload });
}

export async function gridClearAll(): Promise<GridClearAllResponse> {
  return invoke<GridClearAllResponse>("grid_clear_all", {});
}

export async function gridCopy(payload: GridCopyPayload): Promise<GridCopyResponse> {
  return invoke<GridCopyResponse>("grid_copy", { payload });
}

export async function gridPaste(payload: GridPastePayload): Promise<GridPasteResponse> {
  return invoke<GridPasteResponse>("grid_paste", { payload });
}

// ==================== PATTERN COMMANDS ====================

export async function patternUpdate(payload: PatternUpdateRequest): Promise<PatternUpdateResponse> {
  return invoke<PatternUpdateResponse>("pattern_update", { payload });
}

// ==================== CHANNEL COMMANDS ====================

export async function channelAdd(payload: ChannelAddPayload = {}): Promise<ChannelAddResponse> {
  return invoke<ChannelAddResponse>("channel_add", { payload });
}

export async function channelRemove(payload: ChannelRemovePayload): Promise<ChannelRemoveResponse> {
  return invoke<ChannelRemoveResponse>("channel_remove", { payload });
}

export async function channelSetName(payload: ChannelSetNamePayload): Promise<ChannelSetNameResponse> {
  return invoke<ChannelSetNameResponse>("channel_set_name", { payload });
}

export async function channelSetVolume(payload: ChannelSetVolumePayload): Promise<ChannelSetVolumeResponse> {
  return invoke<ChannelSetVolumeResponse>("channel_set_volume", { payload });
}

export async function channelSetPan(payload: ChannelSetPanPayload): Promise<ChannelSetPanResponse> {
  return invoke<ChannelSetPanResponse>("channel_set_pan", { payload });
}

export async function channelSetMute(payload: ChannelSetMutePayload): Promise<ChannelSetMuteResponse> {
  return invoke<ChannelSetMuteResponse>("channel_set_mute", { payload });
}

export async function channelSetSolo(payload: ChannelSetSoloPayload): Promise<ChannelSetSoloResponse> {
  return invoke<ChannelSetSoloResponse>("channel_set_solo", { payload });
}

// ==================== INSTRUMENT COMMANDS ====================

export async function instrumentList(_payload: InstrumentListPayload = {}): Promise<InstrumentListResponse> {
  return invoke<InstrumentListResponse>("instrument_list", { payload: _payload });
}

export async function instrumentLoad(payload: InstrumentLoadPayload): Promise<InstrumentLoadResponse> {
  return invoke<InstrumentLoadResponse>("instrument_load", { payload });
}

export async function instrumentAssign(payload: InstrumentAssignPayload): Promise<InstrumentAssignResponse> {
  return invoke<InstrumentAssignResponse>("instrument_assign", { payload });
}

// ==================== PREVIEW COMMANDS ====================

export async function notePreview(payload: NotePreviewPayload): Promise<NotePreviewResponse> {
  return invoke<NotePreviewResponse>("note_preview", { payload });
}

// ==================== SOURCE REGISTRY COMMANDS ====================

export async function listSources(): Promise<ListSourcesResponse> {
  return invoke<ListSourcesResponse>("list_sources", {});
}

export async function getSourceParams(payload: GetSourceParamsPayload): Promise<GetSourceParamsResponse> {
  return invoke<GetSourceParamsResponse>("get_source_params", { request: payload });
}

export async function channelSetSource(payload: ChannelSetSourcePayload): Promise<ChannelSetSourceResponse> {
  return invoke<ChannelSetSourceResponse>("channel_set_source", { request: payload });
}

export async function setSourceParam(payload: SetSourceParamPayload): Promise<SetSourceParamResponse> {
  return invoke<SetSourceParamResponse>("set_source_param", { request: payload });
}
