<script>
  /**
   * AppShell.svelte - Main application layout
   * 
   * Provides:
   * - Global header with search, notifications, user menu
   * - Collapsible sidebar with navigation
   * - Responsive layout
   */
  import { createEventDispatcher, onMount } from 'svelte';
  import { authStore } from '../stores/auth.js';
  import { toast } from '../stores/toast.js';
  import { themeStore } from '../stores/theme.js';
  import { api } from '../lib/api.js';

  export let activePage = 'dashboard';
  const dispatch = createEventDispatcher();

  // State
  let sidebarCollapsed = false;
  let showUserMenu = false;
  let showNotifications = false;
  let searchQuery = '';
  let showTenantDropdown = false;

  // Navigation groups
  const navGroups = [
    {
      title: 'Overview',
      items: [
        { id: 'dashboard', label: 'Dashboard', icon: '📊' },
      ]
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

  // Derived state
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

  // Functions
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

  function handleSearch(e) {
    if (e.key === 'Enter' && searchQuery.trim()) {
      dispatch('search', searchQuery);
    }
  }

  // Close dropdowns when clicking outside
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

<div class="app-shell" class:sidebar-collapsed={sidebarCollapsed}>
  <!-- ── Sidebar ── -->
  <aside class="sidebar">
    <!-- Logo -->
    <div class="sidebar-header">
      <div class="logo">
        <div class="logo-icon">⚡</div>
        {#if !sidebarCollapsed}
          <div class="logo-text">
            <span class="logo-name">VibeStock</span>
            <span class="logo-tagline">Inventory Pro</span>
          </div>
        {/if}
      </div>
      <button class="collapse-btn" on:click={toggleSidebar} title={sidebarCollapsed ? 'Expand' : 'Collapse'}>
        {sidebarCollapsed ? '→' : '←'}
      </button>
    </div>

    <!-- Navigation -->
    <nav class="sidebar-nav">
      {#each filteredNavGroups as group, gi}
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

    <!-- Sidebar Footer -->
    <div class="sidebar-footer">
      <button class="theme-btn" on:click={toggleTheme} title="Toggle Theme">
        {$themeStore === 'dark' ? '☀️' : '🌙'}
      </button>
    </div>
  </aside>

  <!-- ── Main Area ── -->
  <div class="main-area">
    <!-- Header -->
    <header class="header">
      <div class="header-left">
        <div class="search-box">
          <span class="search-icon">🔍</span>
          <input
            type="text"
            class="search-input"
            placeholder="Search products, categories..."
            bind:value={searchQuery}
            on:keydown={handleSearch}
          />
          <kbd class="search-kbd">⌘K</kbd>
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
              <span class="tenant-name">{tenant?.name || 'Select Tenant'}</span>
              <span class="dropdown-arrow">▼</span>
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
                    {#if t.id === tenant?.id}✓{/if}
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

        <!-- Notifications -->
        <button class="icon-btn" title="Notifications">
          🔔
        </button>

        <!-- User Menu -->
        <div class="user-menu-container">
          <button 
            class="user-btn"
            on:click|stopPropagation={() => showUserMenu = !showUserMenu}
          >
            <div class="avatar avatar-sm">
              {user?.full_name?.charAt(0).toUpperCase() || 'U'}
            </div>
            <div class="user-info">
              <span class="user-name">{user?.full_name || 'User'}</span>
              <span class="user-role">{user?.role || 'staff'}</span>
            </div>
            <span class="dropdown-arrow">▼</span>
          </button>
          
          {#if showUserMenu}
            <div class="dropdown-menu user-dropdown">
              <div class="dropdown-header">
                <div class="avatar">{user?.full_name?.charAt(0).toUpperCase() || 'U'}</div>
                <div>
                  <div class="dropdown-user-name">{user?.full_name}</div>
                  <div class="dropdown-user-email">{user?.username}</div>
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

    <!-- Page Content -->
    <main class="content">
      <slot></slot>
    </main>
  </div>
</div>

<style>
  /* ── App Shell Layout ── */
  .app-shell {
    display: flex;
    min-height: 100vh;
    background-color: var(--bg-base);
  }

  /* ── Sidebar ── */
  .sidebar {
    width: var(--sidebar-width);
    height: 100vh;
    position: fixed;
    left: 0;
    top: 0;
    background-color: var(--bg-surface);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    transition: width var(--transition-base);
    z-index: var(--z-fixed);
  }

  .sidebar-collapsed .sidebar {
    width: var(--sidebar-collapsed-width);
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4);
    border-bottom: 1px solid var(--border-color);
    min-height: 64px;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .logo-icon {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--color-primary), var(--color-accent));
    border-radius: var(--radius-md);
    font-size: 1.2rem;
    flex-shrink: 0;
  }

  .logo-text {
    display: flex;
    flex-direction: column;
    white-space: nowrap;
  }

  .logo-name {
    font-weight: var(--font-bold);
    font-size: var(--text-base);
    color: var(--text-primary);
    line-height: 1.2;
  }

  .logo-tagline {
    font-size: var(--text-xs);
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .collapse-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    font-size: var(--text-xs);
    transition: all var(--transition-fast);
  }

  .collapse-btn:hover {
    background: var(--color-gray-100);
    color: var(--text-primary);
  }

  .sidebar-collapsed .collapse-btn {
    display: none;
  }

  /* ── Navigation ── */
  .sidebar-nav {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-4);
  }

  .nav-group-title {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: var(--space-2) var(--space-3);
    margin-top: var(--space-4);
  }

  .nav-group-title:first-child {
    margin-top: 0;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    padding: var(--space-2) var(--space-3);
    background: transparent;
    border: none;
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    cursor: pointer;
    transition: all var(--transition-fast);
    position: relative;
    text-align: left;
  }

  .nav-item:hover {
    background: var(--color-gray-100);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: rgba(99, 102, 241, 0.1);
    color: var(--color-primary);
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
    height: 20px;
    background: var(--color-primary);
    border-radius: var(--radius-full);
    position: absolute;
    right: 8px;
  }

  .sidebar-collapsed .nav-item {
    justify-content: center;
    padding: var(--space-3);
  }

  .sidebar-collapsed .nav-label,
  .sidebar-collapsed .nav-indicator {
    display: none;
  }

  /* ── Sidebar Footer ── */
  .sidebar-footer {
    padding: var(--space-4);
    border-top: 1px solid var(--border-color);
  }

  .theme-btn {
    width: 100%;
    padding: var(--space-2);
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    cursor: pointer;
    font-size: 1.1rem;
    transition: all var(--transition-fast);
  }

  .theme-btn:hover {
    background: var(--color-gray-100);
  }

  /* ── Main Area ── */
  .main-area {
    flex: 1;
    margin-left: var(--sidebar-width);
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    transition: margin-left var(--transition-base);
  }

  .sidebar-collapsed .main-area {
    margin-left: var(--sidebar-collapsed-width);
  }

  /* ── Header ── */
  .header {
    height: var(--header-height);
    position: sticky;
    top: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-6);
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border-color);
    z-index: var(--z-sticky);
  }

  .header-left {
    flex: 1;
    max-width: 480px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: var(--color-gray-100);
    border-radius: var(--radius-lg);
    border: 1px solid transparent;
    transition: all var(--transition-fast);
  }

  .search-box:focus-within {
    background: var(--bg-surface);
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px var(--ring-color);
  }

  .search-icon {
    font-size: var(--text-sm);
    opacity: 0.5;
  }

  .search-input {
    flex: 1;
    border: none;
    background: transparent;
    font-size: var(--text-sm);
    color: var(--text-primary);
    outline: none;
  }

  .search-input::placeholder {
    color: var(--text-tertiary);
  }

  .search-kbd {
    padding: 2px 6px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    font-size: var(--text-xs);
    color: var(--text-tertiary);
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  /* ── Tenant ── */
  .tenant-btn,
  .tenant-badge {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: var(--color-gray-100);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    color: var(--text-secondary);
    cursor: pointer;
    border: none;
    transition: all var(--transition-fast);
  }

  .tenant-btn:hover,
  .tenant-badge:hover {
    background: var(--color-gray-200);
  }

  .tenant-name {
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dropdown-arrow {
    font-size: 0.6rem;
    opacity: 0.6;
  }

  .tenant-dropdown-container {
    position: relative;
  }

  /* ── User Button ── */
  .user-btn {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-1) var(--space-2);
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .user-btn:hover {
    background: var(--color-gray-100);
  }

  .user-info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    text-align: left;
  }

  .user-name {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--text-primary);
    line-height: 1.2;
  }

  .user-role {
    font-size: var(--text-xs);
    color: var(--text-tertiary);
    text-transform: capitalize;
  }

  .user-menu-container {
    position: relative;
  }

  /* ── Dropdown ── */
  .dropdown-menu {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    min-width: 200px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    z-index: var(--z-dropdown);
    animation: slideInUp var(--transition-fast) ease-out;
    overflow: hidden;
  }

  .dropdown-header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3);
    border-bottom: 1px solid var(--border-color);
  }

  .dropdown-user-name {
    font-weight: var(--font-medium);
    font-size: var(--text-sm);
  }

  .dropdown-user-email {
    font-size: var(--text-xs);
    color: var(--text-tertiary);
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    width: 100%;
    padding: var(--space-2) var(--space-3);
    background: transparent;
    border: none;
    font-size: var(--text-sm);
    color: var(--text-primary);
    cursor: pointer;
    transition: background var(--transition-fast);
    text-align: left;
  }

  .dropdown-item:hover {
    background: var(--color-gray-100);
  }

  .dropdown-item.active {
    background: rgba(99, 102, 241, 0.1);
    color: var(--color-primary);
  }

  .dropdown-divider {
    height: 1px;
    background: var(--border-color);
    margin: var(--space-1) 0;
  }

  /* ── Content ── */
  .content {
    flex: 1;
    padding: var(--space-6);
    overflow-y: auto;
  }
</style>
