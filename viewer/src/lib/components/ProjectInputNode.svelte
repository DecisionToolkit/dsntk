<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { FieldDescriptor } from '$lib/types';

  let { data }: { id: string; data: { name: string; fields: FieldDescriptor[] } } = $props();
</script>

<div class="input-node">
  <Handle type="source" position={Position.Bottom} />
  <div class="name">{data.name}</div>
  {#if data.fields.length > 0}
    <div class="fields">
      {#each data.fields as field}
        <span class="field" class:optional={field.optional}>
          {field.name}<span class="type">:{field.feel_type}</span>
        </span>
      {/each}
    </div>
  {/if}
</div>

<style>
  .input-node {
    background: #0d1117;
    border: 2px solid #58a6ff;
    border-radius: 12px;
    padding: 8px 16px;
    font-family: monospace;
    min-width: 160px;
    text-align: center;
  }
  .name { color: #c9d1d9; font-size: 12px; font-weight: bold; }
  .fields { margin-top: 4px; display: flex; flex-wrap: wrap; gap: 4px; justify-content: center; }
  .field { color: #8b949e; font-size: 9px; background: #161b22; padding: 1px 5px; border-radius: 3px; }
  .field.optional { opacity: 0.6; }
  .type { color: #484f58; }
</style>
