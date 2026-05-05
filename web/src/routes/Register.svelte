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

  // ── OTP state ─────────────────────────────────────────────────────────────────
  let otpSent     = false;
  let otpSending  = false;
  let otp         = '';
  let otpError    = '';
  let otpVerified = false;   // true only after backend confirms the code
  let otpChecking = false;   // spinner while verifying

  // ── Live availability state ───────────────────────────────────────────────────
  // 'idle' | 'checking' | 'available' | 'taken' | 'error'
  let usernameStatus = 'idle';
  let emailStatus    = 'idle';
  let usernameTimer  = null;
  let emailTimer     = null;

  function onUsernameInput() {
    usernameStatus = 'idle';
    clearTimeout(usernameTimer);
    if (username.length < 3) return;
    usernameStatus = 'checking';
    usernameTimer = setTimeout(async () => {
      try {
        const res = await api.auth.checkUsername(username.trim());
        usernameStatus = res.available ? 'available' : 'taken';
      } catch (_) {
        usernameStatus = 'error';
      }
    }, 500);
  }

  function onEmailInput() {
    emailStatus = 'idle';
    clearTimeout(emailTimer);
    // Reset OTP if they change the email after sending
    if (otpSent) { otpSent = false; otp = ''; otpError = ''; otpVerified = false; }
    if (!email.includes('@')) return;
    emailStatus = 'checking';
    emailTimer = setTimeout(async () => {
      try {
        const res = await api.auth.checkEmail(email.trim());
        emailStatus = res.available ? 'available' : 'taken';
      } catch (_) {
        emailStatus = 'error';
      }
    }, 600);
  }

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
    // Reset OTP when switching mode
    otpSent  = false;
    otp      = '';
    otpError = '';
  }

  async function handleSendOtp() {
    if (!email.trim()) {
      error = 'Please enter your email address first';
      return;
    }
    if (emailStatus === 'taken') {
      otpError = 'This email is already registered. Please log in instead.';
      return;
    }
    if (emailStatus === 'checking') return; // wait for check to finish
    otpSending = true;
    otpError   = '';
    error      = '';
    try {
      await api.auth.sendOtp(email.trim());
      otpSent = true;
      toastStore.show('OTP sent! Check your inbox.', 'success');
    } catch (e) {
      otpError = e.message || 'Failed to send OTP';
      if (e.message && e.message.toLowerCase().includes('already registered')) {
        emailStatus = 'taken';
      }
    } finally {
      otpSending = false;
    }
  }

  async function handleResendOtp() {
    otp         = '';
    otpError    = '';
    otpVerified = false;
    await handleSendOtp();
  }

  // Auto-verify the moment the user finishes typing 6 digits
  $: if (otp.length === 6 && otpSent && !otpVerified) {
    checkOtp(otp);
  }

  // Reset verified state if user edits the code after it was verified
  $: if (otp.length < 6) {
    otpVerified = false;
    otpError    = '';
  }

  async function checkOtp(code) {
    otpChecking = true;
    otpError    = '';
    try {
      await api.auth.verifyOtp(email.trim(), code.trim());
      otpVerified = true;
    } catch (e) {
      otpVerified = false;
      otpError    = e.message || 'Incorrect OTP';
    } finally {
      otpChecking = false;
    }
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

    if (!otpSent || !otp.trim()) {
      error = 'Please verify your email with the OTP before registering';
      return;
    }

    try {
      const payload = {
        full_name: fullName,
        username,
        email,
        password,
        mode,
        otp: otp.trim(),
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
              class:field-available={usernameStatus === 'available'}
              class:field-taken={usernameStatus === 'taken'}
              placeholder="johndoe"
              bind:value={username}
              on:input={onUsernameInput}
              on:keydown={handleKeydown}
            />
            {#if usernameStatus === 'checking'}
              <span class="field-status-icon"><span class="mini-spinner"></span></span>
            {:else if usernameStatus === 'available'}
              <span class="field-status-icon status-ok">✓</span>
            {:else if usernameStatus === 'taken'}
              <span class="field-status-icon status-err">✗</span>
            {/if}
          </div>
        </div>

        <div class="input-group">
          <label class="input-label" for="email">Email Address</label>
          <div class="email-otp-row">
            <div class="input-wrapper" style="flex: 1;">
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
                class:verified={otpSent}
                class:field-taken={emailStatus === 'taken'}
                placeholder="john@example.com"
                bind:value={email}
                on:input={onEmailInput}
                on:keydown={handleKeydown}
                disabled={otpSent}
              />
            </div>
            {#if otpSent}
              <button type="button" class="otp-resend-btn" on:click={handleResendOtp} disabled={otpSending}>
                Resend
              </button>
            {:else}
              <button type="button" class="otp-send-btn" on:click={handleSendOtp} disabled={otpSending || !serverConnected || emailStatus === 'taken' || emailStatus === 'checking'}>
                {#if otpSending}
                  <span class="mini-spinner"></span>
                {:else}
                  Send OTP
                {/if}
              </button>
            {/if}
          </div>

          {#if emailStatus === 'taken'}
            <p class="field-taken-msg">This email is already registered. <a href="#/" on:click|preventDefault={() => navigate('/')}>Log in instead?</a></p>
          {:else if emailStatus === 'available'}
            <p class="field-ok-msg">✓ Email is available</p>
          {/if}

          {#if otpError}
            <p class="otp-error">{otpError}</p>
          {/if}

          {#if otpSent}
            <!-- OTP input appears here, directly under the email row -->
            <div class="otp-input-group">
              <label class="input-label" for="otp">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="vertical-align: middle; margin-right: 4px;">
                  <rect x="3" y="11" width="18" height="11" rx="2"/>
                  <path d="M7 11V7a5 5 0 0110 0v4"/>
                </svg>
                Enter the 6-digit OTP sent to your email
              </label>
              <div class="otp-input-wrapper">
                <input
                  type="text"
                  id="otp"
                  class="otp-field"
                  class:otp-valid={otpVerified}
                  class:otp-invalid={otpError && otp.length === 6}
                  placeholder="_ _ _ _ _ _"
                  maxlength="6"
                  bind:value={otp}
                  on:keydown={handleKeydown}
                  autocomplete="one-time-code"
                />
                {#if otpChecking}
                  <span class="otp-status-icon checking">
                    <span class="mini-spinner"></span>
                  </span>
                {:else if otpVerified}
                  <span class="otp-status-icon valid">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                      <path d="M20 6L9 17l-5-5"/>
                    </svg>
                  </span>
                {:else if otpError && otp.length === 6}
                  <span class="otp-status-icon invalid">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                      <path d="M18 6L6 18M6 6l12 12"/>
                    </svg>
                  </span>
                {/if}
              </div>

              {#if otpVerified}
                <p class="otp-ready">
                  <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                    <path d="M20 6L9 17l-5-5"/>
                  </svg>
                  Email verified — you can now submit
                </p>
              {:else if otpError && otp.length === 6}
                <p class="otp-wrong">{otpError}</p>
              {/if}
            </div>
          {:else}
            <p class="field-hint">A 6-digit code will be sent to verify your email.</p>
          {/if}
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

        <button
          type="submit"
          class="login-btn"
          disabled={loading || !serverConnected || !otpVerified}
        >
          {#if loading}
            <span class="spinner"></span>
            {mode === 'join' ? 'Submitting…' : 'Creating…'}
          {:else if !serverConnected}
            Server Unavailable
          {:else if !otpSent}
            Verify Email First
          {:else if !otpVerified}
            Enter OTP to Continue
          {:else if mode === 'join'}
            Request to Join
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
          {:else}
            Create Organization
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
          {/if}
        </button>

        <div class="register-link">
          Already have an account?
          <a href="#/" on:click|preventDefault={() => navigate('/')}>Log in</a>
        </div>
      </form>
    {/if}
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
    max-width: 500px;
    padding: 40px 20px;
    position: relative;
    z-index: 1;
  }

  .login-header {
    text-align: center;
    margin-bottom: 30px;
  }

  .logo {
    display: inline-flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 20px;
  }

  .logo-icon { display: flex; }

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
    min-height: 1.4em;
  }

  /* ── Mode Toggle ─────────────────────────────────────────────────────────── */
  .mode-toggle {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
    margin-bottom: 24px;
  }

  .mode-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 12px 10px;
    background: var(--bg-secondary);
    border: 2px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .mode-btn:hover {
    border-color: var(--accent-primary);
    color: var(--text-primary);
  }

  .mode-btn.active {
    background: rgba(99, 102, 241, 0.12);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  /* ── Form ────────────────────────────────────────────────────────────────── */
  .login-form {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-xl);
    padding: 32px;
    backdrop-filter: blur(20px);
  }

  .input-group {
    margin-bottom: 20px;
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
    pointer-events: none;
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
    box-sizing: border-box;
  }

  .input-field:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
  }

  .input-field::placeholder { color: var(--text-muted); }

  .select-field {
    appearance: none;
    cursor: pointer;
  }

  .field-hint {
    margin-top: 6px;
    font-size: 0.78rem;
    color: var(--text-muted);
    font-style: italic;
  }

  .field-available { border-color: #22c55e !important; }
  .field-taken     { border-color: var(--accent-danger) !important; }

  .field-status-icon {
    position: absolute;
    right: 14px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 0.9rem;
    display: flex;
    align-items: center;
  }

  .status-ok  { color: #22c55e; }
  .status-err { color: var(--accent-danger); }

  .field-taken-msg {
    margin-top: 5px;
    font-size: 0.8rem;
    color: var(--accent-danger);
  }

  .field-taken-msg a { color: var(--accent-danger); font-weight: 600; }

  .field-ok-msg {
    margin-top: 5px;
    font-size: 0.8rem;
    color: #22c55e;
  }

  /* ── OTP email row ───────────────────────────────────────────────────────── */
  .email-otp-row {
    display: flex;
    gap: 8px;
    align-items: stretch;
  }

  .otp-send-btn,
  .otp-resend-btn {
    flex-shrink: 0;
    padding: 0 18px;
    border-radius: var(--radius-md);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 6px;
    white-space: nowrap;
  }

  .otp-send-btn {
    background: linear-gradient(135deg, var(--accent-primary), #8b5cf6);
    color: white;
    box-shadow: 0 2px 10px rgba(99, 102, 241, 0.3);
  }

  .otp-send-btn:hover:not(:disabled) {
    box-shadow: 0 4px 16px rgba(99, 102, 241, 0.4);
    transform: translateY(-1px);
  }

  .otp-send-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .otp-resend-btn {
    background: var(--bg-secondary);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
  }

  .otp-resend-btn:hover:not(:disabled) {
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .input-field.verified {
    border-color: rgba(34, 197, 94, 0.5);
    background: rgba(34, 197, 94, 0.05);
    color: var(--text-muted);
  }

  /* ── OTP input block ──────────────────────────────────────────────────────── */
  .otp-input-group {
    margin-top: 12px;
    padding: 14px;
    background: rgba(99, 102, 241, 0.06);
    border: 1px solid rgba(99, 102, 241, 0.2);
    border-radius: var(--radius-md);
  }

  .otp-input-group .input-label {
    margin-bottom: 8px;
    color: var(--accent-primary);
    font-size: 0.8rem;
  }

  .otp-field {
    width: 100%;
    padding: 14px 16px;
    background: var(--bg-secondary);
    border: 1px solid rgba(99, 102, 241, 0.3);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: 1.4rem;
    font-weight: 700;
    letter-spacing: 10px;
    text-align: center;
    transition: all var(--transition-base);
    box-sizing: border-box;
  }

  .otp-field:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
  }

  .otp-field::placeholder {
    color: var(--text-muted);
    letter-spacing: 8px;
    font-size: 1rem;
    font-weight: 400;
  }

  .otp-input-wrapper {
    position: relative;
  }

  .otp-status-icon {
    position: absolute;
    right: 14px;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
  }

  .otp-status-icon.valid  { color: #22c55e; }
  .otp-status-icon.invalid { color: var(--accent-danger); }
  .otp-status-icon.checking { color: var(--text-muted); }

  .otp-field.otp-valid {
    border-color: #22c55e;
    box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.15);
  }

  .otp-field.otp-invalid {
    border-color: var(--accent-danger);
    box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.15);
  }

  .otp-ready {
    margin-top: 8px;
    font-size: 0.78rem;
    color: #22c55e;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .otp-wrong {
    margin-top: 6px;
    font-size: 0.82rem;
    color: var(--accent-danger);
  }

  .otp-error {
    margin-top: 6px;
    font-size: 0.82rem;
    color: var(--accent-danger);
  }

  /* ── Org loading / empty states ──────────────────────────────────────────── */
  .orgs-loading,
  .orgs-empty {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 14px 16px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  .mini-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid var(--border-color);
    border-top-color: var(--accent-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
  }

  /* ── Error ────────────────────────────────────────────────────────────────── */
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

  /* ── Submit button ────────────────────────────────────────────────────────── */
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
    margin-top: 10px;
  }

  .login-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 30px rgba(99, 102, 241, 0.4);
  }

  .login-btn:active:not(:disabled) { transform: translateY(0); }

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

  @keyframes spin { to { transform: rotate(360deg); } }

  /* ── Back to login link ───────────────────────────────────────────────────── */
  .register-link {
    margin-top: 24px;
    text-align: center;
    font-size: 0.95rem;
    color: var(--text-secondary);
  }

  .register-link a {
    color: var(--accent-primary);
    text-decoration: none;
    font-weight: 600;
    transition: color 0.2s ease;
  }

  .register-link a:hover { color: var(--accent-secondary); }

  /* ── Success card (join pending screen) ───────────────────────────────────── */
  .success-card {
    background: var(--bg-card);
    border: 1px solid rgba(34, 197, 94, 0.3);
    border-radius: var(--radius-xl);
    padding: 40px 32px;
    text-align: center;
    backdrop-filter: blur(20px);
  }

  .success-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: rgba(34, 197, 94, 0.12);
    color: #22c55e;
    margin-bottom: 20px;
  }

  .success-card h2 {
    font-family: var(--font-display);
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 12px;
  }

  .success-card p {
    color: var(--text-secondary);
    font-size: 0.95rem;
    line-height: 1.6;
    margin-bottom: 28px;
  }
</style>
