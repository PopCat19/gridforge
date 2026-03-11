// channel-strip.tsx
//
// Purpose: Channel volume and controls UI
//
// This module:
// - Renders channel strip per channel from project store
// - Displays channel name, volume slider, mute/solo buttons
// - Includes source selector dropdown placeholder

import { Component, For } from "solid-js";
import { Channel } from "../stores/project";
import { PlaybackStoreApi, ChannelLevel } from "../stores/playback";

export interface ChannelStripProps {
  channels: Channel[];
  selectedChannel: number | null;
  playbackStore: PlaybackStoreApi;
  onSelectChannel: (channelId: number) => void;
  onUpdateChannel: (channelId: number, updates: Partial<Omit<Channel, "id">>) => void;
}

const ChannelStrip: Component<ChannelStripProps> = (props) => {
  const sources = ["Chip Synth", "Noise", "Saw", "Sine"];

  return (
    <div class="channel-strips-container">
      <For each={props.channels}>
        {(channel) => {
          const isSelected = () => props.selectedChannel === channel.id;
          const volumePercent = () => Math.round(channel.volume * 100);

          const handleVolumeChange = (e: Event) => {
            const target = e.target as HTMLInputElement;
            const newVolume = parseInt(target.value, 10) / 100;
            props.onUpdateChannel(channel.id, { volume: newVolume });
          };

          const handleMuteToggle = () => {
            props.onUpdateChannel(channel.id, { mute: !channel.mute });
          };

          const handleSoloToggle = () => {
            props.onUpdateChannel(channel.id, { solo: !channel.solo });
          };

          const handleSourceChange = (e: Event) => {
            const target = e.target as HTMLSelectElement;
            props.onUpdateChannel(channel.id, { instrument: target.value });
          };

          // Get level for this channel (if available)
          const channelLevel = (): ChannelLevel | undefined => {
            const levels = props.playbackStore.store.playback.levels;
            return levels[channel.id];
          };

          const levelPercent = () => {
            const lvl = channelLevel();
            return lvl ? Math.min(100, Math.round(lvl.peak * 100)) : 0;
          };

          const levelRmsPercent = () => {
            const lvl = channelLevel();
            return lvl ? Math.min(100, Math.round(lvl.rms * 100)) : 0;
          };

          return (
            <div
              class={`panel channel-strip ${isSelected() ? "selected" : ""}`}
              onClick={() => props.onSelectChannel(channel.id)}
            >
              <h3 class="channel-name">{channel.name}</h3>

              <div class="channel-controls">
                <div class="source-selector">
                  <label>Source</label>
                  <select value={channel.instrument || ""} onChange={handleSourceChange}>
                    <option value="">Select...</option>
                    <For each={sources}>
                      {(source) => <option value={source}>{source}</option>}
                    </For>
                  </select>
                </div>

                <div class="volume-slider">
                  <label>Vol</label>
                  <input
                    type="range"
                    min="0"
                    max="100"
                    value={volumePercent()}
                    onInput={handleVolumeChange}
                  />
                  <span class="volume-value">{volumePercent()}%</span>
                </div>

                <div class="channel-buttons">
                  <button
                    class={`mute-btn ${channel.mute ? "active" : ""}`}
                    onClick={(e) => {
                      e.stopPropagation();
                      handleMuteToggle();
                    }}
                  >
                    M
                  </button>
                  <button
                    class={`solo-btn ${channel.solo ? "active" : ""}`}
                    onClick={(e) => {
                      e.stopPropagation();
                      handleSoloToggle();
                    }}
                  >
                    S
                  </button>
                </div>

                <div class="level-meter">
                  <label>Level</label>
                  <div class="meter-container">
                    <div
                      class="meter-peak"
                      style={{ width: `${levelPercent()}%` }}
                    />
                    <div
                      class="meter-rms"
                      style={{ width: `${levelRmsPercent()}%` }}
                    />
                  </div>
                  <span class="level-value">{levelPercent()}%</span>
                </div>
              </div>
            </div>
          );
        }}
      </For>
      {props.channels.length === 0 && (
        <div class="panel channel-strip empty">
          <p class="placeholder-text">No channels</p>
        </div>
      )}
    </div>
  );
};

export default ChannelStrip;
