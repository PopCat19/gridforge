// instrument-panel.tsx
//
// Purpose: Dynamic instrument panel driven by source registry parameter descriptors
//
// This module:
// - Lists available sources from backend registry
// - Renders parameter controls from source param descriptors
// - Syncs source selection and param changes to backend

import { Component, createSignal, createEffect, For, Show } from "solid-js";
import { listSources, getSourceParams, channelSetSource, setSourceParam } from "../ipc/commands";
import type { SourceInfo, ParamInfo } from "../ipc/dto";
import type { Channel } from "../stores/project";

interface InstrumentPanelProps {
  selectedChannel: number | null;
  channel: Channel | null;
  onSourceChange?: (channelId: number, sourceId: string) => void;
}

const InstrumentPanel: Component<InstrumentPanelProps> = (props) => {
  const [sources, setSources] = createSignal<SourceInfo[]>([]);
  const [params, setParams] = createSignal<ParamInfo[]>([]);
  const [paramValues, setParamValues] = createSignal<Record<string, number>>({});
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  // Fetch sources on mount
  createEffect(async () => {
    try {
      const response = await listSources();
      setSources(response.sources);
    } catch (err) {
      console.error("Failed to load sources:", err);
      setError("Failed to load sources");
    }
  });

  // Fetch params when channel's source changes
  createEffect(async () => {
    const channel = props.channel;
    if (!channel) {
      setParams([]);
      setParamValues({});
      return;
    }

    const sourceId = channel.instrument;
    if (!sourceId) {
      setParams([]);
      setParamValues({});
      return;
    }

    setLoading(true);
    try {
      const response = await getSourceParams({ source_id: sourceId });
      setParams(response.params);

      // Initialize param values from defaults
      const values: Record<string, number> = {};
      for (const param of response.params) {
        values[param.id] = param.default;
      }
      setParamValues(values);
    } catch (err) {
      console.error("Failed to load source params:", err);
      setError("Failed to load parameters");
      setParams([]);
    } finally {
      setLoading(false);
    }
  });

  const handleSourceChange = async (sourceId: string) => {
    const channelId = props.selectedChannel;
    if (channelId === null) return;

    try {
      await channelSetSource({
        channel: channelId,
        source_id: sourceId,
      });
      // Trigger callback for store update if provided
      props.onSourceChange?.(channelId, sourceId);
    } catch (err) {
      console.error("Failed to set source:", err);
      setError("Failed to set source");
    }
  };

  const handleParamChange = async (paramId: string, value: number) => {
    const channelId = props.selectedChannel;
    if (channelId === null) return;

    // Update local state immediately for responsive UI
    setParamValues((prev) => ({ ...prev, [paramId]: value }));

    try {
      await setSourceParam({
        channel: channelId,
        param_id: paramId,
        value,
      });
    } catch (err) {
      console.error("Failed to set param:", err);
    }
  };

  const currentSourceId = () => props.channel?.instrument ?? "";

  return (
    <div class="panel instrument-panel">
      <h3>Instrument</h3>

      <Show when={props.selectedChannel !== null} fallback={<p class="placeholder-text">Select a channel</p>}>
        <div class="instrument-controls">
          <div class="control-group">
            <label>Source</label>
            <select
              value={currentSourceId()}
              onChange={(e) => handleSourceChange(e.target.value)}
            >
              <option value="">-- Select Source --</option>
              <For each={sources()}>
                {(source) => <option value={source.id}>{source.name}</option>}
              </For>
            </select>
          </div>

          <Show when={loading()}>
            <p class="placeholder-text">Loading parameters...</p>
          </Show>

          <Show when={!loading() && params().length > 0}>
            <For each={params()}>
              {(param) => (
                <div class="control-group">
                  <label>
                    {param.name}
                    <span class="param-value">{paramValues()[param.id]?.toFixed(2) ?? param.default}</span>
                    <span class="param-unit">{param.unit}</span>
                  </label>
                  <input
                    type="range"
                    min={param.min}
                    max={param.max}
                    step={(param.max - param.min) / 100}
                    value={paramValues()[param.id] ?? param.default}
                    onInput={(e) => handleParamChange(param.id, parseFloat(e.target.value))}
                  />
                </div>
              )}
            </For>
          </Show>

          <Show when={!loading() && params().length === 0 && currentSourceId()}>
            <p class="placeholder-text">No parameters available</p>
          </Show>
        </div>
      </Show>

      <Show when={error()}>
        <p class="error-text">{error()}</p>
      </Show>
    </div>
  );
};

export default InstrumentPanel;
