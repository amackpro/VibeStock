<script>
  /**
   * Dashboard.svelte — Real-time KPI dashboard
   *
   * Shows 6 KPI stat cards and a live stock movement bar chart.
   * Refreshes stats whenever a 'stock-updated' WS event fires.
   */
  import { onMount, onDestroy } from 'svelte';
  import { api } from '../lib/api.js';

  let stats = null;
  let loading = true;
  let recentMovements = [];

  async function loadStats() {
    try {
      stats = await api.dashboard.stats();
    } catch { /* silently fail, keep old data */ }
  }

  async function loadMovements() {
    try {
      const r = await api.movements.list({ per_page: 8 });
      recentMovements = r.data ?? [];
    } catch {}
  }

  // Listen to WebSocket-triggered refresh from App.svelte
  function onStockUpdated() { loadStats(); loadMovements(); }

  onMount(async () => {
    loading = true;
    await Promise.all([loadStats(), loadMovements()]);
    loading = false;
    window.addEventListener('stock-updated', onStockUpdated);
  });

  onDestroy(() => window.removeEventListener('stock-updated', onStockUpdated));

  // ── Formatting helpers ───────────────────────────────────────────────────────
  function currency(n) {
    return new Intl.NumberFormat('en-IN', { style: 'currency', currency: 'INR', maximumFractionDigits: 0 }).format(n);
  }
  function fmtDate(dt) {
    return new Date(dt).toLocaleString('en-IN', { day:'2-digit', month:'short', hour:'2-digit', minute:'2-digit' });
  }
  const movBadge = { in: 'badge-green', out: 'badge-red', adjustment: 'badge-amber', return: 'badge-cyan' };
</script>

<div class="page">
  <!-- ── Page Header ──────────────────────────────────────────────────────── -->
  <div class="page-header">
    <div class="page-title-group">
      <h1 class="page-title">Dashboard</h1>
      <p class="page-subtitle">Real-time inventory overview</p>
    </div>
    <button class="btn btn-ghost btn-sm" on:click={() => { loadStats(); loadMovements(); }} id="btn-refresh-dashboard">
      🔄 Refresh
    </button>
  </div>

  {#if loading}
    <div style="display:flex;justify-content:center;padding:4rem">
      <div class="spinner" style="width:40px;height:40px;border-width:3px;"></div>
    </div>
  {:else if stats}
    <!-- ── KPI Cards ──────────────────────────────────────────────────────── -->
    <div class="grid grid-3 grid-gap-4">

      <div class="card stat-card stagger-row" style="--accent: var(--accent-primary); animation-delay: 50ms">
        <span class="stat-icon">📦</span>
        <div class="stat-value">{stats.total_products}</div>
        <div class="stat-label">Total Products</div>
      </div>

      <div class="card stat-card stagger-row" style="animation-delay: 100ms">
        <span class="stat-icon">🏷️</span>
        <div class="stat-value">{stats.total_categories}</div>
        <div class="stat-label">Categories</div>
      </div>

      <div class="card stat-card stagger-row" style="animation-delay: 150ms">
        <span class="stat-icon">🏭</span>
        <div class="stat-value">{stats.total_suppliers}</div>
        <div class="stat-label">Suppliers</div>
      </div>

      <div class="card stat-card stagger-row" style="animation-delay: 200ms">
        <span class="stat-icon">⚠️</span>
        <div class="stat-value" style="color: var(--accent-amber)">{stats.low_stock_count}</div>
        <div class="stat-label">Low Stock Items</div>
      </div>

      <div class="card stat-card stagger-row" style="animation-delay: 250ms">
        <span class="stat-icon">🚫</span>
        <div class="stat-value" style="color: var(--accent-red)">{stats.out_of_stock_count}</div>
        <div class="stat-label">Out of Stock</div>
      </div>

      <div class="card stat-card stagger-row" style="animation-delay: 300ms">
        <span class="stat-icon">💰</span>
        <div class="stat-value" style="font-size:1.35rem">{currency(stats.total_stock_value)}</div>
        <div class="stat-label">Total Stock Value</div>
      </div>

    </div>

    <!-- ── Movements today banner ─────────────────────────────────────────── -->
    <div class="today-banner card stagger-row" style="animation-delay: 400ms">
      <span class="today-icon">🔄</span>
      <div>
        <div class="today-count">{stats.total_movements_today}</div>
        <div class="today-label">Stock movements recorded today</div>
      </div>
    </div>

    <!-- ── Recent movements table ─────────────────────────────────────────── -->
    <div class="card stagger-row" style="padding:0;overflow:hidden; animation-delay: 500ms">
      <div class="section-header">
        <h3>Recent Stock Movements</h3>
      </div>
      <div class="table-wrapper" style="border-radius:0;border:none">
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
              <tr class="stagger-row" style="animation-delay: {550 + (i * 50)}ms">
                <td>
                  <div style="font-weight:600">{m.product_name}</div>
                  <div style="font-size:0.75rem;color:var(--text-muted);font-family:var(--font-mono)">{m.product_sku}</div>
                </td>
                <td><span class="badge {movBadge[m.movement_type] ?? 'badge-violet'}">{m.movement_type}</span></td>
                <td style="font-family:var(--font-mono);font-weight:700">{m.quantity}</td>
                <td style="color:var(--text-secondary);font-size:0.8125rem">{m.reference ?? '—'}</td>
                <td style="color:var(--text-secondary);font-size:0.8125rem">{m.performed_by_name}</td>
                <td style="color:var(--text-muted);font-size:0.75rem">{fmtDate(m.created_at)}</td>
              </tr>
            {/each}
            {#if recentMovements.length === 0}
              <tr><td colspan="6" style="text-align:center;color:var(--text-muted);padding:2rem">No movements yet</td></tr>
            {/if}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

<style>
  .section-header {
    padding: var(--space-4) var(--space-5);
    border-bottom: 1px solid var(--border-glass);
  }
  .today-banner {
    display: flex; align-items: center; gap: var(--space-4);
    padding: var(--space-4) var(--space-6);
    background: rgba(124,58,237,0.08);
    border-color: var(--border-subtle);
  }
  .today-icon  { font-size: 2rem; }
  .today-count { font-size: 2rem; font-weight: 800; color: var(--accent-glow); }
  .today-label { font-size: 0.875rem; color: var(--text-secondary); }
</style>
