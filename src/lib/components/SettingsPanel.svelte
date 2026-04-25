<script lang="ts">
  import { settingsStore } from "../stores/settings";

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  const keybindings = [
    { key: "Ctrl + Enter", action: "Run query" },
    { key: "Ctrl + Shift + Enter", action: "Run in new tab" },
    { key: "Escape", action: "Cancel running query" },
    { key: "Double-click cell", action: "Edit cell value" },
    { key: "Enter", action: "Confirm cell edit" },
    { key: "Escape", action: "Cancel cell edit" },
  ];
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm" onclick={(e) => e.target === e.currentTarget && onClose()}>
  <div class="bg-[#141414] border border-[#2a2a2a] rounded-xl w-[520px] max-w-[90vw] max-h-[85vh] flex flex-col shadow-2xl">
    <!-- Header -->
    <div class="flex items-center justify-between px-5 py-3 border-b border-[#2a2a2a]">
      <h2 class="text-sm font-semibold text-[#e8e8e8]">Settings</h2>
      <button onclick={onClose} class="text-[#6b6b6b] hover:text-[#e8e8e8]">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
      </button>
    </div>

    <!-- Body -->
    <div class="flex-1 overflow-auto p-5 space-y-6">
      <!-- Appearance -->
      <div>
        <h3 class="text-[11px] text-[#6b6b6b] uppercase tracking-wider mb-3">Appearance</h3>
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <label class="text-[12px] text-[#a0a0a0]">Theme</label>
            <select
              value={settingsStore.settings.theme}
              onchange={(e) => settingsStore.save({ theme: e.currentTarget.value as any })}
              class="bg-[#0c0c0c] border border-[#2a2a2a] text-[#e8e8e8] text-[11px] rounded px-2 py-1 outline-none focus:border-[#00d4ff]"
            >
              <option value="dark">Dark</option>
              <option value="light">Light</option>
              <option value="system">System</option>
            </select>
          </div>
          <div class="flex items-center justify-between">
            <label class="text-[12px] text-[#a0a0a0]">Editor font size</label>
            <div class="flex items-center gap-2">
              <input
                type="range" min="10" max="20" step="1"
                value={settingsStore.settings.editorFontSize}
                onchange={(e) => settingsStore.save({ editorFontSize: parseInt(e.currentTarget.value) })}
                class="w-24 accent-[#00d4ff]"
              />
              <span class="text-[11px] text-[#a0a0a0] w-6 text-right">{settingsStore.settings.editorFontSize}px</span>
            </div>
          </div>
          <div class="flex items-center justify-between">
            <label class="text-[12px] text-[#a0a0a0]">Result font size</label>
            <div class="flex items-center gap-2">
              <input
                type="range" min="10" max="16" step="1"
                value={settingsStore.settings.resultFontSize}
                onchange={(e) => settingsStore.save({ resultFontSize: parseInt(e.currentTarget.value) })}
                class="w-24 accent-[#00d4ff]"
              />
              <span class="text-[11px] text-[#a0a0a0] w-6 text-right">{settingsStore.settings.resultFontSize}px</span>
            </div>
          </div>
          <div class="flex items-center justify-between">
            <label class="text-[12px] text-[#a0a0a0]">Show line numbers</label>
            <button
              onclick={() => settingsStore.save({ showLineNumbers: !settingsStore.settings.showLineNumbers })}
              class="w-8 h-4 rounded-full transition-colors {settingsStore.settings.showLineNumbers ? 'bg-[#00d4ff]' : 'bg-[#2a2a2a]'} relative"
            >
              <span class="absolute top-0.5 w-3 h-3 rounded-full bg-white transition-transform {settingsStore.settings.showLineNumbers ? 'translate-x-4' : 'translate-x-0.5'}"></span>
            </button>
          </div>
        </div>
      </div>

      <!-- Query -->
      <div>
        <h3 class="text-[11px] text-[#6b6b6b] uppercase tracking-wider mb-3">Query</h3>
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <label class="text-[12px] text-[#a0a0a0]">Query timeout</label>
            <div class="flex items-center gap-2">
              <input
                type="range" min="5" max="120" step="5"
                value={settingsStore.settings.queryTimeout}
                onchange={(e) => settingsStore.save({ queryTimeout: parseInt(e.currentTarget.value) })}
                class="w-24 accent-[#00d4ff]"
              />
              <span class="text-[11px] text-[#a0a0a0] w-10 text-right">{settingsStore.settings.queryTimeout}s</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Keybindings -->
      <div>
        <h3 class="text-[11px] text-[#6b6b6b] uppercase tracking-wider mb-3">Keyboard Shortcuts</h3>
        <div class="border border-[#2a2a2a] rounded overflow-hidden">
          {#each keybindings as kb}
            <div class="flex items-center justify-between px-3 py-2 text-[11px] border-b border-[#1e1e1e] last:border-b-0">
              <span class="text-[#a0a0a0]">{kb.action}</span>
              <kbd class="px-1.5 py-0.5 bg-[#1a1a1a] border border-[#2a2a2a] rounded text-[#e8e8e8] font-mono text-[10px]">{kb.key}</kbd>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end px-5 py-3 border-t border-[#2a2a2a]">
      <button onclick={onClose} class="px-3 py-1.5 text-[11px] text-[#a0a0a0] hover:text-[#e8e8e8]">Close</button>
    </div>
  </div>
</div>
