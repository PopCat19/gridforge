// app.tsx
//
// Purpose: Main Gridforge app layout - orchestrates all panel components

import { Component, createSignal, onMount, onCleanup, createMemo } from "solid-js";
import Transport from "./components/transport";
import GridEditor from "./components/grid-editor";
import ChannelStrip from "./components/channel-strip";
import PatternSeq from "./components/pattern-seq";
import InstrumentPanel from "./components/instrument-panel";
import { createProjectStore, ProjectStoreApi } from "./stores/project";
import { createPlaybackStore, PlaybackStoreApi } from "./stores/playback";
import "./app.css";

const App: Component = () => {
  const [projectStore, setProjectStore] = createSignal<ProjectStoreApi | null>(null);
  const [playbackStore, setPlaybackStore] = createSignal<PlaybackStoreApi | null>(null);

  onMount(async () => {
    // Initialize project store
    const store = createProjectStore();
    setProjectStore(store);

    // Initialize playback store
    const pbStore = createPlaybackStore();
    setPlaybackStore(pbStore);

    // Subscribe to backend events
    const unlistenPlayhead = await pbStore.subscribeToPlayhead();
    const unlistenLevels = await pbStore.subscribeToLevels();

    // Cleanup subscriptions on unmount
    onCleanup(() => {
      unlistenPlayhead();
      unlistenLevels();
    });

    // Add default channels for demo
    store.addChannel("Lead");
    store.addChannel("Bass");
    store.addChannel("Drums");
    store.addChannel("FX");
  });

  // Memoized selected channel data
  const selectedChannelData = createMemo(() => {
    const store = projectStore();
    if (!store) return null;
    const selectedId = store.store.ui.selectedChannel;
    if (selectedId === null) return null;
    return store.store.project.channels.find((c) => c.id === selectedId) ?? null;
  });

  const handleSourceChange = (channelId: number, sourceId: string) => {
    projectStore()?.updateChannel(channelId, { instrument: sourceId });
  };

  return (
    <div class="app-container">
      <header class="app-header">
        <Transport />
      </header>

      <div class="app-main">
        <aside class="app-sidebar">
          {projectStore() && (
            <ChannelStrip
              channels={projectStore()!.store.project.channels}
              selectedChannel={projectStore()!.store.ui.selectedChannel}
              playbackStore={playbackStore()!}
              onSelectChannel={(channelId: number) => projectStore()?.selectChannel(channelId)}
              onUpdateChannel={(channelId: number, updates: any) => projectStore()?.updateChannel(channelId, updates)}
            />
          )}
        </aside>

        <section class="app-content">
          <GridEditor />
          {projectStore() && (
            <PatternSeq store={projectStore()!} playbackStore={playbackStore()!} />
          )}
        </section>
      </div>

      <footer class="app-footer">
        <InstrumentPanel
          selectedChannel={projectStore()?.store.ui.selectedChannel ?? null}
          channel={selectedChannelData()}
          onSourceChange={handleSourceChange}
        />
      </footer>
    </div>
  );
};

export default App;
