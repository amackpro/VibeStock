<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { api } from '../lib/api.js';
  import { authStore } from '../stores/auth.js';
  import { toastStore } from '../stores/toast.js';

  let tenants = [];
  let loading = true;
  let currentUser = null;

  authStore.subscribe(a => currentUser = a.user);

  // ── Modals ────────────────────────────────────────────────────────────────
  let showCreateModal = false;
  let createData = { name: '', slug: '' };
  let slugManuallyEdited = false;

  let editTenant = null;   // tenant object being edited
  let editData   = { name: '', is_active: true };

  let confirmDeleteTenant = null; // tenant pending deletion

  // ── Load ──────────────────────────────────────────────────────────────────
  onMount(async () => {
    await loadTenants();
    gsap.fromTo('.page-header',
      { y: -20, opacity: 0 }, { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' });
    gsap.fromTo('.tenant-card',
      { y: 30, opacity: 0, scale: 0.95 },
      { y: 0, opacity: 1, scale: 1, duration: 0.4, stagger: 0.07, ease: 'back.out(1.5)' });
  });

  async function loadTenants() {
    loading = true;
    try {
      const result = await api.tenants.list();
      tenants = result.data || result;
    } catch (e) {
      toastStore.show('Failed to load organizations', 'error');
    } finally {
      loading = false;
    }
  }

  // ── Create ────────────────────────────────────────────────────────────────
  function deriveSlug(name) {
    return name.toLowerCase().trim().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '');
  }

  function handleNameInput() {
    if (!slugManuallyEdited) {
      createData.slug = deriveSlug(createData.name);
    }
  }

  async function handleCreate() {
    if (!createData.name.trim() || !createData.slug.trim()) {
      toastStore.show('Name and slug are required', 'error');
      return;
    }
    try {
      await api.tenants.create({ name: createData.name.trim(), slug: createData.slug.trim() });
      toastStore.show('Organization created', 'success');
      showCreateModal = false;
      createData = { name: '', slug: '' };
      slugManuallyEdited = false;
      await loadTenants();
    } catch (e) {
      toastStore.show(e.message || 'Failed to create organization', 'error');
    }
  }

  // ── Edit ──────────────────────────────────────────────────────────────────
  function openEdit(tenant) {
    editTenant = tenant;
    editData = { name: tenant.name, is_active: tenant.is_active };
  }

  async function handleEdit() {
    try {
      await api.tenants.update(editTenant.id, {
        name: editData.name.trim() || null,
        is_active: editData.is_active,
      });
      toastStore.show('Organization updated', 'success');
      editTenant = null;
      await loadTenants();
    } catch (e) {
      toastStore.show(e.message || 'Failed to update organization', 'error');
    }
  }

  // ── Delete ────────────────────────────────────────────────────────────────
  async function handleDelete(tenant) {
    try {
      await api.tenants.delete(tenant.id);
      toastStore.show(`"${tenant.name}" deleted`, 'success');
      confirmDeleteTenant = null;
      await loadTenants();
    } catch (e) {
      toastStore.show(e.message || 'Failed to delete organization', 'error');
      confirmDeleteTenant = null;
    }
  }

  // ── Helpers ───────────────────────────────────────────────────────────────
  function formatDate(iso) {
    if (!iso) return '—';
    return new Date(iso).toLocaleDateString('en-IN', { day: '2-digit', month: 'short', year: 'numeric' });
  }
</script>

<div class="tenants-page">
  <div class="page-header">
    <div>
      <h2 class="page-title">Organizations</h2>
      <p class="page-subtitle">Manage all tenants registered in the system</p>
    </div>
    {#if currentUser?.is_global_admin}
      <button class="add-btn" on:click={() => showCreateModal = true}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="12" y1="5" x2="12" y2="19"/>
          <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
        New Organization
      </button>
    {/if}
  </div>

  <!-- Stats bar -->
  <div class="stats-bar">
    <div class="stat-chip">
      <span class="stat-num">{tenants.length}</span>
      <span class="stat-label">Total</span>
    </div>
    <div class="stat-chip active">
      <span class="stat-num">{tenants.filter(t => t.is_active).length}</span>
      <span class="stat-label">Active</span>
    </div>
    <div class="stat-chip inactive">
      <span class="stat-num">{tenants.filter(t => !t.is_active).length}</span>
      <span class="stat-label">Inactive</span>
    </div>
    <div class="stat-chip users">
      <span class="stat-num">{tenants.reduce((s, t) => s + (t.total_users || 0), 0)}</span>
      <span class="stat-label">Total Users</span>
    </div>
    <div class="stat-chip products">
      <span class="stat-num">{tenants.reduce((s, t) => s + (t.total_products || 0), 0)}</span>
      <span class="stat-label">Total Products</span>
    </div>
  </div>

  <!-- Table -->
  <div class="table-card">
    {#if loading}
      <div class="center-state">
        <div class="spinner"></div>
        <span>Loading organizations…</span>
      </div>
    {:else if tenants.length === 0}
      <div class="center-state">
        <svg width="56" height="56" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
          <polyline points="9 22 9 12 15 12 15 22"/>
        </svg>
        <span>No organizations found</span>
      </div>
    {:else}
      <table class="tenant-table">
        <thead>
          <tr>
            <th>Organization</th>
            <th>Slug</th>
            <th class="num">Users</th>
            <th class="num">Products</th>
            <th class="num">Suppliers</th>
            <th>Status</th>
            <th>Created</th>
            {#if currentUser?.is_global_admin}<th>Actions</th>{/if}
          </tr>
        </thead>
        <tbody>
          {#each tenants as tenant (tenant.id)}
            <tr class="tenant-row">
              <td>
                <div class="org-cell">
                  <div class="org-avatar">{tenant.name.charAt(0).toUpperCase()}</div>
                  <div>
                    <div class="org-name">{tenant.name}</div>
                    <div class="org-id">ID: {tenant.id.slice(0, 8)}…</div>
                  </div>
                </div>
              </td>
              <td><code class="slug-pill">{tenant.slug}</code></td>
              <td class="num">{tenant.total_users ?? 0}</td>
              <td class="num">{tenant.total_products ?? 0}</td>
              <td class="num">{tenant.total_suppliers ?? 0}</td>
              <td>
                <span class="status-badge" class:active={tenant.is_active}>
                  {tenant.is_active ? 'Active' : 'Inactive'}
                </span>
              </td>
              <td class="date">{formatDate(tenant.created_at)}</td>
              {#if currentUser?.is_global_admin}
                <td>
                  <div class="action-row">
                    <button class="action-btn edit" title="Edit" on:click={() => openEdit(tenant)}>
                      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                      </svg>
                    </button>
                    <button class="action-btn danger" title="Delete" on:click={() => confirmDeleteTenant = tenant}>
                      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="3 6 5 6 21 6"/>
                        <path d="M19 6l-1 14a2 2 0 01-2 2H8a2 2 0 01-2-2L5 6"/>
                        <path d="M10 11v6M14 11v6"/>
                        <path d="M9 6V4a1 1 0 011-1h4a1 1 0 011 1v2"/>
                      </svg>
                    </button>
                  </div>
                </td>
              {/if}
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>

<!-- ── CREATE MODAL ──────────────────────────────────────────────────────── -->
{#if showCreateModal}
  <div class="modal-backdrop" on:click={() => showCreateModal = false}>
    <div class="modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>New Organization</h3>
        <button class="close-btn" on:click={() => showCreateModal = false}>&times;</button>
      </div>
      <form class="modal-form" on:submit|preventDefault={handleCreate}>
        <div class="form-group">
          <label for="c_name">Organization Name</label>
          <input id="c_name" type="text" bind:value={createData.name}
            on:input={handleNameInput} placeholder="e.g. Acme Corp" required />
        </div>
        <div class="form-group">
          <label for="c_slug">Slug <span class="hint">(URL-safe identifier, auto-filled)</span></label>
          <input id="c_slug" type="text" bind:value={createData.slug}
            on:input={() => slugManuallyEdited = true}
            placeholder="e.g. acme-corp" required />
        </div>
        <div class="modal-actions">
          <button type="button" class="btn-cancel" on:click={() => showCreateModal = false}>Cancel</button>
          <button type="submit" class="btn-submit">Create</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- ── EDIT MODAL ────────────────────────────────────────────────────────── -->
{#if editTenant}
  <div class="modal-backdrop" on:click={() => editTenant = null}>
    <div class="modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Edit Organization</h3>
        <button class="close-btn" on:click={() => editTenant = null}>&times;</button>
      </div>
      <form class="modal-form" on:submit|preventDefault={handleEdit}>
        <div class="form-group">
          <label for="e_name">Organization Name</label>
          <input id="e_name" type="text" bind:value={editData.name} required />
        </div>
        <div class="form-group toggle-group">
          <label for="e_active">Status</label>
          <label class="toggle-switch">
            <input id="e_active" type="checkbox" bind:checked={editData.is_active} />
            <span class="toggle-slider"></span>
            <span class="toggle-label">{editData.is_active ? 'Active' : 'Inactive'}</span>
          </label>
        </div>
        <div class="modal-actions">
          <button type="button" class="btn-cancel" on:click={() => editTenant = null}>Cancel</button>
          <button type="submit" class="btn-submit">Save Changes</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- ── DELETE CONFIRM ────────────────────────────────────────────────────── -->
{#if confirmDeleteTenant}
  <div class="modal-backdrop" on:click={() => confirmDeleteTenant = null}>
    <div class="modal confirm-modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Delete Organization</h3>
        <button class="close-btn" on:click={() => confirmDeleteTenant = null}>&times;</button>
      </div>
      <div class="confirm-body">
        <div class="confirm-icon">
          <svg width="44" height="44" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="8" x2="12" y2="12"/>
            <line x1="12" y1="16" x2="12.01" y2="16"/>
          </svg>
        </div>
        <p>Permanently delete <strong>{confirmDeleteTenant.name}</strong>?</p>
        <p class="confirm-warning">
          This will cascade-delete ALL products, stock movements, suppliers, categories,
          and users belonging to this organization. This cannot be undone.
        </p>
      </div>
      <div class="modal-actions" style="padding: 0 24px 24px;">
        <button class="btn-cancel" on:click={() => confirmDeleteTenant = null}>Cancel</button>
        <button class="btn-delete" on:click={() => handleDelete(confirmDeleteTenant)}>
          Delete Everything
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .tenants-page {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  /* Header */
  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 1.3rem;
    font-weight: 600;
    margin-bottom: 2px;
  }

  .page-subtitle {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .add-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 9px 18px;
    background: var(--accent-primary);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .add-btn:hover {
    background: var(--accent-secondary);
    transform: translateY(-1px);
  }

  /* Stats bar */
  .stats-bar {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
  }

  .stat-chip {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 18px;
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    min-width: 90px;
  }

  .stat-chip.active  { border-color: rgba(16,185,129,0.4); }
  .stat-chip.inactive{ border-color: rgba(239,68,68,0.3); }
  .stat-chip.users   { border-color: rgba(99,102,241,0.4); }
  .stat-chip.products{ border-color: rgba(34,211,238,0.4); }

  .stat-num {
    font-size: 1.3rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .stat-label {
    font-size: 0.78rem;
    color: var(--text-muted);
  }

  /* Table card */
  .table-card {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .center-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 14px;
    padding: 60px 20px;
    color: var(--text-muted);
  }

  .tenant-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.88rem;
  }

  .tenant-table thead tr {
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .tenant-table th {
    padding: 12px 16px;
    text-align: left;
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .tenant-table th.num,
  .tenant-table td.num {
    text-align: center;
  }

  .tenant-table td.date {
    color: var(--text-muted);
    font-size: 0.82rem;
  }

  .tenant-row {
    border-bottom: 1px solid var(--border-color);
    transition: background var(--transition-fast);
  }

  .tenant-row:last-child { border-bottom: none; }

  .tenant-row:hover { background: var(--bg-secondary); }

  .tenant-table td {
    padding: 14px 16px;
    vertical-align: middle;
  }

  /* Org cell */
  .org-cell {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .org-avatar {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--accent-primary), #8b5cf6);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 1rem;
    color: white;
    flex-shrink: 0;
  }

  .org-name {
    font-weight: 600;
    color: var(--text-primary);
  }

  .org-id {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-top: 1px;
  }

  .slug-pill {
    padding: 3px 8px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.82rem;
    color: var(--accent-secondary);
  }

  .status-badge {
    padding: 4px 12px;
    border-radius: 100px;
    font-size: 0.78rem;
    font-weight: 600;
    background: rgba(239,68,68,0.12);
    color: var(--accent-danger);
  }

  .status-badge.active {
    background: rgba(16,185,129,0.12);
    color: var(--accent-success);
  }

  /* Action buttons */
  .action-row {
    display: flex;
    gap: 6px;
  }

  .action-btn {
    padding: 6px;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    background: var(--bg-secondary);
    color: var(--text-muted);
    cursor: pointer;
    transition: all var(--transition-fast);
    display: flex;
    align-items: center;
  }

  .action-btn.edit:hover {
    border-color: var(--accent-primary);
    color: var(--accent-primary);
    background: rgba(99,102,241,0.1);
  }

  .action-btn.danger:hover {
    border-color: var(--accent-danger);
    color: var(--accent-danger);
    background: rgba(239,68,68,0.1);
  }

  /* Modals */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 20px;
  }

  .modal {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    width: 100%;
    max-width: 420px;
    box-shadow: 0 20px 40px rgba(0,0,0,0.4);
    animation: slideUp 0.3s cubic-bezier(0.16,1,0.3,1);
  }

  .confirm-modal { max-width: 440px; }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px 24px;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-header h3 { margin: 0; font-size: 1.1rem; }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 1.5rem;
    cursor: pointer;
    line-height: 1;
    padding: 0;
  }

  .close-btn:hover { color: var(--text-primary); }

  .modal-form {
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-group label {
    font-size: 0.88rem;
    color: var(--text-secondary);
    font-weight: 500;
  }

  .hint {
    font-size: 0.75rem;
    color: var(--text-muted);
    font-weight: 400;
  }

  .form-group input {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    padding: 10px 12px;
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: 0.9rem;
    outline: none;
    transition: border-color var(--transition-fast);
  }

  .form-group input:focus { border-color: var(--accent-primary); }

  /* Toggle switch */
  .toggle-group { flex-direction: row; align-items: center; justify-content: space-between; }

  .toggle-switch {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
  }

  .toggle-switch input { display: none; }

  .toggle-slider {
    width: 40px;
    height: 22px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 100px;
    position: relative;
    transition: background var(--transition-fast);
  }

  .toggle-slider::after {
    content: '';
    position: absolute;
    width: 16px;
    height: 16px;
    background: var(--text-muted);
    border-radius: 50%;
    top: 2px;
    left: 2px;
    transition: all var(--transition-fast);
  }

  .toggle-switch input:checked ~ .toggle-slider {
    background: rgba(16,185,129,0.3);
    border-color: var(--accent-success);
  }

  .toggle-switch input:checked ~ .toggle-slider::after {
    background: var(--accent-success);
    left: 20px;
  }

  .toggle-label { font-size: 0.88rem; color: var(--text-secondary); }

  .modal-actions {
    display: flex;
    gap: 12px;
    margin-top: 4px;
  }

  .btn-cancel, .btn-submit, .btn-delete {
    flex: 1;
    padding: 10px;
    border-radius: var(--radius-md);
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all var(--transition-fast);
  }

  .btn-cancel { background: var(--bg-tertiary); color: var(--text-primary); }
  .btn-cancel:hover { background: var(--border-color); }

  .btn-submit { background: var(--accent-primary); color: white; }
  .btn-submit:hover { opacity: 0.9; }

  .btn-delete { background: var(--accent-danger); color: white; }
  .btn-delete:hover { opacity: 0.85; }

  /* Confirm modal body */
  .confirm-body {
    padding: 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    text-align: center;
  }

  .confirm-icon { color: var(--accent-danger); }

  .confirm-body p {
    color: var(--text-primary);
    font-size: 0.95rem;
    margin: 0;
  }

  .confirm-warning {
    color: var(--text-muted) !important;
    font-size: 0.82rem !important;
    font-style: italic;
  }

  .spinner {
    width: 20px;
    height: 20px;
    border: 2px solid var(--border-color);
    border-top-color: var(--accent-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin { to { transform: rotate(360deg); } }
  @keyframes slideUp {
    from { opacity: 0; transform: translateY(20px); }
    to   { opacity: 1; transform: translateY(0); }
  }
</style>
