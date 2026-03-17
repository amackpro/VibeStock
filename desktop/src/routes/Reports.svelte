<script>
  import { onMount } from 'svelte';
  import { api } from '../lib/api.js';
  import { toast } from '../stores/toast.js';

  let activeReport = 'inventory';
  let loading = false;
  let reportData = null;

  const reportTypes = [
    { id: 'inventory', label: 'Inventory Summary', icon: '📦', desc: 'Full list of all products with quantities and values' },
    { id: 'low-stock', label: 'Low Stock Report', icon: '⚠️', desc: 'Products below reorder level' },
    { id: 'movements', label: 'Stock Movements', icon: '🔄', desc: 'Recent stock in/out transactions' },
    { id: 'valuation', label: 'Inventory Valuation', icon: '💰', desc: 'Total inventory value by category' },
  ];

  async function loadReport(type) {
    activeReport = type;
    loading = true;
    reportData = null;
    try {
      switch (type) {
        case 'inventory':
          reportData = await api.reports.inventory();
          break;
        case 'low-stock':
          reportData = await api.reports.lowStock();
          break;
        case 'movements':
          reportData = await api.reports.movements();
          break;
        case 'valuation':
          reportData = await api.reports.valuation();
          break;
      }
    } catch (e) {
      toast.error(e.message);
    } finally {
      loading = false;
    }
  }

  function exportCSV() {
    if (!reportData || !reportData.items) return;

    let csv = '';
    const items = reportData.items;

    if (activeReport === 'inventory') {
      csv = 'SKU,Name,Category,Supplier,Quantity,Reorder Level,Unit Price,Total Value\n';
      items.forEach(item => {
        csv += `"${item.sku}","${item.name}","${item.category || ''}","${item.supplier || ''}",${item.quantity_in_stock},${item.reorder_level},${item.unit_price},${item.total_value}\n`;
      });
    } else if (activeReport === 'low-stock') {
      csv = 'SKU,Name,Category,Supplier,Quantity,Reorder Level,Unit Price\n';
      items.forEach(item => {
        csv += `"${item.sku}","${item.name}","${item.category || ''}","${item.supplier || ''}",${item.quantity_in_stock},${item.reorder_level},${item.unit_price}\n`;
      });
    } else if (activeReport === 'movements') {
      csv = 'Date,Product SKU,Product Name,Type,Quantity,Reference,Performed By\n';
      items.forEach(item => {
        csv += `"${item.date}","${item.product_sku}","${item.product_name}","${item.movement_type}",${item.quantity},"${item.reference || ''}","${item.performed_by}"\n`;
      });
    } else if (activeReport === 'valuation') {
      csv = 'Category,Total Quantity,Cost Value,Retail Value\n';
      items.forEach(item => {
        csv += `"${item.category || 'Uncategorized'}","${item.total_quantity}",${item.total_cost_value},${item.total_retail_value}\n`;
      });
    }

    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${activeReport}-report-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    URL.revokeObjectURL(url);
    toast.success('Report exported successfully');
  }

  function formatCurrency(value) {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
  }

  onMount(() => {
    loadReport('inventory');
  });
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1>Reports</h1>
      <p class="subtitle">Generate and export inventory reports</p>
    </div>
    {#if reportData}
      <button class="btn btn-primary" on:click={exportCSV}>
        <span>📥</span> Export CSV
      </button>
    {/if}
  </div>

  <div class="report-tabs">
    {#each reportTypes as rt}
      <button
        class="tab-btn"
        class:active={activeReport === rt.id}
        on:click={() => loadReport(rt.id)}
      >
        <span class="tab-icon">{rt.icon}</span>
        <span class="tab-label">{rt.label}</span>
      </button>
    {/each}
  </div>

  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Loading report...</p>
    </div>
  {:else if reportData}
    <div class="report-content">
      {#if activeReport === 'inventory'}
        <div class="summary-cards">
          <div class="summary-card">
            <span class="card-label">Total Products</span>
            <span class="card-value">{reportData.total_items}</span>
          </div>
          <div class="summary-card">
            <span class="card-label">Total Value</span>
            <span class="card-value">{formatCurrency(reportData.total_value)}</span>
          </div>
        </div>
        <div class="table-wrapper">
          <table class="data-table">
            <thead>
              <tr>
                <th>SKU</th>
                <th>Name</th>
                <th>Category</th>
                <th>Supplier</th>
                <th class="text-right">Qty</th>
                <th class="text-right">Reorder</th>
                <th class="text-right">Unit Price</th>
                <th class="text-right">Total Value</th>
              </tr>
            </thead>
            <tbody>
              {#each reportData.items as item}
                <tr>
                  <td><code>{item.sku}</code></td>
                  <td>{item.name}</td>
                  <td>{item.category || '-'}</td>
                  <td>{item.supplier || '-'}</td>
                  <td class="text-right">{item.quantity_in_stock}</td>
                  <td class="text-right">{item.reorder_level}</td>
                  <td class="text-right">{formatCurrency(item.unit_price)}</td>
                  <td class="text-right">{formatCurrency(item.total_value)}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>

      {:else if activeReport === 'low-stock'}
        <div class="summary-cards">
          <div class="summary-card warning">
            <span class="card-label">Low Stock Items</span>
            <span class="card-value">{reportData.total_items}</span>
          </div>
        </div>
        <div class="table-wrapper">
          <table class="data-table">
            <thead>
              <tr>
                <th>SKU</th>
                <th>Name</th>
                <th>Category</th>
                <th>Supplier</th>
                <th class="text-right">Current Qty</th>
                <th class="text-right">Reorder Level</th>
                <th class="text-right">Unit Price</th>
              </tr>
            </thead>
            <tbody>
              {#each reportData.items as item}
                <tr class:low-stock-row={item.quantity_in_stock === 0}>
                  <td><code>{item.sku}</code></td>
                  <td>{item.name}</td>
                  <td>{item.category || '-'}</td>
                  <td>{item.supplier || '-'}</td>
                  <td class="text-right">
                    <span class="qty-badge" class:out={item.quantity_in_stock === 0} class:low={item.quantity_in_stock > 0 && item.quantity_in_stock <= item.reorder_level}>
                      {item.quantity_in_stock}
                    </span>
                  </td>
                  <td class="text-right">{item.reorder_level}</td>
                  <td class="text-right">{formatCurrency(item.unit_price)}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>

      {:else if activeReport === 'movements'}
        <div class="summary-cards">
          <div class="summary-card">
            <span class="card-label">Total Movements</span>
            <span class="card-value">{reportData.total_items}</span>
          </div>
        </div>
        <div class="table-wrapper">
          <table class="data-table">
            <thead>
              <tr>
                <th>Date</th>
                <th>Product SKU</th>
                <th>Product Name</th>
                <th>Type</th>
                <th class="text-right">Qty</th>
                <th>Reference</th>
                <th>Performed By</th>
              </tr>
            </thead>
            <tbody>
              {#each reportData.items as item}
                <tr>
                  <td class="mono">{item.date}</td>
                  <td><code>{item.product_sku}</code></td>
                  <td>{item.product_name}</td>
                  <td>
                    <span class="type-badge type-{item.movement_type}">{item.movement_type}</span>
                  </td>
                  <td class="text-right">{item.quantity}</td>
                  <td>{item.reference || '-'}</td>
                  <td>{item.performed_by}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>

      {:else if activeReport === 'valuation'}
        <div class="summary-cards">
          <div class="summary-card">
            <span class="card-label">Total Quantity</span>
            <span class="card-value">{reportData.total_quantity}</span>
          </div>
          <div class="summary-card">
            <span class="card-label">Cost Value</span>
            <span class="card-value">{formatCurrency(reportData.total_cost_value)}</span>
          </div>
          <div class="summary-card success">
            <span class="card-label">Retail Value</span>
            <span class="card-value">{formatCurrency(reportData.total_retail_value)}</span>
          </div>
        </div>
        <div class="table-wrapper">
          <table class="data-table">
            <thead>
              <tr>
                <th>Category</th>
                <th class="text-right">Total Quantity</th>
                <th class="text-right">Cost Value</th>
                <th class="text-right">Retail Value</th>
                <th class="text-right">Margin</th>
              </tr>
            </thead>
            <tbody>
              {#each reportData.items as item}
                {@const margin = item.total_retail_value > 0 ? ((item.total_retail_value - item.total_cost_value) / item.total_retail_value * 100) : 0}
                <tr>
                  <td>{item.category || 'Uncategorized'}</td>
                  <td class="text-right">{item.total_quantity}</td>
                  <td class="text-right">{formatCurrency(item.total_cost_value)}</td>
                  <td class="text-right">{formatCurrency(item.total_retail_value)}</td>
                  <td class="text-right">
                    <span class="margin-badge" class:positive={margin > 0} class:negative={margin < 0}>
                      {margin.toFixed(1)}%
                    </span>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .report-tabs {
    display: flex;
    gap: var(--space-2);
    margin-bottom: var(--space-6);
    flex-wrap: wrap;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-3) var(--space-4);
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-base);
  }

  .tab-btn:hover {
    background: var(--glass-hover);
    color: var(--text-primary);
  }

  .tab-btn.active {
    background: var(--accent-primary);
    color: white;
    border-color: var(--accent-primary);
  }

  .tab-icon {
    font-size: 1.25rem;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-12);
    color: var(--text-secondary);
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--border-color);
    border-top-color: var(--accent-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-bottom: var(--space-4);
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .summary-cards {
    display: flex;
    gap: var(--space-4);
    margin-bottom: var(--space-6);
    flex-wrap: wrap;
  }

  .summary-card {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: var(--space-4) var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 160px;
  }

  .summary-card.warning {
    border-color: var(--color-warning);
    background: rgba(245, 158, 11, 0.1);
  }

  .summary-card.success {
    border-color: var(--color-success);
    background: rgba(16, 185, 129, 0.1);
  }

  .card-label {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .card-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .table-wrapper {
    overflow-x: auto;
    background: var(--bg-surface);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-color);
  }

  .data-table {
    width: 100%;
    border-collapse: collapse;
  }

  .data-table th,
  .data-table td {
    padding: var(--space-3) var(--space-4);
    text-align: left;
    border-bottom: 1px solid var(--border-color);
  }

  .data-table th {
    background: var(--bg-base);
    font-weight: 600;
    color: var(--text-secondary);
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .data-table tbody tr:hover {
    background: var(--glass-table-hover);
  }

  .data-table tbody tr:last-child td {
    border-bottom: none;
  }

  .text-right {
    text-align: right;
  }

  code {
    background: var(--bg-base);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    font-size: 0.875rem;
  }

  .mono {
    font-family: monospace;
    font-size: 0.8rem;
  }

  .qty-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: var(--radius-full);
    font-size: 0.875rem;
    font-weight: 600;
    background: var(--color-success);
    color: white;
  }

  .qty-badge.low {
    background: var(--color-warning);
  }

  .qty-badge.out {
    background: var(--color-danger);
  }

  .low-stock-row {
    background: rgba(239, 68, 68, 0.05);
  }

  .type-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: var(--radius-sm);
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .type-badge.type-in {
    background: rgba(16, 185, 129, 0.2);
    color: var(--color-success);
  }

  .type-badge.type-out {
    background: rgba(239, 68, 68, 0.2);
    color: var(--color-danger);
  }

  .type-badge.type-adjustment {
    background: rgba(59, 130, 246, 0.2);
    color: #3b82f6;
  }

  .type-badge.type-return {
    background: rgba(168, 85, 247, 0.2);
    color: #a855f7;
  }

  .margin-badge {
    font-weight: 600;
  }

  .margin-badge.positive {
    color: var(--color-success);
  }

  .margin-badge.negative {
    color: var(--color-danger);
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: all var(--transition-base);
  }

  .btn-primary {
    background: var(--accent-primary);
    color: white;
  }

  .btn-primary:hover {
    background: var(--accent-hover);
  }
</style>
