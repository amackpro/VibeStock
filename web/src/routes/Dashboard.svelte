<script>
  import { onMount, onDestroy } from 'svelte';
  import { gsap } from 'gsap';
  import { Chart, registerables } from 'chart.js';
  import { api, openWebSocket } from '../lib/api.js';
  import { toastStore } from '../stores/toast.js';

  Chart.register(...registerables);

  let stats = {
    totalProducts: 0,
    totalValue: 0,
    lowStockItems: 0,
    todayMovements: 0,
    totalSuppliers: 0,
    activeUsers: 0
  };

  let loading = true;
  let ws;
  let chartCanvas;
  let chart;
  let kpiRefs = [];

  onMount(async () => {
    await loadStats();
    initChart();
    connectWebSocket();
    animateEntrance();
  });

  onDestroy(() => {
    if (ws) ws.close();
    if (chart) chart.destroy();
  });

  async function loadStats() {
    try {
      const data = await api.dashboard.stats();
      stats = data;
      animateNumbers();
    } catch (e) {
      toastStore.show('Failed to load dashboard', 'error');
    } finally {
      loading = false;
    }
  }

  function connectWebSocket() {
    ws = openWebSocket((event) => {
      if (event.type === 'stock_update') {
        loadStats();
        toastStore.show(`Stock updated: ${event.movement_type}`, 'info', 2000);
      }
    });
  }

  function animateEntrance() {
    const tl = gsap.timeline();

    tl.fromTo('.kpi-card', 
      { y: 40, opacity: 0, scale: 0.95 },
      { y: 0, opacity: 1, scale: 1, duration: 0.6, stagger: 0.1, ease: 'back.out(1.5)' }
    );

    tl.fromTo('.chart-card', 
      { y: 30, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, stagger: 0.15, ease: 'power3.out' },
      '-=0.4'
    );
  }

  function animateNumbers() {
    const duration = 1.2;
    const ease = 'power3.out';

    gsap.to({ val: 0 }, {
      val: stats.total_products || 0,
      duration,
      ease,
      onUpdate: function() {
        stats.totalProducts = Math.round(this.targets()[0].val);
      }
    });

    gsap.to({ val: 0 }, {
      val: stats.total_stock_value || 0,
      duration,
      ease,
      onUpdate: function() {
        stats.totalValue = Math.round(this.targets()[0].val);
      }
    });

    gsap.to({ val: 0 }, {
      val: stats.total_movements_today || 0,
      duration,
      ease,
      onUpdate: function() {
        stats.todayMovements = Math.round(this.targets()[0].val);
      }
    });
  }

  function initChart() {
    if (!chartCanvas) return;
    
    const ctx = chartCanvas.getContext('2d');
    chart = new Chart(ctx, {
      type: 'line',
      data: {
        labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
        datasets: [{
          label: 'Stock Movements',
          data: [12, 19, 15, 25, 22, 30, 28],
          borderColor: '#6366f1',
          backgroundColor: 'rgba(99, 102, 241, 0.1)',
          fill: true,
          tension: 0.4,
          pointBackgroundColor: '#6366f1',
          pointBorderColor: '#fff',
          pointBorderWidth: 2,
          pointRadius: 4,
          pointHoverRadius: 6
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { display: false }
        },
        scales: {
          x: {
            grid: { color: 'rgba(255,255,255,0.05)' },
            ticks: { color: '#a0a0b0' }
          },
          y: {
            grid: { color: 'rgba(255,255,255,0.05)' },
            ticks: { color: '#a0a0b0' }
          }
        }
      }
    });
  }

  function handleKpiHover(index) {
    gsap.to(kpiRefs[index], { scale: 1.02, duration: 0.3, ease: 'power2.out' });
  }

  function handleKpiLeave(index) {
    gsap.to(kpiRefs[index], { scale: 1, duration: 0.3, ease: 'power2.out' });
  }

  const kpis = [
    { key: 'totalProducts', label: 'Total Products', icon: 'package', color: '#6366f1' },
    { key: 'totalValue', label: 'Stock Value (₹)', icon: 'rupee', color: '#22d3ee', format: 'currency' },
    { key: 'lowStockItems', label: 'Low Stock Alerts', icon: 'alert', color: '#f59e0b' },
    { key: 'todayMovements', label: "Today's Movements", icon: 'activity', color: '#10b981' }
  ];
</script>

<div class="dashboard">
  <div class="kpi-grid">
    {#each kpis as kpi, i}
      <div 
        class="kpi-card" 
        bind:this={kpiRefs[i]}
        on:mouseenter={() => handleKpiHover(i)}
        on:mouseleave={() => handleKpiLeave(i)}
        style="--accent-color: {kpi.color}"
      >
        <div class="kpi-icon">
          {#if kpi.icon === 'package'}
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4a2 2 0 00-1 1.73v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4a2 2 0 001-1.73z"/>
            </svg>
          {:else if kpi.icon === 'rupee'}
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2v20M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6"/>
            </svg>
          {:else if kpi.icon === 'alert'}
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0zM12 9v4M12 17h.01"/>
            </svg>
          {:else if kpi.icon === 'activity'}
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 12h-4l-3 9L9 3l-3 9H2"/>
            </svg>
          {/if}
        </div>
        <div class="kpi-content">
          <span class="kpi-value">
            {#if kpi.format === 'currency'}
              ₹{stats[kpi.key]?.toLocaleString() || 0}
            {:else}
              {stats[kpi.key] || 0}
            {/if}
          </span>
          <span class="kpi-label">{kpi.label}</span>
        </div>
        <div class="kpi-glow"></div>
      </div>
    {/each}
  </div>

  <div class="charts-grid">
    <div class="chart-card">
      <div class="chart-header">
        <h3>Stock Movements</h3>
        <div class="chart-tabs">
          <button class="chart-tab active">Week</button>
          <button class="chart-tab">Month</button>
        </div>
      </div>
      <div class="chart-body">
        <canvas bind:this={chartCanvas}></canvas>
      </div>
    </div>

    <div class="chart-card recent-activity">
      <div class="chart-header">
        <h3>Recent Activity</h3>
        <button class="view-all-btn">View All</button>
      </div>
      <div class="activity-list">
        <div class="activity-item">
          <div class="activity-icon stock-in">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 19V5M5 12l7-7 7 7"/>
            </svg>
          </div>
          <div class="activity-content">
            <span class="activity-title">Stock Added</span>
            <span class="activity-meta">Dell XPS 15 - 50 units</span>
          </div>
          <span class="activity-time">2 min ago</span>
        </div>
        <div class="activity-item">
          <div class="activity-icon stock-out">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 5v14M5 12l7 7 7-7"/>
            </svg>
          </div>
          <div class="activity-content">
            <span class="activity-title">Stock Sold</span>
            <span class="activity-meta">Logitech MX Keys - 10 units</span>
          </div>
          <span class="activity-time">15 min ago</span>
        </div>
        <div class="activity-item">
          <div class="activity-icon low-stock">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/>
            </svg>
          </div>
          <div class="activity-content">
            <span class="activity-title">Low Stock Alert</span>
            <span class="activity-meta">iPad Air - 5 units remaining</span>
          </div>
          <span class="activity-time">1 hour ago</span>
        </div>
        <div class="activity-item">
          <div class="activity-icon adjustment">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
              <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
            </svg>
          </div>
          <div class="activity-content">
            <span class="activity-title">Stock Adjusted</span>
            <span class="activity-meta">MacBook Pro - +3 units</span>
          </div>
          <span class="activity-time">3 hours ago</span>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .dashboard {
    display: flex;
    flex-direction: column;
    gap: 28px;
  }

  .kpi-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 20px;
  }

  @media (max-width: 1200px) {
    .kpi-grid { grid-template-columns: repeat(2, 1fr); }
  }

  @media (max-width: 768px) {
    .kpi-grid { grid-template-columns: 1fr; }
  }

  .kpi-card {
    position: relative;
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: 24px;
    display: flex;
    align-items: flex-start;
    gap: 16px;
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    cursor: pointer;
  }

  .kpi-card:hover {
    border-color: var(--accent-color);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3), 0 0 0 1px var(--accent-color);
    transform: translateY(-4px);
  }

  .kpi-icon {
    width: 52px;
    height: 52px;
    border-radius: var(--radius-md);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    background: color-mix(in srgb, var(--accent-color) 15%, transparent);
    color: var(--accent-color);
    transition: transform 0.3s ease;
  }

  .kpi-card:hover .kpi-icon {
    transform: scale(1.1) rotate(5deg);
  }

  .kpi-content {
    display: flex;
    flex-direction: column;
    gap: 4px;
    z-index: 1;
  }

  .kpi-value {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .kpi-label {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .kpi-glow {
    position: absolute;
    top: 0;
    right: 0;
    width: 150px;
    height: 150px;
    border-radius: 50%;
    background: var(--accent-color);
    filter: blur(50px);
    opacity: 0.12;
    transition: opacity 0.3s ease;
  }

  .kpi-card:hover .kpi-glow {
    opacity: 0.25;
  }

  .charts-grid {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 24px;
  }

  @media (max-width: 1024px) {
    .charts-grid { grid-template-columns: 1fr; }
  }

  .chart-card {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: 24px;
  }

  .chart-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 20px;
  }

  .chart-header h3 {
    font-family: var(--font-display);
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .chart-tabs {
    display: flex;
    gap: 4px;
    background: var(--bg-secondary);
    padding: 4px;
    border-radius: var(--radius-md);
  }

  .chart-tab {
    padding: 6px 14px;
    border-radius: var(--radius-sm);
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--text-muted);
    transition: all 0.2s ease;
  }

  .chart-tab.active {
    background: var(--accent-primary);
    color: white;
  }

  .chart-body {
    height: 280px;
  }

  .view-all-btn {
    font-size: 0.8rem;
    color: var(--accent-primary);
    padding: 4px 12px;
    border-radius: 100px;
    transition: all 0.2s ease;
  }

  .view-all-btn:hover {
    background: rgba(99, 102, 241, 0.1);
  }

  .activity-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .activity-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    background: var(--bg-secondary);
    border-radius: var(--radius-md);
    transition: all 0.2s ease;
  }

  .activity-item:hover {
    background: var(--bg-tertiary);
    transform: translateX(4px);
  }

  .activity-icon {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .activity-icon.stock-in {
    background: rgba(16, 185, 129, 0.15);
    color: var(--accent-success);
  }

  .activity-icon.stock-out {
    background: rgba(239, 68, 68, 0.15);
    color: var(--accent-danger);
  }

  .activity-icon.low-stock {
    background: rgba(245, 158, 11, 0.15);
    color: var(--accent-warning);
  }

  .activity-icon.adjustment {
    background: rgba(99, 102, 241, 0.15);
    color: var(--accent-primary);
  }

  .activity-content {
    flex: 1;
    min-width: 0;
  }

  .activity-title {
    display: block;
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-primary);
  }

  .activity-meta {
    display: block;
    font-size: 0.8rem;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .activity-time {
    font-size: 0.75rem;
    color: var(--text-muted);
    flex-shrink: 0;
  }
</style>
