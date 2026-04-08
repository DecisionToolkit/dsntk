<script lang="ts">
  import { projectData, projectDir, evalResults } from '$lib/projectStores';
  import { fetchProject, evaluateProject } from '$lib/api';
  import type { FieldDescriptor, ProjectNode, ProjectScenario } from '$lib/types';

  let dirPath = $state('');
  let loading = $state(false);
  let evaluating = $state(false);
  let error = $state('');
  let inputValues: Record<string, Record<string, unknown>> = $state({});
  let activeScenario = $state('');

  let inputNodes = $derived(
    ($projectData?.nodes ?? []).filter((n): n is Extract<ProjectNode, { type: 'input_data' }> => n.type === 'input_data')
  );

  let scenarios = $derived($projectData?.scenarios ?? []);

  async function loadProject() {
    if (!dirPath.trim()) return;
    loading = true;
    error = '';
    try {
      const data = await fetchProject(dirPath.trim());
      projectData.set(data);
      projectDir.set(dirPath.trim());
      // Initialize input values from fields
      inputValues = {};
      for (const node of data.nodes) {
        if (node.type === 'input_data' && node.fields.length > 0) {
          const vals: Record<string, unknown> = {};
          for (const field of node.fields) {
            vals[field.path] = fieldDefault(field);
          }
          inputValues[node.id] = vals;
        }
      }
    } catch (e) {
      error = `${e}`;
    } finally {
      loading = false;
    }
  }

  function fieldDefault(field: FieldDescriptor): unknown {
    if (field.allowed_values && field.allowed_values.length > 0) return field.allowed_values[0];
    switch (field.feel_type) {
      case 'number': return 0;
      case 'boolean': return false;
      case 'string': return '';
      default: return '';
    }
  }

  function applyScenario(scenario: ProjectScenario) {
    activeScenario = scenario.name;
    inputValues = {};
    for (const node of inputNodes) {
      const scenarioNodeData = scenario.inputs[node.id];
      if (!scenarioNodeData) continue;
      const vals: Record<string, unknown> = {};
      for (const field of node.fields) {
        const parts = field.path.split('.');
        let value: unknown = scenarioNodeData;
        for (const part of parts) {
          if (value && typeof value === 'object' && part in (value as Record<string, unknown>)) {
            value = (value as Record<string, unknown>)[part];
          } else {
            value = fieldDefault(field);
            break;
          }
        }
        vals[field.path] = value;
      }
      inputValues[node.id] = vals;
    }
  }

  function onFieldChange(nodeId: string, path: string, value: unknown) {
    activeScenario = '';
    if (!inputValues[nodeId]) inputValues[nodeId] = {};
    inputValues[nodeId][path] = value;
  }

  function buildContext(): Record<string, unknown> {
    const ctx: Record<string, unknown> = {};
    for (const node of inputNodes) {
      const vals = inputValues[node.id];
      if (!vals) continue;
      const obj: Record<string, unknown> = {};
      for (const field of node.fields) {
        setNestedValue(obj, field.path, vals[field.path]);
      }
      ctx[node.name] = obj;
    }
    return ctx;
  }

  function setNestedValue(obj: Record<string, unknown>, path: string, value: unknown) {
    const parts = path.split('.');
    let current = obj;
    for (let i = 0; i < parts.length - 1; i++) {
      if (!(parts[i] in current)) current[parts[i]] = {};
      current = current[parts[i]] as Record<string, unknown>;
    }
    current[parts[parts.length - 1]] = value;
  }

  async function evaluate() {
    const dir = dirPath.trim();
    if (!dir || inputNodes.length === 0) return;
    evaluating = true;
    error = '';
    try {
      // Build inputs keyed by node ID with nested structure.
      const inputs: Record<string, Record<string, unknown>> = {};
      for (const node of inputNodes) {
        const vals = inputValues[node.id];
        if (!vals) continue;
        const obj: Record<string, unknown> = {};
        for (const field of node.fields) {
          setNestedValue(obj, field.path, vals[field.path]);
        }
        inputs[node.id] = obj;
      }
      const res = await evaluateProject(dir, inputs);
      evalResults.set(res.results);
    } catch (e) {
      error = `Evaluation failed: ${e}`;
    } finally {
      evaluating = false;
    }
  }
</script>

