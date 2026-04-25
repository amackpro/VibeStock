<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { api, checkConnection } from '../lib/api.js';
  import { authStore } from '../stores/auth.js';
  import { navigate } from '../stores/router.js';
  import { toastStore } from '../stores/toast.js';

  let username = '';
  let password = '';
  let loading = false;
  let error = '';
  let serverConnected = true;

  let container, logoRef, formRef, bgOrbs;

  onMount(async () => {
    serverConnected = await checkConnection();
    if (!serverConnected) {
      error = 'Cannot connect to server. Please ensure the API is running on port 3000.';
      gsap.fromTo('.error-message', 
        { x: -10, opacity: 0 },
        { x: 0, opacity: 1, duration: 0.3 }
      );
    }

    const tl = gsap.timeline();

    tl.fromTo('.login-bg-orb', 
      { scale: 0, opacity: 0 },
      { scale: 1, opacity: 0.6, duration: 1.5, stagger: 0.2, ease: 'power2.out' }
    );

    tl.fromTo(logoRef, 
      { y: -50, opacity: 0, scale: 0.8 },
      { y: 0, opacity: 1, scale: 1, duration: 0.8, ease: 'back.out(1.7)' },
      '-=0.5'
    );

    tl.fromTo('.login-title', 
      { y: 20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.6 },
      '-=0.4'
    );

    tl.fromTo('.login-subtitle', 
      { y: 20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.6 },
      '-=0.4'
    );

    tl.fromTo('.login-form', 
      { y: 40, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.7, ease: 'power3.out' },
      '-=0.3'
    );

    tl.fromTo('.input-group', 
      { x: -30, opacity: 0 },
      { x: 0, opacity: 1, duration: 0.5, stagger: 0.1 },
      '-=0.4'
    );

    tl.fromTo('.login-btn', 
      { y: 20, opacity: 0, scale: 0.9 },
      { y: 0, opacity: 1, scale: 1, duration: 0.5 },
      '-=0.2'
    );

    tl.fromTo('.demo-credentials', 
      { y: 20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5 },
      '-=0.2'
    );
  });

  async function handleLogin() {
    if (!username || !password) {
      error = 'Please enter username and password';
      return;
    }

    loading = true;
    error = '';

    try {
      const response = await api.auth.login({ username, password });
      const user = {
        id: response.user_id,
        username: response.username,
        full_name: response.full_name,
        role: response.role,
        is_global_admin: response.is_global_admin
      };
      const tenant = response.accessible_tenants?.length > 0 
        ? response.accessible_tenants[0] 
        : { id: response.tenant_id, name: response.tenant_name };
      await authStore.login(response.token, user, tenant);
      toastStore.show('Welcome back!', 'success');
      navigate('/dashboard');
    } catch (e) {
      error = e.message || 'Login failed';
      gsap.fromTo('.error-message', 
        { x: -10, opacity: 0 },
        { x: 0, opacity: 1, duration: 0.3 }
      );
    } finally {
      loading = false;
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Enter') handleLogin();
  }
</script>

<div class="login-page" bind:this={container}>
  <div class="login-background">
    <div class="login-bg-orb orb-1"></div>
    <div class="login-bg-orb orb-2"></div>
    <div class="login-bg-orb orb-3"></div>
    <div class="login-grid"></div>
  </div>

  <div class="login-container">
    <div class="login-header">
      <div class="logo" bind:this={logoRef}>
        <div class="logo-icon">
          <svg width="40" height="40" viewBox="0 0 24 24" fill="none">
            <path d="M12 2L2 7l10 5 10-5-10-5z" fill="url(#gradLogin)"/>
            <path d="M2 17l10 5 10-5" stroke="url(#gradLogin)" stroke-width="2"/>
            <path d="M2 12l10 5 10-5" stroke="url(#gradLogin)" stroke-width="2"/>
            <defs>
              <linearGradient id="gradLogin" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#6366f1"/>
                <stop offset="100%" style="stop-color:#22d3ee"/>
              </linearGradient>
            </defs>
          </svg>
        </div>
        <span class="logo-text">VibeStock</span>
      </div>
      
      <h1 class="login-title">Welcome back</h1>
      <p class="login-subtitle">Sign in to manage your inventory</p>
    </div>

    <form class="login-form" bind:this={formRef} on:submit|preventDefault={handleLogin}>
      <div class="input-group">
        <label class="input-label" for="username">Username</label>
        <div class="input-wrapper">
          <span class="input-icon">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"/>
              <circle cx="12" cy="7" r="4"/>
            </svg>
          </span>
          <input 
            type="text" 
            id="username"
            class="input-field" 
            placeholder="Enter your username"
            bind:value={username}
            on:keydown={handleKeydown}
          />
        </div>
      </div>

      <div class="input-group">
        <label class="input-label" for="password">Password</label>
        <div class="input-wrapper">
          <span class="input-icon">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
              <path d="M7 11V7a5 5 0 0110 0v4"/>
            </svg>
          </span>
          <input 
            type="password" 
            id="password"
            class="input-field" 
            placeholder="Enter your password"
            bind:value={password}
            on:keydown={handleKeydown}
          />
        </div>
      </div>

      {#if error}
        <div class="error-message">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 8v4M12 16h.01"/>
          </svg>
          <span>{error}</span>
        </div>
      {/if}

      <button type="submit" class="login-btn" disabled={loading || !serverConnected}>
        {#if loading}
          <span class="spinner"></span>
          Signing in...
        {:else if !serverConnected}
          Server Unavailable
        {:else}
          Sign in
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
        {/if}
      </button>
    </form>

    <div class="demo-credentials">
      <span class="demo-label">Demo credentials</span>
      <code>admin / Password@123</code>
    </div>
  </div>
</div>

<style>
  .login-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
    position: relative;
    overflow: hidden;
  }

  .login-background {
    position: absolute;
    inset: 0;
    pointer-events: none;
  }

  .login-bg-orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
  }

  .orb-1 {
    width: 600px;
    height: 600px;
    background: radial-gradient(circle, rgba(99, 102, 241, 0.3), transparent 70%);
    top: -200px;
    right: -100px;
  }

  .orb-2 {
    width: 500px;
    height: 500px;
    background: radial-gradient(circle, rgba(34, 211, 238, 0.2), transparent 70%);
    bottom: -150px;
    left: -100px;
  }

  .orb-3 {
    width: 400px;
    height: 400px;
    background: radial-gradient(circle, rgba(139, 92, 246, 0.2), transparent 70%);
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .login-grid {
    position: absolute;
    inset: 0;
    background-image: 
      linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px);
    background-size: 60px 60px;
  }

  .login-container {
    width: 100%;
    max-width: 420px;
    padding: 40px;
    position: relative;
    z-index: 1;
  }

  .login-header {
    text-align: center;
    margin-bottom: 40px;
  }

  .logo {
    display: inline-flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 24px;
  }

  .logo-icon {
    display: flex;
  }

  .logo-text {
    font-family: var(--font-display);
    font-size: 1.8rem;
    font-weight: 700;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .login-title {
    font-family: var(--font-display);
    font-size: 2rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 8px;
  }

  .login-subtitle {
    color: var(--text-secondary);
    font-size: 1rem;
  }

  .login-form {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-xl);
    padding: 32px;
    backdrop-filter: blur(20px);
  }

  .input-group {
    margin-bottom: 24px;
  }

  .input-label {
    display: block;
    margin-bottom: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .input-wrapper {
    position: relative;
  }

  .input-icon {
    position: absolute;
    left: 16px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-muted);
    display: flex;
    transition: color var(--transition-fast);
  }

  .input-field {
    width: 100%;
    padding: 14px 16px 14px 48px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: 0.95rem;
    transition: all var(--transition-base);
  }

  .input-field:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
  }

  .input-field:focus + .input-icon,
  .input-field:focus ~ .input-icon {
    color: var(--accent-primary);
  }

  .input-field::placeholder {
    color: var(--text-muted);
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: var(--radius-md);
    color: var(--accent-danger);
    font-size: 0.9rem;
    margin-bottom: 20px;
  }

  .login-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    padding: 16px 24px;
    background: linear-gradient(135deg, var(--accent-primary), #8b5cf6);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-base);
    box-shadow: 0 4px 20px rgba(99, 102, 241, 0.3);
  }

  .login-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 30px rgba(99, 102, 241, 0.4);
  }

  .login-btn:active:not(:disabled) {
    transform: translateY(0);
  }

  .login-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .spinner {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .demo-credentials {
    margin-top: 24px;
    text-align: center;
    padding: 16px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px dashed var(--border-color);
    border-radius: var(--radius-md);
  }

  .demo-label {
    display: block;
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 4px;
  }

  .demo-credentials code {
    font-family: 'DM Sans', monospace;
    color: var(--accent-secondary);
    font-size: 0.9rem;
  }
</style>
