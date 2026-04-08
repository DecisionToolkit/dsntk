import { writable, derived } from 'svelte/store';
import type { EvaluationTrace, ModelInfo, TraceStep } from './types';

export const selectedModel = writable<ModelInfo | null>(null);
export const inputValues = writable<Record<string, unknown>>({});
export const traceData = writable<EvaluationTrace | null>(null);
export const currentStep = writable<number>(0);

export const totalSteps = derived(traceData, ($trace) => $trace ? $trace.steps.length : 0);

export const visibleSteps = derived([traceData, currentStep], ([$trace, $step]) => {
  if (!$trace) return [];
  return $trace.steps.slice(0, $step);
});

export const evaluatedNodeIds = derived(visibleSteps, ($steps) =>
  new Set($steps.map((s: TraceStep) => s.node_id))
);

export const evaluatingNodeId = derived([traceData, currentStep], ([$trace, $step]) => {
  if (!$trace || $step <= 0 || $step > $trace.steps.length) return null;
  return $trace.steps[$step - 1]?.node_id ?? null;
});

export const stepByNodeId = derived(visibleSteps, ($steps) => {
  const map = new Map<string, TraceStep>();
  for (const step of $steps) { map.set(step.node_id, step); }
  return map;
});
