<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { Chart, registerables } from 'chart.js';
  import { api } from '../lib/api.js';
  import { toastStore } from '../stores/toast.js';

  Chart.register(...registerables);

  let activeTab = 'inventory';
  let inventory = [];
  let lowStock = [];
  let movements = [];
  let valuation = [];
  let loading = true;

  let regions = [];
  let countries = [];
  let categories = [];

  let selectedRegion = 'Asia';
  let selectedCountry = '';
  let selectedCategory = '';

  let inventoryChart;
  let movementChart;
  let inventoryCanvas;
  let movementCanvas;

  onMount(async () => {
    await Promise.all([loadFilters(), loadReports()]);
    initCharts();
    animateEntrance();
  });

  async function loadFilters() {
    try {
      const [catRes, regRes] = await Promise.all([
        api.categories.list(),
        api.geography.regions()
      ]);
      categories = catRes.data || catRes;
      regions = regRes.data || regRes;

      const asia = regions.find(r => r.name === 'Asia');
      if (asia) {
        const countryRes = await api.geography.countriesByRegion(asia.id);
        countries = countryRes.data || countryRes;
        const india = countries.find(c => c.name === 'India');
        if (india) {
          selectedCountry = india.name;
        }
      }
    } catch (e) {
      console.error('Failed to load filters', e);
    }
  }

  async function loadReports() {
    loading = true;
    try {
      const params = { region: selectedRegion };
      if (selectedCountry && selectedCountry !== '') params.country = selectedCountry;
      if (selectedCategory && selectedCategory !== '') params.category = selectedCategory;

      const [inv, low, mov, val] = await Promise.all([
        api.reports.inventory(params),
        api.reports.lowStock(params),
        api.reports.movements(params),
        api.reports.valuation(params)
      ]);
      inventory = inv.items || inv.data || inv || [];
      lowStock = low.items || low.data || low || [];
      movements = mov.items || mov.data || mov || [];
      valuation = val.items || val.data || val || [];
      
      updateChartsData();
    } catch (e) {
      console.error('Failed to load reports', e);
      toastStore.show('Failed to load reports', 'error');
    } finally {
      loading = false;
    }
  }

  function updateChartsData() {
    if (inventoryChart) {
      const lowCount = inventory.filter(i => i.quantity_in_stock <= i.reorder_level && i.quantity_in_stock > 0).length;
      const outCount = inventory.filter(i => i.quantity_in_stock <= 0).length;
      const okCount = inventory.length - lowCount - outCount;
      
      inventoryChart.data.datasets[0].data = [okCount, lowCount, outCount];
      inventoryChart.update();
    }

    if (movementChart) {
      // Group movements by last 6 months
      const months = [];
      const now = new Date();
      for (let i = 5; i >= 0; i--) {
        const d = new Date(now.getFullYear(), now.getMonth() - i, 1);
        months.push(d.toLocaleString('default', { month: 'short' }));
      }

      const stockIn = new Array(6).fill(0);
      const stockOut = new Array(6).fill(0);

      movements.forEach(m => {
        const d = new Date(m.date);
        const diff = (now.getFullYear() - d.getFullYear()) * 12 + (now.getMonth() - d.getMonth());
        if (diff >= 0 && diff < 6) {
          const idx = 5 - diff;
          if (m.movement_type === 'IN') stockIn[idx] += m.quantity;
          else if (m.movement_type === 'OUT') stockOut[idx] += m.quantity;
        }
      });

      movementChart.data.labels = months;
      movementChart.data.datasets[0].data = stockIn;
      movementChart.data.datasets[1].data = stockOut;
      movementChart.update();
    }
  }

  function handleFilterChange() {
    loadReports();
  }

  async function handleRegionChange() {
    const region = regions.find(r => r.name === selectedRegion);
    if (region) {
      try {
        const countryRes = await api.geography.countriesByRegion(region.id);
        countries = countryRes.data || countryRes;
        if (!countries.find(c => c.name === selectedCountry)) {
          selectedCountry = '';
        }
      } catch (e) {
        console.error('Failed to update countries', e);
      }
    } else {
      countries = [];
      selectedCountry = '';
    }
    loadReports();
  }

  function animateEntrance() {
    const tl = gsap.timeline();

    tl.fromTo('.page-header', 
      { y: -20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' }
    );

    tl.fromTo('.tabs', 
      { y: -10, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.4, ease: 'power3.out' },
      '-=0.3'
    );

    tl.fromTo('.filter-bar', 
      { y: -10, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.4, ease: 'power3.out' },
      '-=0.2'
    );

    tl.fromTo('.reports-grid .chart-card', 
      { y: 30, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, stagger: 0.15, ease: 'power3.out' },
      '-=0.2'
    );

    tl.fromTo('.table-container', 
      { y: 30, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' },
      '-=0.3'
    );
  }

  function initCharts() {
    setTimeout(() => {
      if (inventoryCanvas) {
        const ctx = inventoryCanvas.getContext('2d');
        inventoryChart = new Chart(ctx, {
          type: 'doughnut',
          data: {
            labels: ['In Stock', 'Low Stock', 'Out of Stock'],
            datasets: [{
              data: [0, 0, 0],
              backgroundColor: ['#6366f1', '#f59e0b', '#ef4444'],
              borderWidth: 0
            }]
          },
          options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
              legend: {
                position: 'bottom',
                labels: { color: '#a0a0b0', padding: 20 }
              }
            },
            cutout: '70%'
          }
        });
      }

      if (movementCanvas) {
        const ctx = movementCanvas.getContext('2d');
        movementChart = new Chart(ctx, {
          type: 'bar',
          data: {
            labels: ['', '', '', '', '', ''],
            datasets: [
              {
                label: 'Stock In',
                data: [0, 0, 0, 0, 0, 0],
                backgroundColor: '#10b981',
                borderRadius: 4
              },
              {
                label: 'Stock Out',
                data: [0, 0, 0, 0, 0, 0],
                backgroundColor: '#ef4444',
                borderRadius: 4
              }
            ]
          },
          options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
              legend: {
                position: 'top',
                labels: { color: '#a0a0b0' }
              }
            },
            scales: {
              x: {
                grid: { display: false },
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
      
      if (inventory.length > 0) updateChartsData();
    }, 100);
  }

  function exportCSV() {
    const data = activeTab === 'inventory' ? inventory : 
                 activeTab === 'lowStock' ? lowStock : 
                 activeTab === 'movements' ? movements : valuation;
    
    if (!data.length) return;

    const locationPart = selectedCountry ? `${selectedRegion}-${selectedCountry}` : selectedRegion;
    const categoryPart = selectedCategory ? `-${selectedCategory.replace(/\s+/g, '')}` : '';
    const timestamp = new Date().toISOString().split('T')[0];
    
    const headers = Object.keys(data[0]);
    const csv = [headers.join(','), ...data.map(row => headers.map(h => row[h]).join(','))].join('\n');
    
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${activeTab}-report-${locationPart}${categoryPart}-${timestamp}.csv`;
    a.click();
  }

  const tabs = [
    { id: 'inventory', label: 'Inventory' },
    { id: 'lowStock', label: 'Low Stock' },
    { id: 'movements', label: 'Movements' },
    { id: 'valuation', label: 'Valuation' }
  ];
</script>

<div class="reports-page">
  <div class="page-header">
    <h2 class="page-title">Reports</h2>
    <button class="btn btn-primary" on:click={exportCSV}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
        <polyline points="7,10 12,15 17,10"/>
        <line x1="12" y1="15" x2="12" y2="3"/>
      </svg>
      Export CSV
    </button>
  </div>

  <div class="tabs">
    {#each tabs as tab}
      <button 
        class="tab" 
        class:active={activeTab === tab.id}
        on:click={() => activeTab = tab.id}
      >
        {tab.label}
      </button>
    {/each}
  </div>

  <div class="filter-bar">
    <div class="filter-group">
      <label class="filter-label">Region</label>
      <select class="filter-select" bind:value={selectedRegion} on:change={handleRegionChange}>
        {#each regions as region}
          <option value={region.name}>{region.name}</option>
        {/each}
      </select>
    </div>
    <div class="filter-group">
      <label class="filter-label">Country</label>
      <select class="filter-select" bind:value={selectedCountry} on:change={handleFilterChange}>
        <option value="">All Countries</option>
        {#each countries as country}
          <option value={country.name}>{country.name}</option>
        {/each}
      </select>
    </div>
    <div class="filter-group">
      <label class="filter-label">Category</label>
      <select class="filter-select" bind:value={selectedCategory} on:change={handleFilterChange}>
        <option value="">All Categories</option>
        {#each categories as cat}
          <option value={cat.name}>{cat.name}</option>
        {/each}
      </select>
    </div>
  </div>

  <div class="reports-grid">
    <div class="chart-card">
      <h3>Stock Overview</h3>
      <div class="chart-container">
        <canvas bind:this={inventoryCanvas}></canvas>
      </div>
    </div>

    <div class="chart-card">
      <h3>Movement Trends</h3>
      <div class="chart-container">
        <canvas bind:this={movementCanvas}></canvas>
      </div>
    </div>
  </div>

  <div class="table-container">
    {#if loading}
      <div class="loading-state">
        <div class="spinner"></div>
        Loading reports...
      </div>
    {:else if activeTab === 'inventory' && inventory.length === 0}
      <div class="empty-state">
        <span>No inventory data found</span>
      </div>
    {:else if activeTab === 'inventory'}
      <table>
        <thead>
          <tr>
            <th>SKU</th>
            <th>Product</th>
            <th>Category</th>
            <th>Stock</th>
            <th>Price</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {#each inventory as item, i}
            <tr style="animation-delay: {i * 0.03}s">
              <td><span class="sku-badge">{item.sku}</span></td>
              <td>{item.name}</td>
              <td>{item.category || '-'}</td>
              <td>{item.quantity_in_stock || 0}</td>
              <td>₹{item.unit_price?.toLocaleString()}</td>
              <td>₹{(item.total_value || (item.quantity_in_stock * item.unit_price) || 0).toLocaleString()}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else if activeTab === 'lowStock' && lowStock.length === 0}
      <div class="empty-state">
        <span>No low stock items found</span>
      </div>
    {:else if activeTab === 'lowStock'}
      <table>
        <thead>
          <tr>
            <th>SKU</th>
            <th>Product</th>
            <th>Current Stock</th>
            <th>Threshold</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {#each lowStock as item, i}
            <tr style="animation-delay: {i * 0.03}s">
              <td><span class="sku-badge">{item.sku}</span></td>
              <td>{item.name}</td>
              <td class="stock-critical">{item.quantity_in_stock || 0}</td>
              <td>{item.reorder_level || 0}</td>
              <td><span class="badge badge-warning">Low Stock</span></td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else if activeTab === 'movements'}
      <table>
        <thead>
          <tr>
            <th>Date</th>
            <th>Product</th>
            <th>Type</th>
            <th>Quantity</th>
            <th>User</th>
          </tr>
        </thead>
        <tbody>
          {#each movements as item, i}
            <tr style="animation-delay: {i * 0.03}s">
              <td>{item.created_at ? new Date(item.created_at).toLocaleDateString() : '-'}</td>
              <td>{item.product_name || item.product || 'Unknown'}</td>
              <td><span class="type-badge type-{item.movement_type?.toLowerCase()}">{item.movement_type}</span></td>
              <td>{item.quantity}</td>
              <td>{item.performed_by_name || '-'}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else if activeTab === 'valuation'}
      <table>
        <thead>
          <tr>
            <th>Category</th>
            <th>Total Qty</th>
            <th>Cost Value</th>
            <th>Retail Value</th>
          </tr>
        </thead>
        <tbody>
          {#each valuation as item, i}
            <tr style="animation-delay: {i * 0.03}s">
              <td>{item.category || 'Uncategorized'}</td>
              <td>{item.total_quantity?.toLocaleString() || 0}</td>
              <td>₹{(item.total_cost_value || 0).toLocaleString()}</td>
              <td>₹{(item.total_retail_value || 0).toLocaleString()}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>

<style>
  .reports-page {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 1.3rem;
    font-weight: 600;
  }

  .tabs {
    display: flex;
    gap: 4px;
    background: var(--bg-secondary);
    padding: 4px;
    border-radius: var(--radius-md);
    width: fit-content;
  }

  .tab {
    padding: 10px 20px;
    border-radius: var(--radius-sm);
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-secondary);
    transition: all var(--transition-base);
  }

  .tab:hover {
    color: var(--text-primary);
  }

  .tab.active {
    background: var(--accent-primary);
    color: white;
  }

  .reports-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 24px;
  }

  @media (max-width: 1024px) {
    .reports-grid {
      grid-template-columns: 1fr;
    }
  }

  .chart-card {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: 24px;
  }

  .chart-card h3 {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: 20px;
    color: var(--text-primary);
  }

  .chart-container {
    height: 280px;
  }

  .table-container {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  table {
    width: 100%;
  }

  th {
    padding: 16px 20px;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-muted);
    background: var(--bg-secondary);
    text-align: left;
  }

  td {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color);
    color: var(--text-primary);
  }

  .loading-state {
    text-align: center;
    padding: 48px;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
  }

  .sku-badge {
    padding: 4px 10px;
    background: rgba(99, 102, 241, 0.15);
    color: var(--accent-primary);
    border-radius: var(--radius-sm);
    font-size: 0.8rem;
    font-weight: 600;
  }

  .stock-critical {
    color: var(--accent-warning);
    font-weight: 600;
  }

  .type-badge {
    padding: 4px 10px;
    border-radius: 100px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .type-badge.type-in {
    background: rgba(16, 185, 129, 0.15);
    color: var(--accent-success);
  }

  .type-badge.type-out {
    background: rgba(239, 68, 68, 0.15);
    color: var(--accent-danger);
  }

  .type-badge.type-adjustment {
    background: rgba(99, 102, 241, 0.15);
    color: var(--accent-primary);
  }

  tr {
    animation: slideIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    opacity: 0;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(-10px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .filter-bar {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px 20px;
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    margin-bottom: 20px;
    flex-wrap: wrap;
  }

  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
    min-width: 150px;
  }

  .filter-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .filter-select {
    padding: 10px 14px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .filter-select:hover {
    border-color: var(--accent-primary);
  }

  .filter-select:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
  }
</style>
