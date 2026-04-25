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
    try {
      const result = await api.users.list();
      users = result.data || result;
    } catch (e) {
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

  const roles = ['Admin', 'Manager', 'Staff'];
</script>

<div class="users-page">
  <div class="page-header">
    <h2 class="page-title">User Management</h2>
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
            <div class="user-avatar" style="background: {user.role === 'Admin' ? 'linear-gradient(135deg, #ef4444, #f97316)' : user.role === 'Manager' ? 'linear-gradient(135deg, #6366f1, #8b5cf6)' : 'linear-gradient(135deg, #10b981, #22d3ee)'}">
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
                disabled={currentUser?.role !== 'Admin' || user.id === currentUser?.id}
              >
                {#each roles as role}
                  <option value={role}>{role}</option>
                {/each}
              </select>
            </div>
            <div class="detail-row">
              <span class="detail-label">Status</span>
              {#if currentUser?.role === 'Admin' && user.id !== currentUser?.id}
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
</style>
