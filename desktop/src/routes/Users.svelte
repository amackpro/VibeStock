<script>
  import { onMount } from 'svelte';
  import { api }     from '../lib/api.js';
  import { toast }   from '../stores/toast.js';

  let users = [];
  let loading = true;
  let processingId = null;

  async function loadUsers() {
    loading = true;
    try {
      users = await api.users.list();
    } catch (e) {
      toast.error(e.message || "Failed to load users");
    } finally {
      loading = false;
    }
  }

  onMount(loadUsers);

  async function updateRole(user, newRole) {
    if (user.role === newRole) return;
    processingId = user.id;
    try {
      await api.users.updateRole(user.id, newRole);
      toast.success(`${user.username} is now a ${newRole}`);
      await loadUsers();
    } catch (e) {
      toast.error(e.message || "Failed to update role");
    } finally {
      processingId = null;
    }
  }

  async function toggleStatus(user) {
    const action = user.is_active ? "suspend" : "activate";
    if (!confirm(`Are you sure you want to ${action} ${user.username}?`)) return;

    processingId = user.id;
    try {
      await api.users.toggleStatus(user.id, !user.is_active);
      toast.success(`${user.username} has been ${user.is_active ? 'suspended' : 'activated'}`);
      await loadUsers();
    } catch (e) {
      toast.error(e.message || "Failed to update status");
    } finally {
      processingId = null;
    }
  }
</script>

<div class="page-header">
  <div>
    <h1 class="page-title">User Management</h1>
    <p class="page-sub">Admin control panel for staff access and roles</p>
  </div>
  <button class="btn btn-primary" on:click={loadUsers} disabled={loading}>
    {loading ? '↻ Refreshing...' : '↻ Refresh Data'}
  </button>
</div>

<div class="card table-container">
  {#if loading && users.length === 0}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Loading user database...</p>
    </div>
  {:else}
    <table class="table">
      <thead>
        <tr>
          <th>User</th>
          <th>Contact / Email</th>
          <th>Role</th>
          <th>Status</th>
          <th class="text-right">Admin Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each users as u (u.id)}
          <tr class:suspended={!u.is_active}>
            <td>
              <div class="flex items-center gap-2">
                <div class="user-avatar-sm">{u.full_name.charAt(0).toUpperCase()}</div>
                <div>
                  <div class="font-medium">{u.full_name}</div>
                  <div class="text-xs text-muted">@{u.username}</div>
                </div>
              </div>
            </td>
            <td>{u.email}</td>
            <td>
              {#if u.role === 'admin'}
                <span class="badge badge-error">Admin</span>
              {:else if u.role === 'manager'}
                <span class="badge badge-warning">Manager</span>
              {:else}
                <span class="badge badge-success">Staff</span>
              {/if}
            </td>
            <td>
              {#if u.is_active}
                <span class="badge badge-success">Active</span>
              {:else}
                <span class="badge badge-error">Suspended</span>
              {/if}
            </td>
            <td class="text-right">
              <div class="action-buttons">
                <!-- Role Dropdown -->
                <select 
                  class="input select-sm" 
                  value={u.role}
                  on:change={(e) => updateRole(u, e.target.value)}
                  disabled={processingId === u.id}
                >
                  <option value="admin">Admin</option>
                  <option value="manager">Manager</option>
                  <option value="staff">Staff</option>
                </select>

                <!-- Status Toggle -->
                {#if u.is_active}
                  <button 
                    class="btn btn-error btn-sm" 
                    on:click={() => toggleStatus(u)}
                    disabled={processingId === u.id}
                    title="Suspend User"
                  >
                    🚫 Block
                  </button>
                {:else}
                  <button 
                    class="btn btn-success btn-sm" 
                    on:click={() => toggleStatus(u)}
                    disabled={processingId === u.id}
                    title="Reactivate User"
                  >
                    ✅ Restore
                  </button>
                {/if}
              </div>
            </td>
          </tr>
        {:else}
          <tr><td colspan="5" class="text-center p-8 text-muted">No users found in database.</td></tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .user-avatar-sm {
    width: 32px; height: 32px; border-radius: 50%;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-cyan));
    display: flex; align-items: center; justify-content: center;
    font-size: 0.85rem; font-weight: bold; color: white;
  }
  .action-buttons {
    display: flex; align-items: center; justify-content: flex-end; gap: var(--space-2);
  }
  .select-sm {
    padding: var(--space-1) var(--space-2);
    font-size: 0.8rem; height: auto; width: 110px;
  }
  .suspended td { opacity: 0.6; }
</style>
