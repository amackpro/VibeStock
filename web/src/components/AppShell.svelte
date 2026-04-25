<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { currentRoute, navigate, getRouteName } from '../stores/router.js';
  import { authStore } from '../stores/auth.js';
  import { api } from '../lib/api.js';

  let sidebar;
  let user = null;
  let tenants = [];
  let selectedTenant = null;
  let showTenantMenu = false;
  let hoverIndex = -1;

  authStore.subscribe(a => {
    user = a.user;
    selectedTenant = a.tenant;
  });

  async function loadTenants() {
    if (user?.role === 'admin' || user?.is_global_admin) {
      try {
        const result = await api.tenants.list();
        tenants = result.data || result;
      } catch (e) {
        console.error('Failed to load tenants:', e);
      }
    }
  }

  function handleTenantChange(tenant) {
    if (tenant.id === selectedTenant?.id) return;
    authStore.setTenant(tenant);
    showTenantMenu = false;
    // Hard reload to refresh all data stores for the new tenant
    window.location.reload();
  }

  const menuItems = [
    { path: '/dashboard', icon: 'dashboard', label: 'Dashboard' },
    { path: '/products', icon: 'products', label: 'Products' },
    { path: '/suppliers', icon: 'suppliers', label: 'Suppliers' },
    { path: '/movements', icon: 'movements', label: 'Movements' },
    { path: '/users', icon: 'users', label: 'Users' },
    { path: '/reports', icon: 'reports', label: 'Reports' }
  ];

  function handleNav(path) {
    if (path === currentPath) return;
    gsap.to('.main-content', {
      opacity: 0,
      x: -20,
      duration: 0.2,
      ease: 'power2.in',
      onComplete: () => {
        navigate(path);
        gsap.fromTo('.main-content', 
          { opacity: 0, x: 20 },
          { opacity: 1, x: 0, duration: 0.35, ease: 'power2.out', clearProps: 'all' }
        );
      }
    });
  }

  function handleMouseEnter(index) {
    hoverIndex = index;
    gsap.to(`.nav-item-${index}`, {
      x: 6,
      duration: 0.25,
      ease: 'power2.out'
    });
  }

  function handleMouseLeave(index) {
    hoverIndex = -1;
    gsap.to(`.nav-item-${index}`, {
      x: 0,
      duration: 0.25,
      ease: 'power2.out'
    });
  }

  function logout() {
    gsap.to('.app-shell', {
      opacity: 0,
      duration: 0.3,
      onComplete: () => {
        authStore.logout();
        navigate('/');
      }
    });
  }

  let currentPath = '/';
  currentRoute.subscribe(p => currentPath = p);

  onMount(() => {
    loadTenants();
    const tl = gsap.timeline();

    tl.fromTo('.sidebar', 
      { x: -100, opacity: 0 },
      { x: 0, opacity: 1, duration: 0.6, ease: 'power3.out' }
    );

    tl.fromTo('.logo', 
      { y: -20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'back.out(1.5)' },
      '-=0.3'
    );

    tl.fromTo('.nav-item', 
      { x: -30, opacity: 0 },
      { x: 0, opacity: 1, duration: 0.4, stagger: 0.08, ease: 'power3.out' },
      '-=0.2'
    );

    tl.fromTo('.sidebar-footer', 
      { y: 20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.4, ease: 'power2.out' },
      '-=0.2'
    );

    tl.fromTo('.header', 
      { y: -30, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' },
      '-=0.3'
    );

    tl.fromTo('.main-content', 
      { opacity: 0, y: 30 },
      { opacity: 1, y: 0, duration: 0.5, ease: 'power3.out', clearProps: 'all' },
      '-=0.2'
    );
  });
</script>

<div class="app-shell">
  <aside class="sidebar" bind:this={sidebar}>
    <div class="logo">
      <div class="logo-icon">
        <svg width="28" height="28" viewBox="0 0 24 24" fill="none">
          <path d="M12 2L2 7l10 5 10-5-10-5z" fill="url(#grad1)"/>
          <path d="M2 17l10 5 10-5" stroke="url(#grad1)" stroke-width="2"/>
          <path d="M2 12l10 5 10-5" stroke="url(#grad1)" stroke-width="2"/>
          <defs>
            <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" style="stop-color:#6366f1"/>
              <stop offset="100%" style="stop-color:#22d3ee"/>
            </linearGradient>
          </defs>
        </svg>
      </div>
      <span class="logo-text">VibeStock</span>
    </div>

    <nav class="nav-menu">
      {#each menuItems as item, i}
        <button 
          class="nav-item nav-item-{i}" 
          class:active={currentPath === item.path}
          on:click={() => handleNav(item.path)}
          on:mouseenter={() => handleMouseEnter(i)}
          on:mouseleave={() => handleMouseLeave(i)}
        >
          <span class="nav-icon">
            {#if item.icon === 'dashboard'}
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="3" y="3" width="7" height="9" rx="1"/>
                <rect x="14" y="3" width="7" height="5" rx="1"/>
                <rect x="14" y="12" width="7" height="9" rx="1"/>
                <rect x="3" y="16" width="7" height="5" rx="1"/>
              </svg>
            {:else if item.icon === 'products'}
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4a2 2 0 00-1 1.73v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4a2 2 0 001-1.73z"/>
                <path d="M3.27 6.96L12 12.01l8.73-5.05M12 22.08V12"/>
              </svg>
            {:else if item.icon === 'suppliers'}
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M16 21v-2a4 4 0 00-4-4H6a4 4 0 00-4 4v2"/>
                <circle cx="9" cy="7" r="4"/>
                <path d="M22 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75"/>
              </svg>
            {:else if item.icon === 'movements'}
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2v20M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6"/>
              </svg>
            {:else if item.icon === 'users'}
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2"/>
                <circle cx="9" cy="7" r="4"/>
                <path d="M23 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75"/>
              </svg>
            {:else if item.icon === 'reports'}
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M18 20V10M12 20V4M6 20v-6"/>
              </svg>
            {/if}
          </span>
          <span class="nav-label">{item.label}</span>
          {#if currentPath === item.path}
            <span class="nav-indicator"></span>
          {/if}
        </button>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <div class="user-info">
        <div class="user-avatar">
          {user?.username?.charAt(0).toUpperCase() || 'U'}
        </div>
        <div class="user-details">
          <span class="user-name">{user?.username || 'User'}</span>
          <span class="user-role">{user?.role || 'Staff'}</span>
        </div>
      </div>
      <button class="logout-btn" on:click={logout} title="Logout">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4M16 17l5-5-5-5M21 12H9"/>
        </svg>
      </button>
    </div>
  </aside>

  <main class="main-area">
    <header class="header">
      <h1 class="page-title">{getRouteName(currentPath)}</h1>
      <div class="header-actions">
        {#if tenants.length > 1 || user?.is_global_admin}
          <div class="tenant-switcher">
            <button class="tenant-btn" on:click={() => showTenantMenu = !showTenantMenu}>
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 21h18M3 7v1a3 3 0 006 0V7m6 0v1a3 3 0 006 0V7m-9 0h.01M4 21V10m16 11V10M12 21V10M7 21h10M12 7V3m0 0a3 3 0 110 6 3 3 0 010-6z"/>
              </svg>
              <span class="tenant-name">{selectedTenant?.name || 'Select Tenant'}</span>
              <svg class="chevron" class:open={showTenantMenu} width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M19 9l-7 7-7-7"/>
              </svg>
            </button>

            {#if showTenantMenu}
              <div class="tenant-dropdown">
                <div class="dropdown-header">Switch Organization</div>
                <div class="tenant-list">
                  {#each tenants as t}
                    <button 
                      class="tenant-option" 
                      class:active={t.id === selectedTenant?.id}
                      on:click={() => handleTenantChange(t)}
                    >
                      <div class="tenant-option-info">
                        <span class="t-name">{t.name}</span>
                        <span class="t-slug">{t.slug}</span>
                      </div>
                      {#if t.id === selectedTenant?.id}
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                          <path d="M20 6L9 17l-5-5"/>
                        </svg>
                      {/if}
                    </button>
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        {/if}

        <div class="connection-status online">
          <span class="status-dot"></span>
          <span>Live</span>
        </div>
      </div>
    </header>

    <div class="main-content">
      <slot />
    </div>
  </main>
</div>

<style>
  .app-shell {
    display: flex;
    min-height: 100vh;
    background: var(--bg-primary);
  }

  .sidebar {
    width: 260px;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    position: fixed;
    height: 100vh;
    z-index: 100;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 24px;
    border-bottom: 1px solid var(--border-color);
  }

  .logo-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .logo-text {
    font-family: var(--font-display);
    font-size: 1.4rem;
    font-weight: 700;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .nav-menu {
    flex: 1;
    padding: 16px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: 0.9rem;
    font-weight: 500;
    transition: all var(--transition-base);
    position: relative;
    overflow: hidden;
  }

  .nav-indicator {
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    width: 3px;
    height: 0;
    background: var(--accent-primary);
    border-radius: 0 2px 2px 0;
    transition: height 0.3s ease;
  }

  .nav-item.active .nav-indicator {
    height: 24px;
  }

  .nav-item:hover {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: rgba(99, 102, 241, 0.15);
    color: var(--accent-primary);
  }

  .nav-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: transform 0.2s ease;
  }

  .nav-item:hover .nav-icon {
    transform: scale(1.1);
  }

  .sidebar-footer {
    padding: 16px;
    border-top: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .user-info {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .user-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--accent-primary), #8b5cf6);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 1rem;
    color: white;
  }

  .user-details {
    display: flex;
    flex-direction: column;
  }

  .user-name {
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text-primary);
  }

  .user-role {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .logout-btn {
    padding: 8px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    transition: all var(--transition-fast);
  }

  .logout-btn:hover {
    background: rgba(239, 68, 68, 0.1);
    color: var(--accent-danger);
  }

  .main-area {
    flex: 1;
    margin-left: 260px;
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .header {
    height: 72px;
    padding: 0 32px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    position: sticky;
    top: 0;
    z-index: 50;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .connection-status {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 14px;
    border-radius: 100px;
    font-size: 0.8rem;
    font-weight: 600;
  }

  .connection-status.online {
    background: rgba(16, 185, 129, 0.1);
    color: var(--accent-success);
  }

  .connection-status.offline {
    background: rgba(239, 68, 68, 0.1);
    color: var(--accent-danger);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: currentColor;
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(0.9); }
  }

  .tenant-switcher {
    position: relative;
    z-index: 1000;
  }

  .tenant-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 16px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: 0.9rem;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .tenant-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: var(--accent-primary);
  }

  .tenant-name {
    max-width: 150px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chevron {
    transition: transform 0.2s ease;
    color: var(--text-muted);
  }

  .chevron.open {
    transform: rotate(180deg);
  }

  .tenant-dropdown {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    width: 280px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-xl);
    overflow: hidden;
    animation: dropdownFade 0.2s ease-out;
  }

  @keyframes dropdownFade {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .dropdown-header {
    padding: 12px 16px;
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    background: rgba(255, 255, 255, 0.02);
    border-bottom: 1px solid var(--border-color);
  }

  .tenant-list {
    max-height: 300px;
    overflow-y: auto;
    padding: 4px;
  }

  .tenant-option {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    transition: all 0.2s ease;
    text-align: left;
  }

  .tenant-option:hover {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-primary);
  }

  .tenant-option.active {
    background: rgba(99, 102, 241, 0.1);
    color: var(--accent-primary);
  }

  .tenant-option-info {
    display: flex;
    flex-direction: column;
  }

  .t-name {
    font-size: 0.9rem;
    font-weight: 600;
  }

  .t-slug {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .main-content {
    flex: 1;
    padding: 32px;
  }
</style>
