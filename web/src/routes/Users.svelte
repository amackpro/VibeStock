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

  let showAddModal = false;
  let newUserData = {
    username: '',
    email: '',
    full_name: '',
    password: ''
  };

  async function handleAddUser() {
    try {
      // Add tenant_id from currentUser so they belong to the same organization
      const payload = {
        ...newUserData,
        tenant_id: currentUser.tenant_id || currentUser.tenant?.id || authStore.tenant?.id // Try to get the tenant id
      };
      
      // If we don't have tenant_id in the user object directly, we can get it from the store
      let authState;
      authStore.subscribe(s => authState = s)();
      
      if (!payload.tenant_id && authState?.tenant?.id) {
        payload.tenant_id = authState.tenant.id;
      }

      await api.auth.register(payload);
      toastStore.show('User added successfully', 'success');
      showAddModal = false;
      newUserData = { username: '', email: '', full_name: '', password: '' };
      await loadUsers();
    } catch (e) {
      toastStore.show(e.message || 'Failed to add user', 'error');
    }
  }

  const roles = ['admin', 'manager', 'staff'];
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
                  <option value={role} style="text-transform: capitalize;">{role}</option>
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
        
        <div class="modal-actions">
          <button type="button" class="btn-cancel" on:click={() => showAddModal = false}>Cancel</button>
          <button type="submit" class="btn-submit">Add User</button>
        </div>
      </form>
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

  /* Modal Styles */
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

  .modal-actions {
    display: flex;
    gap: 12px;
    margin-top: 8px;
  }

  .btn-cancel, .btn-submit {
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

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
