<script lang="ts">
  import type { QueryResult } from "../types";

  interface Props {
    result: QueryResult;
  }

  let { result }: Props = $props();

  let chartType = $state<"bar" | "line">("bar");

  // Detect numeric columns (skip first column as label)
  let numericColumns = $derived(() => {
    const nums: number[] = [];
    for (let i = 1; i < result.columns.length; i++) {
      const allNumeric = result.rows.every((row) => {
        const v = row[i];
        if (v === null) return true;
        return !isNaN(parseFloat(v)) && isFinite(parseFloat(v));
      });
      if (allNumeric) nums.push(i);
    }
    return nums;
  });

  let selectedCol = $state(1);

  $effect(() => {
    const nums = numericColumns();
    if (nums.length > 0 && !nums.includes(selectedCol)) {
      selectedCol = nums[0];
    }
  });

  let chartData = $derived(() => {
    const data = result.rows
      .map((row) => ({
        label: String(row[0] ?? ""),
        value: parseFloat(row[selectedCol] ?? "0") || 0,
      }))
      .filter((d) => d.label !== "")
      .slice(0, 50); // max 50 items

    const maxValue = Math.max(...data.map((d) => d.value), 1);
    return { data, maxValue };
  });

  // SVG dimensions
  const WIDTH = 800;
  const HEIGHT = 320;
  const PADDING = { top: 20, right: 20, bottom: 60, left: 60 };
  const CHART_W = WIDTH - PADDING.left - PADDING.right;
  const CHART_H = HEIGHT - PADDING.top - PADDING.bottom;

  let { data, maxValue } = $derived(chartData());

  function barX(i: number) {
    return PADDING.left + (i * CHART_W) / data.length;
  }

  function barWidth() {
    return (CHART_W / data.length) * 0.7;
  }

  function barHeight(v: number) {
    return (v / maxValue) * CHART_H;
  }

  function barY(v: number) {
    return PADDING.top + CHART_H - barHeight(v);
  }

  function lineX(i: number) {
    return PADDING.left + (i * CHART_W) / Math.max(data.length - 1, 1);
  }

  function lineY(v: number) {
    return PADDING.top + CHART_H - (v / maxValue) * CHART_H;
  }

  let linePath = $derived(() => {
    if (data.length === 0) return "";
    return data
      .map((d, i) => `${i === 0 ? "M" : "L"} ${lineX(i)} ${lineY(d.value)}`)
      .join(" ");
  });

  let areaPath = $derived(() => {
    if (data.length === 0) return "";
    const top = data
      .map((d, i) => `${i === 0 ? "M" : "L"} ${lineX(i)} ${lineY(d.value)}`)
      .join(" ");
    return `${top} L ${lineX(data.length - 1)} ${PADDING.top + CHART_H} L ${lineX(0)} ${PADDING.top + CHART_H} Z`;
  });

  // Y-axis ticks
  let yTicks = $derived(() => {
    const ticks = 5;
    const arr: number[] = [];
    for (let i = 0; i <= ticks; i++) {
      arr.push((maxValue / ticks) * i);
    }
    return arr;
  });
</script>

<div class="flex flex-col h-full bg-[#0c0c0c]">
  <!-- Toolbar -->
  <div class="flex items-center justify-between px-3 py-2 border-b border-[#2a2a2a]">
    <div class="flex items-center gap-2">
      <button
        onclick={() => (chartType = "bar")}
        class="text-[11px] font-medium px-2.5 py-1 rounded {chartType === 'bar' ? 'bg-[#00d4ff] text-black' : 'bg-[#1a1a1a] text-[#a0a0a0] hover:text-[#e8e8e8]'}">Bar</button>
      <button
        onclick={() => (chartType = "line")}
        class="text-[11px] font-medium px-2.5 py-1 rounded {chartType === 'line' ? 'bg-[#00d4ff] text-black' : 'bg-[#1a1a1a] text-[#a0a0a0] hover:text-[#e8e8e8]'}">Line</button>
    </div>
    {#if numericColumns().length > 1}
      <select
        bind:value={selectedCol}
        class="bg-[#1a1a1a] border border-[#2a2a2a] text-[#a0a0a0] text-[11px] rounded px-2 py-1 outline-none focus:border-[#00d4ff]">
        {#each numericColumns() as colIdx}
          <option value={colIdx}>{result.columns[colIdx].name}</option>
        {/each}
      </select>
    {:else}
      <span class="text-[11px] text-[#6b6b6b]">{result.columns[selectedCol]?.name}</span>
    {/if}
  </div>

  <!-- Chart -->
  <div class="flex-1 flex items-center justify-center overflow-auto p-4">
    {#if data.length === 0}
      <p class="text-[12px] text-[#6b6b6b]">No chartable data</p>
    {:else}
      <svg viewBox="0 0 {WIDTH} {HEIGHT}" class="w-full max-w-4xl">
        <!-- Grid lines -->
        {#each yTicks() as tick}
          {@const y = PADDING.top + CHART_H - (tick / maxValue) * CHART_H}
          <line x1={PADDING.left} y1={y} x2={WIDTH - PADDING.right} y2={y} stroke="#1e1e1e" stroke-width="1" />
          <text x={PADDING.left - 8} y={y + 3} text-anchor="end" fill="#6b6b6b" font-size="10" font-family="monospace">{Math.round(tick).toLocaleString()}</text>
        {/each}

        {#if chartType === "bar"}
          <!-- Bars -->
          {#each data as d, i}
            {@const x = barX(i) + (CHART_W / data.length - barWidth()) / 2}
            {@const y = barY(d.value)}
            {@const h = barHeight(d.value)}
            <rect {x} {y} width={barWidth()} {h} fill="#00d4ff" opacity="0.85" rx="2">
              <title>{d.label}: {d.value.toLocaleString()}</title>
            </rect>
          {/each}
        {:else}
          <!-- Area under line -->
          <path d={areaPath()} fill="rgba(0,212,255,0.08)" />
          <!-- Line -->
          <path d={linePath()} fill="none" stroke="#00d4ff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          <!-- Dots -->
          {#each data as d, i}
            <circle cx={lineX(i)} cy={lineY(d.value)} r="3" fill="#00d4ff" stroke="#0c0c0c" stroke-width="1.5">
              <title>{d.label}: {d.value.toLocaleString()}</title>
            </circle>
          {/each}
        {/if}

        <!-- X axis labels -->
        {#each data as d, i}
          {#if data.length <= 15 || i % Math.ceil(data.length / 15) === 0}
            <text
              x={chartType === "bar" ? barX(i) + CHART_W / data.length / 2 : lineX(i)}
              y={PADDING.top + CHART_H + 16}
              text-anchor="middle"
              fill="#6b6b6b"
              font-size="9"
              font-family="monospace"
              transform={chartType === "bar" ? "rotate(-30, {barX(i) + CHART_W / data.length / 2}, {PADDING.top + CHART_H + 16})" : ""}>
              {d.label.length > 12 ? d.label.slice(0, 12) + "..." : d.label}
            </text>
          {/if}
        {/each}

        <!-- Axes -->
        <line x1={PADDING.left} y1={PADDING.top} x2={PADDING.left} y2={PADDING.top + CHART_H} stroke="#2a2a2a" stroke-width="1" />
        <line x1={PADDING.left} y1={PADDING.top + CHART_H} x2={WIDTH - PADDING.right} y2={PADDING.top + CHART_H} stroke="#2a2a2a" stroke-width="1" />
      </svg>
    {/if}
  </div>
</div>
