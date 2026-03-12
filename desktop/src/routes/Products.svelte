<script>
  /**
   * Products.svelte — Full product management page
   *
   * Features:
   *  - Paginated, searchable product table
   *  - Add / Edit product modal with all fields
   *  - Soft-delete
   *  - Stock status badges (in-stock / low / out)
   */
  import { onMount } from 'svelte';
  import { api }   from '../lib/api.js';
  import { toast } from '../stores/toast.js';

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
  let editing        = null;   // null = new product

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

  // ── Data fetching ────────────────────────────────────────────────────────────
  async function loadProducts() {
    loading = true;
    try {
      const r = await api.products.list({ page, per_page: perPage, search });
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
  }

  onMount(() => { loadProducts(); loadMeta(); });

  // ── Search debounce ──────────────────────────────────────────────────────────
  let searchTimer;
  function onSearch() {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(() => { page = 1; loadProducts(); }, 350);
  }

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
    if (p.quantity_in_stock === 0)                           return { cls: 'badge-red',   label: 'Out of Stock' };
    if (p.quantity_in_stock <= p.reorder_level)              return { cls: 'badge-amber', label: 'Low Stock'    };
    return                                                          { cls: 'badge-green', label: 'In Stock'     };
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
      <p class="page-subtitle">{total} total products</p>
    </div>
    <div class="flex gap-3">
      <!-- Search -->
      <div class="search-bar" style="width:220px">
        <span>🔍</span>
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

  <!-- ── Table card ──────────────────────────────────────────────────────────── -->
  <div class="card" style="padding:0;overflow:hidden;flex:1">
    {#if loading}
      <div style="display:flex;justify-content:center;padding:4rem">
        <div class="spinner" style="width:32px;height:32px;border-width:3px"></div>
      </div>
    {:else}
      <div class="table-wrapper" style="border-radius:0;border:none;max-height:calc(100vh - 260px)">
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
            {#each products as p (p.id)}
              {@const badge = stockBadge(p)}
              <tr>
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
                    <button class="btn btn-ghost btn-sm btn-icon" on:click={() => openEdit(p)} title="Edit">✏️</button>
                    <button class="btn btn-danger btn-sm btn-icon" on:click={() => deleteProduct(p)} title="Delete">🗑️</button>
                  </div>
                </td>
              </tr>
            {/each}
            {#if products.length === 0}
              <tr><td colspan="8" style="text-align:center;color:var(--text-muted);padding:3rem">No products found</td></tr>
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
        <button class="btn btn-ghost btn-icon" on:click={closeModal}>✕</button>
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
              {#each suppliers as s}<option value={s.id}>{s.name}</option>{/each}
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
    background: rgba(124,58,237,0.1);
    padding: 2px 6px; border-radius: 4px;
    color: var(--accent-glow);
  }
</style>
