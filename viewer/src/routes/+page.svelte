<script lang="ts">
  import InputPanel from '$lib/components/InputPanel.svelte';
  import DagCanvas from '$lib/components/DagCanvas.svelte';
  import '../app.css';
</script>

<div class="layout">
  <InputPanel />
  <DagCanvas />
</div>

<style>
  .layout {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }
</style>
