<script>
  import { onMount, onDestroy } from 'svelte';
  import { gsap } from 'gsap';
  import { authStore } from './stores/auth.js';
  import { currentRoute, navigate } from './stores/router.js';
  import { toastStore } from './stores/toast.js';

  import Login from './routes/Login.svelte';
  import Dashboard from './routes/Dashboard.svelte';
  import Products from './routes/Products.svelte';
  import Suppliers from './routes/Suppliers.svelte';
  import Movements from './routes/Movements.svelte';
  import Users from './routes/Users.svelte';
  import Reports from './routes/Reports.svelte';
  import AppShell from './components/AppShell.svelte';
  import Toast from './components/Toast.svelte';

  let route;
  let authenticated;

  const unsubRoute = currentRoute.subscribe(r => route = r);
  const unsubAuth = authStore.subscribe(a => authenticated = a.isAuthenticated);

  onMount(() => {
    gsap.fromTo('#app', 
      { opacity: 0 }, 
      { opacity: 1, duration: 0.5 }
    );
  });

  onDestroy(() => {
    unsubRoute();
    unsubAuth();
  });
</script>

<Toast />

{#if !authenticated && route === '/'}
  <Login />
{:else if authenticated && route === '/'}
  <AppShell>
    <Dashboard />
  </AppShell>
{:else if authenticated}
  <AppShell>
    {#if route === '/dashboard'}
      <Dashboard />
    {:else if route === '/products'}
      <Products />
    {:else if route === '/suppliers'}
      <Suppliers />
    {:else if route === '/movements'}
      <Movements />
    {:else if route === '/users'}
      <Users />
    {:else if route === '/reports'}
      <Reports />
    {/if}
  </AppShell>
{:else}
  <Login />
{/if}
