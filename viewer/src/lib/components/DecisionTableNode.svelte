<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import { evaluatedNodeIds, evaluatingNodeId, stepByNodeId } from '$lib/stores';
  import type { TraceRule } from '$lib/types';

  let { id, data }: {
    id: string;
    data: {
      name: string;
      hit_policy: string;
      input_columns: string[];
      output_columns: string[];
      rules: TraceRule[];
    };
  } = $props();

  let isEvaluated = $derived($evaluatedNodeIds.has(id));
  let isEvaluating = $derived($evaluatingNodeId === id);
  let step = $derived($stepByNodeId.get(id));
  let matchedRules = $derived(step ? new Set(step.matched_rules) : new Set<number>());
</script>

<div class="dt-node" class:evaluated={isEvaluated} class:evaluating={isEvaluating}>
  <Handle type="target" position={Position.Top} />
  <Handle type="source" position={Position.Bottom} />

  <div class="header">
    <span class="name">{data.name}</span>
    <span class="hit-policy">{data.hit_policy}</span>
  </div>

  <div class="columns">
    {#each data.input_columns as col}
      <span class="col">{col}</span>
    {/each}
    <span class="sep">→</span>
    {#each data.output_columns as col}
      <span class="col">{col}</span>
    {/each}
  </div>

  <div class="rules">
    {#each data.rules as rule}
      <div class="rule" class:matched={matchedRules.has(rule.index)}>
        {#each rule.input_entries as entry}
          <span class="cell">{entry}</span>
        {/each}
        <span class="sep">→</span>
        {#each rule.output_entries as entry}
          <span class="cell output">{entry}</span>
        {/each}
      </div>
    {/each}
  </div>

  {#if step}
    <div class="footer">
      <span>Rule {step.matched_rules.join(', ')} matched</span>
      <span class="result">→ {step.output_value}</span>
    </div>
  {/if}
</div>

<style>
  .dt-node {
    background: #0d1117;
    border: 2px dashed #30363d;
    border-radius: 8px;
    font-family: monospace;
    font-size: 11px;
    min-width: 280px;
    overflow: hidden;
  }
  .dt-node.evaluating { border: 2px solid #d29922; box-shadow: 0 0 12px rgba(210, 153, 34, 0.3); }
  .dt-node.evaluated { border: 2px solid #3fb950; }
  .header {
    background: #161b22; padding: 8px 12px; display: flex;
    justify-content: space-between; align-items: center; border-bottom: 1px solid #30363d;
  }
  .evaluated .header { background: #1a3a1a; }
  .name { color: #c9d1d9; font-weight: bold; font-size: 12px; }
  .evaluated .name { color: #3fb950; }
  .hit-policy { background: #21262d; color: #8b949e; padding: 2px 6px; border-radius: 4px; font-size: 9px; }
  .columns {
    display: flex; gap: 8px; padding: 4px 12px;
    background: #161b22; border-bottom: 1px solid #30363d; color: #8b949e; font-size: 10px;
  }
  .sep { color: #484f58; }
  .rules { padding: 2px 0; }
  .rule {
    display: flex; gap: 8px; padding: 4px 12px; color: #484f58; border-bottom: 1px solid #21262d;
  }
  .rule.matched { background: #1a3a1a; border-left: 3px solid #3fb950; color: #3fb950; }
  .cell { flex: 1; }
  .output { font-weight: bold; }
  .footer {
    background: #1a3a1a; padding: 6px 12px; border-top: 1px solid #3fb950;
    color: #3fb950; display: flex; justify-content: space-between; font-size: 10px;
  }
  .result { font-weight: bold; }
</style>
