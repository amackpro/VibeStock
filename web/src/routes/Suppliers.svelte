<script>
  import { onMount } from 'svelte';
  import { api }   from '../lib/api.js';
  import { toast } from '../stores/toast.js';
  import { Pencil, Trash2, X, MapPin, Filter, ChevronDown } from 'lucide-svelte';

  let suppliers  = [];
  let loading    = false;
  let showModal  = false;
  let editing    = null;
  let form       = emptyForm();
  
  // Geography filters
  let regions         = [];
  let countries       = [];
  let cities          = [];
  let selectedRegion  = '';
  let selectedCountry = '';
  let selectedCity    = '';
  let geoLoading      = false;
  let showFilters     = false;

  // Filter labels for chips
  $: regionLabel  = regions.find(r => r.id === selectedRegion)?.name ?? '';
  $: countryLabel = countries.find(c => c.id === selectedCountry)?.name ?? '';
  $: cityLabel    = cities.find(c => c.id === selectedCity)?.name ?? '';
  $: hasFilters   = !!(selectedRegion);

  function emptyForm() {
    return { name: '', contact_name: '', email: '', phone: '', address: '', city_id: '' };
  }

  async function load() {
    loading = true;
    try { 
      const params = {};
      if (selectedRegion)  params.region_id = selectedRegion;
      if (selectedCountry) params.country_id = selectedCountry;
      if (selectedCity)    params.city_id = selectedCity;
      
      suppliers = await api.suppliers.list(params); 
    }
    catch (e) { toast.error(e.message); }
    finally { loading = false; }
  }

  // Geography logic
  async function loadRegions() {
    try { regions = await api.geography.regions(); } catch {}
  }

  async function onRegionChange() {
    selectedCountry = ''; selectedCity = ''; countries = []; cities = [];
    if (selectedRegion) {
      geoLoading = true;
      try { countries = await api.geography.countriesByRegion(selectedRegion); } catch {}
      finally { geoLoading = false; }
    }
    load();
  }

  async function onCountryChange() {
    selectedCity = ''; cities = [];
    if (selectedCountry) {
      geoLoading = true;
      try { cities = await api.geography.citiesByCountry(selectedCountry); } catch {}
      finally { geoLoading = false; }
    }
    load();
  }

  function onCityChange() { load(); }

  function clearFilters() {
    selectedRegion = ''; selectedCountry = ''; selectedCity = '';
    countries = []; cities = [];
    load();
  }

  // Modal logic
  let allCities = [];
  async function loadAllCitiesForModal() {
    try {
      // For assignment, we'll just show cities that have inventory or a sensible subset
      // In this demo, we'll fetch all cities via the regions/countries loop once
      // to keep the assignment dropdown useful.
      const regs = await api.geography.regions();
      const list = [];
      for (const r of regs) {
        const counts = await api.geography.countriesByRegion(r.id);
        for (const c of counts) {
          const cts = await api.geography.citiesByCountry(c.id);
          cts.forEach(city => list.push({ ...city, display: `${city.name}, ${c.name}` }));
        }
      }
      allCities = list.sort((a,b) => a.display.localeCompare(b.display));
    } catch {}
  }

  onMount(async () => {
    await Promise.all([load(), loadRegions(), loadAllCitiesForModal()]);
  });

  function openAdd()  { editing = null; form = emptyForm(); showModal = true; }
  function openEdit(s) {
    editing = s;
    form = {
      name: s.name,
      contact_name: s.contact_name ?? '',
      email: s.email ?? '',
      phone: s.phone ?? '',
      address: s.address ?? '',
      city_id: s.city_id ?? ''
    };
    showModal = true;
  }
  function closeModal() { showModal = false; }

  async function save() {
    try {
      if (editing) { await api.suppliers.update(editing.id, form); toast.success('Supplier updated'); }
      else         { await api.suppliers.create(form);             toast.success('Supplier added');   }
      closeModal(); load();
    } catch (e) { toast.error(e.message); }
  }

  async function del(s) {
    if (!confirm(`Delete supplier "${s.name}"? Products linked to this supplier will be unaffected.`)) return;
    try { await api.suppliers.delete(s.id); toast.success('Supplier deleted'); load(); }
    catch (e) { toast.error(e.message); }
  }
</script>

