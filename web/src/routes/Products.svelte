<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { api } from '../lib/api.js';
  import { toastStore } from '../stores/toast.js';

  let products = [];
  let categories = [];
  let regions = [];
  let countries = [];
  let suppliers = [];
  let loading = true;
  let searchQuery = '';
  let showModal = false;
  let editingProduct = null;
  let tableBody;

  let selectedRegion = 'Asia';
  let selectedCountry = '';
  let selectedCategory = 'Electronics';
  
  let form = {
    sku: '',
    name: '',
    category_id: '',
    barcode: '',
    low_stock_threshold: 10,
    unit_price: 0,
    cost_price: 0,
    supplier_id: '',
    current_stock: 0
  };

  onMount(async () => {
    await Promise.all([loadFilters(), loadProducts()]);
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
          const supRes = await api.suppliers.list({ country: 'India' });
          suppliers = supRes.data || supRes;
        }
      }

      const electronics = categories.find(c => c.name === 'Electronics');
      if (electronics) {
        selectedCategory = electronics.name;
      }
    } catch (e) {
      console.error('Failed to load filters', e);
    }
  }

  async function loadProducts() {
    loading = true;
    try {
      const params = { region: selectedRegion };
      if (selectedCountry && selectedCountry !== '') params.country = selectedCountry;
      if (selectedCategory && selectedCategory !== '') params.category = selectedCategory;
      
      const result = await api.products.list(params);
      products = result.data || result;
    } catch (e) {
      toastStore.show('Failed to load products', 'error');
    } finally {
      loading = false;
    }
  }

  function handleFilterChange() {
    loadProducts();
    loadSuppliersByCountry();
  }

  async function loadSuppliersByCountry() {
    try {
      if (selectedCountry) {
        const supRes = await api.suppliers.list({ country: selectedCountry });
        suppliers = supRes.data || supRes;
      } else {
        const supRes = await api.suppliers.list({});
        suppliers = supRes.data || supRes;
      }
    } catch (e) {
      console.error('Failed to load suppliers', e);
    }
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
    loadProducts();
  }

  function animateEntrance() {
    const tl = gsap.timeline();

    tl.fromTo('.page-header', 
      { y: -20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' }
    );

    tl.fromTo('.filter-bar', 
      { y: -10, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.4, ease: 'power3.out' },
      '-=0.3'
    );

    tl.fromTo('.table-container', 
      { y: 30, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' },
      '-=0.3'
    );
  }

  function openModal(product = null) {
    editingProduct = product;
    if (product) {
      form = { 
        sku: product.sku || '',
        name: product.name || '',
        category_id: product.category_id || '',
        barcode: product.barcode || '',
        low_stock_threshold: product.reorder_level || 10,
        unit_price: product.unit_price || 0,
        cost_price: product.cost_price || 0,
        supplier_id: product.supplier_id || '',
        current_stock: product.quantity_in_stock || 0
      };
    } else {
      form = {
        sku: '',
        name: '',
        category_id: '',
        barcode: '',
        low_stock_threshold: 10,
        unit_price: 0,
        cost_price: 0,
        supplier_id: '',
        current_stock: 0
      };
    }
    showModal = true;
    setTimeout(() => {
      gsap.fromTo('.modal-content', 
        { scale: 0.9, opacity: 0, y: 20 },
        { scale: 1, opacity: 1, y: 0, duration: 0.3, ease: 'back.out(1.5)' }
      );
    }, 0);
  }

  function closeModal() {
    gsap.to('.modal-content', {
      scale: 0.9,
      opacity: 0,
      duration: 0.2,
      onComplete: () => {
        showModal = false;
        editingProduct = null;
      }
    });
  }

  async function saveProduct() {
    try {
      if (editingProduct) {
        await api.products.update(editingProduct.id, form);
        toastStore.show('Product updated successfully', 'success');
      } else {
        await api.products.create(form);
        toastStore.show('Product created successfully', 'success');
      }
      await loadProducts();
      closeModal();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }

  async function deleteProduct(id) {
    if (!confirm('Are you sure you want to delete this product?')) return;
    
    try {
      await api.products.delete(id);
      toastStore.show('Product deleted', 'success');
      await loadProducts();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }

  function animateList() {
    gsap.fromTo('.product-row', 
      { opacity: 0, x: -20 },
      { opacity: 1, x: 0, duration: 0.3, stagger: 0.05, delay: 0.2 }
    );
  }

  $: filteredProducts = products.filter(p => {
    const q = searchQuery.toLowerCase();
    return (p.name || '').toLowerCase().includes(q) ||
           (p.sku || '').toLowerCase().includes(q) ||
           (p.barcode || '').toLowerCase().includes(q);
  });
</script>

<div class="products-page">
  <div class="page-header">
    <div class="header-left">
      <div class="search-box">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <path d="M21 21l-4.35-4.35"/>
        </svg>
        <input 
          type="text" 
          placeholder="Search products..." 
          bind:value={searchQuery}
          class="search-input"
        />
      </div>
    </div>
    <div class="header-right">
      <button class="btn btn-primary" on:click={() => openModal()}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 5v14M5 12h14"/>
        </svg>
        Add Product
      </button>
    </div>
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
    <div class="filter-stats">
      <span class="stat-item">
        <strong>{products.length}</strong> products
      </span>
    </div>
  </div>

  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th>SKU</th>
          <th>Name</th>
          <th>Category</th>
          <th>Stock</th>
          <th>Price</th>
          <th>Supplier</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#if loading}
          <tr>
            <td colspan="7" class="loading-cell">
              <div class="spinner"></div>
              Loading products...
            </td>
          </tr>
        {:else if filteredProducts.length === 0}
          <tr>
            <td colspan="7" class="empty-cell">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4a2 2 0 00-1 1.73v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4a2 2 0 001-1.73z"/>
              </svg>
              <span>No products found</span>
            </td>
          </tr>
        {:else}
          {#each filteredProducts as product, i (product.id)}
            <tr class="product-row" style="animation-delay: {i * 0.03}s">
              <td>
                <span class="sku-badge">{product.sku}</span>
              </td>
              <td>
                <div class="product-name">
                  <span class="name">{product.name}</span>
                  {#if product.barcode}
                    <span class="barcode">{product.barcode}</span>
                  {/if}
                </div>
              </td>
              <td>
                <span class="category-badge">{product.category_name || '-'}</span>
              </td>
              <td>
                <div class="stock-cell">
                  <span class="stock-qty" class:low-stock={product.quantity_in_stock <= product.reorder_level}>
                    {product.quantity_in_stock || 0}
                  </span>
                  {#if product.quantity_in_stock <= product.reorder_level}
                    <span class="low-stock-badge">Low</span>
                  {/if}
                </div>
              </td>
              <td>
                <span class="price">₹{(product.unit_price || 0).toLocaleString()}</span>
              </td>
              <td>
                <span class="supplier-name">{product.supplier_name || '-'}</span>
              </td>
              <td>
                <div class="actions">
                  <button class="action-btn" on:click={() => openModal(product)} title="Edit">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                      <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
                    </svg>
                  </button>
                  <button class="action-btn danger" on:click={() => deleteProduct(product.id)} title="Delete">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>

{#if showModal}
  <div class="modal-overlay" on:click={closeModal}>
    <div class="modal-content" on:click|stopPropagation>
      <div class="modal-header">
        <h2 class="modal-title">{editingProduct ? 'Edit Product' : 'Add Product'}</h2>
        <button class="modal-close" on:click={closeModal}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>

      <form on:submit|preventDefault={saveProduct}>
        <div class="form-grid">
          <div class="form-group">
            <label class="form-label">SKU</label>
            <input type="text" class="input-field" bind:value={form.sku} required />
          </div>
          <div class="form-group">
            <label class="form-label">Barcode</label>
            <input type="text" class="input-field" bind:value={form.barcode} />
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">Product Name</label>
          <input type="text" class="input-field" bind:value={form.name} required />
        </div>

        <div class="form-grid">
          <div class="form-group">
            <label class="form-label">Category</label>
            <select class="input-field" bind:value={form.category_id}>
              <option value="">Select category</option>
              {#each categories as cat}
                <option value={cat.id}>{cat.name}</option>
              {/each}
            </select>
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">Supplier</label>
          <select class="input-field" bind:value={form.supplier_id}>
            <option value="">Select supplier</option>
            {#each suppliers as sup}
              <option value={sup.id}>{sup.name}</option>
            {/each}
          </select>
        </div>

        <div class="form-grid">
          <div class="form-group">
            <label class="form-label">Cost Price (₹)</label>
            <input type="number" class="input-field" bind:value={form.cost_price} min="0" step="0.01" />
          </div>
          <div class="form-group">
            <label class="form-label">Unit Price (₹)</label>
            <input type="number" class="input-field" bind:value={form.unit_price} min="0" step="0.01" />
          </div>
        </div>

        <div class="form-grid">
          <div class="form-group">
            <label class="form-label">Current Stock</label>
            <input type="number" class="input-field" bind:value={form.current_stock} min="0" />
          </div>
          <div class="form-group">
            <label class="form-label">Low Stock Threshold</label>
            <input type="number" class="input-field" bind:value={form.low_stock_threshold} min="0" />
          </div>
        </div>

        <div class="modal-actions">
          <button type="button" class="btn btn-secondary" on:click={closeModal}>Cancel</button>
          <button type="submit" class="btn btn-primary">
            {editingProduct ? 'Update Product' : 'Create Product'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .products-page {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    min-width: 300px;
    transition: all var(--transition-base);
  }

  .search-box:focus-within {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
  }

  .search-box svg {
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    background: none;
    border: none;
    color: var(--text-primary);
    font-size: 0.95rem;
    outline: none;
  }

  .search-input::placeholder {
    color: var(--text-muted);
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
  }

  td {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .loading-cell, .empty-cell {
    text-align: center;
    padding: 48px 20px;
    color: var(--text-muted);
    display: flex;
    flex-direction: column;
    align-items: center;
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

  .product-name {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .product-name .name {
    font-weight: 500;
    color: var(--text-primary);
  }

  .product-name .barcode {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .category-badge {
    padding: 4px 10px;
    background: rgba(34, 211, 238, 0.15);
    color: var(--accent-secondary);
    border-radius: 100px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .stock-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .stock-qty {
    font-weight: 600;
  }

  .stock-qty.low-stock {
    color: var(--accent-warning);
  }

  .low-stock-badge {
    padding: 2px 8px;
    background: rgba(245, 158, 11, 0.15);
    color: var(--accent-warning);
    border-radius: 100px;
    font-size: 0.7rem;
    font-weight: 600;
  }

  .price {
    font-weight: 600;
    color: var(--text-primary);
  }

  .supplier-name {
    color: var(--text-secondary);
  }

  .actions {
    display: flex;
    gap: 8px;
  }

  .action-btn {
    padding: 8px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    transition: all var(--transition-fast);
  }

  .action-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .action-btn.danger:hover {
    background: rgba(239, 68, 68, 0.1);
    color: var(--accent-danger);
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 20px;
  }

  .modal-content {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-xl);
    padding: 32px;
    width: 100%;
    max-width: 560px;
    max-height: 90vh;
    overflow-y: auto;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 28px;
  }

  .modal-title {
    font-family: var(--font-display);
    font-size: 1.4rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .modal-close {
    padding: 8px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    transition: all var(--transition-fast);
  }

  .modal-close:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .form-group {
    margin-bottom: 20px;
  }

  .form-label {
    display: block;
    margin-bottom: 8px;
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 28px;
  }

  .product-row {
    animation: slideIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    opacity: 0;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(-20px);
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

  .filter-stats {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .stat-item {
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .stat-item strong {
    color: var(--accent-primary);
    font-weight: 700;
  }
</style>
