<script>
  /**
   * Login.svelte — Authentication page
   * Calls the Axum API, stores JWT in auth store on success.
   */
  import { api }       from '../lib/api.js';
  import { authStore } from '../stores/auth.js';
  import { toast }     from '../stores/toast.js';
  import { themeStore } from '../stores/theme.js';

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
  <!-- Theme Toggle -->
  <button class="theme-toggle top-right" on:click={themeStore.toggle} title="Toggle Dark/Light Mode">
    {$themeStore === 'dark' ? '☀️' : '🌙'}
  </button>

  <!-- Decorative blobs -->
  <div class="blob blob-1"></div>
  <div class="blob blob-2"></div>

  <div class="login-card card stagger-row">
    <!-- Logo -->
    <div class="login-logo stagger-row">
      <div class="logo-icon">⚡</div>
      <h1 class="logo-title">VibeStock</h1>
      <p class="logo-sub">Inventory Management System</p>
    </div>

    <!-- Form -->
    <form class="login-form" on:submit|preventDefault={handleSubmit}>
      <div class="form-group stagger-row">
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
        <div class="form-group slide-down stagger-row">
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

        <div class="form-group slide-down stagger-row">
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

      <div class="form-group stagger-row">
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
    position: relative; overflow: hidden;
  }

  .theme-toggle.top-right {
    position: absolute; top: var(--space-6); right: var(--space-6); z-index: 10;
    background: var(--glass-card); border: 1px solid var(--border-glass);
    cursor: pointer; font-size: 1.5rem; padding: var(--space-2);
    border-radius: var(--radius-full); box-shadow: var(--shadow-sm);
    color: var(--text-secondary); transition: all var(--transition-fast);
  }
  .theme-toggle.top-right:hover {
    color: var(--text-primary); transform: scale(1.1); background: var(--glass-hover);
  }

  /* Animated ethereal background blobs - Toned Down & GPU Optimized */
  .blob {
    position: absolute; border-radius: 50%;
    opacity: 0.15; pointer-events: none;
    animation: moveBlob 30s infinite alternate cubic-bezier(0.4, 0, 0.2, 1);
  }
  .blob-1 { 
    width: 60vw; height: 60vw; 
    background: radial-gradient(circle, var(--accent-primary) 0%, transparent 60%); 
    top: -20%; left: -10%; 
  }
  .blob-2 { 
    width: 50vw; height: 50vw; 
    background: radial-gradient(circle, var(--accent-cyan) 0%, transparent 60%); 
    bottom: -10%; right: -10%; 
    animation-direction: alternate-reverse; animation-duration: 35s; 
  }
  @keyframes moveBlob {
    0% { transform: translate(0, 0) scale(1); }
    100% { transform: translate(10vw, 15vh) scale(1.1); }
  }

  .login-card {
    width: 420px; max-width: 95vw;
    padding: var(--space-8) var(--space-10);
    position: relative; z-index: 1;
    display: flex; flex-direction: column; gap: var(--space-6);
    background: var(--glass-card);
    border: 1px solid var(--border-glass);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-md);
  }

  .login-logo { text-align: center; display: flex; flex-direction: column; align-items: center; gap: var(--space-3); }
  .logo-icon {
    width: 64px; height: 64px; border-radius: var(--radius-xl);
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-cyan));
    display: flex; align-items: center; justify-content: center;
    font-size: 2rem; color: white;
    box-shadow: 0 12px 32px rgba(139,92,246,0.4), inset 0 2px 0 rgba(255,255,255,0.3);
    animation: float 6s ease-in-out infinite;
  }
  .logo-title { font-size: 2rem; font-weight: 800; margin-top: var(--space-2); letter-spacing: -0.04em; color: var(--text-primary); }
  .logo-sub   { color: var(--text-secondary); font-size: 0.9375rem; letter-spacing: 0.05em; text-transform: uppercase; }

  .login-form  { display: flex; flex-direction: column; gap: var(--space-5); }
  .login-btn   { width: 100%; justify-content: center; padding: 12px; font-size: 1.05rem; margin-top: var(--space-2); }
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
    background: var(--bg-surface);
    border: 1px solid var(--border-surface);
    padding: 1px 4px; border-radius: 4px;
    color: var(--accent-cyan);
    font-weight: 600;
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
