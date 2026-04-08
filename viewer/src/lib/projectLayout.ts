import dagre from 'dagre';
import type { ProjectResponse, ProjectNode } from './types';
import type { Node, Edge } from '@xyflow/svelte';

const NODE_WIDTH = 340;
const NODE_HEIGHT_INPUT = 60;
const NODE_HEIGHT_KS = 50;
const HEADER_HEIGHT = 36;
const COLUMN_ROW_HEIGHT = 24;
const RULE_ROW_HEIGHT = 22;
const FOOTER_HEIGHT = 8;
const MIN_DECISION_HEIGHT = 80;

function nodeHeight(node: ProjectNode): number {
  if (node.type === 'input_data') return NODE_HEIGHT_INPUT;
  if (node.type === 'knowledge_source') return NODE_HEIGHT_KS;
  if (node.type === 'decision' || node.type === 'bkm') {
    const ruleCount = node.rules?.length ?? 0;
    if (ruleCount === 0) return MIN_DECISION_HEIGHT;
    return HEADER_HEIGHT + COLUMN_ROW_HEIGHT + ruleCount * RULE_ROW_HEIGHT + FOOTER_HEIGHT;
  }
  return MIN_DECISION_HEIGHT;
}

function nodeType(node: ProjectNode): string {
  switch (node.type) {
    case 'input_data': return 'projectInput';
    case 'decision': return 'projectDecision';
    case 'bkm': return 'projectBkm';
    case 'knowledge_source': return 'projectKs';
    default: return 'projectDecision';
  }
}

export function computeProjectLayout(project: ProjectResponse): { nodes: Node[]; edges: Edge[] } {
  const g = new dagre.graphlib.Graph();
  g.setDefaultEdgeLabel(() => ({}));
  g.setGraph({ rankdir: 'TB', nodesep: 60, ranksep: 80 });

  for (const node of project.nodes) {
    const h = nodeHeight(node);
    g.setNode(node.id, { width: NODE_WIDTH, height: h });
  }

  for (const edge of project.edges) {
    g.setEdge(edge.source, edge.target);
  }

  dagre.layout(g);

  const nodes: Node[] = project.nodes.map((node) => {
    const pos = g.node(node.id);
    const h = nodeHeight(node);
    return {
      id: node.id,
      type: nodeType(node),
      position: { x: pos.x - NODE_WIDTH / 2, y: pos.y - h / 2 },
      data: node,
    };
  });

  const edgeKindStyle: Record<string, string> = {
    'requires': '#58a6ff',
    'governed-by': '#d29922',
    'supported-by': '#8b949e',
  };

  const edges: Edge[] = project.edges.map((edge, i) => ({
    id: `e-${i}`,
    source: edge.source,
    target: edge.target,
    type: 'default',
    style: `stroke: ${edgeKindStyle[edge.kind] ?? '#30363d'}`,
    data: { label: edge.label, kind: edge.kind },
  }));

  return { nodes, edges };
}
