<script>
  /**
   * AppShell.svelte - Main application layout
   * Uses the original glassmorphism theme from app.css
   */
  import { createEventDispatcher, onMount } from 'svelte';
  import { authStore } from '../stores/auth.js';
  import { toast } from '../stores/toast.js';
  import { themeStore } from '../stores/theme.js';

  export let activePage = 'dashboard';
  const dispatch = createEventDispatcher();

  let sidebarCollapsed = false;
  let showUserMenu = false;
  let searchQuery = '';
  let showTenantDropdown = false;

  const navGroups = [
    {
      title: 'Overview',
      items: [{ id: 'dashboard', label: 'Dashboard', icon: '📊' }]
    },
    {
      title: 'Inventory',
      items: [
        { id: 'products', label: 'Products', icon: '📦' },
        { id: 'suppliers', label: 'Suppliers', icon: '🏭' },
        { id: 'movements', label: 'Movements', icon: '🔄' },
      ]
    },
    {
      title: 'Administration',
      items: [
        { id: 'users', label: 'Users', icon: '👥' },
        { id: 'tenants', label: 'Tenants', icon: '🏢', requires: 'globalAdmin' },
      ]
    }
  ];

  $: user = $authStore.user;
  $: tenant = $authStore.tenant;
  $: accessibleTenants = $authStore.accessibleTenants || [];
  $: isGlobalAdmin = user?.is_global_admin === true;
  
  $: filteredNavGroups = navGroups.map(group => ({
    ...group,
    items: group.items.filter(item => {
      if (item.requires === 'globalAdmin') return isGlobalAdmin;
      return true;
    })
  })).filter(group => group.items.length > 0);

  function navigate(page) {
    dispatch('navigate', page);
  }

  function toggleSidebar() {
    sidebarCollapsed = !sidebarCollapsed;
  }

  function toggleTheme() {
    themeStore.toggle();
  }

  async function switchTenant(tenantId) {
    if (tenantId === tenant?.id) {
      showTenantDropdown = false;
      return;
    }
    authStore.switchTenant(tenantId);
    toast.success(`Switched to ${tenant?.name}`);
    showTenantDropdown = false;
    window.location.reload();
  }

  function logout() {
    if (confirm('Are you sure you want to log out?')) {
      authStore.logout();
      toast.success('Logged out successfully');
    }
  }

  function handleClickOutside(e) {
    if (!e.target.closest('.user-menu-container')) {
      showUserMenu = false;
    }
    if (!e.target.closest('.tenant-dropdown-container')) {
      showTenantDropdown = false;
    }
  }

  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    return () => document.removeEventListener('click', handleClickOutside);
  });
</script>

