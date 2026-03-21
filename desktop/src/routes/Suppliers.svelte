<script>
  /**
   * Suppliers.svelte — Supplier management page
   * Full CRUD with add/edit modal and delete confirmation.
   */
  import { onMount } from 'svelte';
  import { api }   from '../lib/api.js';
  import { toast } from '../stores/toast.js';
  import { Pencil, Trash2, X } from 'lucide-svelte';

  let suppliers  = [];
  let loading    = false;
  let showModal  = false;
  let editing    = null;
  let form       = emptyForm();
  let cities     = [];
  let loadingCities = false;

  function emptyForm() {
    return { name: '', contact_name: '', email: '', phone: '', address: '', city_id: '' };
  }

  async function load() {
    loading = true;
    try { suppliers = await api.suppliers.list(); }
    catch (e) { toast.error(e.message); }
    finally { loading = false; }
  }

  async function loadCities() {
    loadingCities = true;
    try {
      // Load all regions and their countries/cities
      const regions = await api.geography.regions();
      const allCities = [];

      for (const region of regions) {
        const countries = await api.geography.countriesByRegion(region.id);
        for (const country of countries) {
          const countryCities = await api.geography.citiesByCountry(country.id);
          countryCities.forEach(city => {
            allCities.push({
              ...city,
              country_name: country.name,
              display_name: `${city.name}, ${country.name}`
            });
          });
        }
      }

      // Sort by population (largest first)
      cities = allCities.sort((a, b) => b.population - a.population);
    } catch (e) {
      toast.error('Failed to load cities');
    } finally {
      loadingCities = false;
    }
  }

  onMount(() => {
    load();
    loadCities();
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

  function getCityName(cityId) {
    if (!cityId) return null;
    const city = cities.find(c => c.id === cityId);
    return city ? city.display_name : null;
  }
</script>

<div class="page">
  <div class="page-header">
    <div class="page-title-group">
      <h1 class="page-title">Suppliers</h1>
      <p class="page-subtitle">{suppliers.length} suppliers registered</p>
    </div>
    <button id="btn-add-supplier" class="btn btn-primary" on:click={openAdd}>+ Add Supplier</button>
  </div>

  {#if loading}
    <div style="display:flex;justify-content:center;padding:4rem"><div class="spinner" style="width:32px;height:32px;border-width:3px"></div></div>
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
              <th>City</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each suppliers as s, i (s.id)}
              {@const cityName = getCityName(s.city_id)}
              <tr class="stagger-row" style="animation-delay: {100 + (i * 50)}ms">
                <td><div style="font-weight:600">{s.name}</div></td>
                <td style="color:var(--text-secondary)">{s.contact_name ?? '—'}</td>
                <td>
                  {#if s.email}
                    <a href="mailto:{s.email}" style="color:var(--accent-cyan);text-decoration:none">{s.email}</a>
                  {:else}—{/if}
                </td>
                <td style="font-family:var(--font-mono);color:var(--text-secondary)">{s.phone ?? '—'}</td>
                <td>
                  {#if cityName}
                    <span style="color:var(--text-primary);font-size:0.875rem">{cityName}</span>
                  {:else}
                    <span style="color:var(--text-muted);font-size:0.75rem">Not set</span>
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
              <tr><td colspan="6" style="text-align:center;color:var(--text-muted);padding:3rem">No suppliers yet. Add your first supplier.</td></tr>
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
        <h3>{editing ? 'Edit Supplier' : 'Add Supplier'}</h3>
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
            <label class="label" for="s-address">Address</label>
            <textarea id="s-address" class="textarea input" rows="2" bind:value={form.address} placeholder="Full address…"></textarea>
          </div>
          <div class="form-group">
            <label class="label" for="s-city">City / Location</label>
            {#if loadingCities}
              <div style="padding: 0.5rem; color: var(--text-secondary)">Loading cities...</div>
            {:else}
              <select id="s-city" class="input" bind:value={form.city_id}>
                <option value="">-- No city assigned --</option>
                {#each cities as city (city.id)}
                  <option value={city.id}>{city.display_name}</option>
                {/each}
              </select>
              <small style="color: var(--text-muted); font-size: 0.75rem">
                Assigning a city will make this supplier visible on the Globe View
              </small>
            {/if}
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
