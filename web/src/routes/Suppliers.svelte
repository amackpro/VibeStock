<script>
  import { onMount } from 'svelte';
  import { fade, scale } from 'svelte/transition';
  import { backOut } from 'svelte/easing';
  import { gsap } from 'gsap';
  import { api } from '../lib/api.js';
  import { toastStore } from '../stores/toast.js';

  let suppliers = [];
  let regions = [];
  let countries = [];
  let loading = true;
  let searchQuery = '';
  let showModal = false;
  let editingSupplier = null;
  
  let selectedRegion = 'Asia';
  let selectedCountry = '';
  
  let form = {
    name: '',
    contact_name: '',
    email: '',
    phone: '',
    address: ''
  };

  onMount(async () => {
    await Promise.all([loadFilters(), loadSuppliers()]);
    animateEntrance();
  });

  async function loadFilters() {
    try {
      const regRes = await api.geography.regions();
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

  async function loadSuppliers() {
    loading = true;
    try {
      const params = { region: selectedRegion };
      if (selectedCountry && selectedCountry !== '') params.country = selectedCountry;
      
      const result = await api.suppliers.list(params);
      suppliers = result.data || result;
    } catch (e) {
      toastStore.show('Failed to load suppliers', 'error');
    } finally {
      loading = false;
    }
  }

  function handleFilterChange() {
    loadSuppliers();
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
    loadSuppliers();
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

    tl.fromTo('.suppliers-grid', 
      { y: 30, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' },
      '-=0.3'
    );
  }

  function openModal(supplier = null) {
    editingSupplier = supplier;
    if (supplier) {
      form = { 
        name: supplier.name || '',
        contact_name: supplier.contact_name || '',
        email: supplier.email || '',
        phone: supplier.phone || '',
        address: supplier.address || ''
      };
    } else {
      form = {
        name: '',
        contact_name: '',
        email: '',
        phone: '',
        address: ''
      };
    }
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    editingSupplier = null;
  }

  async function saveSupplier() {
    try {
      const payload = { ...form };
      
      if (!payload.email) payload.email = null;
      if (!payload.phone) payload.phone = null;
      if (!payload.address) payload.address = null;

      if (editingSupplier) {
        await api.suppliers.update(editingSupplier.id, payload);
        toastStore.show('Supplier updated successfully', 'success');
      } else {
        await api.suppliers.create(payload);
        toastStore.show('Supplier created successfully', 'success');
      }
      await loadSuppliers();
      closeModal();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }

  async function deleteSupplier(id) {
    if (!confirm('Are you sure you want to delete this supplier?')) return;
    
    try {
      await api.suppliers.delete(id);
      toastStore.show('Supplier deleted', 'success');
      await loadSuppliers();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }

  $: filteredSuppliers = suppliers.filter(s => {
    const q = searchQuery.toLowerCase();
    return (s.name || '').toLowerCase().includes(q) ||
           (s.contact_name || '').toLowerCase().includes(q) ||
           (s.email || '').toLowerCase().includes(q);
  });
</script>

<div class="suppliers-page">
  <div class="page-header">
    <div class="header-left">
      <div class="search-box">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <path d="M21 21l-4.35-4.35"/>
        </svg>
        <input 
          type="text" 
          placeholder="Search suppliers..." 
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
        Add Supplier
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
    <div class="filter-stats">
      <span class="stat-item">
        <strong>{suppliers.length}</strong> suppliers
      </span>
    </div>
  </div>

  <div class="suppliers-grid">
    {#if loading}
      <div class="loading-state">
        <div class="spinner"></div>
        <span>Loading suppliers...</span>
      </div>
    {:else if filteredSuppliers.length === 0}
      <div class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M16 21v-2a4 4 0 00-4-4H6a4 4 0 00-4 4v2"/>
          <circle cx="9" cy="7" r="4"/>
          <path d="M22 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75"/>
        </svg>
        <span>No suppliers found</span>
      </div>
    {:else}
      {#each filteredSuppliers as supplier, i (supplier.id)}
        <div class="supplier-card" style="animation-delay: {i * 0.05}s">
          <div class="supplier-header">
            <div class="supplier-avatar">
              {supplier.name.charAt(0).toUpperCase()}
            </div>
            <div class="supplier-info">
              <h3 class="supplier-name">{supplier.name}</h3>
              {#if supplier.contact_name}
                <span class="contact-person">{supplier.contact_name}</span>
              {/if}
            </div>
          </div>

          <div class="supplier-details">
            {#if supplier.email}
              <div class="detail-item">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/>
                  <path d="M22 6l-10 7L2 6"/>
                </svg>
                <span>{supplier.email}</span>
              </div>
            {/if}
            {#if supplier.phone}
              <div class="detail-item">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07 19.5 19.5 0 01-6-6 19.79 19.79 0 01-3.07-8.67A2 2 0 014.11 2h3a2 2 0 012 1.72 12.84 12.84 0 00.7 2.81 2 2 0 01-.45 2.11L8.09 9.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45 12.84 12.84 0 002.81.7A2 2 0 0122 16.92z"/>
                </svg>
                <span>{supplier.phone}</span>
              </div>
            {/if}
            {#if supplier.address}
              <div class="detail-item">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0118 0z"/>
                  <circle cx="12" cy="10" r="3"/>
                </svg>
                <span>{supplier.address}</span>
              </div>
            {/if}
          </div>

          <div class="supplier-actions">
            <button class="action-btn" on:click={() => openModal(supplier)} title="Edit">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
              </svg>
            </button>
            <button class="action-btn danger" on:click={() => deleteSupplier(supplier.id)} title="Delete">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
              </svg>
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

{#if showModal}
  <div class="modal-overlay" on:click={closeModal}>
    <div class="modal-content" on:click|stopPropagation>
      <div class="modal-header">
        <h2 class="modal-title">{editingSupplier ? 'Edit Supplier' : 'Add Supplier'}</h2>
        <button class="modal-close" on:click={closeModal}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>

      <form on:submit|preventDefault={saveSupplier}>
        <div class="form-group">
          <label class="form-label">Company Name</label>
          <input type="text" class="input-field" bind:value={form.name} required />
        </div>

        <div class="form-group">
          <label class="form-label">Contact Person</label>
          <input type="text" class="input-field" bind:value={form.contact_name} />
        </div>

        <div class="form-grid">
          <div class="form-group">
            <label class="form-label">Email</label>
            <input type="email" class="input-field" bind:value={form.email} />
          </div>
          <div class="form-group">
            <label class="form-label">Phone</label>
            <input type="tel" class="input-field" bind:value={form.phone} />
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">Address</label>
          <textarea class="input-field textarea" bind:value={form.address} rows="3"></textarea>
        </div>

        <div class="modal-actions">
          <button type="button" class="btn btn-secondary" on:click={closeModal}>Cancel</button>
          <button type="submit" class="btn btn-primary">
            {editingSupplier ? 'Update Supplier' : 'Create Supplier'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .suppliers-page {
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

  .suppliers-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 20px;
  }

  .loading-state, .empty-state {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 64px 20px;
    color: var(--text-muted);
  }

  .supplier-card {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: 24px;
    transition: all var(--transition-base);
    animation: slideUp 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    opacity: 0;
  }

  .supplier-card:hover {
    border-color: var(--border-glow);
    box-shadow: var(--shadow-glow);
    transform: translateY(-2px);
  }

  .supplier-header {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 20px;
  }

  .supplier-avatar {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--accent-primary), #8b5cf6);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 1.2rem;
    color: white;
    flex-shrink: 0;
  }

  .supplier-info {
    flex: 1;
    min-width: 0;
  }

  .supplier-name {
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 2px;
  }

  .contact-person {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .supplier-details {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 20px;
  }

  .detail-item {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .detail-item svg {
    flex-shrink: 0;
    color: var(--text-muted);
  }

  .supplier-actions {
    display: flex;
    gap: 8px;
    padding-top: 16px;
    border-top: 1px solid var(--border-color);
  }

  .action-btn {
    flex: 1;
    padding: 10px;
    border-radius: var(--radius-md);
    color: var(--text-muted);
    background: var(--bg-secondary);
    transition: all var(--transition-fast);
    display: flex;
    align-items: center;
    justify-content: center;
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
    align-items: flex-start;
    justify-content: center;
    z-index: 1000;
    padding: 40px 20px;
    overflow-y: auto;
  }

  .modal-content {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-xl);
    padding: 32px;
    width: 100%;
    max-width: 500px;
    margin-top: 40px;
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

  .textarea {
    resize: vertical;
    min-height: 80px;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 28px;
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
