<script>
  /**
   * App.svelte — Root application shell
   *
   * Handles:
   *  - Auth gate: shows Login page if not authenticated
   *  - Client-side routing via a simple `page` string store
   *  - Toast notification container (global)
   *  - WebSocket subscription for real-time stock alerts
   */
  import { onMount, onDestroy } from 'svelte';
  import { authStore }   from './stores/auth.js';
  import { toasts, toast } from './stores/toast.js';
  import { openWebSocket } from './lib/api.js';

  import Sidebar   from './components/Sidebar.svelte';
  import Login     from './routes/Login.svelte';
  import Dashboard from './routes/Dashboard.svelte';
  import Products  from './routes/Products.svelte';
  import Suppliers from './routes/Suppliers.svelte';
  import Movements from './routes/Movements.svelte';
  import Users     from './routes/Users.svelte';

  // ── Routing ─────────────────────────────────────────────────────────────────
  let page = 'dashboard'; // default page
  function navigate(to) { page = to; }

  // ── Auth reactive state ──────────────────────────────────────────────────────
  $: isAuthed = !!$authStore.token;

  // Reset page to dashboard whenever a user logs in
  $: if (isAuthed) {
    page = 'dashboard';
  }

  // ── WebSocket ────────────────────────────────────────────────────────────────
  let ws;

  function handleWsEvent(event) {
    if (event.event === 'LowStock') {
      toast.warning(`⚠️ Low stock: ${event.payload.product_name} (${event.payload.quantity} left)`);
    } else if (event.event === 'StockUpdated') {
      // Notify dashboard to refresh stats
      window.dispatchEvent(new CustomEvent('stock-updated'));
    }
  }

  $: if (isAuthed && !ws) {
    ws = openWebSocket(handleWsEvent);
  }

  onDestroy(() => { ws?.close(); });
</script>

<!-- ── Auth Gate ──────────────────────────────────────────────────────────── -->
{#if !isAuthed}
  <Login />
{:else}
  <!-- ── App Shell ──────────────────────────────────────────────────────────── -->
  <div class="app-shell">
    <Sidebar activePage={page} on:navigate={(e) => navigate(e.detail)} />

    <main class="main-content">
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
      {/if}
    </main>
  </div>
{/if}

<!-- ── Toast Notifications (global) ──────────────────────────────────────── -->
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
  .app-shell {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }
  .main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background: var(--bg-base);
    /* Subtle mesh gradient background */
    background-image:
      radial-gradient(ellipse at 20% 50%, rgba(124,58,237,0.06) 0%, transparent 50%),
      radial-gradient(ellipse at 80% 20%, rgba(6,182,212,0.05) 0%, transparent 50%);
  }
  .toast-icon { font-size: 1rem; flex-shrink: 0; }
</style>
