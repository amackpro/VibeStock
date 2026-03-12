<script>
  /**
   * Sidebar.svelte — Glassmorphism navigation sidebar
   * Emits 'navigate' event on menu item click.
   * Receives `activePage` prop to highlight current route.
   */
  import { createEventDispatcher } from 'svelte';
  import { authStore } from '../stores/auth.js';
  import { toast } from '../stores/toast.js';

  export let activePage = 'dashboard';
  const dispatch = createEventDispatcher();

  // Navigation items definition
  const navItems = [
    { id: 'dashboard', label: 'Dashboard',       icon: '⚡' },
    { id: 'products',  label: 'Products',         icon: '📦' },
    { id: 'movements', label: 'Stock Movements',  icon: '🔄' },
    { id: 'suppliers', label: 'Suppliers',         icon: '🏭' },
  ];

  function nav(id) { dispatch('navigate', id); }
  
  function logout() {
    if (confirm("Are you sure you want to log out?")) {
      authStore.logout();
      toast.success("Logged out successfully");
    }
  }

  $: user = $authStore.user;
</script>

<aside class="sidebar">
  <!-- Logo / Brand -->
  <div class="brand">
    <div class="brand-icon">⚡</div>
    <div class="brand-text">
      <div class="brand-name">VibeStock</div>
      <div class="brand-sub">Inventory Pro</div>
    </div>
  </div>

  <!-- Navigation -->
  <nav class="nav">
    {#each navItems as item}
      <button
        class="nav-item"
        class:active={activePage === item.id}
        on:click={() => nav(item.id)}
        id="nav-{item.id}"
      >
        <span class="nav-icon">{item.icon}</span>
        <span class="nav-label">{item.label}</span>
        {#if activePage === item.id}
          <span class="nav-indicator"></span>
        {/if}
      </button>
    {/each}
  </nav>

  <!-- User footer -->
  <div class="sidebar-footer">
    <div class="user-card">
      <div class="user-avatar">
        {user?.full_name?.charAt(0).toUpperCase() ?? 'U'}
      </div>
      <div class="user-info">
        <div class="user-name">{user?.full_name ?? 'User'}</div>
        <div class="user-role">{user?.role ?? 'staff'}</div>
      </div>
    </div>
    <button class="logout-btn" on:click={logout} title="Logout">⏏</button>
  </div>
</aside>

<style>
  .sidebar {
    width: 240px;
    min-width: 240px;
    height: 100%;
    background: rgba(255,255,255,0.03);
    border-right: 1px solid var(--border-glass);
    display: flex;
    flex-direction: column;
    padding: var(--space-5) var(--space-3);
    gap: var(--space-6);
    backdrop-filter: blur(20px);
  }

  /* Brand */
  .brand {
    display: flex; align-items: center; gap: var(--space-3);
    padding: 0 var(--space-2);
  }
  .brand-icon {
    width: 36px; height: 36px; border-radius: var(--radius-md);
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-cyan));
    display: flex; align-items: center; justify-content: center;
    font-size: 1.1rem;
    box-shadow: 0 4px 12px rgba(124,58,237,0.4);
  }
  .brand-name { font-weight: 800; font-size: 1rem; letter-spacing: -0.02em; }
  .brand-sub  { font-size: 0.7rem; color: var(--text-muted); letter-spacing: 0.08em; text-transform: uppercase; }

  /* Nav */
  .nav { display: flex; flex-direction: column; gap: var(--space-1); flex: 1; }

  .nav-item {
    display: flex; align-items: center; gap: var(--space-3);
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-md);
    border: none; background: transparent;
    color: var(--text-secondary);
    cursor: pointer; font-size: 0.875rem; font-weight: 500;
    transition: all var(--transition-fast);
    text-align: left; position: relative; width: 100%;
    font-family: var(--font-sans);
  }
  .nav-item:hover {
    background: rgba(255,255,255,0.05);
    color: var(--text-primary);
  }
  .nav-item.active {
    background: rgba(124,58,237,0.15);
    color: var(--accent-glow);
  }
  .nav-icon   { font-size: 1.1rem; width: 20px; text-align: center; }
  .nav-label  { flex: 1; }
  .nav-indicator {
    width: 4px; height: 20px; border-radius: var(--radius-full);
    background: linear-gradient(180deg, var(--accent-primary), var(--accent-cyan));
    position: absolute; right: -3px;
    box-shadow: 0 0 8px rgba(124,58,237,0.6);
  }

  /* Footer */
  .sidebar-footer {
    display: flex; align-items: center; gap: var(--space-2);
    padding: var(--space-3);
    background: rgba(255,255,255,0.03);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-glass);
  }
  .user-card  { display: flex; align-items: center; gap: var(--space-2); flex: 1; min-width: 0; }
  .user-avatar {
    width: 30px; height: 30px; border-radius: var(--radius-full);
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-cyan));
    display: flex; align-items: center; justify-content: center;
    font-size: 0.75rem; font-weight: 700; flex-shrink: 0;
  }
  .user-name { font-size: 0.8125rem; font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .user-role { font-size: 0.7rem; color: var(--text-muted); text-transform: capitalize; }
  .logout-btn {
    background: none; border: none; cursor: pointer;
    color: var(--text-muted); font-size: 1.1rem; padding: 4px;
    border-radius: var(--radius-sm);
    transition: color var(--transition-fast);
  }
  .logout-btn:hover { color: var(--accent-red); }
</style>
