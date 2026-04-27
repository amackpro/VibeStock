<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { api } from '../lib/api.js';
  import { authStore } from '../stores/auth.js';
  import { toastStore } from '../stores/toast.js';

  let users = [];
  let loading = true;
  let currentUser = null;

  authStore.subscribe(a => currentUser = a.user);

  onMount(async () => {
    await loadUsers();
    animateEntrance();
  });

  function animateEntrance() {
    const tl = gsap.timeline();

    tl.fromTo('.page-header', 
      { y: -20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' }
    );

    tl.fromTo('.user-card', 
      { y: 30, opacity: 0, scale: 0.95 },
      { y: 0, opacity: 1, scale: 1, duration: 0.4, stagger: 0.08, ease: 'back.out(1.5)' },
      '-=0.3'
    );
  }

  async function loadUsers() {
    console.log('DEBUG: loadUsers called');
    try {
      const result = await api.users.list();
      console.log('DEBUG: Users result from API:', result);
      users = result.data || result;
      console.log('DEBUG: users array set to:', users);
    } catch (e) {
      console.error('DEBUG: loadUsers error:', e);
      toastStore.show('Failed to load users', 'error');
    } finally {
      loading = false;
    }
  }

  async function updateRole(userId, role) {
    try {
      await api.users.updateRole(userId, role);
      toastStore.show('Role updated', 'success');
      await loadUsers();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }

  async function toggleStatus(userId, isActive) {
    try {
      await api.users.toggleStatus(userId, isActive);
      toastStore.show(`User ${isActive ? 'activated' : 'deactivated'}`, 'success');
      await loadUsers();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }

  let confirmDelete = null; // holds the user object pending confirmation

  async function handleDeleteUser(user) {
    try {
      await api.users.delete(user.id);
      toastStore.show(`User "${user.username}" removed successfully`, 'success');
      confirmDelete = null;
      await loadUsers();
    } catch (e) {
      toastStore.show(e.message || 'Failed to delete user', 'error');
      confirmDelete = null;
    }
  }

  let showAddModal = false;
  let newUserData = {
    username: '',
    email: '',
    full_name: '',
    password: '',
    role: 'staff'
  };

  async function handleAddUser() {
    try {
      // Use the dedicated admin endpoint — it automatically scopes the new user
      // to the calling admin's tenant. No tenant_id juggling needed.
      await api.users.create(newUserData);
      toastStore.show('User added successfully', 'success');
      showAddModal = false;
      newUserData = { username: '', email: '', full_name: '', password: '', role: 'staff' };
      await loadUsers();
    } catch (e) {
      toastStore.show(e.message || 'Failed to add user', 'error');
    }
  }

  const roles = ['admin', 'manager', 'staff'];

  // True when the tenant already has exactly one admin (the current user).
  // For global admins seeing multi-tenant lists, we skip the UI restriction
  // and let the backend enforce it per-tenant.
  $: tenantHasAdmin = currentUser?.is_global_admin
    ? false
    : users.some(u => u.role === 'admin' && !u.is_global_admin);
</script>

<div class="users-page">
  <div class="page-header">
    <h2 class="page-title">User Management</h2>
    {#if currentUser?.role === 'admin'}
      <button class="add-btn" on:click={() => showAddModal = true}>
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
        Add User
      </button>
    {/if}
  </div>

  <div class="users-grid">
    {#if loading}
      <div class="loading-state">
        <div class="spinner"></div>
        <span>Loading users...</span>
      </div>
    {:else if users.length === 0}
      <div class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2"/>
          <circle cx="9" cy="7" r="4"/>
          <path d="M23 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75"/>
        </svg>
        <span>No users found</span>
      </div>
    {:else}
      {#each users as user, i (user.id)}
        <div class="user-card" style="animation-delay: {i * 0.05}s">
          <div class="user-header">
            <div class="user-avatar" style="background: {user.role === 'admin' ? 'linear-gradient(135deg, #ef4444, #f97316)' : user.role === 'manager' ? 'linear-gradient(135deg, #6366f1, #8b5cf6)' : 'linear-gradient(135deg, #10b981, #22d3ee)'}">
              {user.username.charAt(0).toUpperCase()}
            </div>
            <div class="user-info">
              <h3 class="user-name">{user.username}</h3>
              <span class="user-email">{user.email || '-'}</span>
            </div>
            <div class="user-status" class:active={user.is_active}>
              {user.is_active ? 'Active' : 'Inactive'}
            </div>
          </div>

          <div class="user-details">
            <div class="detail-row">
              <span class="detail-label">Role</span>
              <select
                class="role-select"
                value={user.role}
                on:change={(e) => updateRole(user.id, e.target.value)}
                disabled={currentUser?.role !== 'admin' || user.id === currentUser?.id}
                style="text-transform: capitalize;"
              >
                {#each roles as role}
                  <option
                    value={role}
                    style="text-transform: capitalize;"
                    disabled={role === 'admin' && tenantHasAdmin && user.role !== 'admin'}
                  >
                    {role}{role === 'admin' && tenantHasAdmin && user.role !== 'admin' ? ' (taken)' : ''}
                  </option>
                {/each}
              </select>
            </div>
            {#if user.tenant_name}
              <div class="detail-row">
                <span class="detail-label">Organization</span>
                <span class="detail-value">{user.tenant_name}</span>
              </div>
            {/if}
            <div class="detail-row">
              <span class="detail-label">Status</span>
              {#if currentUser?.role === 'admin' && user.id !== currentUser?.id}
                <button
                  class="status-toggle"
                  class:active={user.is_active}
                  on:click={() => toggleStatus(user.id, !user.is_active)}
                >
                  {user.is_active ? 'Active' : 'Inactive'}
                </button>
              {:else}
                <span class="status-text" class:active={user.is_active}>
                  {user.is_active ? 'Active' : 'Inactive'}
                </span>
              {/if}
            </div>

            {#if currentUser?.role === 'admin' && user.id !== currentUser?.id && !user.is_global_admin}
              <div class="detail-row" style="margin-top: 8px; padding-top: 12px; border-top: 1px solid var(--border-color);">
                <span class="detail-label" style="color: var(--accent-danger); font-size: 0.8rem;">Danger Zone</span>
                <button class="delete-btn" on:click={() => confirmDelete = user}>
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="3 6 5 6 21 6"></polyline>
                    <path d="M19 6l-1 14a2 2 0 01-2 2H8a2 2 0 01-2-2L5 6"></path>
                    <path d="M10 11v6M14 11v6"></path>
                    <path d="M9 6V4a1 1 0 011-1h4a1 1 0 011 1v2"></path>
                  </svg>
                  Remove User
                </button>
              </div>
            {/if}
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

{#if showAddModal}
  <div class="modal-backdrop" on:click={() => showAddModal = false}>
    <div class="modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Add New User</h3>
        <button class="close-btn" on:click={() => showAddModal = false}>&times;</button>
      </div>
      
      <form class="modal-form" on:submit|preventDefault={handleAddUser}>
        <div class="form-group">
          <label for="username">Username</label>
          <input type="text" id="username" bind:value={newUserData.username} required />
        </div>
        
        <div class="form-group">
          <label for="full_name">Full Name</label>
          <input type="text" id="full_name" bind:value={newUserData.full_name} required />
        </div>
        
        <div class="form-group">
          <label for="email">Email</label>
          <input type="email" id="email" bind:value={newUserData.email} required />
        </div>
        
        <div class="form-group">
          <label for="password">Password</label>
          <input type="password" id="password" bind:value={newUserData.password} required minlength="6" />
        </div>

        <div class="form-group">
          <label for="new_user_role">Role</label>
          <select id="new_user_role" bind:value={newUserData.role} class="role-select-modal">
            {#each roles as role}
              <option
                value={role}
                style="text-transform: capitalize;"
                disabled={role === 'admin' && tenantHasAdmin}
              >
                {role.charAt(0).toUpperCase() + role.slice(1)}{role === 'admin' && tenantHasAdmin ? ' (taken)' : ''}
              </option>
            {/each}
          </select>
          {#if tenantHasAdmin}
            <p class="role-hint">Only one admin per organization is allowed.</p>
          {/if}
        </div>

        <div class="modal-actions">
          <button type="button" class="btn-cancel" on:click={() => showAddModal = false}>Cancel</button>
          <button type="submit" class="btn-submit">Add User</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Delete confirmation modal -->
{#if confirmDelete}
  <div class="modal-backdrop" on:click={() => confirmDelete = null}>
    <div class="modal confirm-modal" on:click|stopPropagation>
      <div class="modal-header">
        <h3>Remove User</h3>
        <button class="close-btn" on:click={() => confirmDelete = null}>&times;</button>
      </div>
      <div class="confirm-body">
        <div class="confirm-icon">
          <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="8" x2="12" y2="12"/>
            <line x1="12" y1="16" x2="12.01" y2="16"/>
          </svg>
        </div>
        <p>Are you sure you want to permanently remove <strong>{confirmDelete.username}</strong> from your organization?</p>
        <p class="confirm-warning">This action cannot be undone. All activity by this user will remain in the audit log.</p>
      </div>
      <div class="modal-actions" style="padding: 0 24px 24px;">
        <button class="btn-cancel" on:click={() => confirmDelete = null}>Cancel</button>
        <button class="btn-delete" on:click={() => handleDeleteUser(confirmDelete)}>Yes, Remove User</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .users-page {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 1.3rem;
    font-weight: 600;
  }

  .add-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: var(--accent-primary);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .add-btn:hover {
    background: var(--accent-secondary);
    transform: translateY(-1px);
  }

  .users-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
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

  .user-card {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: 24px;
    transition: all var(--transition-base);
    animation: slideUp 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    opacity: 0;
  }

  .user-card:hover {
    border-color: var(--border-glow);
    box-shadow: var(--shadow-glow);
  }

  .user-header {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 20px;
  }

  .user-avatar {
    width: 52px;
    height: 52px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 1.3rem;
    color: white;
    flex-shrink: 0;
  }

  .user-info {
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 2px;
  }

  .user-email {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .user-status {
    padding: 4px 12px;
    border-radius: 100px;
    font-size: 0.75rem;
    font-weight: 600;
    background: rgba(239, 68, 68, 0.15);
    color: var(--accent-danger);
  }

  .user-status.active {
    background: rgba(16, 185, 129, 0.15);
    color: var(--accent-success);
  }

  .user-details {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding-top: 16px;
    border-top: 1px solid var(--border-color);
  }

  .detail-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .detail-label {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .detail-value {
    font-size: 0.85rem;
    color: var(--text-primary);
    font-weight: 500;
  }

  .role-select {
    padding: 8px 12px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: 0.85rem;
    cursor: pointer;
  }

  .role-select:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .status-toggle {
    padding: 6px 14px;
    border-radius: 100px;
    font-size: 0.8rem;
    font-weight: 600;
    background: rgba(239, 68, 68, 0.15);
    color: var(--accent-danger);
    transition: all var(--transition-fast);
    border: none;
    cursor: pointer;
  }

  .status-toggle.active {
    background: rgba(16, 185, 129, 0.15);
    color: var(--accent-success);
  }

  .status-text {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .status-text.active {
    color: var(--accent-success);
  }

  .delete-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: rgba(239, 68, 68, 0.1);
    color: var(--accent-danger);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: var(--radius-sm);
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .delete-btn:hover {
    background: rgba(239, 68, 68, 0.2);
    border-color: var(--accent-danger);
  }

  /* Modal shared styles */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
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
    max-width: 400px;
    box-shadow: 0 20px 40px rgba(0,0,0,0.4);
    animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px 24px;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1.2rem;
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 1.5rem;
    cursor: pointer;
    line-height: 1;
    padding: 0;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .modal-form {
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-group label {
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .form-group input {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    padding: 10px 12px;
    border-radius: var(--radius-md);
    color: var(--text-primary);
    outline: none;
    transition: border-color var(--transition-fast);
  }

  .form-group input:focus {
    border-color: var(--accent-primary);
  }

  .role-select-modal {
    width: 100%;
    padding: 10px 12px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: 0.9rem;
    cursor: pointer;
    outline: none;
    transition: border-color var(--transition-fast);
  }

  .role-select-modal:focus {
    border-color: var(--accent-primary);
  }

  .role-hint {
    margin-top: 6px;
    font-size: 0.78rem;
    color: var(--text-muted);
    font-style: italic;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
    margin-top: 8px;
  }

  .btn-cancel, .btn-submit, .btn-delete {
    flex: 1;
    padding: 10px;
    border-radius: var(--radius-md);
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all var(--transition-fast);
  }

  .btn-cancel {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .btn-cancel:hover {
    background: var(--border-color);
  }

  .btn-submit {
    background: var(--accent-primary);
    color: white;
  }

  .btn-submit:hover {
    background: var(--accent-secondary);
  }

  .btn-delete {
    background: var(--accent-danger);
    color: white;
  }

  .btn-delete:hover {
    opacity: 0.85;
  }

  /* Confirm modal */
  .confirm-modal {
    max-width: 420px;
  }

  .confirm-body {
    padding: 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    text-align: center;
  }

  .confirm-icon {
    color: var(--accent-danger);
    margin-bottom: 4px;
  }

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

  @keyframes slideUp {
    from { opacity: 0; transform: translateY(20px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .spinner {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>