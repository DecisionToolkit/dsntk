import { writable, derived } from 'svelte/store';
import type { ProjectResponse, NodeEvalResult } from './types';

export const projectData = writable<ProjectResponse | null>(null);
export const projectDir = writable<string>('');
export const evalResults = writable<NodeEvalResult[]>([]);

/** Map from node_id to its evaluation result. */
export const evalByNodeId = derived(evalResults, ($results) => {
  const map = new Map<string, NodeEvalResult>();
  for (const r of $results) {
    map.set(r.node_id, r);
  }
  return map;
});

/** Set of node IDs that have been evaluated. */
export const evaluatedNodeIds = derived(evalResults, ($results) =>
  new Set($results.map((r) => r.node_id))
);
