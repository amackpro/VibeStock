<script>
  import { onMount } from 'svelte';
  import { api }     from '../lib/api.js';
  import { toast }   from '../stores/toast.js';
  import { authStore } from '../stores/auth.js';
  import { Building2, User, Package, Factory, Trash2 } from 'lucide-svelte';

  let tenants = [];
  let loading = true;
  let processing = false;
  let showCreateForm = false;
  let newTenant = { name: '', slug: '' };

  async function loadTenants() {
    loading = true;
    try {
      tenants = await api.tenants.list();
    } catch (e) {
      toast.error(e.message || "Failed to load tenants");
    } finally {
      loading = false;
    }
  }

  onMount(loadTenants);

  async function createTenant() {
    if (!newTenant.name || !newTenant.slug) {
      toast.error("Please fill in all fields");
      return;
    }

    processing = true;
    try {
      await api.tenants.create(newTenant);
      toast.success("Tenant created successfully");
      showCreateForm = false;
      newTenant = { name: '', slug: '' };
      await loadTenants();
    } catch (e) {
      toast.error(e.message || "Failed to create tenant");
    } finally {
      processing = false;
    }
  }

  async function deleteTenant(tenant) {
    if (!confirm(`Are you sure you want to delete "${tenant.name}"? This will delete all its data!`)) {
      return;
    }

    processing = true;
    try {
      await api.tenants.delete(tenant.id);
      toast.success("Tenant deleted successfully");
      await loadTenants();
    } catch (e) {
      toast.error(e.message || "Failed to delete tenant");
    } finally {
      processing = false;
    }
  }

  async function toggleTenantStatus(tenant) {
    const action = tenant.is_active ? "deactivate" : "activate";
    if (!confirm(`Are you sure you want to ${action} "${tenant.name}"?`)) {
      return;
    }

    processing = true;
    try {
      await api.tenants.update(tenant.id, { is_active: !tenant.is_active });
      toast.success(`Tenant ${action}d successfully`);
      await loadTenants();
    } catch (e) {
      toast.error(e.message || "Failed to update tenant");
    } finally {
      processing = false;
    }
  }

  function generateSlug(name) {
    newTenant.slug = name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');
  }
</script>

<div class="page">
  <div class="page-header">
    <div class="page-title-group">
      <h1 class="page-title">Tenant Management</h1>
      <p class="page-subtitle">Manage organizations and databases</p>
    </div>
    <div class="header-actions">
      <button class="btn btn-secondary" on:click={loadTenants} disabled={loading}>
        {loading ? '↻ Refreshing...' : '↻ Refresh'}
      </button>
      <button class="btn btn-primary" on:click={() => showCreateForm = !showCreateForm}>
        {showCreateForm ? '✕ Cancel' : '+ New Tenant'}
      </button>
    </div>
  </div>

  <!-- Create Form -->
  {#if showCreateForm}
    <div class="card form-card stagger-row">
      <h3 class="form-title">Create New Tenant</h3>
      <div class="form-grid">
        <div class="form-group">
          <label class="label" for="tenant-name">Organization Name</label>
          <input
            id="tenant-name"
            class="input"
            type="text"
            placeholder="Acme Corporation"
            bind:value={newTenant.name}
            on:input={() => generateSlug(newTenant.name)}
          />
        </div>
        <div class="form-group">
          <label class="label" for="tenant-slug">Slug (URL-friendly)</label>
          <input
            id="tenant-slug"
            class="input"
            type="text"
            placeholder="acme-corp"
            bind:value={newTenant.slug}
          />
        </div>
      </div>
      <div class="form-actions">
        <button class="btn btn-primary" on:click={createTenant} disabled={processing}>
          {processing ? 'Creating...' : 'Create Tenant'}
        </button>
      </div>
    </div>
  {/if}

  <!-- Tenants Table -->
  <div class="card table-container stagger-row" style="padding:0;overflow:hidden;animation-delay: 50ms">
    {#if loading && tenants.length === 0}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading tenants...</p>
      </div>
    {:else}
      <table class="table">
        <thead>
          <tr>
            <th>Tenant</th>
            <th>Slug</th>
            <th>Stats</th>
            <th>Status</th>
            <th class="text-right">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each tenants as t, i (t.id)}
            <tr class="stagger-row" class:inactive={!t.is_active} style="animation-delay: {100 + (i * 50)}ms">
              <td>
                <div class="tenant-info">
                  <div class="tenant-avatar"><Building2 size={20} /></div>
                  <div>
                    <div style="font-weight:600">{t.name}</div>
                    <div style="font-size:0.75rem;color:var(--text-muted)">ID: {t.id.slice(0,8)}...</div>
                  </div>
                </div>
              </td>
              <td>
                <code class="slug-badge">{t.slug}</code>
              </td>
              <td>
                <div class="stats-grid">
                  <span class="stat"><User size={14} /> {t.total_users || 0}</span>
                  <span class="stat"><Package size={14} /> {t.total_products || 0}</span>
                  <span class="stat"><Factory size={14} /> {t.total_suppliers || 0}</span>
                </div>
              </td>
              <td>
                {#if t.is_active}
                  <span class="badge badge-green">Active</span>
                {:else}
                  <span class="badge badge-red">Inactive</span>
                {/if}
              </td>
              <td style="text-align:right">
                <div class="action-buttons">
                  <button 
                    class="btn btn-sm" 
                    class:btn-success={!t.is_active}
                    class:btn-warning={t.is_active}
                    on:click={() => toggleTenantStatus(t)}
                    disabled={processing}
                    title={t.is_active ? "Deactivate" : "Activate"}
                  >
                    {t.is_active ? '⏸️ Pause' : '▶️ Activate'}
                  </button>
                  <button 
                    class="btn btn-danger btn-sm" 
                    on:click={() => deleteTenant(t)}
                    disabled={processing}
                    title="Delete Tenant"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              </td>
            </tr>
          {:else}
            <tr><td colspan="5" class="text-center p-8 text-muted">No tenants found.</td></tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>

<style>
  .header-actions {
    display: flex; gap: var(--space-3);
  }
  .form-card {
    margin-bottom: var(--space-6);
    padding: var(--space-6);
  }
  .form-title {
    font-size: 1.1rem; font-weight: 700; margin-bottom: var(--space-4);
  }
  .form-grid {
    display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4);
  }
  .form-actions {
    margin-top: var(--space-4);
    display: flex; justify-content: flex-end;
  }
  .tenant-info {
    display: flex; align-items: center; gap: var(--space-3);
  }
  .tenant-avatar {
    width: 36px; height: 36px; border-radius: var(--radius-md);
    background: var(--glass-hover);
    display: flex; align-items: center; justify-content: center;
    font-size: 1.1rem;
  }
  .slug-badge {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    background: var(--bg-surface);
    padding: 2px 8px;
    border-radius: var(--radius-sm);
    color: var(--accent-cyan);
  }
  .stats-grid {
    display: flex; gap: var(--space-3); font-size: 0.8rem;
  }
  .stat {
    color: var(--text-secondary);
  }
  .action-buttons {
    display: flex; justify-content: flex-end; gap: var(--space-2);
  }
  .inactive td {
    opacity: 0.6;
  }
  .btn-success {
    background: var(--accent-green); color: white;
  }
  .btn-warning {
    background: var(--accent-amber); color: white;
  }
</style>
