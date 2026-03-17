<script>
  /**
   * App.svelte — Root application component
   * 
   * Handles:
   *  - Auth gate: shows Login page if not authenticated
   *  - AppShell layout with sidebar and header
   *  - Client-side routing
   *  - Toast notifications
   *  - WebSocket for real-time stock alerts
   */
  import { onMount, onDestroy } from 'svelte';
  import { authStore } from './stores/auth.js';
  import { toasts, toast } from './stores/toast.js';
  import { openWebSocket } from './lib/api.js';
  import { themeStore } from './stores/theme.js';

  import AppShell from './components/AppShell.svelte';
  import Login from './routes/Login.svelte';
  import Dashboard from './routes/Dashboard.svelte';
  import Products from './routes/Products.svelte';
  import Suppliers from './routes/Suppliers.svelte';
  import Movements from './routes/Movements.svelte';
  import Users from './routes/Users.svelte';
  import Tenants from './routes/Tenants.svelte';

  import './app.css';

  // Routing
  let page = 'dashboard';
  function navigate(e) {
    page = e.detail;
  }

  // Auth
  $: isAuthed = !!$authStore.token;

  // Theme
  onMount(() => {
    themeStore.init();
  });

  // WebSocket
  let ws;

  function handleWsEvent(event) {
    if (event.event === 'LowStock') {
      toast.warning(`⚠️ Low stock: ${event.payload.product_name} (${event.payload.quantity} left)`);
    } else if (event.event === 'StockUpdated') {
      window.dispatchEvent(new CustomEvent('stock-updated'));
    }
  }

  $: if (isAuthed && !ws) {
    ws = openWebSocket(handleWsEvent);
  }

  onDestroy(() => {
    ws?.close();
  });
</script>

{#if !isAuthed}
  <Login />
{:else}
  <AppShell activePage={page} on:navigate={navigate}>
    {#if page === 'dashboard'}
      <Dashboard />
    {:else if page === 'products'}
      <Products />
    {:else if page === 'suppliers'}
      <Suppliers />
    {:else if page === 'movements'}
      <Movements />
    {:else if page === 'users'}
      <Users />
    {:else if page === 'tenants'}
      <Tenants />
    {/if}
  </AppShell>
{/if}

<!-- Toast Notifications -->
<div class="toast-container">
  {#each $toasts as t (t.id)}
    <div class="toast toast-{t.type}">
      <span class="toast-icon">
        {#if t.type === 'success'}✅{:else if t.type === 'error'}❌{:else}⚠️{/if}
      </span>
      <span>{t.message}</span>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: var(--space-6);
    right: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    z-index: 9999;
    pointer-events: none;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    background: var(--bg-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    border: 1px solid var(--border-color);
    animation: slideInUp var(--transition-base) ease-out;
    pointer-events: auto;
    max-width: 400px;
  }

  .toast-success {
    border-left: 4px solid var(--color-success);
  }

  .toast-error {
    border-left: 4px solid var(--color-danger);
  }

  .toast-warning {
    border-left: 4px solid var(--color-warning);
  }

  .toast-icon {
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  @keyframes slideInUp {
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
