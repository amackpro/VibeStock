<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { api, checkConnection } from '../lib/api.js';
  import { navigate } from '../stores/router.js';
  import { toastStore } from '../stores/toast.js';

  // ── Mode ──────────────────────────────────────────────────────────────────────
  let mode = 'new'; // 'new' | 'join'

  // ── Fields ────────────────────────────────────────────────────────────────────
  let fullName    = '';
  let username    = '';
  let email       = '';
  let password    = '';
  let tenantName  = '';  // mode === 'new'
  let tenantSlug  = '';  // mode === 'join'

  // ── Orgs for join dropdown ────────────────────────────────────────────────────
  let orgs        = [];
  let orgsLoading = false;

  // ── UI state ──────────────────────────────────────────────────────────────────
  let loading         = false;
  let error           = '';
  let successMsg      = '';
  let serverConnected = true;

  let logoRef, formRef;

  async function fetchOrgs() {
    orgsLoading = true;
    try {
      orgs = await api.auth.orgs();
    } catch (_) {
      orgs = [];
    } finally {
      orgsLoading = false;
    }
  }

  onMount(async () => {
    serverConnected = await checkConnection();
    if (!serverConnected) {
      error = 'Cannot connect to server. Please ensure the API is running on port 3000.';
    }

    fetchOrgs();

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
  });

  function setMode(m) {
    mode       = m;
    error      = '';
    successMsg = '';
    tenantName = '';
    tenantSlug = '';
  }

  async function handleRegister() {
    if (!fullName || !username || !email || !password) {
      error = 'Please fill in all fields';
      return;
    }
    if (mode === 'new' && !tenantName.trim()) {
      error = 'Please enter your new organization name';
      return;
    }
    if (mode === 'join' && !tenantSlug) {
      error = 'Please select an organization to join';
      return;
    }

    loading    = true;
    error      = '';
    successMsg = '';

    try {
      const payload = {
        full_name: fullName,
        username,
        email,
        password,
        mode,
        ...(mode === 'new'
          ? { tenant_name: tenantName.trim() }
          : { tenant_slug: tenantSlug }),
      };

      const res = await api.auth.register(payload);

      if (mode === 'join') {
        successMsg = res.message ||
          'Join request submitted! Your account is pending activation by the organization admin.';
      } else {
        toastStore.show('Organization created! You can now log in.', 'success');
        navigate('/');
      }
    } catch (e) {
      error = e.message || 'Registration failed';
      gsap.fromTo('.error-message',
        { x: -10, opacity: 0 },
        { x: 0, opacity: 1, duration: 0.3 }
      );
    } finally {
      loading = false;
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Enter') handleRegister();
  }
</script>

<div class="login-page">
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
            <path d="M12 2L2 7l10 5 10-5-10-5z" fill="url(#gradReg)"/>
            <path d="M2 17l10 5 10-5" stroke="url(#gradReg)" stroke-width="2"/>
            <path d="M2 12l10 5 10-5" stroke="url(#gradReg)" stroke-width="2"/>
            <defs>
              <linearGradient id="gradReg" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#6366f1"/>
                <stop offset="100%" style="stop-color:#22d3ee"/>
              </linearGradient>
            </defs>
          </svg>
        </div>
        <span class="logo-text">NexStock</span>
      </div>

      <h1 class="login-title">Create an Account</h1>
      <p class="login-subtitle">
        {mode === 'new'
          ? "Start fresh — you'll be the admin of your new organization"
          : "Request to join an existing organization"}
      </p>
    </div>

    {#if successMsg}
      <!-- ── Pending Approval Screen ─────────────────────────────────────── -->
      <div class="success-card">
        <div class="success-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M9 12l2 2 4-4"/>
            <circle cx="12" cy="12" r="10"/>
          </svg>
        </div>
        <h2>Request Submitted!</h2>
        <p>{successMsg}</p>
        <button class="login-btn" on:click={() => navigate('/')}>
          Back to Login
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
        </button>
      </div>
    {:else}
      <form class="login-form" bind:this={formRef} on:submit|preventDefault={handleRegister}>

        <!-- ── Mode Toggle ───────────────────────────────────────────────── -->
        <div class="mode-toggle">
          <button
            type="button"
            class="mode-btn"
            class:active={mode === 'new'}
            on:click={() => setMode('new')}
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 5v14M5 12h14"/>
            </svg>
            New Organization
          </button>
          <button
            type="button"
            class="mode-btn"
            class:active={mode === 'join'}
            on:click={() => setMode('join')}
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2"/>
              <circle cx="9" cy="7" r="4"/>
              <path d="M23 21v-2a4 4 0 00-3-3.87"/>
              <path d="M16 3.13a4 4 0 010 7.75"/>
            </svg>
            Join Existing
          </button>
        </div>

        <!-- ── Personal Fields ───────────────────────────────────────────── -->
        <div class="input-group">
          <label class="input-label" for="fullName">Full Name</label>
          <div class="input-wrapper">
            <span class="input-icon">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"/>
                <circle cx="12" cy="7" r="4"/>
              </svg>
            </span>
            <input
              type="text"
              id="fullName"
              class="input-field"
              placeholder="John Doe"
              bind:value={fullName}
              on:keydown={handleKeydown}
            />
          </div>
        </div>

        <div class="input-group">
          <label class="input-label" for="username">Username</label>
          <div class="input-wrapper">
            <span class="input-icon">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="16" x2="12" y2="12"/>
                <line x1="12" y1="8" x2="12.01" y2="8"/>
              </svg>
            </span>
            <input
              type="text"
              id="username"
              class="input-field"
              placeholder="johndoe"
              bind:value={username}
              on:keydown={handleKeydown}
            />
          </div>
        </div>

        <div class="input-group">
          <label class="input-label" for="email">Email Address</label>
          <div class="input-wrapper">
            <span class="input-icon">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/>
                <polyline points="22,6 12,13 2,6"/>
              </svg>
            </span>
            <input
              type="email"
              id="email"
              class="input-field"
              placeholder="john@example.com"
              bind:value={email}
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
              placeholder="Create a strong password"
              bind:value={password}
              on:keydown={handleKeydown}
            />
          </div>
        </div>

        <!-- ── Conditional org field ─────────────────────────────────────── -->
        {#if mode === 'new'}
          <div class="input-group">
            <label class="input-label" for="tenantName">Organization Name</label>
            <div class="input-wrapper">
              <span class="input-icon">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
                  <polyline points="9 22 9 12 15 12 15 22"/>
                </svg>
              </span>
              <input
                type="text"
                id="tenantName"
                class="input-field"
                placeholder="e.g. Acme Corp (must be unique)"
                bind:value={tenantName}
                on:keydown={handleKeydown}
              />
            </div>
            <p class="field-hint">You'll become the admin of this new organization.</p>
          </div>
        {:else}
          <div class="input-group">
            <label class="input-label" for="tenantSlug">Select Organization</label>
            {#if orgsLoading}
              <div class="orgs-loading">
                <span class="mini-spinner"></span>
                Loading organizations…
              </div>
            {:else if orgs.length === 0}
              <div class="orgs-empty">
                No active organizations found. Ask your admin, or create a new one.
              </div>
            {:else}
              <div class="input-wrapper">
                <span class="input-icon">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
                    <polyline points="9 22 9 12 15 12 15 22"/>
                  </svg>
                </span>
                <select
                  id="tenantSlug"
                  class="input-field select-field"
                  bind:value={tenantSlug}
                >
                  <option value="">— choose an organization —</option>
                  {#each orgs as org}
                    <option value={org.slug}>{org.name}</option>
                  {/each}
                </select>
              </div>
            {/if}
            <p class="field-hint">Your account will be inactive until the org admin approves it.</p>
          </div>
        {/if}

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
         