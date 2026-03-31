<script>
  /**
   * Dashboard.svelte — Real-time KPI dashboard
   */
  import { onMount, onDestroy } from 'svelte';
  import { api } from '../lib/api.js';
  import { 
    Package, Tags, Factory, AlertTriangle, XCircle, 
    DollarSign, RotateCcw, RefreshCw 
  } from 'lucide-svelte';

  let stats = null;
  let loading = true;
  let recentMovements = [];

  async function loadStats() {
    try {
      stats = await api.dashboard.stats();
    } catch { /* silently fail */ }
  }

  async function loadMovements() {
    try {
      const r = await api.movements.list({ per_page: 8 });
      recentMovements = r.data ?? [];
    } catch {}
  }

  function onStockUpdated() { 
    loadStats(); 
    loadMovements(); 
  }

  onMount(async () => {
    loading = true;
    await Promise.all([loadStats(), loadMovements()]);
    loading = false;
    window.addEventListener('stock-updated', onStockUpdated);
  });

  onDestroy(() => window.removeEventListener('stock-updated', onStockUpdated));

  function currency(n) {
    return new Intl.NumberFormat('en-IN', { style: 'currency', currency: 'INR', maximumFractionDigits: 0 }).format(n);
  }

  function fmtDate(dt) {
    return new Date(dt).toLocaleString('en-IN', { day:'2-digit', month:'short', hour:'2-digit', minute:'2-digit' });
  }

  const movBadge = { 
    in: 'badge-green', 
    out: 'badge-red', 
    adjustment: 'badge-amber', 
    return: 'badge-cyan' 
  };
</script>

<div class="page">
  <!-- Page Header -->
  <div class="page-header">
    <div class="page-title-group">
      <h1 class="page-title">Dashboard</h1>
      <p class="page-subtitle">Real-time inventory overview</p>
    </div>
    <button class="btn btn-ghost" on:click={() => { loadStats(); loadMovements(); }}>
      <RefreshCw size={16} />
      <span>Refresh</span>
    </button>
  </div>

  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Loading dashboard...</p>
    </div>
  {:else if stats}
    <!-- KPI Cards -->
    <div class="grid grid-3 grid-gap-4">
      <div class="card stat-card stagger-row">
        <span class="stat-icon"><Package size={32} /></span>
        <div class="stat-value">{stats.total_products}</div>
        <div class="stat-label">Total Products</div>
      </div>

      <div class="card stat-card stagger-row">
        <span class="stat-icon"><Tags size={32} /></span>
        <div class="stat-value">{stats.total_categories}</div>
        <div class="stat-label">Categories</div>
      </div>

      <div class="card stat-card stagger-row">
        <span class="stat-icon"><Factory size={32} /></span>
        <div class="stat-value">{stats.total_suppliers}</div>
        <div class="stat-label">Suppliers</div>
      </div>

      <div class="card stat-card stagger-row">
        <span class="stat-icon"><AlertTriangle size={32} /></span>
        <div class="stat-value" style="color: var(--accent-amber)">{stats.low_stock_count}</div>
        <div class="stat-label">Low Stock Items</div>
      </div>

      <div class="card stat-card stagger-row">
        <span class="stat-icon"><XCircle size={32} /></span>
        <div class="stat-value" style="color: var(--accent-red)">{stats.out_of_stock_count}</div>
        <div class="stat-label">Out of Stock</div>
      </div>

      <div class="card stat-card stagger-row">
        <span class="stat-icon"><DollarSign size={32} /></span>
        <div class="stat-value" style="font-size: 1.4rem">{currency(stats.total_stock_value)}</div>
        <div class="stat-label">Total Stock Value</div>
      </div>
    </div>

    <!-- Today's Activity Banner -->
    <div class="card stagger-row" style="display: flex; align-items: center; gap: 16px; padding: 20px 24px; background: rgba(99,102,241,0.08);">
      <RotateCcw size={32} style="color: var(--accent-primary)" />
      <div>
        <div style="font-size: 1.5rem; font-weight: 800; color: var(--accent-glow)">{stats.total_movements_today}</div>
        <div style="font-size: 0.875rem; color: var(--text-secondary)">Stock movements recorded today</div>
      </div>
    </div>

    <!-- Recent Movements Table -->
    <div class="card stagger-row" style="padding: 0; overflow: hidden;">
      <div style="padding: 16px 20px; border-bottom: 1px solid var(--border-surface)">
        <h3>Recent Stock Movements</h3>
      </div>
      <div class="table-wrapper">
        <table>
          <thead>
            <tr>
              <th>Product</th>
              <th>Type</th>
              <th>Qty</th>
              <th>Reference</th>
              <th>By</th>
              <th>Time</th>
            </tr>
          </thead>
          <tbody>
            {#each recentMovements as m, i}
              <tr class="stagger-row">
                <td>
                  <div style="font-weight: 600">{m.product_name}</div>
                  <div style="font-size: 0.75rem; color: var(--text-muted); font-family: var(--font-mono)">{m.product_sku}</div>
                </td>
                <td>
                  <span class="badge {movBadge[m.movement_type] ?? 'badge-violet'}">
                    {m.movement_type}
                  </span>
                </td>
                <td style="font-family: var(--font-mono); font-weight: 700">{m.quantity}</td>
                <td style="color: var(--text-secondary); font-size: 0.85rem">{m.reference ?? '—'}</td>
                <td style="color: var(--text-secondary); font-size: 0.85rem">{m.performed_by_name}</td>
                <td style="color: var(--text-muted); font-size: 0.75rem">{fmtDate(m.created_at)}</td>
              </tr>
            {:else}
              <tr>
                <td colspan="6" style="text-align: center; color: var(--text-muted); padding: 32px !important">
                  No movements recorded yet
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

<style>
  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 64px;
    color: var(--text-secondary);
  }

  .loading-state p {
    margin-top: 16px;
  }
</style>
