<script lang="ts">
  import { onMount } from 'svelte';
  import { selectedModel, inputValues, traceData, currentStep } from '$lib/stores';
  import { fetchModels, evaluateTrace } from '$lib/api';
  import type { ModelInfo } from '$lib/types';

  let models: ModelInfo[] = $state([]);
  let loading = $state(false);
  let error = $state('');
  let rawJson = $state('{}');

  onMount(async () => {
    try {
      const res = await fetchModels();
      models = res.models;
      if (models.length > 0) {
        selectedModel.set(models[0]);
      }
    } catch (e) {
      error = `Failed to load models: ${e}`;
    }
  });

  function onModelChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const model = models.find(
      (m) => (m as any).path === target.value || m.name === target.value
    );
    if (model) selectedModel.set(model);
  }

  async function evaluate() {
    if (!$selectedModel) return;
    loading = true;
    error = '';
    try {
      let inputs: Record<string, unknown>;
      try {
        inputs = JSON.parse(rawJson);
      } catch {
        error = 'Invalid JSON';
        loading = false;
        return;
      }

      const path =
        ($selectedModel as any).path ||
        `${$selectedModel.namespace}/${$selectedModel.name}/${$selectedModel.invocables[0]}`;
      const res = await evaluateTrace(path, inputs);
      traceData.set(res.trace ?? null);
      currentStep.set(0);
    } catch (e) {
      error = `Evaluation failed: ${e}`;
    } finally {
      loading = false;
    }
  }
</script>

<div class="panel">
  <div class="section">
    <label class="label">Model</label>
    {#if models.length > 0}
      <select class="select" onchange={onModelChange}>
        {#each models as model}
          <option value={(model as any).path || model.name}>
            {model.invocables[0] || model.name}
          </option>
        {/each}
      </select>
    {:else if !error}
      <p class="hint">Loading models...</p>
    {/if}
  </div>

  <div class="section grow">
    <label class="label">Input Data (JSON)</label>
    <textarea
      class="json-input"
      bind:value={rawJson}
      placeholder={'{{"key": "value"}'}
    ></textarea>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="section footer">
    <button class="evaluate-btn" onclick={evaluate} disabled={loading}>
      {loading ? 'Evaluating...' : 'Evaluate'}
    </button>
  </div>
</div>

<style>
  .panel {
    width: 280px;
    background: #161b22;
    border-right: 1px solid #30363d;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow-y: auto;
    flex-shrink: 0;
  }

  .section {
    padding: 12px;
    border-bottom: 1px solid #30363d;
  }

  .section.grow {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .footer {
    margin-top: auto;
    border-top: 1px solid #30363d;
    border-bottom: none;
  }

  .label {
    color: #8b949e;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 1px;
    margin-bottom: 8px;
    display: block;
  }

  .select {
    width: 100%;
    background: #0d1117;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 8px 10px;
    color: #c9d1d9;
    font-size: 13px;
    font-family: monospace;
  }

  .json-input {
    width: 100%;
    flex: 1;
    min-height: 120px;
    background: #0d1117;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 8px 10px;
    color: #58a6ff;
    font-size: 12px;
    font-family: monospace;
    resize: vertical;
    box-sizing: border-box;
  }

  .hint {
    color: #484f58;
    font-size: 12px;
  }

  .error {
    color: #f85149;
    font-size: 11px;
    padding: 8px 12px;
  }

  .evaluate-btn {
    width: 100%;
    background: #238636;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 10px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
  }

  .evaluate-btn:hover {
    background: #2ea043;
  }

  .evaluate-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
