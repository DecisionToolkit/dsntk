<script lang="ts">
  import InputPanel from '$lib/components/InputPanel.svelte';
  import DagCanvas from '$lib/components/DagCanvas.svelte';
  import ProjectInputPanel from '$lib/components/ProjectInputPanel.svelte';
  import ProjectCanvas from '$lib/components/ProjectCanvas.svelte';
  import '../app.css';

  let mode: 'trace' | 'project' = $state('project');
</script>

<div class="app">
  <div class="mode-switcher">
    <button class:active={mode === 'project'} onclick={() => mode = 'project'}>Project</button>
    <button class:active={mode === 'trace'} onclick={() => mode = 'trace'}>Trace</button>
  </div>
  <div class="layout">
    {#if mode === 'project'}
      <ProjectInputPanel />
      <ProjectCanvas />
    {:else}
      <InputPanel />
      <DagCanvas />
    {/if}
  </div>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .mode-switcher {
    display: flex;
    gap: 0;
    background: #0d1117;
    border-bottom: 1px solid #30363d;
    padding: 0 12px;
    flex-shrink: 0;
  }

  .mode-switcher button {
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    color: #8b949e;
    padding: 8px 16px;
    font-size: 12px;
    font-family: monospace;
    cursor: pointer;
  }

  .mode-switcher button:hover {
    color: #c9d1d9;
  }

  .mode-switcher button.active {
    color: #58a6ff;
    border-bottom-color: #58a6ff;
  }

  .layout {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
</style>
