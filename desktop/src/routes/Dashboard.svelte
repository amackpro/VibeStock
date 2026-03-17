<script>
  /**
   * Dashboard.svelte — Real-time KPI dashboard
   * 
   * Shows 6 KPI stat cards and recent stock movements.
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
    in: 'badge-success', 
    out: 'badge-danger', 
    adjustment: 'badge-warning', 
    return: 'badge-info' 
  };

  const kpiCards = [
    { key: 'total_products', label: 'Total Products', icon: '📦', color: 'primary' },
    { key: 'total_categories', label: 'Categories', icon: '🏷️', color: 'info' },
    { key: 'total_suppliers', label: 'Suppliers', icon: '🏭', color: 'success' },
  ];
</script>

<div class="page">
  <!-- Page Header -->
  <div class="page-header">
    <div>
      <h1 class="page-title">Dashboard</h1>
      <p class="page-subtitle">Real-time inventory overview</p>
    </div>
    <button class="btn btn-secondary" on:click={() => { loadStats(); loadMovements(); }}>
      🔄 Refresh
    </button>
  </div>

  {#if loading}
    <div class="loading-state">
      <div class="spinner spinner-lg"></div>
      <p>Loading dashboard...</p>
    </div>
  {:else if stats}
    <!-- KPI Cards Grid -->
    <div class="kpi-grid">
      {#each kpiCards as card, i}
        <div class="kpi-card stagger-item" style="animation-delay: {i * 50}ms">
          <div class="kpi-icon" style="--kpi-color: var(--color-{card.color})">{card.icon}</div>
          <div class="kpi-content">
            <div class="kpi-value">{stats[card.key]}</div>
            <div class="kpi-label">{card.label}</div>
          </div>
        </div>
      {/each}
      
      <!-- Warning Cards -->
      <div class="kpi-card stagger-item warning" style="animation-delay: 150ms">
        <div class="kpi-icon" style="--kpi-color: var(--color-warning)">⚠️</div>
        <div class="kpi-content">
          <div class="kpi-value">{stats.low_stock_count}</div>
          <div class="kpi-label">Low Stock Items</div>
        </div>
      </div>

      <div class="kpi-card stagger-item danger" style="animation-delay: 200ms">
        <div class="kpi-icon" style="--kpi-color: var(--color-danger)">🚫</div>
        <div class="kpi-content">
          <div class="kpi-value">{stats.out_of_stock_count}</div>
          <div class="kpi-label">Out of Stock</div>
        </div>
      </div>

      <div class="kpi-card stagger-item accent" style="animation-delay: 250ms">
        <div class="kpi-icon" style="--kpi-color: var(--color-accent)">💰</div>
        <div class="kpi-content">
          <div class="kpi-value currency">{currency(stats.total_stock_value)}</div>
          <div class="kpi-label">Total Stock Value</div>
        </div>
      </div>
    </div>

    <!-- Today's Activity -->
    <div class="activity-card card stagger-item" style="animation-delay: 300ms">
      <div class="activity-header">
        <div class="activity-info">
          <span class="activity-icon">🔄</span>
          <div>
            <div class="activity-title">Today's Activity</div>
            <div class="activity-subtitle">{stats.total_movements_today} stock movements recorded</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Movements Table -->
    <div class="card table-card stagger-item" style="animation-delay: 350ms">
      <div class="card-header">
        <h3 class="card-title">Recent Stock Movements</h3>
      </div>
      <div class="table-container">
        <table class="table">
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
              <tr class="stagger-item" style="animation-delay: {400 + (i * 30)}ms">
                <td>
                  <div class="product-cell">
                    <div class="product-name">{m.product_name}</div>
                    <div class="product-sku">{m.product_sku}</div>
                  </div>
                </td>
                <td>
                  <span class="badge {movBadge[m.movement_type] ?? 'badge-primary'}">
                    {m.movement_type}
                  </span>
                </td>
                <td class="qty-cell">{m.quantity}</td>
                <td class="ref-cell">{m.reference ?? '—'}</td>
                <td class="user-cell">{m.performed_by_name}</td>
                <td class="time-cell">{fmtDate(m.created_at)}</td>
              </tr>
            {:else}
              <tr>
                <td colspan="6" class="empty-cell">No movements recorded yet</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

<style>
  .page {
    max-width: 1400px;
    margin: 0 auto;
  }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: var(--space-6);
  }

  .page-title {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    margin-bottom: var(--space-1);
  }

  .page-subtitle {
    color: var(--text-secondary);
    font-size: var(--text-sm);
    margin: 0;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-16);
    color: var(--text-secondary);
  }

  .loading-state p {
    margin-top: var(--space-4);
  }

  /* KPI Grid */
  .kpi-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: var(--space-4);
    margin-bottom: var(--space-6);
  }

  .kpi-card {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-5);
    background: var(--bg-surface);
    border-radius: var(--radius-xl);
    border: 1px solid var(--border-color);
    transition: all var(--transition-fast);
  }

  .kpi-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .kpi-card.warning {
    border-color: rgba(245, 158, 11, 0.3);
    background: linear-gradient(135deg, var(--bg-surface), rgba(245, 158, 11, 0.05));
  }

  .kpi-card.danger {
    border-color: rgba(239, 68, 68, 0.3);
    background: linear-gradient(135deg, var(--bg-surface), rgba(239, 68, 68, 0.05));
  }

  .kpi-card.accent {
    border-color: rgba(6, 182, 212, 0.3);
    background: linear-gradient(135deg, var(--bg-surface), rgba(6, 182, 212, 0.05));
  }

  .kpi-icon {
    width: 56px;
    height: 56px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in srgb, var(--kpi-color) 15%, transparent);
    border-radius: var(--radius-lg);
    font-size: 1.5rem;
  }

  .kpi-value {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--text-primary);
    line-height: 1.2;
  }

  .kpi-value.currency {
    font-size: var(--text-xl);
  }

  .kpi-label {
    font-size: var(--text-sm);
    color: var(--text-secondary);
    margin-top: var(--space-1);
  }

  /* Activity Card */
  .activity-card {
    margin-bottom: var(--space-6);
    background: linear-gradient(135deg, rgba(99, 102, 241, 0.08), rgba(6, 182, 212, 0.08));
    border-color: rgba(99, 102, 241, 0.2);
  }

  .activity-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .activity-info {
    display: flex;
    align-items: center;
    gap: var(--space-4);
  }

  .activity-icon {
    font-size: 2rem;
  }

  .activity-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
  }

  .activity-subtitle {
    font-size: var(--text-sm);
    color: var(--text-secondary);
  }

  /* Table */
  .table-card {
    overflow: hidden;
  }

  .card-header {
    padding: var(--space-4) var(--space-5);
    border-bottom: 1px solid var(--border-color);
  }

  .card-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    margin: 0;
  }

  .table {
    margin: 0;
  }

  .product-cell {
    display: flex;
    flex-direction: column;
  }

  .product-name {
    font-weight: var(--font-medium);
  }

  .product-sku {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--text-tertiary);
  }

  .qty-cell {
    font-family: var(--font-mono);
    font-weight: var(--font-semibold);
    text-align: right;
  }

  .ref-cell {
    color: var(--text-secondary);
    font-size: var(--text-sm);
  }

  .user-cell {
    color: var(--text-secondary);
    font-size: var(--text-sm);
  }

  .time-cell {
    color: var(--text-tertiary);
    font-size: var(--text-xs);
    white-space: nowrap;
  }

  .empty-cell {
    text-align: center;
    color: var(--text-tertiary);
    padding: var(--space-8) !important;
  }
</style>
