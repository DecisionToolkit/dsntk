<script lang="ts">
  import { SvelteFlow, MiniMap, Controls, Background } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import { projectData } from '$lib/projectStores';
  import { computeProjectLayout } from '$lib/projectLayout';
  import ProjectInputNode from './ProjectInputNode.svelte';
  import ProjectDecisionNode from './ProjectDecisionNode.svelte';
  import ProjectBkmNode from './ProjectBkmNode.svelte';
  import ProjectKsNode from './ProjectKsNode.svelte';

  const nodeTypes = {
    projectInput: ProjectInputNode,
    projectDecision: ProjectDecisionNode,
    projectBkm: ProjectBkmNode,
    projectKs: ProjectKsNode,
  };

  let layoutResult = $derived($projectData ? computeProjectLayout($projectData) : null);
  let nodes = $derived(layoutResult ? layoutResult.nodes : []);
  let edges = $derived(layoutResult ? layoutResult.edges : []);
</script>

<div class="canvas">
  {#if $projectData}
    <div class="info-bar">
      <span>{$projectData.nodes.length} nodes</span>
      <span>{$projectData.edges.length} edges</span>
      <span>eval order: {$projectData.evaluation_order.length}</span>
    </div>
    <SvelteFlow {nodes} {edges} {nodeTypes} fitView minZoom={0.1} maxZoom={2}>
      <Background />
      <Controls />
      <MiniMap />
    </SvelteFlow>
  {:else}
    <div class="empty">
      <p>Enter a project directory path and click <strong>Load Project</strong> to visualize the DRG.</p>
    </div>
  {/if}
</div>

<style>
  .canvas { flex: 1; position: relative; background: #0d1117; }
  .info-bar {
    position: absolute; top: 12px; left: 12px; z-index: 10;
    display: flex; gap: 12px;
    background: rgba(22, 27, 34, 0.9); border: 1px solid #30363d; border-radius: 6px;
    padding: 6px 12px; color: #8b949e; font-size: 11px; font-family: monospace;
  }
  .empty {
    display: flex; align-items: center; justify-content: center;
    height: 100%; color: #484f58; font-size: 14px;
  }
</style>
