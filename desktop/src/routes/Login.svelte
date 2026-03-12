<script>
  /**
   * Login.svelte — Authentication page
   * Calls the Axum API, stores JWT in auth store on success.
   */
  import { api }       from '../lib/api.js';
  import { authStore } from '../stores/auth.js';
  import { toast }     from '../stores/toast.js';

  let isRegisterMode = false;
  let username = '';
  let password = '';
  let email = '';
  let full_name = '';
  let loading  = false;
  let error    = '';

  async function handleSubmit() {
    if (isRegisterMode) {
      if (!username || !password || !email || !full_name) { error = 'Please fill in all fields.'; return; }
      loading = true; error = '';
      try {
        await api.auth.register({ username, password, email, full_name });
        toast.success("Account created successfully! Please log in.");
        isRegisterMode = false;
        password = ''; // clear password on switch
      } catch (e) {
        error = e.message ?? 'Registration failed.';
      } finally {
        loading = false;
      }
    } else {
      if (!username || !password) { error = 'Please fill in all fields.'; return; }
      loading = true; error = '';
      try {
        const resp = await api.auth.login({ username, password });
        authStore.login(resp);
      } catch (e) {
        error = e.message ?? 'Login failed. Please try again.';
      } finally {
        loading = false;
      }
    }
  }

  function toggleMode() {
    isRegisterMode = !isRegisterMode;
    error = '';
  }

  function onKey(e) { if (e.key === 'Enter') handleSubmit(); }
</script>

<!-- Full-screen login with animated gradient mesh background -->
<div class="login-screen">
  <!-- Decorative blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>

  <div class="login-card card">
    <!-- Logo -->
    <div class="login-logo">
      <div class="logo-icon">⚡</div>
      <h1 class="logo-title">VibeStock</h1>
      <p class="logo-sub">Inventory Management System</p>
    </div>

    <!-- Form -->
    <form class="login-form" on:submit|preventDefault={handleSubmit}>
      <div class="form-group">
        <label class="label" for="username">Username</label>
        <input
          id="username"
          class="input"
          type="text"
          placeholder="admin"
          bind:value={username}
          on:keydown={onKey}
          autocomplete="username"
          disabled={loading}
        />
      </div>

      {#if isRegisterMode}
        <div class="form-group slide-down">
          <label class="label" for="full_name">Full Name</label>
          <input
            id="full_name"
            class="input"
            type="text"
            placeholder="John Doe"
            bind:value={full_name}
            on:keydown={onKey}
            autocomplete="name"
            disabled={loading}
          />
        </div>

        <div class="form-group slide-down">
          <label class="label" for="email">Email</label>
          <input
            id="email"
            class="input"
            type="email"
            placeholder="john@example.com"
            bind:value={email}
            on:keydown={onKey}
            autocomplete="email"
            disabled={loading}
          />
        </div>
      {/if}

      <div class="form-group">
        <label class="label" for="password">Password</label>
        <input
          id="password"
          class="input"
          type="password"
          placeholder="••••••••••"
          bind:value={password}
          on:keydown={onKey}
          autocomplete="current-password"
          disabled={loading}
        />
      </div>

      {#if error}
        <div class="error-msg">❌ {error}</div>
      {/if}

      <button
        class="btn btn-primary login-btn"
        type="submit"
        disabled={loading}
        id="btn-login"
      >
        {#if loading}
          <div class="spinner" style="width:16px;height:16px;border-width:2px;"></div>
          {isRegisterMode ? 'Creating...' : 'Signing in…'}
        {:else}
          {isRegisterMode ? 'Create Account' : 'Sign In →'}
        {/if}
      </button>

      <div class="toggle-mode">
        {#if isRegisterMode}
          Already have an account? <button type="button" class="link-btn" on:click={toggleMode}>Sign in</button>
        {:else}
          Need an account? <button type="button" class="link-btn" on:click={toggleMode}>Register</button>
        {/if}
      </div>
    </form>

    {#if !isRegisterMode}
      <p class="demo-hint">Demo: <code>admin</code> / <code>Password@123</code></p>
    {/if}
  </div>
</div>

<style>
  .login-screen {
    width: 100%; height: 100%;
    display: flex; align-items: center; justify-content: center;
    background: var(--bg-base);
    background-image:
      radial-gradient(ellipse at 30% 40%, rgba(124,58,237,0.12) 0%, transparent 55%),
      radial-gradient(ellipse at 75% 65%, rgba(6,182,212,0.08) 0%, transparent 50%);
    position: relative; overflow: hidden;
  }

  /* Animated background blobs */
  .blob {
    position: absolute; border-radius: 50%;
    filter: blur(80px); opacity: 0.35;
    animation: float 8s ease-in-out infinite;
  }
  .blob-1 { width: 400px; height: 400px; background: var(--accent-primary); top: -100px; left: -100px; animation-delay: 0s; }
  .blob-2 { width: 300px; height: 300px; background: var(--accent-cyan);    bottom: -80px; right: -80px; animation-delay: 3s; }
  @keyframes float {
    0%,100% { transform: translateY(0) scale(1); }
    50%      { transform: translateY(-30px) scale(1.05); }
  }

  .login-card {
    width: 400px; max-width: 95vw;
    padding: var(--space-8);
    position: relative; z-index: 1;
    display: flex; flex-direction: column; gap: var(--space-6);
  }

  .login-logo { text-align: center; display: flex; flex-direction: column; align-items: center; gap: var(--space-2); }
  .logo-icon {
    width: 60px; height: 60px; border-radius: var(--radius-lg);
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-cyan));
    display: flex; align-items: center; justify-content: center;
    font-size: 1.8rem;
    box-shadow: 0 8px 24px rgba(124,58,237,0.4);
  }
  .logo-title { font-size: 1.75rem; font-weight: 800; margin-top: var(--space-2); }
  .logo-sub   { color: var(--text-secondary); font-size: 0.875rem; }

  .login-form  { display: flex; flex-direction: column; gap: var(--space-4); }
  .login-btn   { width: 100%; justify-content: center; padding: var(--space-3); font-size: 1rem; }
  .login-btn:disabled { opacity: 0.6; cursor: not-allowed; transform: none !important; }

  .error-msg {
    background: rgba(239,68,68,0.1);
    border: 1px solid rgba(239,68,68,0.25);
    border-radius: var(--radius-md);
    padding: var(--space-2) var(--space-3);
    font-size: 0.8125rem; color: var(--accent-red);
  }

  .demo-hint {
    text-align: center; font-size: 0.75rem; color: var(--text-muted);
    padding-top: var(--space-2);
  }
  code {
    font-family: var(--font-mono);
    background: var(--glass-bg);
    padding: 1px 4px; border-radius: 4px;
    color: var(--accent-cyan);
  }

  .toggle-mode {
    text-align: center; font-size: 0.8125rem; color: var(--text-secondary);
    margin-top: var(--space-2);
  }
  .link-btn {
    background: none; border: none; cursor: pointer; color: var(--accent-cyan);
    font-weight: 600; font-size: 0.8125rem; padding: 0;
  }
  .link-btn:hover { text-decoration: underline; color: var(--accent-primary); }

  .slide-down {
    animation: slideDown 0.3s ease forwards;
    transform-origin: top;
  }
  @keyframes slideDown {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
