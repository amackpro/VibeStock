<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { api } from '../lib/api.js';
  import { toastStore } from '../stores/toast.js';

  let movements = [];
  let products = [];
  let loading = true;
  let showModal = false;
  
  let form = {
    product_id: '',
    movement_type: 'in',
    quantity: 1,
    notes: ''
  };

  onMount(async () => {
    await Promise.all([loadMovements(), loadProducts()]);
    animateEntrance();
  });

  function animateEntrance() {
    const tl = gsap.timeline();

    tl.fromTo('.page-header', 
      { y: -20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' }
    );

    tl.fromTo('.table-container', 
      { y: 30, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' },
      '-=0.3'
    );
  }

  async function loadMovements() {
    try {
      const result = await api.movements.list();
      movements = result.data || result;
    } catch (e) {
      toastStore.show('Failed to load movements', 'error');
    } finally {
      loading = false;
    }
  }

  async function loadProducts() {
    try {
      products = await api.products.list();
    } catch (e) {
      console.error('Failed to load products', e);
    }
  }

  function openModal() {
    form = {
      product_id: '',
      movement_type: 'IN',
      quantity: 1,
      notes: ''
    };
    
    gsap.fromTo('.modal-content', 
      { scale: 0.9, opacity: 0, y: 20 },
      { scale: 1, opacity: 1, y: 0, duration: 0.3, ease: 'back.out(1.5)' }
    );
    
    showModal = true;
  }

  function closeModal() {
    gsap.to('.modal-content', {
      scale: 0.9,
      opacity: 0,
      duration: 0.2,
      onComplete: () => {
        showModal = false;
      }
    });
  }

  async function createMovement() {
    try {
      await api.movements.create(form);
      toastStore.show('Movement recorded successfully', 'success');
      await loadMovements();
      closeModal();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }

  function formatDate(dateStr) {
    if (!dateStr) return '-';
    return new Date(dateStr).toLocaleDateString('en-IN', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
</script>

<div class="movements-page">
  <div class="page-header">
    <h2 class="page-title">Stock Movements</h2>
    <button class="btn btn-primary" on:click={openModal}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M12 5v14M5 12h14"/>
      </svg>
      New Movement
    </button>
  </div>

  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th>Date</th>
          <th>Product</th>
          <th>Type</th>
          <th>Quantity</th>
          <th>Notes</th>
          <th>User</th>
        </tr>
      </thead>
      <tbody>
        {#if loading}
          <tr>
            <td colspan="6" class="loading-cell">
              <div class="spinner"></div>
              Loading movements...
            </td>
          </tr>
        {:else if movements.length === 0}
          <tr>
            <td colspan="6" class="empty-cell">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M12 2v20M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6"/>
              </svg>
              <span>No movements recorded</span>
            </td>
          </tr>
        {:else}
          {#each movements as movement, i (movement.id)}
            <tr class="movement-row" style="animation-delay: {i * 0.03}s">
              <td class="date-cell">
                <span class="date">{formatDate(movement.created_at)}</span>
              </td>
              <td>
                <div class="product-cell">
                  <span class="product-name">{movement.product_name || 'Unknown'}</span>
                  <span class="product-sku">{movement.product_sku || ''}</span>
                </div>
              </td>
              <td>
                <span class="type-badge type-{movement.movement_type.toLowerCase()}">
                  {#if movement.movement_type === 'in'}
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                      <path d="M12 19V5M5 12l7-7 7 7"/>
                    </svg>
                  {:else if movement.movement_type === 'out'}
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                      <path d="M12 5v14M5 12l7 7 7-7"/>
                    </svg>
                  {:else}
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                      <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                      <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
                    </svg>
                  {/if}
                  {movement.movement_type}
                </span>
              </td>
              <td>
                <span class="quantity" class:positive={movement.movement_type === 'in'} class:negative={movement.movement_type === 'out'}>
                  {movement.movement_type === 'in' ? '+' : movement.movement_type === 'out' ? '-' : ''}{movement.quantity}
                </span>
              </td>
              <td>
                <span class="notes">{movement.notes || '-'}</span>
              </td>
              <td>
                <span class="user">{movement.performed_by_name || '-'}</span>
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>

{#if showModal}
  <div class="modal-overlay" on:click={closeModal}>
    <div class="modal-content" on:click|stopPropagation>
      <div class="modal-header">
        <h2 class="modal-title">Record Movement</h2>
        <button class="modal-close" on:click={closeModal}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>

      <form on:submit|preventDefault={createMovement}>
        <div class="form-group">
          <label class="form-label">Product</label>
          <select class="input-field" bind:value={form.product_id} required>
            <option value="">Select product</option>
            {#each products as prod}
              <option value={prod.id}>{prod.name} ({prod.sku})</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label class="form-label">Movement Type</label>
          <div class="type-selector">
            <button type="button" class="type-btn" class:active={form.movement_type === 'in'} on:click={() => form.movement_type = 'in'}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 19V5M5 12l7-7 7 7"/>
              </svg>
              Stock In
            </button>
            <button type="button" class="type-btn" class:active={form.movement_type === 'out'} on:click={() => form.movement_type = 'out'}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 5v14M5 12l7 7 7-7"/>
              </svg>
              Stock Out
            </button>
            <button type="button" class="type-btn" class:active={form.movement_type === 'adjustment'} on:click={() => form.movement_type = 'adjustment'}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
              </svg>
              Adjustment
            </button>
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">Quantity</label>
          <input type="number" class="input-field" bind:value={form.quantity} min="1" required />
        </div>

        <div class="form-group">
          <label class="form-label">Notes</label>
          <textarea class="input-field textarea" bind:value={form.notes} rows="3" placeholder="Reason for movement..."></textarea>
        </div>

        <div class="modal-actions">
          <button type="button" class="btn btn-secondary" on:click={closeModal}>Cancel</button>
          <button type="submit" class="btn btn-primary">Record Movement</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .movements-page {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 1.3rem;
    font-weight: 600;
  }

  .table-container {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  table {
    width: 100%;
  }

  th {
    padding: 16px 20px;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-muted);
    background: var(--bg-secondary);
  }

  td {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .loading-cell, .empty-cell {
    text-align: center;
    padding: 48px 20px;
    color: var(--text-muted);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }

  .date-cell .date {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .product-cell {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .product-cell .product-name {
    font-weight: 500;
    color: var(--text-primary);
  }

  .product-cell .product-sku {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .type-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 100px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .type-badge.type-in {
    background: rgba(16, 185, 129, 0.15);
    color: var(--accent-success);
  }

  .type-badge.type-out {
    background: rgba(239, 68, 68, 0.15);
    color: var(--accent-danger);
  }

  .type-badge.type-adjustment {
    background: rgba(99, 102, 241, 0.15);
    color: var(--accent-primary);
  }

  .quantity {
    font-weight: 600;
    font-size: 1rem;
  }

  .quantity.positive {
    color: var(--accent-success);
  }

  .quantity.negative {
    color: var(--accent-danger);
  }

  .notes {
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .user {
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  .movement-row {
    animation: slideIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    opacity: 0;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(-20px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 20px;
  }

  .modal-content {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-xl);
    padding: 32px;
    width: 100%;
    max-width: 500px;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 28px;
  }

  .modal-title {
    font-family: var(--font-display);
    font-size: 1.4rem;
    font-weight: 600;
  }

  .modal-close {
    padding: 8px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
  }

  .modal-close:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .form-group {
    margin-bottom: 20px;
  }

  .form-label {
    display: block;
    margin-bottom: 8px;
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .type-selector {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .type-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 16px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-weight: 500;
    transition: all var(--transition-base);
  }

  .type-btn:hover {
    border-color: var(--accent-primary);
  }

  .type-btn.active {
    background: rgba(99, 102, 241, 0.15);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .textarea {
    resize: vertical;
    min-height: 80px;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 28px;
  }
</style>
