<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import { evalByNodeId } from '$lib/projectStores';
  import type { ProjectRule, ProjectParam } from '$lib/types';

  let { id, data }: {
    id: string;
    data: {
      name: string;
      hit_policy: string;
      input_columns: string[];
      output_columns: string[];
      rules: ProjectRule[];
      parameters: ProjectParam[];
      feel_expression?: string;
    };
  } = $props();

  let evalResult = $derived($evalByNodeId.get(id));
  let isEvaluated = $derived(!!evalResult);
  let matchedRules = $derived(evalResult ? new Set(evalResult.matched_rules) : new Set<number>());
</script>

<div class="bkm-node" class:evaluated={isEvaluated}>
  <Handle type="target" position={Position.Top} />
  <Handle type="source" position={Position.Bottom} />

  <div class="header">
    <span class="name">{data.name}</span>
    <span class="badge">BKM</span>
  </div>

  {#if data.parameters.length > 0}
    <div class="params">
      {#each data.parameters as param}
        <span class="param">{param.name}: {param.param_type}</span>
      {/each}
    </div>
  {/if}

  {#if data.feel_expression}
    <div class="expression">
      <span class="expr-label">FEEL</span>
      <code class="expr-text">{data.feel_expression}</code>
    </div>
  {:else if data.rules.length > 0}
    <div class="columns">
      {#each data.input_columns as col}
        <span class="col">{col}</span>
      {/each}
      <span class="sep">&rarr;</span>
      {#each data.output_columns as col}
        <span class="col out">{col}</span>
      {/each}
    </div>
    <div class="rules">
      {#each data.rules as rule}
        <div class="rule" class:matched={matchedRules.has(rule.index)}>
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
  {/if}

  {#if evalResult}
    <div class="eval-footer">
      <span>&rarr; {JSON.stringify(evalResult.output_value)}</span>
    </div>
  {/if}
</div>

<style>
  .bkm-node {
    background: #0d1117;
    border: 2px dashed #8b949e;
    border-radius: 8px;
    font-family: monospace;
    font-size: 11px;
    min-width: 200px;
    overflow: hidden;
  }
  .bkm-node.evaluated { border-color: #3fb950; }
  .header {
    background: #161b22; padding: 8px 12px; display: flex;
    justify-content: space-between; align-items: center; border-bottom: 1px solid #30363d;
  }
  .evaluated .header { background: #1a3a1a; }
  .name { color: #c9d1d9; font-weight: bold; font-size: 12px; }
  .evaluated .name { color: #3fb950; }
  .badge { background: #21262d; color: #8b949e; padding: 2px 6px; border-radius: 4px; font-size: 9px; }
  .expression { padding: 8px 12px; border-bottom: 1px solid #30363d; }
  .expr-label { color: #8b949e; font-size: 9px; margin-right: 6px; }
  .expr-text { color: #d2a8ff; font-size: 11px; }
  .evaluated .expr-text { color: #3fb950; }
  .params { padding: 4px 12px; display: flex; gap: 6px; flex-wrap: wrap; border-bottom: 1px solid #30363d; }
  .param { color: #8b949e; font-size: 9px; background: #161b22; padding: 1px 5px; border-radius: 3px; }
  .columns {
    display: flex; gap: 8px; padding: 4px 12px;
    background: #161b22; border-bottom: 1px solid #30363d; color: #8b949e; font-size: 10px;
  }
  .col.out { color: #58a6ff; }
  .sep { color: #484f58; }
  .rules { padding: 2px 0; }
  .rule { display: flex; gap: 6px; padding: 3px 12px; color: #8b949e; border-bottom: 1px solid #21262d; }
  .rule.matched { background: #1a3a1a; border-left: 3px solid #3fb950; color: #3fb950; }
  .cell { flex: 1; font-size: 10px; }
  .output { font-weight: bold; }
  .rule.matched .output { color: #3fb950; }
  .eval-footer {
    background: #1a3a1a; padding: 6px 12px; border-top: 1px solid #3fb950;
    color: #3fb950; font-size: 10px; font-weight: bold;
  }
</style>