<div class="page">
  <div class="page-header">
    <div class="page-title-group">
      <h1 class="page-title">Suppliers</h1>
      <div class="flex gap-2 items-center flex-wrap">
        <p class="page-subtitle">
          {suppliers.length} supplier{suppliers.length !== 1 ? 's' : ''}
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
        <span class="chevron" class:rotated={showFilters}>
          <ChevronDown size={13} />
        </span>
      </button>
      <button id="btn-add-supplier" class="btn btn-primary" on:click={openAdd}>+ Add Supplier</button>
    </div>
  </div>

  <!-- ── Filter Panel ── -->
  {#if showFilters}
    <div class="filter-panel stagger-row" style="animation-delay:0ms">
      <div class="filter-panel-inner">
        <div class="filter-header">
          <span class="filter-title">
            <Filter size={14} style="color:var(--accent-glow)" />
            Filter by Location
          </span>
          {#if hasFilters}
            <button class="btn btn-ghost btn-sm" on:click={clearFilters}>Clear All</button>
          {/if}
        </div>
        <div class="filter-row">
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
              <select class="f-select" bind:value={selectedCountry} on:change={onCountryChange} disabled={!selectedRegion}>
                <option value="">All Countries</option>
                {#each countries as c}<option value={c.id}>{c.name}</option>{/each}
              </select>
            </div>
          </div>
          <span class="loc-arrow">›</span>
          <div class="filter-group">
            <label class="filter-label">City</label>
            <div class="sel-wrap">
              <select class="f-select" bind:value={selectedCity} on:change={onCityChange} disabled={!selectedCountry}>
                <option value="">All Cities</option>
                {#each cities as c}<option value={c.id}>{c.name}</option>{/each}
              </select>
            </div>
          </div>
          {#if geoLoading}
            <div class="spinner" style="width:16px;height:16px;border-width:2px;align-self:flex-end;margin-bottom:6px"></div>
          {/if}
        </div>
      </div>
    </div>
  {/if}

  {#if loading}
    <div style="display:flex;justify-content:center;padding:4rem">
      <div class="spinner" style="width:32px;height:32px;border-width:3px"></div>
    </div>
  {:else}
    <div class="card stagger-row" style="padding:0;overflow:hidden;animation-delay: 50ms">
      <div class="table-wrapper" style="border-radius:0;border:none">
        <table>
          <thead>
            <tr>
              <th>Supplier Name</th>
              <th>Contact Person</th>
              <th>Email</th>
              <th>Phone</th>
              <th class="tr">Products</th>
              <th>City</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each suppliers as s, i (s.id)}
              <tr class="stagger-row" style="animation-delay: {100 + (i * 20)}ms">
                <td>
                  <div style="font-weight:600; color:var(--text-primary)">{s.name}</div>
                </td>
                <td style="color:var(--text-secondary)">{s.contact_name ?? '—'}</td>
                <td>
                  {#if s.email}
                    <a href="mailto:{s.email}" class="link">{s.email}</a>
                  {:else}—{/if}
                </td>
                <td style="font-family:var(--font-mono);color:var(--text-secondary);font-size:0.85rem">{s.phone ?? '—'}</td>
                <td class="tr">
                   <span class="count-badge">{s.product_count}</span>
                </td>
                <td>
                  {#if s.city_name}
                    <div style="display:flex; flex-direction:column">
                      <span style="color:var(--text-primary);font-size:0.875rem;font-weight:500">{s.city_name}</span>
                      <span style="color:var(--text-muted);font-size:0.75rem">{s.country_name || ''}</span>
                    </div>
                  {:else}
                    <span style="color:var(--text-muted);font-size:0.75rem;font-style:italic">Not set</span>
                  {/if}
                </td>
                <td>
                  <div class="flex gap-2">
                    <button class="btn btn-ghost btn-sm btn-icon" on:click={() => openEdit(s)} title="Edit">
                      <Pencil size={14} />
                    </button>
                    <button class="btn btn-danger btn-sm btn-icon" on:click={() => del(s)} title="Delete">
                      <Trash2 size={14} />
                    </button>
                  </div>
                </td>
              </tr>
            {/each}
            {#if suppliers.length === 0}
              <tr><td colspan="7" class="empty-state">No suppliers found.</td></tr>
            {/if}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

{#if showModal}
  <div class="modal-backdrop" on:click|self={closeModal}>
    <div class="modal">
      <div class="modal-header">
        <h3 class="modal-title">{editing ? 'Edit Supplier' : 'Add Supplier'}</h3>
        <button class="btn btn-ghost btn-icon" on:click={closeModal}>
          <X size={18} />
        </button>
      </div>
      <div class="modal-body">
        <div class="flex flex-col gap-4">
          <div class="form-group">
            <label class="label" for="s-name">Company Name *</label>
            <input id="s-name" class="input" bind:value={form.name} placeholder="e.g. TechTrade India Ltd." />
          </div>
          <div class="form-group">
            <label class="label" for="s-contact">Contact Person</label>
            <input id="s-contact" class="input" bind:value={form.contact_name} placeholder="Full name" />
          </div>
          <div class="grid grid-2 grid-gap-4">
            <div class="form-group">
              <label class="label" for="s-email">Email</label>
              <input id="s-email" class="input" type="email" bind:value={form.email} placeholder="supplier@example.com" />
            </div>
            <div class="form-group">
              <label class="label" for="s-phone">Phone</label>
              <input id="s-phone" class="input" bind:value={form.phone} placeholder="+91-9876543210" />
            </div>
          </div>
          <div class="form-group">
            <label class="label" for="s-address">Street Address</label>
            <textarea id="s-address" class="textarea input" rows="2" bind:value={form.address} placeholder="Street, Building…"></textarea>
          </div>
          <div class="form-group">
            <label class="label" for="s-city">City / Location Assignment</label>
            <div class="sel-wrap">
              <select id="s-city" class="input" bind:value={form.city_id}>
                <option value="">-- No city assigned (Hidden from Globe) --</option>
                {#each allCities as city (city.id)}
                  <option value={city.id}>{city.display}</option>
                {/each}
              </select>
            </div>
            <p class="form-hint">Assigning a city enables location-based reports and Globe View visualization.</p>
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" on:click={closeModal}>Cancel</button>
        <button id="btn-save-supplier" class="btn btn-primary" on:click={save}>
          {editing ? 'Save Changes' : 'Add Supplier'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* ── Page styles ── */
  .link { color: var(--accent-cyan); text-decoration: none; transition: opacity 0.2s; }
  .link:hover { opacity: 0.8; text-decoration: underline; }
  .tr { text-align: right; }
  .empty-state { text-align: center; color: var(--text-muted); padding: 4rem !important; }

  /* ── Filter panel ── */
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
    cursor: pointer; outline: none; appearance: none;
  }
  .f-select:focus { border-color: var(--accent-glow); }
  .loc-arrow { color: var(--text-muted); font-size: 1.4rem; align-self: flex-end; padding-bottom: 8px; }

  .filter-btn {
    display: flex; align-items: center; gap: 6px;
    border: 1px solid rgba(255,255,255,0.1); border-radius: 10px;
    padding: 7px 14px; font-size: 0.85rem;
  }
  .filter-btn.active {
    border-color: var(--accent-glow); color: var(--accent-glow);
    background: rgba(139,92,246,0.1);
  }
  .chevron { transition: transform 0.2s; display: flex; align-items: center; }
  .chevron.rotated { transform: rotate(180deg); }

  /* ── Chips ── */
  .chip {
    display: inline-flex; align-items: center; gap: 5px;
    padding: 3px 10px; border-radius: 20px; font-size: 0.78rem; font-weight: 500;
  }
  .chip-region  { background: rgba(99,102,241,0.15); color: #818cf8; border: 1px solid rgba(99,102,241,0.3); }
  .chip-country { background: rgba(14,165,233,0.15); color: #38bdf8; border: 1px solid rgba(14,165,233,0.3); }
  .chip-city    { background: rgba(16,185,129,0.15); color: #34d399; border: 1px solid rgba(16,185,129,0.3); }
  .chip-arrow   { color: var(--text-muted); }
  .chip-x { background: none; border: none; cursor: pointer; color: inherit; opacity: 0.7; }

  /* ── Badges ── */
  .count-badge {
    display: inline-block; padding: 2px 10px; border-radius: 20px;
    background: rgba(255,255,255,0.05); color: var(--text-primary);
    font-size: 0.85rem; font-weight: 600; font-family: var(--font-mono);
  }

  .form-hint { font-size: 0.75rem; color: var(--text-muted); margin-top: 4px; line-height: 1.4; }
  
  .spinner {
    width: 40px; height: 40px;
    border: 3px solid var(--border-color);
    border-top-color: var(--accent-primary);
    border-radius: 50%; animation: spin 0.8s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
