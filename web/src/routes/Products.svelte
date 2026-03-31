<script>
  /**
   * Products.svelte — Full product management page
   *
   * Features:
   *  - Paginated, searchable product table
   *  - Cascading Region → Country → City location filter
   *  - Supplier filter (linked from geography or standalone)
   *  - Add / Edit product modal with all fields
   *  - Soft-delete
   *  - Stock status badges (in-stock / low / out)
   */
  import { onMount, onDestroy } from 'svelte';
  import { api }   from '../lib/api.js';
  import { toast } from '../stores/toast.js';
  import { Pencil, Trash2, X, Search, XCircle, MapPin, ChevronDown } from 'lucide-svelte';

  // ── Props ────────────────────────────────────────────────────────────────────
  export let initialSupplierId = null;

  // ── State ────────────────────────────────────────────────────────────────────
  let products       = [];
  let categories     = [];
  let suppliers      = [];
  let total          = 0;
  let page           = 1;
  let perPage        = 15;
  let search         = '';
  let loading        = false;
  let showModal      = false;
  let editing        = null;

  // Location filter state
  let regions        = [];
  let countries      = [];
  let cities         = [];
  let selectedRegion  = '';
  let selectedCountry = '';
  let selectedCity    = '';
  let locationLoading = false;
  let showLocationPanel = false;

  // Supplier filter derived from geography or direct
  let supplierFilter = initialSupplierId;
  let filteredSupplierName = '';

  // Filtered suppliers (based on city selection)
  let citySuppliers  = [];   // suppliers in selected city
  $: displaySuppliers = selectedCity ? citySuppliers : suppliers;

  // Form fields
  let form = emptyForm();

  function emptyForm() {
    return {
      name: '', description: '', sku: '', barcode: '',
      category_id: '', supplier_id: '',
      unit_price: '', cost_price: '',
      quantity_in_stock: 0, reorder_level: 10,
      unit_of_measure: 'pcs',
    };
  }

  // ── Geography loading ─────────────────────────────────────────────────────────
  async function loadRegions() {
    try {
      regions = await api.geography.regions();
    } catch (e) { toast.error('Failed to load regions'); }
  }

  async function onRegionChange() {
    selectedCountry = '';
    selectedCity    = '';
    countries       = [];
    cities          = [];
    citySuppliers   = [];
    supplierFilter  = null;
    if (!selectedRegion) { page = 1; loadProducts(); return; }
    locationLoading = true;
    try {
      countries = await api.geography.countriesByRegion(selectedRegion);
    } catch (e) { toast.error('Failed to load countries'); }
    finally { locationLoading = false; }
    page = 1; loadProducts();
  }

  async function onCountryChange() {
    selectedCity   = '';
    cities         = [];
    citySuppliers  = [];
    supplierFilter = null;
    if (!selectedCountry) { page = 1; loadProducts(); return; }
    locationLoading = true;
    try {
      cities = await api.geography.citiesByCountry(selectedCountry);
    } catch (e) { toast.error('Failed to load cities'); }
    finally { locationLoading = false; }
    page = 1; loadProducts();
  }

  async function onCityChange() {
    supplierFilter = null;
    citySuppliers  = [];
    if (!selectedCity) { page = 1; loadProducts(); return; }
    locationLoading = true;
    try {
      // Get suppliers in this city from the city dashboard data
      const dash = await api.geography.cityDashboard(selectedCity);
      citySuppliers = dash.suppliers ?? [];
    } catch (e) {
      // Fallback: filter all suppliers by city_id
      citySuppliers = suppliers.filter(s => s.city_id === selectedCity);
    } finally { locationLoading = false; }
    page = 1; loadProducts();
  }

  function clearLocation() {
    selectedRegion  = '';
    selectedCountry = '';
    selectedCity    = '';
    countries       = [];
    cities          = [];
    citySuppliers   = [];
    supplierFilter  = null;
    filteredSupplierName = '';
    page = 1;
    loadProducts();
  }

  // ── Data fetching ────────────────────────────────────────────────────────────
  async function loadProducts() {
    loading = true;
    try {
      const params = { page, per_page: perPage, search };

      // Priority: explicit supplier filter > city filter
      if (supplierFilter) {
        params.supplier_id = supplierFilter;
      } else if (selectedCity && citySuppliers.length > 0) {
        // Filter by first supplier in city as a proxy (API only supports one supplier_id)
        // We'll do client-side filtering across all city suppliers
        const citySupplierIds = citySuppliers.map(s => s.id);
        const allResults = await Promise.all(
          citySupplierIds.map(sid => 
            api.products.list({ page: 1, per_page: 100, supplier_id: sid, search })
              .then(r => r.data ?? [])
              .catch(() => [])
          )
        );
        products = allResults.flat();
        total = products.length;
        loading = false;
        return;
      } else if (selectedCity && citySuppliers.length === 0) {
        // City has no suppliers → no products
        products = [];
        total = 0;
        loading = false;
        return;
      }

      const r = await api.products.list(params);
      products = r.data ?? [];
      total    = r.total ?? 0;
    } catch (e) {
      toast.error(e.message);
    } finally { loading = false; }
  }

  async function loadMeta() {
    const [cats, sups] = await Promise.all([api.categories.list(), api.suppliers.list()]);
    categories = cats;
    suppliers  = sups;

    if (supplierFilter && sups.length > 0) {
      const supplier = sups.find(s => s.id === supplierFilter);
      if (supplier) filteredSupplierName = supplier.name;
    }
  }

  onMount(() => {
    loadProducts();
    loadMeta();
    loadRegions();
  });

  onDestroy(() => { clearTimeout(searchTimer); });

  function clearSupplierFilter() {
    supplierFilter = null;
    filteredSupplierName = '';
    page = 1;
    loadProducts();
  }

  // ── Search debounce ──────────────────────────────────────────────────────────
  let searchTimer;
  function onSearch() {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(() => { page = 1; loadProducts(); }, 350);
  }

  // ── Computed label helpers ────────────────────────────────────────────────────
  $: regionLabel  = regions.find(r => r.id === selectedRegion)?.name ?? '';
  $: countryLabel = countries.find(c => c.id === selectedCountry)?.name ?? '';
  $: cityLabel    = cities.find(c => c.id === selectedCity)?.name ?? '';
  $: hasLocationFilter = !!(selectedRegion || selectedCountry || selectedCity);

  // ── Modal ────────────────────────────────────────────────────────────────────
  function openAdd() {
    editing = null; form = emptyForm(); showModal = true;
  }
  function openEdit(p) {
    editing = p;
    form = {
      name: p.name, description: p.description ?? '', sku: p.sku,
      barcode: p.barcode ?? '', category_id: p.category_id ?? '',
      supplier_id: p.supplier_id ?? '',
      unit_price: p.unit_price, cost_price: p.cost_price,
      quantity_in_stock: p.quantity_in_stock, reorder_level: p.reorder_level,
      unit_of_measure: p.unit_of_measure,
    };
    showModal = true;
  }
  function closeModal() { showModal = false; }

  async function saveProduct() {
    try {
      const payload = {
        ...form,
        unit_price: parseFloat(form.unit_price),
        cost_price: parseFloat(form.cost_price),
        quantity_in_stock: parseInt(form.quantity_in_stock),
        reorder_level:     parseInt(form.reorder_level),
        category_id: form.category_id || null,
        supplier_id: form.supplier_id || null,
        barcode:     form.barcode || null,
      };

      if (editing) {
        await api.products.update(editing.id, payload);
        toast.success('Product updated successfully');
      } else {
        await api.products.create(payload);
        toast.success('Product created successfully');
      }
      closeModal(); loadProducts();
    } catch (e) {
      toast.error(e.message);
    }
  }

  async function deleteProduct(p) {
    if (!confirm(`Delete "${p.name}"?`)) return;
    try {
      await api.products.delete(p.id);
      toast.success('Product deleted');
      loadProducts();
    } catch (e) { toast.error(e.message); }
  }

  // ── Helpers ──────────────────────────────────────────────────────────────────
  function stockBadge(p) {
    if (p.quantity_in_stock === 0)              return { cls: 'badge-red',   label: 'Out of Stock' };
    if (p.quantity_in_stock <= p.reorder_level) return { cls: 'badge-amber', label: 'Low Stock'    };
    return                                             { cls: 'badge-green', label: 'In Stock'     };
  }

  function currency(n) {
    return new Intl.NumberFormat('en-IN', { style:'currency', currency:'INR', maximumFractionDigits:0 }).format(n);
  }

  $: totalPages = Math.ceil(total / perPage);
