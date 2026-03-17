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
  import { themeStore }    from './stores/theme.js';

  import Sidebar   from './components/Sidebar.svelte';
  import Login     from './routes/Login.svelte';
  import Dashboard from './routes/Dashboard.svelte';
  import Products  from './routes/Products.svelte';
  import Suppliers from './routes/Suppliers.svelte';
  import Movements from './routes/Movements.svelte';
  import Users     from './routes/Users.svelte';
  import Tenants   from './routes/Tenants.svelte';

  // ── Routing ─────────────────────────────────────────────────────────────────
  let page = 'dashboard'; // default page
  function navigate(to) { page = to; }

  // ── Auth reactive state ──────────────────────────────────────────────────────
  $: isAuthed = !!$authStore.token;

  // Initialize theme tracking (creates data-theme html tag attribute)
  onMount(() => {
    themeStore.init();
  });

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
  <!-- ── Ambient Animated Background ── -->
  <div class="ambient-bg">
    <div class="orb orb-1"></div>
    <div class="orb orb-2"></div>
    <div class="orb orb-3"></div>
  </div>

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
      {:else if page === 'tenants'}
        <Tenants />
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
  /* Ambient Animated Orbs Layer - Toned Down & GPU Optimized */
  .ambient-bg {
    position: fixed; inset: 0; z-index: -1;
    overflow: hidden; background: var(--bg-base);
    pointer-events: none; /* explicitly ignore inputs */
  }
  .orb {
    position: absolute; border-radius: 50%;
    opacity: 0.15;
    animation: moveOrbs 30s infinite alternate cubic-bezier(0.4, 0, 0.2, 1);
  }
  .orb-1 {
    width: 60vw; height: 60vw;
    background: radial-gradient(circle, var(--accent-primary) 0%, transparent 60%);
    top: -10%; left: -10%; animation-duration: 35s;
  }
  .orb-2 {
    width: 50vw; height: 50vw;
    background: radial-gradient(circle, var(--accent-cyan) 0%, transparent 60%);
    bottom: -20%; right: -10%; animation-duration: 40s; animation-direction: alternate-reverse;
  }
  .orb-3 {
    width: 40vw; height: 40vw;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.4) 0%, transparent 60%);
    top: 30%; left: 40%; animation-duration: 45s; animation-delay: -5s;
  }

  @keyframes moveOrbs {
    0%   { transform: translate(0, 0) scale(1); }
    50%  { transform: translate(10vw, 5vh) scale(1.1); }
    100% { transform: translate(-5vw, 15vh) scale(0.9); }
  }

  /* Shell */
  .app-shell {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
    position: relative; z-index: 1; /* Above ambient bg */
    background: transparent; /* Changed from solid to let orbs show */
  }
  .main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background: var(--glass-panel); /* swapped from hardcoded rgba to css var */
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-left: 1px solid var(--border-glass);
    box-shadow: -10px 0 40px rgba(0,0,0,0.5);
  }
  .toast-icon { font-size: 1.25rem; flex-shrink: 0; }
</style>
