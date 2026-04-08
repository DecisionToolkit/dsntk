import dagre from 'dagre';
import type { TraceGraph } from './types';
import type { Node, Edge } from '@xyflow/svelte';

const NODE_WIDTH = 320;
const NODE_HEIGHT_INPUT = 60;
const HEADER_HEIGHT = 36;
const COLUMN_ROW_HEIGHT = 24;
const RULE_ROW_HEIGHT = 22;
const FOOTER_HEIGHT = 28;
const MIN_DECISION_HEIGHT = 80;

function decisionNodeHeight(node: TraceGraph['nodes'][number]): number {
  if (node.type !== 'decision_table') return MIN_DECISION_HEIGHT;
  const ruleCount = node.rules?.length ?? 0;
  if (ruleCount === 0) return MIN_DECISION_HEIGHT;
  return HEADER_HEIGHT + COLUMN_ROW_HEIGHT + ruleCount * RULE_ROW_HEIGHT + FOOTER_HEIGHT;
}

export function computeLayout(graph: TraceGraph): { nodes: Node[]; edges: Edge[] } {
  const g = new dagre.graphlib.Graph();
  g.setDefaultEdgeLabel(() => ({}));
  g.setGraph({ rankdir: 'TB', nodesep: 60, ranksep: 80 });

  for (const node of graph.nodes) {
    const height = node.type === 'input_data' ? NODE_HEIGHT_INPUT : decisionNodeHeight(node);
    g.setNode(node.id, { width: NODE_WIDTH, height });
  }

  for (const edge of graph.edges) {
    g.setEdge(edge.source, edge.target);
  }

  dagre.layout(g);

  const nodes: Node[] = graph.nodes.map((node) => {
    const pos = g.node(node.id);
    const height = node.type === 'input_data' ? NODE_HEIGHT_INPUT : decisionNodeHeight(node);
    return {
      id: node.id,
      type: node.type === 'input_data' ? 'inputData' : 'decisionTable',
      position: { x: pos.x - NODE_WIDTH / 2, y: pos.y - height / 2 },
      data: node,
    };
  });

  const edges: Edge[] = graph.edges.map((edge, i) => ({
    id: `e-${i}`,
    source: edge.source,
    target: edge.target,
    type: 'animated',
    data: { label: edge.label },
  }));

  return { nodes, edges };
}
