<script>
  import { fade, fly } from 'svelte/transition';
  import { toastStore } from '../stores/toast.js';
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';

  let toasts = [];
  toastStore.subscribe(t => toasts = t);

  function dismiss(id) {
    toastStore.dismiss(id);
  }
</script>

<div class="toast-container">
  {#each toasts as toast (toast.id)}
    <div 
      class="toast toast-{toast.type}"
      in:fly={{ y: -20, duration: 300 }}
      out:fade={{ duration: 200 }}
    >
      <div class="toast-icon">
        {#if toast.type === 'success'}
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20 6L9 17l-5-5"/>
          </svg>
        {:else if toast.type === 'error'}
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M15 9l-6 6M9 9l6 6"/>
          </svg>
        {:else if toast.type === 'warning'}
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 9v4M12 17h.01"/>
            <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/>
          </svg>
        {:else}
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 16v-4M12 8h.01"/>
          </svg>
        {/if}
      </div>
      <span class="toast-message">{toast.message}</span>
      <button class="toast-close" on:click={() => dismiss(toast.id)}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M18 6L6 18M6 6l12 12"/>
        </svg>
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 24px;
    right: 24px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 12px;
    max-width: 380px;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 18px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg);
    backdrop-filter: blur(20px);
  }

  .toast-success {
    border-left: 3px solid var(--accent-success);
  }

  .toast-success .toast-icon {
    color: var(--accent-success);
  }

  .toast-error {
    border-left: 3px solid var(--accent-danger);
  }

  .toast-error .toast-icon {
    color: var(--accent-danger);
  }

  .toast-warning {
    border-left: 3px solid var(--accent-warning);
  }

  .toast-warning .toast-icon {
    color: var(--accent-warning);
  }

  .toast-info {
    border-left: 3px solid var(--accent-secondary);
  }

  .toast-info .toast-icon {
    color: var(--accent-secondary);
  }

  .toast-icon {
    flex-shrink: 0;
    display: flex;
  }

  .toast-message {
    flex: 1;
    font-size: 0.9rem;
    color: var(--text-primary);
  }

  .toast-close {
    flex-shrink: 0;
    color: var(--text-muted);
    transition: color var(--transition-fast);
  }

  .toast-close:hover {
    color: var(--text-primary);
  }
</style>
