<script lang="ts">
  import type { QueryResult } from "../types";

  interface Props {
    explainData: any;
  }

  let { explainData }: Props = $props();

  interface PlanNode {
    nodeType: string;
    relationName?: string;
    schema?: string;
    alias?: string;
    startupCost: number;
    totalCost: number;
    planRows: number;
    planWidth: number;
    actualTime?: number;
    actualRows?: number;
    loops?: number;
    indexName?: string;
    joinType?: string;
    filter?: string;
    condition?: string;
    plans?: PlanNode[];
    parallelAware?: boolean;
    partialMode?: string;
  }

  function getPlans(data: any): PlanNode | null {
    if (!data || !Array.isArray(data) || data.length === 0) return null;
    return data[0]?.["Plan"] ?? null;
  }

  let plan = $derived(getPlans(explainData));

  function formatCost(cost: number): string {
    return cost.toFixed(2);
  }

  function getNodeColor(nodeType: string): string {
    const colors: Record<string, string> = {
      "Seq Scan": "#ff6b6b",
      "Index Scan": "#4ecdc4",
      "Index Only Scan": "#45b7d1",
      "Bitmap Heap Scan": "#96ceb4",
      "Bitmap Index Scan": "#88d8b0",
      "Nested Loop": "#ffeaa7",
      "Hash Join": "#dfe6e9",
      "Merge Join": "#fdcb6e",
      "Hash": "#e17055",
      "Sort": "#74b9ff",
      "Aggregate": "#a29bfe",
      "GroupAggregate": "#a29bfe",
      "HashAggregate": "#a29bfe",
      "Limit": "#fab1a0",
      "Materialize": "#55efc4",
      "Subquery Scan": "#81ecec",
    };
    return colors[nodeType] ?? "#b2bec3";
  }

  function getNodeLabel(node: PlanNode): string {
    let label = node.nodeType;
    if (node.relationName) {
      label += ` \u00b7 ${node.relationName}`;
      if (node.alias && node.alias !== node.relationName) {
        label += ` (${node.alias})`;
      }
    }
    if (node.indexName) {
      label += ` \u00b7 ${node.indexName}`;
    }
    if (node.joinType) {
      label = `${node.joinType} ${label}`;
    }
    return label;
  }

  let expandedNodes = $state<Set<string>>(new Set());

  function toggleNode(key: string) {
    const next = new Set(expandedNodes);
    if (next.has(key)) {
      next.delete(key);
    } else {
      next.add(key);
    }
    expandedNodes = next;
  }

  function renderTree(node: PlanNode, depth: number = 0, path: string = "0"): string {
    const isExpanded = expandedNodes.has(path) || depth < 2;
    const hasChildren = node.plans && node.plans.length > 0;
    const color = getNodeColor(node.nodeType);
    const label = getNodeLabel(node);

    let html = `<div class="select-none">`;

    // Node row
    html += `<div class="flex items-center gap-2 py-1.5 hover:bg-[rgba(255,255,255,0.03)] rounded cursor-pointer" style="padding-left: ${depth * 24}px" onclick="window.dispatchEvent(new CustomEvent('toggleExplainNode', {detail: '${path}'}))">`;

    if (hasChildren) {
      html += `<span class="text-[10px] text-[#6b6b6b] w-3">${isExpanded ? "\u25bc" : "\u25b6"}</span>`;
    } else {
      html += `<span class="w-3"></span>`;
    }

    html += `<div class="w-2.5 h-2.5 rounded-full shrink-0" style="background-color: ${color}"></div>`;

    html += `<div class="flex-1 min-w-0">`;
    html += `<div class="text-[11px] text-[#e8e8e8] truncate">${escapeHtml(label)}</div>`;
    html += `<div class="text-[9px] text-[#6b6b6b] flex gap-2">`;
    html += `<span>cost: ${formatCost(node.totalCost)}</span>`;
    html += `<span>rows: ${node.planRows.toLocaleString()}</span>`;
    html += `<span>width: ${node.planWidth}</span>`;
    if (node.actualTime !== undefined) {
      html += `<span>time: ${node.actualTime.toFixed(2)}ms</span>`;
    }
    if (node.actualRows !== undefined) {
      html += `<span>actual: ${node.actualRows.toLocaleString()}</span>`;
    }
    if (node.loops !== undefined && node.loops > 1) {
      html += `<span>loops: ${node.loops}</span>`;
    }
    html += `</div>`;
    html += `</div>`;

    html += `</div>`;

    // Children
    if (hasChildren && isExpanded) {
      for (let i = 0; i < node.plans!.length; i++) {
        html += renderTree(node.plans![i], depth + 1, `${path}-${i}`);
      }
    }

    html += `</div>`;
    return html;
  }

  function escapeHtml(text: string): string {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML;
  }

  let treeHtml = $derived(plan ? renderTree(plan) : "");

  $effect(() => {
    const handler = (e: CustomEvent) => toggleNode(e.detail);
    window.addEventListener("toggleExplainNode", handler as any);
    return () => window.removeEventListener("toggleExplainNode", handler as any);
  });
</script>

<div class="flex flex-col h-full bg-[#0c0c0c]">
  <div class="flex items-center justify-between px-3 py-1.5 bg-[#141414] border-b border-[#2a2a2a] shrink-0">
    <span class="text-[11px] text-[#6b6b6b] uppercase tracking-wider">EXPLAIN Plan</span>
  </div>
  <div class="flex-1 overflow-auto p-3">
    {#if plan}
      {@html treeHtml}
    {:else}
      <p class="text-[12px] text-[#6b6b6b]">No EXPLAIN data available</p>
    {/if}
  </div>
</div>
