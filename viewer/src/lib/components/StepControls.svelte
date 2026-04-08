<script lang="ts">
  import { onDestroy } from 'svelte';
  import { currentStep, totalSteps } from '$lib/stores';

  let playing = $state(false);
  let speed = $state(1000);
  let interval: ReturnType<typeof setInterval> | null = $state(null);

  function stepForward() { currentStep.update((n) => Math.min(n + 1, $totalSteps)); }
  function stepBack() { currentStep.update((n) => Math.max(n - 1, 0)); }
  function skipToStart() { currentStep.set(0); }
  function skipToEnd() { currentStep.set($totalSteps); }

  function togglePlay() {
    if (playing) { stopPlaying(); } else { startPlaying(); }
  }

  function startPlaying() {
    playing = true;
    interval = setInterval(() => {
      currentStep.update((n) => {
        if (n >= $totalSteps) { stopPlaying(); return n; }
        return n + 1;
      });
    }, speed);
  }

  function stopPlaying() {
    playing = false;
    if (interval) { clearInterval(interval); interval = null; }
  }

  onDestroy(() => { if (interval) clearInterval(interval); });
</script>

<div class="controls">
  <button onclick={skipToStart} title="Skip to start">⏮</button>
  <button onclick={stepBack} title="Step back">◀</button>
  <button onclick={togglePlay} class:playing title={playing ? 'Pause' : 'Play'}>
    {playing ? '⏸' : '▶'}
  </button>
  <button onclick={stepForward} title="Step forward">▶</button>
  <button onclick={skipToEnd} title="Skip to end">⏭</button>
  <span class="counter">Step {$currentStep} / {$totalSteps}</span>
  <label class="speed">
    <input type="range" min="200" max="3000" step="100" bind:value={speed} />
    {speed}ms
  </label>
</div>

<style>
  .controls {
    display: flex; align-items: center; gap: 4px;
    background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 6px 10px;
  }
  button {
    background: #21262d; border: 1px solid #30363d; border-radius: 6px;
    color: #c9d1d9; padding: 6px 10px; cursor: pointer; font-size: 12px;
  }
  button:hover { background: #30363d; }
  button.playing { background: #238636; border-color: #2ea043; color: #fff; }
  .counter { color: #8b949e; font-size: 12px; margin-left: 8px; font-family: monospace; }
  .speed {
    color: #8b949e; font-size: 10px; margin-left: 8px;
    display: flex; align-items: center; gap: 4px;
  }
  .speed input { width: 80px; }
</style>
