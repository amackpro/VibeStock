<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { api } from '../lib/api.js';
  import Globe3D from '../components/Globe3D.svelte';
  import { X, Package } from 'lucide-svelte';

  const dispatch = createEventDispatcher();

  let cities = [];
  let loading = true;
  let error = null;
  let selectedCity = null;
  let cityStats = null;
  let loadingStats = false;

  onMount(async () => {
    await loadCitiesWithInventory();
  });

  async function loadCitiesWithInventory() {
    try {
      loading = true;
      error = null;
      cities = await api.geography.citiesWithInventory();
      console.log(`Loaded ${cities.length} cities with inventory`);

      // If no cities with inventory, show helpful message
      if (cities.length === 0) {
        error = 'No suppliers have been assigned to cities yet. Please edit your suppliers to assign them to cities.';
      }
    } catch (err) {
      console.error('Failed to load cities:', err);
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function handleCityClick(event) {
    const city = event.detail.city;
    selectedCity = city;
    loadingStats = true;
    cityStats = null;

    try {
      cityStats = await api.geography.cityDashboard(city.id);
    } catch (err) {
      console.error('Failed to load city stats:', err);
      error = err.message;
    } finally {
      loadingStats = false;
    }
  }

  function closeModal() {
    selectedCity = null;
    cityStats = null;
  }

  function formatCurrency(value) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(value);
  }

  function viewSupplierProducts(supplier) {
    closeModal();
    dispatch('navigate', { page: 'products', supplierId: supplier.id });
  }
</script>

<div class="globe-container">
  <!-- Header Overlay -->
  <div class="header-overlay">
    <h1 class="title">VibeStock Global</h1>
    <p class="subtitle">
      {#if loading}
        Loading inventory locations...
      {:else if cities.length > 0}
        {cities.length} inventory location{cities.length === 1 ? '' : 's'} worldwide
      {:else}
        No inventory locations yet
      {/if}
    </p>
  </div>

  <!-- Globe Visualization -->
  {#if loading}
    <div class="loading-overlay">
      <div class="spinner"></div>
      <p>Loading globe...</p>
    </div>
  {:else if error && cities.length === 0}
    <div class="error-overlay">
      <p class="error-message">{error}</p>
      <button class="retry-btn" on:click={loadCitiesWithInventory}>Retry</button>
    </div>
  {:else}
    <Globe3D {cities} on:cityClick={handleCityClick} />
  {/if}

  <!-- City Dashboard Modal -->
  {#if selectedCity}
    <div class="modal-backdrop" on:click={closeModal}>
      <div class="modal-content" on:click|stopPropagation>
        <div class="modal-header">
          <div>
            <h2 class="modal-title">{selectedCity.name}</h2>
            <p class="modal-subtitle">{selectedCity.country_name}</p>
          </div>
          <button class="close-btn" on:click={closeModal}>
            <X size={24} />
          </button>
        </div>

        <div class="modal-body">
          {#if loadingStats}
            <div class="modal-loading">
              <div class="spinner"></div>
              <p>Loading dashboard...</p>
            </div>
          {:else if cityStats}
            <div class="stats-grid">
              <!-- Suppliers Card -->
              <div class="stat-card">
                <div class="stat-icon suppliers">
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
                    <circle cx="9" cy="7" r="4"></circle>
                    <path d="M22 21v-2a4 4 0 0 0-3-3.87"></path>
                    <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
                  </svg>
                </div>
                <div class="stat-content">
                  <div class="stat-label">Suppliers</div>
                  <div class="stat-value">{cityStats.total_suppliers}</div>
                </div>
              </div>

              <!-- Products Card -->
              <div class="stat-card">
                <div class="stat-icon products">
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
                  </svg>
                </div>
                <div class="stat-content">
                  <div class="stat-label">Products</div>
                  <div class="stat-value">{cityStats.total_products}</div>
                </div>
              </div>

              <!-- Stock Value Card -->
              <div class="stat-card">
                <div class="stat-icon value">
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="12" y1="1" x2="12" y2="23"></line>
                    <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>
                  </svg>
                </div>
                <div class="stat-content">
                  <div class="stat-label">Stock Value</div>
                  <div class="stat-value">{formatCurrency(cityStats.total_stock_value)}</div>
                </div>
              </div>

              <!-- Low Stock Card -->
              <div class="stat-card">
                <div class="stat-icon warning">
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
                    <line x1="12" y1="9" x2="12" y2="13"></line>
                    <line x1="12" y1="17" x2="12.01" y2="17"></line>
                  </svg>
                </div>
                <div class="stat-content">
                  <div class="stat-label">Low Stock</div>
                  <div class="stat-value">{cityStats.low_stock_count}</div>
                </div>
              </div>

              <!-- Out of Stock Card -->
              <div class="stat-card">
                <div class="stat-icon danger">
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"></circle>
                    <line x1="15" y1="9" x2="9" y2="15"></line>
                    <line x1="9" y1="9" x2="15" y2="15"></line>
                  </svg>
                </div>
                <div class="stat-content">
                  <div class="stat-label">Out of Stock</div>
                  <div class="stat-value">{cityStats.out_of_stock_count}</div>
                </div>
              </div>

              <!-- Recent Movements Card -->
              <div class="stat-card">
                <div class="stat-icon movements">
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
                  </svg>
                </div>
                <div class="stat-content">
                  <div class="stat-label">Movements (7d)</div>
                  <div class="stat-value">{cityStats.recent_movements}</div>
                </div>
              </div>
            </div>

            <!-- Suppliers List -->
            {#if cityStats.suppliers && cityStats.suppliers.length > 0}
              <div class="suppliers-section">
                <h3 class="section-title">Suppliers in {selectedCity.name}</h3>
                <div class="suppliers-list">
                  {#each cityStats.suppliers as supplier}
                    <div class="supplier-card">
                      <div class="supplier-info">
                        <h4 class="supplier-name">{supplier.name}</h4>
                        {#if supplier.contact_name}
                          <p class="supplier-contact">{supplier.contact_name}</p>
                        {/if}
                        {#if supplier.email}
                          <p class="supplier-email">{supplier.email}</p>
                        {/if}
                        {#if supplier.phone}
                          <p class="supplier-phone">{supplier.phone}</p>
                        {/if}
                        <p class="supplier-products">{supplier.product_count} product{supplier.product_count === 1 ? '' : 's'}</p>
                      </div>
                      <button class="btn-view-products" on:click={() => viewSupplierProducts(supplier)}>
                        <Package size={16} />
                        <span>View Products</span>
                      </button>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          {:else}
            <p class="error-message">Failed to load city statistics</p>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .globe-container {
    position: relative;
    width: 100%;
    height: 100vh;
    overflow: hidden;
    background: var(--bg-base);
  }

  /* Header Overlay */
  .header-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    padding: 2rem;
    background: linear-gradient(to bottom, var(--bg-surface-alpha-70), transparent);
    color: var(--text-primary);
    z-index: 10;
    pointer-events: none;
  }

  .title {
    font-size: 2.5rem;
    font-weight: 700;
    margin: 0;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subtitle {
    font-size: 1rem;
    margin: 0.5rem 0 0 0;
    opacity: 0.8;
  }

  /* Loading Overlay */
  .loading-overlay,
  .error-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-primary);
    z-index: 5;
  }

  .spinner {
    width: 48px;
    height: 48px;
    border: 4px solid rgba(255, 255, 255, 0.1);
    border-top-color: #667eea;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error-message {
    color: #ff6b6b;
    margin: 1rem 0;
  }

  .retry-btn {
    padding: 0.75rem 1.5rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 0.5rem;
    cursor: pointer;
    font-size: 1rem;
    transition: background 0.2s;
  }

  .retry-btn:hover {
    background: #5568d3;
  }

  /* Modal */
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 1rem;
  }

  .modal-content {
    background: var(--bg-surface);
    border-radius: var(--radius-xl);
    max-width: 900px;
    width: 100%;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: var(--shadow-xl);
    border: 1px solid var(--border-color);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-title {
    font-size: 1.75rem;
    font-weight: 700;
    margin: 0;
    color: var(--text-primary);
  }

  .modal-subtitle {
    font-size: 1rem;
    margin: 0.25rem 0 0 0;
    color: var(--text-secondary);
  }

  .close-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.5rem;
    color: var(--text-secondary);
    transition: color 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .modal-body {
    padding: 1.5rem;
  }

  .modal-loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .stat-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.5rem;
    background: var(--bg-base);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-color);
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .stat-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .stat-icon.suppliers {
    background: #dbeafe;
    color: #3b82f6;
  }

  .stat-icon.products {
    background: #e0e7ff;
    color: #6366f1;
  }

  .stat-icon.value {
    background: #d1fae5;
    color: #10b981;
  }

  .stat-icon.warning {
    background: #fef3c7;
    color: #f59e0b;
  }

  .stat-icon.danger {
    background: #fee2e2;
    color: #ef4444;
  }

  .stat-icon.movements {
    background: #ddd6fe;
    color: #8b5cf6;
  }

  .stat-content {
    flex: 1;
  }

  .stat-label {
    font-size: 0.875rem;
    color: var(--text-secondary);
    margin-bottom: 0.25rem;
  }

  .stat-value {
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  /* Suppliers Section */
  .suppliers-section {
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--border-color);
  }

  .section-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 1rem 0;
  }

  .suppliers-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .supplier-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem;
    background: var(--bg-base);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-color);
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .supplier-card:hover {
    transform: translateX(4px);
    box-shadow: var(--shadow-md);
  }

  .supplier-info {
    flex: 1;
  }

  .supplier-name {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 0.25rem 0;
  }

  .supplier-contact,
  .supplier-email,
  .supplier-phone,
  .supplier-products {
    font-size: 0.875rem;
    color: var(--text-secondary);
    margin: 0.125rem 0;
  }

  .supplier-products {
    font-weight: 500;
    color: var(--text-primary);
    margin-top: 0.5rem;
  }

  .btn-view-products {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    transition: background 0.2s, transform 0.2s;
    flex-shrink: 0;
  }

  .btn-view-products:hover {
    background: #5568d3;
    transform: translateY(-1px);
  }

  .btn-view-products:active {
    transform: translateY(0);
  }
</style>