</script>

<!-- ── Page ──────────────────────────────────────────────────────────────────── -->
<div class="page">
  <div class="page-header">
    <div class="page-title-group">
      <h1 class="page-title">Products</h1>
      <div class="flex gap-2 items-center flex-wrap">
        <p class="page-subtitle">{total} total products</p>

        <!-- Active location chips -->
        {#if regionLabel}
          <span class="filter-chip chip-region">
            <MapPin size={11} />
            {regionLabel}
            {#if !countryLabel}
              <button class="chip-clear" on:click={clearLocation} title="Clear"><XCircle size={12}/></button>
            {/if}
          </span>
        {/if}
        {#if countryLabel}
          <span class="chip-arrow">›</span>
          <span class="filter-chip chip-country">
            {countryLabel}
            {#if !cityLabel}
              <button class="chip-clear" on:click={clearLocation} title="Clear"><XCircle size={12}/></button>
            {/if}
          </span>
        {/if}
        {#if cityLabel}
          <span class="chip-arrow">›</span>
          <span class="filter-chip chip-city">
            {cityLabel}
            <button class="chip-clear" on:click={clearLocation} title="Clear"><XCircle size={12}/></button>
          </span>
        {/if}

        {#if supplierFilter && filteredSupplierName}
          <span class="filter-chip chip-supplier">
            Supplier: {filteredSupplierName}
            <button class="chip-clear" on:click={clearSupplierFilter} title="Clear"><XCircle size={12}/></button>
          </span>
        {/if}
      </div>
    </div>

    <div class="flex gap-3 items-center">
      <!-- Location filter toggle button -->
      <button
        id="btn-location-filter"
        class="btn btn-ghost location-btn"
        class:active={hasLocationFilter}
        on:click={() => showLocationPanel = !showLocationPanel}
      >
        <MapPin size={15} />
        {hasLocationFilter ? 'Location: Active' : 'Filter by Location'}
        <span class="chevron" class:rotated={showLocationPanel} style="display:flex;align-items:center">
          <ChevronDown size={14} />
        </span>
      </button>

      <!-- Search -->
      <div class="search-bar" style="width:220px">
        <Search size={16} />
        <input
          id="product-search"
          type="text"
          placeholder="Search name / SKU…"
          bind:value={search}
          on:input={onSearch}
        />
      </div>
      <button id="btn-add-product" class="btn btn-primary" on:click={openAdd}>+ Add Product</button>
    </div>
  </div>

  <!-- ── Location Filter Panel ────────────────────────────────────────────────── -->
  {#if showLocationPanel}
    <div class="location-panel stagger-row" style="animation-delay:0ms">
      <div class="location-panel-inner">
        <div class="location-header">
          <div class="flex items-center gap-2">
            <MapPin size={16} style="color:var(--accent-glow)" />
            <span class="location-title">Filter by Region / Location</span>
            {#if locationLoading}
              <div class="spinner" style="width:14px;height:14px;border-width:2px"></div>
            {/if}
          </div>
          {#if hasLocationFilter}
            <button class="btn btn-ghost btn-sm" on:click={clearLocation}>Clear All</button>
          {/if}
        </div>

        <div class="location-selects">
          <!-- Region -->
          <div class="loc-select-group">
            <label class="loc-label">Region</label>
            <div class="select-wrapper">
              <select
                id="filter-region"
                class="loc-select"
                bind:value={selectedRegion}
                on:change={onRegionChange}
              >
                <option value="">All Regions</option>
                {#each regions as r}
                  <option value={r.id}>{r.name}</option>
                {/each}
              </select>
            </div>
          </div>

          <!-- Arrow -->
          <span class="loc-arrow">›</span>

          <!-- Country -->
          <div class="loc-select-group">
            <label class="loc-label">Country</label>
            <div class="select-wrapper">
              <select
                id="filter-country"
                class="loc-select"
                bind:value={selectedCountry}
                on:change={onCountryChange}
                disabled={!selectedRegion || countries.length === 0}
              >
                <option value="">All Countries</option>
                {#each countries as c}
                  <option value={c.id}>{c.name}</option>
                {/each}
              </select>
            </div>
          </div>

          <!-- Arrow -->
          <span class="loc-arrow">›</span>

          <!-- City -->
          <div class="loc-select-group">
            <label class="loc-label">City</label>
            <div class="select-wrapper">
              <select
                id="filter-city"
                class="loc-select"
                bind:value={selectedCity}
                on:change={onCityChange}
                disabled={!selectedCountry || cities.length === 0}
              >
                <option value="">All Cities</option>
                {#each cities as c}
                  <option value={c.id}>{c.name}</option>
                {/each}
              </select>
            </div>
          </div>

          <!-- City supplier count -->
          {#if selectedCity}
            <div class="city-info">
              <span class="city-stat">
                <strong>{citySuppliers.length}</strong> supplier{citySuppliers.length !== 1 ? 's' : ''} in this city
              </span>
              {#if citySuppliers.length > 0}
                <div class="city-supplier-list">
                  {#each citySuppliers.slice(0, 3) as sup}
                    <button
                      class="city-supplier-pill"
                      class:active={supplierFilter === sup.id}
                      on:click={() => {
                        supplierFilter = supplierFilter === sup.id ? null : sup.id;
                        filteredSupplierName = sup.name;
                        page = 1;
                        loadProducts();
                      }}
                    >{sup.name}</button>
                  {/each}
                  {#if citySuppliers.length > 3}
                    <span class="city-supplier-more">+{citySuppliers.length - 3} more</span>
                  {/if}
                </div>
              {/if}
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}

  <!-- ── Table card ──────────────────────────────────────────────────────────── -->
  <div class="card stagger-row" style="padding:0;overflow:hidden;flex:1;animation-delay: 50ms">
    {#if loading}
      <div style="display:flex;justify-content:center;padding:4rem">
        <div class="spinner" style="width:32px;height:32px;border-width:3px"></div>
      </div>
    {:else}
      <div class="table-wrapper" style="border-radius:0;border:none;max-height:calc(100vh - 320px)">
        <table>
          <thead>
            <tr>
              <th>Product</th>
              <th>SKU</th>
              <th>Category</th>
              <th>Supplier</th>
              <th>Price</th>
              <th>Stock</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each products as p, i (p.id)}
              {@const badge = stockBadge(p)}
              <tr class="stagger-row" style="animation-delay: {100 + (i * 50)}ms">
                <td>
                  <div style="font-weight:600;max-width:180px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{p.name}</div>
                </td>
                <td><code class="sku">{p.sku}</code></td>
                <td style="color:var(--text-secondary)">{p.category_name ?? '—'}</td>
                <td style="color:var(--text-secondary)">{p.supplier_name ?? '—'}</td>
                <td style="font-family:var(--font-mono);font-weight:600">{currency(p.unit_price)}</td>
                <td style="font-family:var(--font-mono);text-align:center;font-weight:700;color:{p.quantity_in_stock===0?'var(--accent-red)':p.quantity_in_stock<=p.reorder_level?'var(--accent-amber)':'var(--text-primary)'}">{p.quantity_in_stock}</td>
                <td><span class="badge {badge.cls}">{badge.label}</span></td>
                <td>
                  <div class="flex gap-2">
                    <button class="btn btn-ghost btn-sm btn-icon" on:click={() => openEdit(p)} title="Edit">
                      <Pencil size={14} />
                    </button>
                    <button class="btn btn-danger btn-sm btn-icon" on:click={() => deleteProduct(p)} title="Delete">
                      <Trash2 size={14} />
                    </button>
                  </div>
                </td>
              </tr>
            {/each}
            {#if products.length === 0}
              <tr><td colspan="8" style="text-align:center;color:var(--text-muted);padding:3rem">
                {selectedCity && citySuppliers.length === 0 ? 'No suppliers in this city' : 'No products found'}
              </td></tr>
            {/if}
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      {#if totalPages > 1}
        <div class="pagination" style="padding-bottom:var(--space-4)">
          <button class="page-btn" on:click={() => { page--; loadProducts(); }} disabled={page === 1}>‹</button>
          {#each Array(Math.min(totalPages, 7)) as _, i}
            {@const pg = i + 1}
            <button class="page-btn" class:active={pg === page} on:click={() => { page = pg; loadProducts(); }}>{pg}</button>
          {/each}
          <button class="page-btn" on:click={() => { page++; loadProducts(); }} disabled={page >= totalPages}>›</button>
        </div>
      {/if}
    {/if}
  </div>
</div>

<!-- ── Add / Edit Modal ──────────────────────────────────────────────────────── -->
{#if showModal}
  <div class="modal-backdrop" on:click|self={closeModal}>
    <div class="modal">
      <div class="modal-header">
        <h3>{editing ? 'Edit Product' : 'Add New Product'}</h3>
        <button class="btn btn-ghost btn-icon" on:click={closeModal}>
          <X size={18} />
        </button>
      </div>
      <div class="modal-body">
        <div class="grid grid-2 grid-gap-4">
          <div class="form-group" style="grid-column:span 2">
            <label class="label" for="p-name">Product Name *</label>
            <input id="p-name" class="input" bind:value={form.name} placeholder="e.g. Dell Laptop Inspiron" />
          </div>
          <div class="form-group">
            <label class="label" for="p-sku">SKU *</label>
            <input id="p-sku" class="input" bind:value={form.sku} placeholder="ELEC-LAP-001" disabled={!!editing} />
          </div>
          <div class="form-group">
            <label class="label" for="p-barcode">Barcode</label>
            <input id="p-barcode" class="input" bind:value={form.barcode} placeholder="EAN-13 / QR" />
          </div>
          <div class="form-group">
            <label class="label" for="p-category">Category</label>
            <select id="p-category" class="select input" bind:value={form.category_id}>
              <option value="">— None —</option>
              {#each categories as c}<option value={c.id}>{c.name}</option>{/each}
            </select>
          </div>
          <div class="form-group">
            <label class="label" for="p-supplier">Supplier</label>
            <select id="p-supplier" class="select input" bind:value={form.supplier_id}>
              <option value="">— None —</option>
              {#each displaySuppliers as s}<option value={s.id}>{s.name}</option>{/each}
            </select>
          </div>
          <div class="form-group">
            <label class="label" for="p-unit-price">Unit Price (₹) *</label>
            <input id="p-unit-price" class="input" type="number" min="0" step="0.01" bind:value={form.unit_price} />
          </div>
          <div class="form-group">
            <label class="label" for="p-cost-price">Cost Price (₹) *</label>
            <input id="p-cost-price" class="input" type="number" min="0" step="0.01" bind:value={form.cost_price} />
          </div>
          {#if !editing}
          <div class="form-group">
            <label class="label" for="p-qty">Initial Stock Qty</label>
            <input id="p-qty" class="input" type="number" min="0" bind:value={form.quantity_in_stock} />
          </div>
          {/if}
          <div class="form-group">
            <label class="label" for="p-reorder">Reorder Level</label>
            <input id="p-reorder" class="input" type="number" min="0" bind:value={form.reorder_level} />
          </div>
          <div class="form-group">
            <label class="label" for="p-uom">Unit of Measure</label>
            <input id="p-uom" class="input" bind:value={form.unit_of_measure} placeholder="pcs / kg / ltr" />
          </div>
          <div class="form-group" style="grid-column:span 2">
            <label class="label" for="p-desc">Description</label>
            <textarea id="p-desc" class="textarea input" rows="2" bind:value={form.description} placeholder="Optional description…"></textarea>
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" on:click={closeModal}>Cancel</button>
        <button id="btn-save-product" class="btn btn-primary" on:click={saveProduct}>
          {editing ? 'Save Changes' : 'Create Product'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .sku {
    font-family: var(--font-mono); font-size: 0.8rem;
    background: rgba(139, 92, 246, 0.15);
    padding: 3px 8px; border-radius: 6px;
    color: var(--accent-glow);
    border: 1px solid rgba(139, 92, 246, 0.3);
  }

  /* ── Location Panel ─────────────────────────────────────── */
  .location-panel {
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.08);
    border-radius: 14px;
    margin-bottom: var(--space-4);
    overflow: hidden;
  }
  .location-panel-inner { padding: var(--space-4) var(--space-5); }
  .location-header {
    display: flex; align-items: center; justify-content: space-between;
    margin-bottom: var(--space-3);
  }
  .location-title { font-weight: 600; font-size: 0.9rem; color: var(--text-primary); }
  .location-selects {
    display: flex; align-items: flex-end; gap: var(--space-2); flex-wrap: wrap;
  }
  .loc-select-group { display: flex; flex-direction: column; gap: 6px; min-width: 170px; }
  .loc-label { font-size: 0.75rem; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; }
  .select-wrapper { position: relative; }
  .loc-select {
    width: 100%;
    background: rgba(255,255,255,0.05);
    border: 1px solid rgba(255,255,255,0.12);
    border-radius: 10px;
    color: var(--text-primary);
    padding: 8px 12px;
    font-size: 0.875rem;
    cursor: pointer;
    outline: none;
    transition: border-color 0.2s, background 0.2s;
    appearance: none;
    -webkit-appearance: none;
  }
  .loc-select:not(:disabled):hover { border-color: var(--accent-glow); background: rgba(139,92,246,0.08); }
  .loc-select:disabled { opacity: 0.4; cursor: not-allowed; }
  .loc-select:focus { border-color: var(--accent-glow); box-shadow: 0 0 0 3px rgba(139,92,246,0.18); }
  .loc-arrow { color: var(--text-muted); font-size: 1.5rem; padding-bottom: 6px; }

  /* City info box */
  .city-info {
    background: rgba(139,92,246,0.06);
    border: 1px solid rgba(139,92,246,0.2);
    border-radius: 10px;
    padding: 10px 14px;
    min-width: 180px;
  }
  .city-stat { font-size: 0.82rem; color: var(--text-secondary); display: block; margin-bottom: 8px; }
  .city-stat strong { color: var(--accent-glow); }
  .city-supplier-list { display: flex; gap: 6px; flex-wrap: wrap; }
  .city-supplier-pill {
    background: rgba(139,92,246,0.12);
    border: 1px solid rgba(139,92,246,0.25);
    border-radius: 20px;
    padding: 3px 10px;
    font-size: 0.78rem;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
  }
  .city-supplier-pill:hover { background: rgba(139,92,246,0.25); color: var(--text-primary); }
  .city-supplier-pill.active { background: var(--accent-glow); color: #fff; border-color: transparent; }
  .city-supplier-more { font-size: 0.78rem; color: var(--text-muted); align-self: center; }

  /* ── Location toggle button ─────────────────────────────── */
  .location-btn {
    display: flex; align-items: center; gap: 6px;
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 10px;
    padding: 7px 14px;
    font-size: 0.85rem;
    transition: all 0.2s;
  }
  .location-btn.active {
    border-color: var(--accent-glow);
    color: var(--accent-glow);
    background: rgba(139,92,246,0.1);
  }
  .chevron { transition: transform 0.2s; }
  .chevron.rotated { transform: rotate(180deg); }

  /* ── Filter chips ───────────────────────────────────────── */
  .filter-chip {
    display: inline-flex; align-items: center; gap: 5px;
    padding: 3px 10px; border-radius: 20px;
    font-size: 0.8rem; font-weight: 500;
  }
  .chip-region  { background: rgba(99,102,241,0.15); color: #818cf8; border: 1px solid rgba(99,102,241,0.3); }
  .chip-country { background: rgba(14,165,233,0.15); color: #38bdf8; border: 1px solid rgba(14,165,233,0.3); }
  .chip-city    { background: rgba(16,185,129,0.15); color: #34d399; border: 1px solid rgba(16,185,129,0.3); }
  .chip-supplier{ background: rgba(102,126,234,0.15); color: #667eea; border: 1px solid rgba(102,126,234,0.3); }
  .chip-arrow   { color: var(--text-muted); font-size: 1rem; }

  .chip-clear {
    background: none; border: none; cursor: pointer; padding: 0;
    display: flex; align-items: center; opacity: 0.7; transition: opacity 0.2s;
    color: inherit;
  }
  .chip-clear:hover { opacity: 1; }
</style>
