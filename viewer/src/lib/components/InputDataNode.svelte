<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import { evaluatedNodeIds } from '$lib/stores';

  let { id, data }: { id: string; data: { name: string; value?: unknown } } = $props();

  let isEvaluated = $derived($evaluatedNodeIds.has(id));
</script>

<div class="input-node" class:evaluated={isEvaluated}>
  <Handle type="source" position={Position.Bottom} />
  <div class="name">{data.name}</div>
  {#if data.value !== undefined && data.value !== null}
    <div class="value">{data.value}</div>
  {/if}
</div>

<style>
  .input-node {
    background: #0d1117;
    border: 2px solid #58a6ff;
    border-radius: 12px;
    padding: 8px 16px;
    font-family: monospace;
    min-width: 120px;
    text-align: center;
  }
  .input-node.evaluated {
    border-color: #3fb950;
  }
  .name { color: #c9d1d9; font-size: 12px; }
  .value { color: #58a6ff; font-size: 14px; font-weight: bold; margin-top: 2px; }
  .evaluated .value { color: #3fb950; }
</style>
