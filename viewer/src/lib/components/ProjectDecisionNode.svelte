<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import { evalByNodeId } from '$lib/projectStores';
  import type { ProjectRule } from '$lib/types';

  let { id, data }: {
    id: string;
    data: {
      name: string;
      hit_policy: string;
      input_columns: string[];
      output_columns: string[];
      rules: ProjectRule[];
      annotation_columns?: string[];
    };
  } = $props();

  let evalResult = $derived($evalByNodeId.get(id));
  let isEvaluated = $derived(!!evalResult);
  let matchedRules = $derived(evalResult ? new Set(evalResult.matched_rules) : new Set<number>());
</script>

<div class="dt-node" class:evaluated={isEvaluated}>
  <Handle type="target" position={Position.Top} />
  <Handle type="source" position={Position.Bottom} />

  <div class="header">
    <span class="name">{data.name}</span>
    <span class="hit-policy">{data.hit_policy}</span>
  </div>

  {#if data.input_columns.length > 0 || data.output_columns.length > 0}
    <div class="columns">
      {#each data.input_columns as col}
        <span class="col">{col}</span>
      {/each}
      <span class="sep">&rarr;</span>
      {#each data.output_columns as col}
        <span class="col out">{col}</span>
      {/each}
    </div>
  {/if}

  <div class="rules">
    {#each data.rules as rule}
      <div class="rule" class:matched={matchedRules.has(rule.index)}>
        <span class="rule-num">{rule.index + 1}</span>
        {#each rule.input_entries as entry}
          <span class="cell">{entry}</span>
        {/each}
        <span class="sep">&rarr;</span>
        {#each rule.output_entries as entry}
          <span class="cell output">{entry}</span>
        {/each}
      </div>
    {/each}
  </div>

  {#if evalResult}
    <div class="eval-footer">
      <span class="eval-label">
        {evalResult.matched_rules.length} rule{evalResult.matched_rules.length !== 1 ? 's' : ''} matched
      </span>
      <span class="eval-value">&rarr; {JSON.stringify(evalResult.output_value)}</span>
    </div>
  {/if}
</div>

<style>
  .dt-node {
    background: #0d1117;
    border: 2px solid #30363d;
    border-radius: 8px;
    font-family: monospace;
    font-size: 11px;
    min-width: 280px;
    overflow: hidden;
  }
  .dt-node.evaluated {
    border-color: #3fb950;
  }
  .header {
    background: #161b22; padding: 8px 12px; display: flex;
    justify-content: space-between; align-items: center; border-bottom: 1px solid #30363d;
  }
  .evaluated .header { background: #1a3a1a; }
  .name { color: #c9d1d9; font-weight: bold; font-size: 12px; }
  .evaluated .name { color: #3fb950; }
  .hit-policy { background: #21262d; color: #d29922; padding: 2px 6px; border-radius: 4px; font-size: 9px; }
  .columns {
    display: flex; gap: 8px; padding: 4px 12px;
    background: #161b22; border-bottom: 1px solid #30363d; color: #8b949e; font-size: 10px;
  }
  .col.out { color: #58a6ff; }
  .sep { color: #484f58; }
  .rules { padding: 2px 0; max-height: 300px; overflow-y: auto; }
  .rule {
    display: flex; gap: 6px; padding: 3px 12px; color: #484f58; border-bottom: 1px solid #21262d;
    align-items: center;
  }
  .rule.matched {
    background: #1a3a1a;
    border-left: 3px solid #3fb950;
    color: #3fb950;
  }
  .rule-num { color: #484f58; font-size: 9px; min-width: 16px; }
  .rule.matched .rule-num { color: #3fb950; }
  .cell { flex: 1; font-size: 10px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .output { font-weight: bold; }
  .rule.matched .output { color: #3fb950; }
  .eval-footer {
    background: #1a3a1a; padding: 6px 12px; border-top: 1px solid #3fb950;
    color: #3fb950; display: flex; justify-content: space-between; font-size: 10px;
  }
  .eval-value { font-weight: bold; max-width: 180px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
