<script lang="ts">
  import { BaseEdge, getBezierPath } from '@xyflow/svelte';
  import { evaluatedNodeIds, evaluatingNodeId } from '$lib/stores';

  let {
    id,
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
    data = undefined,
    target = '',
  }: {
    id: string;
    sourceX: number;
    sourceY: number;
    targetX: number;
    targetY: number;
    sourcePosition: any;
    targetPosition: any;
    data?: { label?: string } | undefined;
    target?: string;
  } = $props();

  let pathResult = $derived(getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
  }));

  let path = $derived(pathResult[0]);
  let labelX = $derived(pathResult[1]);
  let labelY = $derived(pathResult[2]);

  let isCompleted = $derived($evaluatedNodeIds.has(target));
  let isAnimating = $derived($evaluatingNodeId === target);

  let style = $derived(isCompleted
    ? 'stroke: #3fb950; stroke-width: 2;'
    : isAnimating
      ? 'stroke: #d29922; stroke-width: 2;'
      : 'stroke: #30363d; stroke-width: 1.5; stroke-dasharray: 5 5;');
</script>

<BaseEdge {id} {path} {style} />

{#if isAnimating}
  <circle r="4" fill="#d29922">
    <animateMotion dur="0.8s" repeatCount="indefinite" {path} />
  </circle>
{/if}

{#if isCompleted && data?.label}
  <foreignObject
    x={labelX - 40}
    y={labelY - 10}
    width="80"
    height="20"
  >
    <div class="edge-label">→ {data.label}</div>
  </foreignObject>
{/if}

<style>
  .edge-label {
    background: #1a3a1a;
    color: #3fb950;
    font-size: 9px;
    font-family: monospace;
    padding: 2px 6px;
    border-radius: 4px;
    text-align: center;
    white-space: nowrap;
  }
</style>