<div class="app-shell" class:collapsed={sidebarCollapsed}>
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-header">
      <div class="logo">
        <div class="logo-icon">⚡</div>
        {#if !sidebarCollapsed}
          <div class="logo-text">
            <span class="logo-name">VibeStock</span>
          </div>
        {/if}
      </div>
      <button class="collapse-btn" on:click={toggleSidebar}>
        {sidebarCollapsed ? '→' : '←'}
      </button>
    </div>

    <nav class="sidebar-nav">
      {#each filteredNavGroups as group}
        {#if !sidebarCollapsed}
          <div class="nav-group-title">{group.title}</div>
        {/if}
        <div class="nav-group">
          {#each group.items as item}
            <button
              class="nav-item"
              class:active={activePage === item.id}
              on:click={() => navigate(item.id)}
              title={sidebarCollapsed ? item.label : ''}
            >
              <span class="nav-icon">{item.icon}</span>
              {#if !sidebarCollapsed}
                <span class="nav-label">{item.label}</span>
              {/if}
              {#if activePage === item.id}
                <span class="nav-indicator"></span>
              {/if}
            </button>
          {/each}
        </div>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <button class="theme-btn" on:click={toggleTheme}>
        {$themeStore === 'dark' ? '☀️' : '🌙'}
      </button>
    </div>
  </aside>

  <!-- Main Area -->
  <div class="main-area">
    <!-- Header -->
    <header class="header">
      <div class="header-left">
        <div class="search-bar">
          <span>🔍</span>
          <input
            type="text"
            placeholder="Search products, categories..."
            bind:value={searchQuery}
          />
        </div>
      </div>

      <div class="header-right">
        <!-- Tenant Switcher -->
        {#if isGlobalAdmin}
          <div class="tenant-dropdown-container">
            <button 
              class="tenant-btn"
              on:click|stopPropagation={() => showTenantDropdown = !showTenantDropdown}
            >
              <span>🏢</span>
              <span>{tenant?.name || 'Select'}</span>
              <span class="arrow">▼</span>
            </button>
            {#if showTenantDropdown}
              <div class="dropdown-menu">
                {#each accessibleTenants as t}
                  <button 
                    class="dropdown-item" 
                    class:active={t.id === tenant?.id}
                    on:click={() => switchTenant(t.id)}
                  >
                    {t.name}
                    {#if t.id === tenant?.id} ✓{/if}
                  </button>
                {/each}
              </div>
            {/if}
          </div>
        {:else if tenant}
          <div class="tenant-badge">
            <span>🏢</span>
            <span>{tenant.name}</span>
          </div>
        {/if}

        <!-- User Menu -->
        <div class="user-menu-container">
          <button 
            class="user-btn"
            on:click|stopPropagation={() => showUserMenu = !showUserMenu}
          >
            <div class="avatar-sm">
              {user?.full_name?.charAt(0).toUpperCase() || 'U'}
            </div>
            <div class="user-info">
              <span class="user-name">{user?.full_name || 'User'}</span>
              <span class="user-role">{user?.role || 'staff'}</span>
            </div>
          </button>
          
          {#if showUserMenu}
            <div class="dropdown-menu user-dropdown">
              <div class="dropdown-header">
                <div class="avatar-sm">
                  {user?.full_name?.charAt(0).toUpperCase() || 'U'}
                </div>
                <div>
                  <div class="dropdown-user-name">{user?.full_name}</div>
                  <div class="dropdown-user-email">@{user?.username}</div>
                </div>
              </div>
              <div class="dropdown-divider"></div>
              <button class="dropdown-item" on:click={logout}>
                🚪 Logout
              </button>
            </div>
          {/if}
        </div>
      </div>
    </header>

    <!-- Content -->
    <main class="content">
      <slot></slot>
    </main>
  </div>
</div>

<style>
  .app-shell {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  /* Sidebar */
  .sidebar {
    width: 260px;
    height: 100vh;
    position: fixed;
    left: 0;
    top: 0;
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    -webkit-backdrop-filter: blur(var(--glass-blur));
    border-right: 1px solid var(--border-glass);
    display: flex;
    flex-direction: column;
    transition: width 0.3s ease;
    z-index: 100;
  }

  .collapsed .sidebar {
    width: 72px;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px 16px;
    border-bottom: 1px solid var(--border-surface);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .logo-icon {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-cyan));
    border-radius: 10px;
    font-size: 1.2rem;
    flex-shrink: 0;
  }

  .logo-name {
    font-weight: 800;
    font-size: 1.1rem;
    white-space: nowrap;
  }

  .collapse-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid var(--border-glass);
    border-radius: 8px;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 0.75rem;
    transition: all 0.15s ease;
  }

  .collapse-btn:hover {
    background: var(--glass-hover);
    color: var(--text-primary);
  }

  .collapsed .collapse-btn {
    display: none;
  }

  /* Navigation */
  .sidebar-nav {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
  }

  .nav-group-title {
    font-size: 0.7rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 12px 12px 8px;
    margin-top: 8px;
  }

  .nav-group-title:first-child {
    margin-top: 0;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 10px 12px;
    background: transparent;
    border: none;
    border-radius: 10px;
    color: var(--text-secondary);
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    position: relative;
    text-align: left;
  }

  .nav-item:hover {
    background: var(--glass-hover);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: rgba(99, 102, 241, 0.15);
    color: var(--accent-glow);
  }

  .nav-icon {
    font-size: 1.1rem;
    width: 24px;
    text-align: center;
    flex-shrink: 0;
  }

  .nav-label {
    flex: 1;
  }

  .nav-indicator {
    width: 4px;
    height: 18px;
    background: var(--accent-primary);
    border-radius: 4px;
    position: absolute;
    right: 10px;
  }

  .collapsed .nav-item {
    justify-content: center;
    padding: 12px;
  }

  .collapsed .nav-label,
  .collapsed .nav-indicator {
    display: none;
  }

  /* Sidebar Footer */
  .sidebar-footer {
    padding: 16px;
    border-top: 1px solid var(--border-surface);
  }

  .theme-btn {
    width: 100%;
    padding: 10px;
    background: transparent;
    border: 1px solid var(--border-glass);
    border-radius: 10px;
    cursor: pointer;
    font-size: 1.1rem;
    transition: all 0.15s ease;
  }

  .theme-btn:hover {
    background: var(--glass-hover);
  }

  /* Main Area */
  .main-area {
    flex: 1;
    margin-left: 260px;
    display: flex;
    flex-direction: column;
    height: 100vh;
    transition: margin-left 0.3s ease;
  }

  .collapsed .main-area {
    margin-left: 72px;
  }

  /* Header */
  .header {
    height: 64px;
    position: sticky;
    top: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 24px;
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    -webkit-backdrop-filter: blur(var(--glass-blur));
    border-bottom: 1px solid var(--border-glass);
    z-index: 50;
  }

  .header-left {
    flex: 1;
    max-width: 400px;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  /* Tenant */
  .tenant-btn,
  .tenant-badge {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: var(--glass-hover);
    border: 1px solid var(--border-surface);
    border-radius: 10px;
    font-size: 0.85rem;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .tenant-btn:hover,
  .tenant-badge:hover {
    background: var(--border-glass);
  }

  .arrow {
    font-size: 0.6rem;
    opacity: 0.6;
  }

  .tenant-dropdown-container {
    position: relative;
  }

  /* User Button */
  .user-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 10px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .user-btn:hover {
    background: var(--glass-hover);
  }

  .avatar-sm {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-cyan));
    border-radius: 50%;
    font-size: 0.75rem;
    font-weight: 700;
    color: white;
    flex-shrink: 0;
  }

  .user-info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .user-name {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .user-role {
    font-size: 0.7rem;
    color: var(--text-muted);
    text-transform: capitalize;
  }

  .user-menu-container {
    position: relative;
  }

  /* Dropdown */
  .dropdown-menu {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    min-width: 200px;
    background: var(--bg-surface);
    border: 1px solid var(--border-glass);
    border-radius: 12px;
    box-shadow: var(--shadow-md);
    z-index: 200;
    overflow: hidden;
    animation: scaleIn 0.15s ease;
  }

  .dropdown-header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    border-bottom: 1px solid var(--border-surface);
  }

  .dropdown-user-name {
    font-weight: 600;
    font-size: 0.9rem;
  }

  .dropdown-user-email {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .dropdown-divider {
    height: 1px;
    background: var(--border-surface);
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 10px 12px;
    background: transparent;
    border: none;
    font-size: 0.85rem;
    color: var(--text-primary);
    cursor: pointer;
    transition: background 0.15s ease;
    text-align: left;
  }

  .dropdown-item:hover {
    background: var(--glass-hover);
  }

  .dropdown-item.active {
    background: rgba(99, 102, 241, 0.1);
    color: var(--accent-glow);
  }

  /* Content */
  .content {
    flex: 1;
    overflow-y: auto;
  }

  @keyframes scaleIn {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }
</style>
