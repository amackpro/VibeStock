<script>
  import { onMount } from 'svelte';
  import { api } from '../lib/api.js';
  import { toast } from '../stores/toast.js';
  import {
    Package, AlertTriangle, ArrowLeftRight, DollarSign,
    Download, MapPin, Filter, X, ChevronDown
  } from 'lucide-svelte';

  // ── Report type ──────────────────────────────────────────────────────────────
  let activeReport = 'inventory';
  let loading      = false;
  let reportData   = null;

  const reportTypes = [
    { id: 'inventory', label: 'Inventory',   icon: Package,       desc: 'All products with quantities & values' },
    { id: 'low-stock', label: 'Low Stock',   icon: AlertTriangle, desc: 'Products below reorder level' },
    { id: 'movements', label: 'Movements',   icon: ArrowLeftRight,desc: 'Stock in/out transactions' },
    { id: 'valuation', label: 'Valuation',   icon: DollarSign,    desc: 'Inventory value by category' },
  ];

  // ── Geography filter state ────────────────────────────────────────────────────
  let regions         = [];
  let countries       = [];
  let cities          = [];
  let selectedRegion  = '';
  let selectedCountry = '';
  let selectedCity    = '';
  let geoLoading      = false;
  let showFilters     = false;

  // ── Category filter ───────────────────────────────────────────────────────────
  let categories      = [];
  let selectedCat     = '';

  // ── Date range (movements only) ───────────────────────────────────────────────
  let dateFrom = '';
  let dateTo   = '';

  // ── Computed labels ───────────────────────────────────────────────────────────
  $: regionLabel  = regions.find(r => r.id === selectedRegion)?.name ?? '';
  $: countryLabel = countries.find(c => c.id === selectedCountry)?.name ?? '';
  $: cityLabel    = cities.find(c => c.id === selectedCity)?.name ?? '';
  $: catLabel     = categories.find(c => c.id === selectedCat)?.name ?? '';
  $: hasFilters   = !!(selectedRegion || selectedCat || (activeReport === 'movements' && (dateFrom || dateTo)));

  // ── Geography loaders ─────────────────────────────────────────────────────────
  async function loadRegions() {
    try { regions = await api.geography.regions(); } catch {}
  }

  async function onRegionChange() {
    selectedCountry = ''; selectedCity = ''; countries = []; cities = [];
    if (!selectedRegion) { reload(); return; }
    geoLoading = true;
    try { countries = await api.geography.countriesByRegion(selectedRegion); } catch {}
    finally { geoLoading = false; }
    reload();
  }

  async function onCountryChange() {
    selectedCity = ''; cities = [];
    if (!selectedCountry) { reload(); return; }
    geoLoading = true;
    try { cities = await api.geography.citiesByCountry(selectedCountry); } catch {}
    finally { geoLoading = false; }
    reload();
  }

  function onCityChange() { reload(); }

  function clearFilters() {
    selectedRegion = ''; selectedCountry = ''; selectedCity = '';
    countries = []; cities = [];
    selectedCat = '';
    dateFrom = ''; dateTo = '';
    reload();
  }

  // ── Build query params from current filters ───────────────────────────────────
  function buildParams() {
    const p = {};
    if (selectedCat)  p.category_id = selectedCat;
    if (selectedCity) p.city_id     = selectedCity;
    if (activeReport === 'movements') {
      if (dateFrom) p.date_from = dateFrom;
      if (dateTo)   p.date_to   = dateTo;
    }
    return p;
  }

  // ── Load report ───────────────────────────────────────────────────────────────
  function reload() { loadReport(activeReport); }

  async function loadReport(type) {
    activeReport = type;
    loading      = true;
    reportData   = null;
    try {
      const params = buildParams();
      switch (type) {
        case 'inventory': reportData = await api.reports.inventory(params);  break;
        case 'low-stock': reportData = await api.reports.lowStock(params);   break;
        case 'movements': reportData = await api.reports.movements(params);  break;
        case 'valuation': reportData = await api.reports.valuation(params);  break;
      }
    } catch (e) {
      toast.error(e.message);
    } finally {
      loading = false;
    }
  }

  // Backend now filters — items are already correct, just alias
  $: filteredItems = reportData?.items ?? [];

  // ── Valuation bar chart helper ────────────────────────────────────────────────
  $: maxVal = filteredItems.length
    ? Math.max(...filteredItems.map(i => i.total_retail_value ?? 0), 1)
    : 1;

  // ── CSV Export ────────────────────────────────────────────────────────────────
  function exportCSV() {
    if (!filteredItems.length) return;
    let csv = '';
    if (activeReport === 'inventory') {
      csv = 'SKU,Name,Category,Supplier,Qty,Reorder,Unit Price,Total Value\n';
      filteredItems.forEach(i => {
        csv += `"${i.sku}","${i.name}","${i.category||''}","${i.supplier||''}",${i.quantity_in_stock},${i.reorder_level},${i.unit_price},${i.total_value}\n`;
      });
    } else if (activeReport === 'low-stock') {
      csv = 'SKU,Name,Category,Supplier,Qty,Reorder,Unit Price\n';
      filteredItems.forEach(i => {
        csv += `"${i.sku}","${i.name}","${i.category||''}","${i.supplier||''}",${i.quantity_in_stock},${i.reorder_level},${i.unit_price}\n`;
      });
    } else if (activeReport === 'movements') {
      csv = 'Date,SKU,Product,Type,Qty,Reference,Performed By\n';
      filteredItems.forEach(i => {
        csv += `"${i.date}","${i.product_sku}","${i.product_name}","${i.movement_type}",${i.quantity},"${i.reference||''}","${i.performed_by}"\n`;
      });
    } else if (activeReport === 'valuation') {
      csv = 'Category,Total Qty,Cost Value,Retail Value,Margin %\n';
      filteredItems.forEach(i => {
        const m = i.total_retail_value > 0
          ? ((i.total_retail_value - i.total_cost_value) / i.total_retail_value * 100).toFixed(1) : 0;
        csv += `"${i.category||'Uncategorized'}",${i.total_quantity},${i.total_cost_value},${i.total_retail_value},${m}\n`;
      });
    }
    const blob = new Blob([csv], { type: 'text/csv' });
    const url  = URL.createObjectURL(blob);
    const a    = document.createElement('a');
    a.href = url;
    a.download = `${activeReport}-report-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    URL.revokeObjectURL(url);
    toast.success('Report exported');
  }

  function fmt(n) {
    return new Intl.NumberFormat('en-IN', { style: 'currency', currency: 'INR', maximumFractionDigits: 0 }).format(n);
  }

  onMount(async () => {
    await Promise.all([loadRegions(), api.categories.list().then(c => categories = c).catch(() => {})]);
    loadReport('inventory');
  });
</script>

<div class="page">
  <!-- ── Header ─────────────────────────────────────────────────────────────── -->
  <div class="page-header">
    <div class="page-title-group">
      <h1 class="page-title">Reports</h1>
      <div class="flex gap-2 items-center flex-wrap">
        <p class="page-subtitle">
          {filteredItems.length} record{filteredItems.length !== 1 ? 's' : ''}
          {hasFilters ? '(filtered)' : ''}
        </p>
        {#if regionLabel}
          <span class="chip chip-region"><MapPin size={11}/>{regionLabel}
            {#if !countryLabel}<button class="chip-x" on:click={clearFilters}><X size={11}/></button>{/if}
          </span>
        {/if}
        {#if countryLabel}
          <span class="chip-arrow">›</span>
          <span class="chip chip-country">{countryLabel}
            {#if !cityLabel}<button class="chip-x" on:click={clearFilters}><X size={11}/></button>{/if}
          </span>
        {/if}
        {#if cityLabel}
          <span class="chip-arrow">›</span>
          <span class="chip chip-city">{cityLabel}
            <button class="chip-x" on:click={clearFilters}><X size={11}/></button>
          </span>
        {/if}
        {#if catLabel}
          <span class="chip chip-cat">{catLabel}
            <button class="chip-x" on:click={() => { selectedCat = ''; reload(); }}><X size={11}/></button>
          </span>
        {/if}
      </div>
    </div>
    <div class="flex gap-3 items-center">
      <button
        class="btn btn-ghost filter-btn"
        class:active={showFilters}
        on:click={() => showFilters = !showFilters}
      >
        <Filter size={15} />
        Filters
        <span class="chevron" class:rotated={showFilters} style="display:flex;align-items:center">
          <ChevronDown size={13} />
        </span>
      </button>
      {#if filteredItems.length}
        <button class="btn btn-primary" on:click={exportCSV}>
          <Download size={15} />
          Export CSV
        </button>
      {/if}
    </div>
  </div>

  <!-- ── Report Type Tabs ───────────────────────────────────────────────────── -->
  <div class="report-tabs">
    {#each reportTypes as rt}
      <button
        class="tab-btn"
        class:active={activeReport === rt.id}
        on:click={() => loadReport(rt.id)}
      >
        <svelte:component this={rt.icon} size={16} />
        <span>{rt.label}</span>
      </button>
    {/each}
  </div>

  <!-- ── Filter Panel ───────────────────────────────────────────────────────── -->
  {#if showFilters}
    <div class="filter-panel stagger-row" style="animation-delay:0ms">
      <div class="filter-panel-inner">
        <div class="filter-header">
          <span class="filter-title">
            <Filter size={14} style="color:var(--accent-glow)" />
            Filter Report
          </span>
          {#if hasFilters}
            <button class="btn btn-ghost btn-sm" on:click={clearFilters}>Clear All</button>
          {/if}
        </div>

        <div class="filter-row">
          <!-- Location drilldown -->
          <div class="filter-group">
            <label class="filter-label">Region</label>
            <div class="sel-wrap">
              <select class="f-select" bind:value={selectedRegion} on:change={onRegionChange}>
                <option value="">All Regions</option>
                {#each regions as r}<option value={r.id}>{r.name}</option>{/each}
              </select>
            </div>
          </div>

          <span class="loc-arrow">›</span>

          <div class="filter-group">
            <label class="filter-label">Country</label>
            <div class="sel-wrap">
              <select class="f-select" bind:value={selectedCountry} on:change={onCountryChange}
                disabled={!selectedRegion || countries.length === 0}>
                <option value="">All Countries</option>
                {#each countries as c}<option value={c.id}>{c.name}</option>{/each}
              </select>
            </div>
          </div>

          <span class="loc-arrow">›</span>

          <div class="filter-group">
            <label class="filter-label">City</label>
            <div class="sel-wrap">
              <select class="f-select" bind:value={selectedCity} on:change={onCityChange}
                disabled={!selectedCountry || cities.length === 0}>
                <option value="">All Cities</option>
                {#each cities as c}<option value={c.id}>{c.name}</option>{/each}
              </select>
            </div>
          </div>

          <!-- Divider -->
          <div class="filter-divider"></div>

          <!-- Category -->
          <div class="filter-group">
            <label class="filter-label">Category</label>
            <div class="sel-wrap">
              <select class="f-select" bind:value={selectedCat} on:change={reload}>
                <option value="">All Categories</option>
                {#each categories as c}<option value={c.id}>{c.name}</option>{/each}
              </select>
            </div>
          </div>

          <!-- Date range (movements only) -->
          {#if activeReport === 'movements'}
            <div class="filter-divider"></div>
            <div class="filter-group">
              <label class="filter-label">From</label>
              <input type="date" class="f-date" bind:value={dateFrom} on:change={reload} />
            </div>
            <div class="filter-group">
              <label class="filter-label">To</label>
              <input type="date" class="f-date" bind:value={dateTo} on:change={reload} />
            </div>
          {/if}

          {#if geoLoading}
            <div class="spinner" style="width:16px;height:16px;border-width:2px;align-self:flex-end;margin-bottom:6px"></div>
          {/if}
        </div>
      </div>
    </div>
  {/if}

  <!-- ── Loading ────────────────────────────────────────────────────────────── -->
  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Loading report…</p>
    </div>

  <!-- ── Report Content ─────────────────────────────────────────────────────── -->
  {:else if reportData}
    <div class="report-content stagger-row" style="animation-delay:50ms">

      <!-- INVENTORY -->
      {#if activeReport === 'inventory'}
        <div class="summary-cards">
          <div class="summary-card">
            <span class="card-label">Products</span>
            <span class="card-value">{filteredItems.length}</span>
          </div>
          <div class="summary-card success">
            <span class="card-label">Total Value</span>
            <span class="card-value">{fmt(filteredItems.reduce((s,i)=>s+(i.total_value??0),0))}</span>
          </div>
          <div class="summary-card">
            <span class="card-label">Total Units</span>
            <span class="card-value">{filteredItems.reduce((s,i)=>s+(i.quantity_in_stock??0),0).toLocaleString()}</span>
          </div>
        </div>
        <div class="table-wrapper">
          <table class="data-table">
            <thead><tr>
              <th>SKU</th><th>Name</th><th>Category</th><th>Supplier</th>
              <th class="tr">Qty</th><th class="tr">Reorder</th>
              <th class="tr">Unit Price</th><th class="tr">Total Value</th>
            </tr></thead>
            <tbody>
              {#each filteredItems as item}
                <tr>
                  <td><code>{item.sku}</code></td>
                  <td>{item.name}</td>
                  <td><span class="cat-pill">{item.category||'—'}</span></td>
                  <td style="color:var(--text-secondary)">{item.supplier||'—'}</td>
                  <td class="tr fw">{item.quantity_in_stock}</td>
                  <td class="tr" style="color:var(--text-muted)">{item.reorder_level}</td>
                  <td class="tr mono">{fmt(item.unit_price)}</td>
                  <td class="tr mono fw">{fmt(item.total_value)}</td>
                </tr>
              {/each}
              {#if filteredItems.length === 0}
                <tr><td colspan="8" class="empty">No products found for these filters</td></tr>
              {/if}
            </tbody>
          </table>
        </div>

      <!-- LOW STOCK -->
      {:else if activeReport === 'low-stock'}
        <div class="summary-cards">
          <div class="summary-card warning">
            <span class="card-label">Low / Out of Stock Items</span>
            <span class="card-value">{filteredItems.length}</span>
          </div>
          <div class="summary-card danger">
            <span class="card-label">Out of Stock</span>
            <span class="card-value">{filteredItems.filter(i=>i.quantity_in_stock===0).length}</span>
          </div>
        </div>
        <div class="table-wrapper">
          <table class="data-table">
            <thead><tr>
              <th>SKU</th><th>Name</th><th>Category</th><th>Supplier</th>
              <th class="tr">Stock</th><th class="tr">Reorder Level</th><th class="tr">Unit Price</th>
            </tr></thead>
            <tbody>
              {#each filteredItems as item}
                <tr class:danger-row={item.quantity_in_stock === 0}>
                  <td><code>{item.sku}</code></td>
                  <td>{item.name}</td>
                  <td><span class="cat-pill">{item.category||'—'}</span></td>
                  <td style="color:var(--text-secondary)">{item.supplier||'—'}</td>
                  <td class="tr">
                    <span class="qty-badge" class:out={item.quantity_in_stock===0} class:low={item.quantity_in_stock>0}>
                      {item.quantity_in_stock}
                    </span>
                  </td>
                  <td class="tr" style="color:var(--text-muted)">{item.reorder_level}</td>
                  <td class="tr mono">{fmt(item.unit_price)}</td>
                </tr>
              {/each}
              {#if filteredItems.length === 0}
                <tr><td colspan="7" class="empty">No low-stock items found 🎉</td></tr>
              {/if}
            </tbody>
          </table>
        </div>

      <!-- MOVEMENTS -->
      {:else if activeReport === 'movements'}
        <div class="summary-cards">
          <div class="summary-card">
            <span class="card-label">Total Movements</span>
            <span class="card-value">{filteredItems.length}</span>
          </div>
          <div class="summary-card success">
            <span class="card-label">Stock In</span>
            <span class="card-value">{filteredItems.filter(i=>i.movement_type==='in').length}</span>
          </div>
          <div class="summary-card danger">
            <span class="card-label">Stock Out</span>
            <span class="card-value">{filteredItems.filter(i=>i.movement_type==='out').length}</span>
          </div>
        </div>
        <div class="table-wrapper">
          <table class="data-table">
            <thead><tr>
              <th>Date</th><th>SKU</th><th>Product</th>
              <th>Type</th><th class="tr">Qty</th><th>Reference</th><th>Performed By</th>
            </tr></thead>
            <tbody>
              {#each filteredItems as item}
                <tr>
                  <td class="mono">{item.date}</td>
                  <td><code>{item.product_sku}</code></td>
                  <td>{item.product_name}</td>
                  <td><span class="type-badge type-{item.movement_type}">{item.movement_type}</span></td>
                  <td class="tr fw">{item.quantity}</td>
                  <td style="color:var(--text-muted)">{item.reference||'—'}</td>
                  <td style="color:var(--text-secondary)">{item.performed_by}</td>
                </tr>
              {/each}
              {#if filteredItems.length === 0}
                <tr><td colspan="7" class="empty">No movements found for these filters</td></tr>
              {/if}
            </tbody>
          </table>
        </div>

      <!-- VALUATION -->
      {:else if activeReport === 'valuation'}
        <div class="summary-cards">
          <div class="summary-card">
            <span class="card-label">Total Units</span>
            <span class="card-value">{filteredItems.reduce((s,i)=>s+(i.total_quantity??0),0).toLocaleString()}</span>
          </div>
          <div class="summary-card">
            <span class="card-label">Cost Value</span>
            <span class="card-value">{fmt(filteredItems.reduce((s,i)=>s+(i.total_cost_value??0),0))}</span>
          </div>
          <div class="summary-card success">
            <span class="card-label">Retail Value</span>
            <span class="card-value">{fmt(filteredItems.reduce((s,i)=>s+(i.total_retail_value??0),0))}</span>
          </div>
        </div>

        <!-- Bar chart -->
        <div class="bar-chart-card">
          <div class="bar-chart-title">Retail Value by Category</div>
          <div class="bar-rows">
            {#each filteredItems as item}
              {@const pct = maxVal > 0 ? (item.total_retail_value / maxVal * 100) : 0}
              {@const margin = item.total_retail_value > 0
                ? ((item.total_retail_value - item.total_cost_value) / item.total_retail_value * 100) : 0}
              <div class="bar-row">
                <div class="bar-label">{item.category || 'Uncategorized'}</div>
                <div class="bar-track">
                  <div class="bar-fill" style="width:{pct}%"></div>
                </div>
                <div class="bar-meta">
                  <span class="bar-value">{fmt(item.total_retail_value)}</span>
                  <span class="margin-badge" class:positive={margin>0}>{margin.toFixed(1)}%</span>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <div class="table-wrapper" style="margin-top:var(--space-4)">
          <table class="data-table">
            <thead><tr>
              <th>Category</th>
              <th class="tr">Total Qty</th>
              <th class="tr">Cost Value</th>
              <th class="tr">Retail Value</th>
              <th class="tr">Margin</th>
            </tr></thead>
            <tbody>
              {#each filteredItems as item}
                {@const margin = item.total_retail_value > 0
                  ? ((item.total_retail_value - item.total_cost_value) / item.total_retail_value * 100) : 0}
                <tr>
                  <td><span class="cat-pill">{item.category||'Uncategorized'}</span></td>
                  <td class="tr fw">{item.total_quantity?.toLocaleString()}</td>
                  <td class="tr mono">{fmt(item.total_cost_value)}</td>
                  <td class="tr mono fw">{fmt(item.total_retail_value)}</td>
                  <td class="tr">
                    <span class="margin-badge" class:positive={margin>0} class:negative={margin<0}>
                      {margin.toFixed(1)}%
                    </span>
                  </td>
                </tr>
              {/each}
              {#if filteredItems.length === 0}
                <tr><td colspan="5" class="empty">No valuation data found</td></tr>
              {/if}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  /* ── Report tabs ──────────────────────────────────────────────────────────── */
  .report-tabs {
    display: flex; gap: var(--space-2);
    margin-bottom: var(--space-4); flex-wrap: wrap;
  }
  .tab-btn {
    display: flex; align-items: center; gap: var(--space-2);
    padding: 8px 18px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    color: var(--text-secondary);
    cursor: pointer; font-size: 0.875rem; font-weight: 500;
    transition: all 0.2s;
  }
  .tab-btn:hover  { background: var(--glass-hover); color: var(--text-primary); }
  .tab-btn.active { background: var(--accent-primary); color: #fff; border-color: var(--accent-primary); }

  /* ── Filter panel ─────────────────────────────────────────────────────────── */
  .filter-panel {
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.08);
    border-radius: 14px; margin-bottom: var(--space-4); overflow: hidden;
  }
  .filter-panel-inner { padding: var(--space-4) var(--space-5); }
  .filter-header {
    display: flex; align-items: center; justify-content: space-between;
    margin-bottom: var(--space-3);
  }
  .filter-title {
    display: flex; align-items: center; gap: 6px;
    font-weight: 600; font-size: 0.9rem; color: var(--text-primary);
  }
  .filter-row { display: flex; align-items: flex-end; gap: var(--space-2); flex-wrap: wrap; }
  .filter-group { display: flex; flex-direction: column; gap: 6px; min-width: 160px; }
  .filter-label {
    font-size: 0.72rem; font-weight: 600; color: var(--text-muted);
    text-transform: uppercase; letter-spacing: 0.06em;
  }
  .sel-wrap { position: relative; }
  .f-select {
    width: 100%;
    background: rgba(255,255,255,0.05);
    border: 1px solid rgba(255,255,255,0.12);
    border-radius: 10px; color: var(--text-primary);
    padding: 8px 12px; font-size: 0.875rem;
    cursor: pointer; outline: none; transition: border-color 0.2s;
    appearance: none; -webkit-appearance: none;
  }
  .f-select:not(:disabled):hover { border-color: var(--accent-glow); }
  .f-select:disabled { opacity: 0.4; cursor: not-allowed; }
  .f-select:focus { border-color: var(--accent-glow); box-shadow: 0 0 0 3px rgba(139,92,246,0.18); }
  .f-date {
    background: rgba(255,255,255,0.05);
    border: 1px solid rgba(255,255,255,0.12);
    border-radius: 10px; color: var(--text-primary);
    padding: 8px 12px; font-size: 0.875rem;
    outline: none; transition: border-color 0.2s;
    color-scheme: dark;
  }
  .f-date:focus { border-color: var(--accent-glow); box-shadow: 0 0 0 3px rgba(139,92,246,0.18); }
  .filter-divider { width: 1px; background: rgba(255,255,255,0.1); align-self: stretch; margin: 0 4px; }
  .loc-arrow { color: var(--text-muted); font-size: 1.4rem; align-self: flex-end; padding-bottom: 8px; }

  /* ── Filter toggle button ─────────────────────────────────────────────────── */
  .filter-btn {
    display: flex; align-items: center; gap: 6px;
    border: 1px solid rgba(255,255,255,0.1); border-radius: 10px;
    padding: 7px 14px; font-size: 0.85rem; transition: all 0.2s;
  }
  .filter-btn.active {
    border-color: var(--accent-glow); color: var(--accent-glow);
    background: rgba(139,92,246,0.1);
  }
  .chevron { transition: transform 0.2s; }
  .chevron.rotated { transform: rotate(180deg); }

  /* ── Active filter chips ──────────────────────────────────────────────────── */
  .chip {
    display: inline-flex; align-items: center; gap: 5px;
    padding: 3px 10px; border-radius: 20px;
    font-size: 0.78rem; font-weight: 500;
  }
  .chip-region  { background: rgba(99,102,241,0.15); color: #818cf8; border: 1px solid rgba(99,102,241,0.3); }
  .chip-country { background: rgba(14,165,233,0.15); color: #38bdf8; border: 1px solid rgba(14,165,233,0.3); }
  .chip-city    { background: rgba(16,185,129,0.15); color: #34d399; border: 1px solid rgba(16,185,129,0.3); }
  .chip-cat     { background: rgba(245,158,11,0.15); color: #fbbf24; border: 1px solid rgba(245,158,11,0.3); }
  .chip-arrow   { color: var(--text-muted); }
  .chip-x {
    background: none; border: none; cursor: pointer; padding: 0;
    display: flex; align-items: center; color: inherit; opacity: 0.7;
    transition: opacity 0.2s;
  }
  .chip-x:hover { opacity: 1; }

  /* ── Loading ──────────────────────────────────────────────────────────────── */
  .loading-state {
    display: flex; flex-direction: column; align-items: center;
    justify-content: center; padding: var(--space-12);
    color: var(--text-secondary);
  }
  .spinner {
    width: 36px; height: 36px;
    border: 3px solid var(--border-color);
    border-top-color: var(--accent-primary);
    border-radius: 50%; animation: spin 0.8s linear infinite;
    margin-bottom: var(--space-4);
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  /* ── Summary cards ────────────────────────────────────────────────────────── */
  .summary-cards {
    display: flex; gap: var(--space-4);
    margin-bottom: var(--space-5); flex-wrap: wrap;
  }
  .summary-card {
    background: var(--bg-surface); border: 1px solid var(--border-color);
    border-radius: var(--radius-lg); padding: var(--space-4) var(--space-6);
    display: flex; flex-direction: column; gap: var(--space-1); min-width: 160px;
    transition: transform 0.2s;
  }
  .summary-card:hover { transform: translateY(-2px); }
  .summary-card.success { border-color: var(--color-success); background: rgba(16,185,129,0.08); }
  .summary-card.warning { border-color: var(--color-warning); background: rgba(245,158,11,0.08); }
  .summary-card.danger  { border-color: var(--color-danger);  background: rgba(239,68,68,0.08);  }
  .card-label { font-size: 0.8rem; color: var(--text-secondary); }
  .card-value { font-size: 1.5rem; font-weight: 700; color: var(--text-primary); }

  /* ── Table ────────────────────────────────────────────────────────────────── */
  .table-wrapper {
    overflow-x: auto; background: var(--bg-surface);
    border-radius: var(--radius-lg); border: 1px solid var(--border-color);
    max-height: calc(100vh - 420px); overflow-y: auto;
  }
  .data-table { width: 100%; border-collapse: collapse; }
  .data-table th, .data-table td { padding: 10px 14px; text-align: left; border-bottom: 1px solid var(--border-color); }
  .data-table th {
    background: var(--bg-base); font-weight: 600; color: var(--text-muted);
    font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.05em;
    position: sticky; top: 0; z-index: 1;
  }
  .data-table tbody tr:hover { background: var(--glass-table-hover); }
  .data-table tbody tr:last-child td { border-bottom: none; }
  .tr   { text-align: right; }
  .fw   { font-weight: 700; }
  .mono { font-family: var(--font-mono); font-size: 0.875rem; }
  .empty { text-align: center; color: var(--text-muted); padding: 3rem !important; }
  .danger-row { background: rgba(239,68,68,0.04); }

  /* ── Badges ───────────────────────────────────────────────────────────────── */
  .cat-pill {
    display: inline-block; padding: 2px 10px; border-radius: 20px;
    font-size: 0.78rem; font-weight: 500;
    background: rgba(139,92,246,0.12); color: var(--accent-glow);
    border: 1px solid rgba(139,92,246,0.25);
  }
  .qty-badge {
    display: inline-block; padding: 2px 10px; border-radius: 20px;
    font-size: 0.875rem; font-weight: 600;
  }
  .qty-badge.low { background: rgba(245,158,11,0.2); color: var(--color-warning); }
  .qty-badge.out { background: rgba(239,68,68,0.2);  color: var(--color-danger);  }
  .type-badge {
    display: inline-block; padding: 2px 10px; border-radius: 6px;
    font-size: 0.72rem; font-weight: 700; text-transform: uppercase;
  }
  .type-in          { background: rgba(16,185,129,0.15); color: var(--color-success); }
  .type-out         { background: rgba(239,68,68,0.15);  color: var(--color-danger);  }
  .type-adjustment  { background: rgba(59,130,246,0.15); color: #3b82f6; }
  .type-return      { background: rgba(168,85,247,0.15); color: #a855f7; }
  .margin-badge { font-weight: 600; font-size: 0.875rem; }
  .margin-badge.positive { color: var(--color-success); }
  .margin-badge.negative { color: var(--color-danger);  }

  code {
    background: rgba(139,92,246,0.12); color: var(--accent-glow);
    padding: 2px 8px; border-radius: 6px; font-size: 0.8rem;
    border: 1px solid rgba(139,92,246,0.25);
  }

  /* ── Bar chart ────────────────────────────────────────────────────────────── */
  .bar-chart-card {
    background: var(--bg-surface); border: 1px solid var(--border-color);
    border-radius: var(--radius-lg); padding: var(--space-5);
  }
  .bar-chart-title {
    font-weight: 600; font-size: 0.9rem; color: var(--text-primary);
    margin-bottom: var(--space-4);
  }
  .bar-rows { display: flex; flex-direction: column; gap: 14px; }
  .bar-row  { display: grid; grid-template-columns: 140px 1fr 200px; align-items: center; gap: 14px; }
  .bar-label { font-size: 0.875rem; color: var(--text-secondary); font-weight: 500; text-align: right; }
  .bar-track {
    height: 10px; background: rgba(255,255,255,0.06);
    border-radius: 10px; overflow: hidden;
  }
  .bar-fill {
    height: 100%; border-radius: 10px;
    background: linear-gradient(90deg, var(--accent-primary), var(--accent-glow));
    transition: width 0.6s cubic-bezier(0.34,1.56,0.64,1);
  }
  .bar-meta { display: flex; align-items: center; gap: 12px; justify-content: flex-end; }
  .bar-value { font-family: var(--font-mono); font-size: 0.875rem; font-weight: 600; color: var(--text-primary); }
</style>
