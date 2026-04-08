<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';

  let { data }: { id: string; data: { name: string; owner?: string } } = $props();
</script>

<div class="ks-node">
  <Handle type="target" position={Position.Top} />
  <Handle type="source" position={Position.Bottom} />
  <div class="name">{data.name}</div>
  {#if data.owner}
    <div class="owner">{data.owner}</div>
  {/if}
</div>

<style>
  .ks-node {
    background: #0d1117;
    border: 2px solid #d29922;
    border-radius: 4px;
    padding: 8px 16px;
    font-family: monospace;
    min-width: 120px;
    text-align: center;
  }
  .name { color: #d29922; font-size: 11px; font-weight: bold; }
  .owner { color: #484f58; font-size: 9px; margin-top: 2px; }
</style>
