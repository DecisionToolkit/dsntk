<script lang="ts">
  import { SvelteFlow, MiniMap, Controls, Background } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import { traceData } from '$lib/stores';
  import { computeLayout } from '$lib/layout';
  import InputDataNode from './InputDataNode.svelte';
  import DecisionTableNode from './DecisionTableNode.svelte';
  import AnimatedEdge from './AnimatedEdge.svelte';
  import StepControls from './StepControls.svelte';

  const nodeTypes = { inputData: InputDataNode, decisionTable: DecisionTableNode };
  const edgeTypes = { animated: AnimatedEdge };

  let layoutResult = $derived($traceData ? computeLayout($traceData.graph) : null);
  let nodes = $derived(layoutResult ? layoutResult.nodes : []);
  let edges = $derived(layoutResult ? layoutResult.edges : []);
</script>

<div class="canvas">
  {#if $traceData}
    <div class="step-controls-overlay">
      <StepControls />
    </div>
    <SvelteFlow {nodes} {edges} {nodeTypes} {edgeTypes} fitView minZoom={0.2} maxZoom={2}>
      <Background />
      <Controls />
      <MiniMap />
    </SvelteFlow>
  {:else}
    <div class="empty">
      <p>Select a model and click <strong>Evaluate</strong> to visualize the decision graph.</p>
    </div>
  {/if}
</div>

<style>
  .canvas { flex: 1; position: relative; background: #0d1117; }
  .step-controls-overlay { position: absolute; top: 12px; left: 12px; z-index: 10; }
  .empty {
    display: flex; align-items: center; justify-content: center;
    height: 100%; color: #484f58; font-size: 14px;
  }
</style>
