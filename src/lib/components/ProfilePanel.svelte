<script lang="ts">
  interface Props {
    profileData: any;
  }

  let { profileData }: Props = $props();

  interface ProfileNode {
    nodeType: string;
    relationName?: string;
    alias?: string;
    indexName?: string;
    joinType?: string;
    actualTotalTime?: number;
    actualRows?: number;
    actualLoops?: number;
    sharedHitBlocks?: number;
    sharedReadBlocks?: number;
    plans?: ProfileNode[];
    filter?: string;
    condition?: string;
  }

  interface FlatNode {
    path: string;
    depth: number;
    node: ProfileNode;
  }

  function getPlan(data: any): { plan: ProfileNode | null; planningTime: number; executionTime: number } {
    if (!data || !Array.isArray(data) || data.length === 0) {
      return { plan: null, planningTime: 0, executionTime: 0 };
    }
    const entry = data[0];
    return {
      plan: entry?.["Plan"] ?? null,
      planningTime: entry?.["Planning Time"] ?? 0,
      executionTime: entry?.["Execution Time"] ?? 0,
    };
  }

  let parsed = $derived(getPlan(profileData));
  let plan = $derived(parsed.plan);
  let planningTime = $derived(parsed.planningTime);
  let executionTime = $derived(parsed.executionTime);
  let totalTime = $derived(planningTime + executionTime);

  function flattenNodes(node: ProfileNode, depth: number = 0, path: string = "0", result: FlatNode[] = []): FlatNode[] {
    result.push({ path, depth, node });
    if (node.plans) {
      for (let i = 0; i < node.plans.length; i++) {
        flattenNodes(node.plans[i], depth + 1, `${path}-${i}`, result);
      }
    }
    return result;
  }

  let flatNodes = $derived(plan ? flattenNodes(plan) : []);

  let maxNodeTime = $derived(
    flatNodes.length > 0
      ? Math.max(...flatNodes.map((n) => n.node.actualTotalTime ?? 0))
      : 1
  );

  function getNodeLabel(node: ProfileNode): string {
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

  function getRecommendations(nodes: FlatNode[]): string[] {
    const recs: string[] = [];
    const hasSeqScan = nodes.some((n) => n.node.nodeType === "Seq Scan");
    const hasIndexScan = nodes.some((n) => n.node.nodeType.includes("Index Scan"));
    const hasSort = nodes.some((n) => n.node.nodeType === "Sort");
    const hasHash = nodes.some((n) => n.node.nodeType === "Hash");

    if (hasSeqScan && !hasIndexScan) {
      recs.push("Consider adding indexes to avoid sequential scans");
    }
    if (hasSort) {
      recs.push("Sorting detected — consider adding an ORDER BY index");
    }
    if (hasHash) {
      recs.push("Hash operation detected — ensure adequate work_mem setting");
    }
    if (nodes.some((n) => (n.node.actualTotalTime ?? 0) > totalTime * 0.5)) {
      recs.push("One node dominates execution time — focus optimization there");
    }
    if (recs.length === 0) {
      recs.push("Query plan looks efficient — no major bottlenecks detected");
    }
    return recs;
  }

  let recommendations = $derived(flatNodes.length > 0 ? getRecommendations(flatNodes) : []);

  function formatTime(ms: number): string {
    if (ms < 1) return `${(ms * 1000).toFixed(1)}\u03bcs`;
    if (ms < 1000) return `${ms.toFixed(2)}ms`;
    return `${(ms / 1000).toFixed(2)}s`;
  }

  function formatNumber(n: number): string {
    if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
    if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
    return n.toLocaleString();
  }

  function isBottleneck(node: ProfileNode): boolean {
    const time = node.actualTotalTime ?? 0;
    return time > totalTime * 0.3;
  }
</script>

<div class="flex flex-col h-full bg-[#0c0c0c]">
  <div class="flex items-center justify-between px-3 py-1.5 bg-[#141414] border-b border-[#2a2a2a] shrink-0">
    <span class="text-[11px] text-[#6b6b6b] uppercase tracking-wider">Query Profiler</span>
    {#if totalTime > 0}
      <span class="text-[10px] text-[#a0a0a0] font-mono">ANALYZE \u00b7 {formatTime(totalTime)} total</span>
    {/if}
  </div>

  {#if plan}
    <div class="flex-1 overflow-auto">
      <!-- Summary Cards -->
      <div class="grid grid-cols-4 gap-2 p-3">
        <div class="bg-[#141414] border border-[#2a2a2a] rounded p-2.5">
          <div class="text-[9px] text-[#6b6b6b] uppercase tracking-wider mb-1">Total Time</div>
          <div class="text-[16px] font-mono text-[#e8e8e8]">{formatTime(totalTime)}</div>
        </div>
        <div class="bg-[#141414] border border-[#2a2a2a] rounded p-2.5">
          <div class="text-[9px] text-[#6b6b6b] uppercase tracking-wider mb-1">Planning</div>
          <div class="text-[16px] font-mono text-[#e8e8e8]">{formatTime(planningTime)}</div>
        </div>
        <div class="bg-[#141414] border border-[#2a2a2a] rounded p-2.5">
          <div class="text-[9px] text-[#6b6b6b] uppercase tracking-wider mb-1">Execution</div>
          <div class="text-[16px] font-mono text-[#e8e8e8]">{formatTime(executionTime)}</div>
        </div>
        <div class="bg-[#141414] border border-[#2a2a2a] rounded p-2.5">
          <div class="text-[9px] text-[#6b6b6b] uppercase tracking-wider mb-1">Rows</div>
          <div class="text-[16px] font-mono text-[#e8e8e8]">{formatNumber(plan.actualRows ?? 0)}</div>
        </div>
      </div>

      <!-- Recommendations -->
      <div class="px-3 pb-2">
        <div class="bg-[#141414] border border-[#2a2a2a] rounded p-2.5">
          <div class="text-[9px] text-[#6b6b6b] uppercase tracking-wider mb-1.5">Recommendations</div>
          <div class="flex flex-wrap gap-1.5">
            {#each recommendations as rec}
              <span class="text-[10px] px-2 py-0.5 rounded bg-[#1a3a2a] text-[#7ee787] border border-[#2a4a3a]">{rec}</span>
            {/each}
          </div>
        </div>
      </div>

      <!-- Node Breakdown -->
      <div class="px-3 pb-3">
        <div class="bg-[#141414] border border-[#2a2a2a] rounded overflow-hidden">
          <div class="text-[9px] text-[#6b6b6b] uppercase tracking-wider px-2.5 py-1.5 border-b border-[#2a2a2a]">Node Breakdown</div>
          <div class="divide-y divide-[#2a2a2a]">
            {#each flatNodes as item, i}
              {@const node = item.node}
              {@const time = node.actualTotalTime ?? 0}
              {@const pct = maxNodeTime > 0 ? (time / maxNodeTime) * 100 : 0}
              {@const totalPct = totalTime > 0 ? (time / totalTime) * 100 : 0}
              {@const bottleneck = isBottleneck(node)}
              {@const label = getNodeLabel(node)}
              {@const color = getNodeColor(node.nodeType)}

              <div class="flex items-center gap-2 px-2.5 py-2 hover:bg-[rgba(255,255,255,0.02)]" style="padding-left: {12 + item.depth * 16}px">
                <div class="w-2 h-2 rounded-full shrink-0" style="background-color: {color}"></div>
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-1.5">
                    <span class="text-[11px] text-[#e8e8e8] truncate">{label}</span>
                    {#if bottleneck}
                      <span class="text-[9px] px-1 py-0.5 rounded bg-[#3a1a1a] text-[#ff6b6b] border border-[#4a2a2a] shrink-0">Bottleneck</span>
                    {/if}
                  </div>
                  <div class="flex items-center gap-3 mt-0.5">
                    <div class="flex-1 h-1.5 bg-[#1a1a1a] rounded-full overflow-hidden max-w-[200px]">
                      <div class="h-full rounded-full" style="width: {pct}%; background-color: {color}; opacity: 0.8"></div>
                    </div>
                    <span class="text-[10px] font-mono text-[#a0a0a0] whitespace-nowrap">{formatTime(time)}</span>
                    <span class="text-[10px] font-mono text-[#6b6b6b] whitespace-nowrap">{totalPct.toFixed(1)}%</span>
                  </div>
                </div>
                <div class="text-[10px] font-mono text-[#6b6b6b] text-right whitespace-nowrap shrink-0">
                  <div>{formatNumber(node.actualRows ?? 0)} rows</div>
                  {#if node.actualLoops && node.actualLoops > 1}
                    <div>{node.actualLoops}\u00d7 loops</div>
                  {/if}
                </div>
                {#if (node.sharedHitBlocks ?? 0) > 0 || (node.sharedReadBlocks ?? 0) > 0}
                  <div class="text-[10px] font-mono text-[#6b6b6b] text-right whitespace-nowrap shrink-0 w-20">
                    <div class="text-[#4ecdc4]">{formatNumber(node.sharedHitBlocks ?? 0)} hit</div>
                    <div class="text-[#ff6b6b]">{formatNumber(node.sharedReadBlocks ?? 0)} read</div>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      </div>
    </div>
  {:else}
    <div class="flex-1 flex items-center justify-center p-4">
      <p class="text-[12px] text-[#6b6b6b]">No profile data available</p>
    </div>
  {/if}
</div>