<div class="panel">
  <div class="section">
    <label class="label">Project Directory</label>
    <input
      class="dir-input"
      type="text"
      bind:value={dirPath}
      placeholder="/path/to/project"
      onkeydown={(e) => e.key === 'Enter' && loadProject()}
    />
    <button class="load-btn" onclick={loadProject} disabled={loading}>
      {loading ? 'Loading...' : 'Load Project'}
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if scenarios.length > 0}
    <div class="section scenarios">
      <span class="label">Scenarios</span>
      <div class="scenario-list">
        {#each scenarios as scenario}
          <button
            class="scenario-btn"
            class:active={activeScenario === scenario.name}
            onclick={() => applyScenario(scenario)}
            title={scenario.description ?? ''}
          >
            {scenario.name}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if inputNodes.length > 0}
    <div class="inputs-scroll">
      {#each inputNodes as node}
        {#if node.fields.length > 0}
          <div class="input-group">
            <div class="group-header">{node.name}</div>
            {#each node.fields as field}
              <div class="field-row">
                <label class="field-label">
                  {field.name}
                  {#if field.optional}<span class="optional">opt</span>{/if}
                </label>
                {#if field.allowed_values && field.allowed_values.length > 0}
                  <select
                    class="field-control"
                    value={inputValues[node.id]?.[field.path] ?? ''}
                    onchange={(e) => onFieldChange(node.id, field.path, (e.target as HTMLSelectElement).value)}
                  >
                    {#each field.allowed_values as av}
                      <option value={av}>{av}</option>
                    {/each}
                  </select>
                {:else if field.feel_type === 'number'}
                  <input
                    class="field-control"
                    type="number"
                    step="any"
                    value={inputValues[node.id]?.[field.path] ?? 0}
                    oninput={(e) => onFieldChange(node.id, field.path, parseFloat((e.target as HTMLInputElement).value) || 0)}
                  />
                {:else if field.feel_type === 'boolean'}
                  <label class="toggle">
                    <input
                      type="checkbox"
                      checked={!!inputValues[node.id]?.[field.path]}
                      onchange={(e) => onFieldChange(node.id, field.path, (e.target as HTMLInputElement).checked)}
                    />
                    <span class="toggle-label">{inputValues[node.id]?.[field.path] ? 'true' : 'false'}</span>
                  </label>
                {:else}
                  <input
                    class="field-control"
                    type="text"
                    value={inputValues[node.id]?.[field.path] ?? ''}
                    oninput={(e) => onFieldChange(node.id, field.path, (e.target as HTMLInputElement).value)}
                  />
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      {/each}
    </div>

    <div class="section footer">
      <button class="evaluate-btn" onclick={evaluate} disabled={evaluating}>
        {evaluating ? 'Evaluating...' : 'Evaluate'}
      </button>
      <details class="context-preview">
        <summary>Input Context</summary>
        <pre class="context-json">{JSON.stringify(buildContext(), null, 2)}</pre>
      </details>
    </div>
  {/if}
</div>

<style>
  .panel {
    width: 300px;
    background: #161b22;
    border-right: 1px solid #30363d;
    display: flex;
    flex-direction: column;
    height: 100%;
    flex-shrink: 0;
  }

  .section {
    padding: 12px;
    border-bottom: 1px solid #30363d;
  }

  .label {
    color: #8b949e;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 1px;
    margin-bottom: 8px;
    display: block;
  }

  .dir-input {
    width: 100%;
    background: #0d1117;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 8px 10px;
    color: #c9d1d9;
    font-size: 12px;
    font-family: monospace;
    margin-bottom: 8px;
    box-sizing: border-box;
  }

  .load-btn {
    width: 100%;
    background: #238636;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 8px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
  }
  .load-btn:hover { background: #2ea043; }
  .load-btn:disabled { opacity: 0.6; cursor: not-allowed; }

  .error {
    color: #f85149;
    font-size: 11px;
    padding: 8px 12px;
  }

  .scenarios {
    padding: 8px 12px;
  }

  .scenario-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .scenario-btn {
    background: #0d1117;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 6px 10px;
    color: #c9d1d9;
    font-size: 11px;
    font-family: monospace;
    cursor: pointer;
    text-align: left;
    transition: border-color 0.15s;
  }

  .scenario-btn:hover {
    border-color: #58a6ff;
    color: #58a6ff;
  }

  .scenario-btn.active {
    border-color: #58a6ff;
    background: #0c2d6b;
    color: #58a6ff;
  }

  .inputs-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 0;
  }

  .input-group {
    border-bottom: 1px solid #30363d;
  }

  .group-header {
    background: #0d1117;
    color: #58a6ff;
    font-size: 11px;
    font-weight: bold;
    padding: 8px 12px;
    font-family: monospace;
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .field-row {
    padding: 4px 12px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .field-label {
    color: #8b949e;
    font-size: 10px;
    font-family: monospace;
  }

  .optional {
    color: #484f58;
    font-size: 8px;
    margin-left: 4px;
  }

  .field-control {
    width: 100%;
    background: #0d1117;
    border: 1px solid #30363d;
    border-radius: 4px;
    padding: 5px 8px;
    color: #c9d1d9;
    font-size: 12px;
    font-family: monospace;
    box-sizing: border-box;
  }

  select.field-control {
    cursor: pointer;
  }

  .toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .toggle input {
    accent-color: #58a6ff;
  }

  .toggle-label {
    color: #8b949e;
    font-size: 11px;
    font-family: monospace;
  }

  .footer {
    border-top: 1px solid #30363d;
    padding: 8px 12px;
    margin-top: auto;
  }

  .evaluate-btn {
    width: 100%;
    background: #1f6feb;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 10px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    margin-bottom: 8px;
  }
  .evaluate-btn:hover { background: #388bfd; }
  .evaluate-btn:disabled { opacity: 0.6; cursor: not-allowed; }

  .context-preview summary {
    color: #8b949e;
    font-size: 10px;
    cursor: pointer;
    font-family: monospace;
  }

  .context-json {
    background: #0d1117;
    color: #58a6ff;
    font-size: 10px;
    padding: 8px;
    border-radius: 4px;
    max-height: 200px;
    overflow-y: auto;
    margin-top: 4px;
  }
</style>
