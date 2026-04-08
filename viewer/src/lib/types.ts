export type TraceNode =
  | { type: 'input_data'; id: string; name: string; value?: unknown }
  | { type: 'decision_table'; id: string; name: string; hit_policy: string; input_columns: string[]; output_columns: string[]; rules: TraceRule[] };

export interface TraceRule {
  index: number;
  input_entries: string[];
  output_entries: string[];
}

export interface TraceEdge {
  source: string;
  target: string;
  label: string;
}

export interface TraceGraph {
  nodes: TraceNode[];
  edges: TraceEdge[];
}

export interface CellEvaluation {
  rule_index: number;
  column_index: number;
  expression: string;
  input_value: unknown;
  result: boolean;
}

export interface TraceStep {
  node_id: string;
  input_values: Record<string, unknown>;
  matched_rules: number[];
  output_value: unknown;
  cell_evaluations: CellEvaluation[];
}

export interface EvaluationTrace {
  graph: TraceGraph;
  evaluation_order: string[];
  steps: TraceStep[];
}

export interface TraceResponse {
  data: unknown;
  trace?: EvaluationTrace;
}

export interface InputInfo {
  name: string;
  feel_type: string;
  allowed_values?: string[];
  optional: boolean;
}

export interface ModelInfo {
  path: string;
  workspace: string;
  model_namespace: string;
  model_name: string;
  invocable_name: string;
  inputs: InputInfo[];
}

export type NodeEvalState = 'unevaluated' | 'evaluating' | 'evaluated';
export type EdgeEvalState = 'inactive' | 'animating' | 'completed';

// Project types (markdown-based DMN)

export interface FieldDescriptor {
  path: string;
  name: string;
  feel_type: string;
  allowed_values?: string[];
  optional: boolean;
}

export interface ProjectRule {
  index: number;
  input_entries: string[];
  output_entries: string[];
  annotations?: string[];
}

export interface ProjectParam {
  name: string;
  param_type: string;
}

export type ProjectNode =
  | { type: 'input_data'; id: string; name: string; fields: FieldDescriptor[] }
  | { type: 'decision'; id: string; name: string; hit_policy: string; input_columns: string[]; output_columns: string[]; rules: ProjectRule[]; annotation_columns: string[] }
  | { type: 'bkm'; id: string; name: string; hit_policy: string; input_columns: string[]; output_columns: string[]; rules: ProjectRule[]; parameters: ProjectParam[]; feel_expression?: string }
  | { type: 'knowledge_source'; id: string; name: string; owner?: string };

export interface ProjectEdge {
  source: string;
  target: string;
  kind: string;
  label: string;
}

export interface ProjectScenario {
  name: string;
  description?: string;
  inputs: Record<string, Record<string, unknown>>;
}

export interface NodeEvalResult {
  node_id: string;
  node_name: string;
  matched_rules: number[];
  output_value: unknown;
  cell_evaluations: { rule_index: number; column_index: number; result: boolean }[];
}

export interface EvaluateProjectResponse {
  results: NodeEvalResult[];
  evaluation_order: string[];
}

export interface ProjectResponse {
  nodes: ProjectNode[];
  edges: ProjectEdge[];
  evaluation_order: string[];
  scenarios?: ProjectScenario[];
}
