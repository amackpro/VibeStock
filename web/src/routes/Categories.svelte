<script>
  import { onMount } from 'svelte';
  import { gsap } from 'gsap';
  import { api } from '../lib/api.js';
  import { toastStore } from '../stores/toast.js';

  let categories = [];
  let loading = true;
  let showModal = false;
  let editingCategory = null;
  
  let form = {
    name: '',
    description: ''
  };

  onMount(async () => {
    await loadCategories();
    animateEntrance();
  });

  async function loadCategories() {
    loading = true;
    try {
      const result = await api.categories.list();
      categories = result.data || result;
    } catch (e) {
      toastStore.show('Failed to load categories', 'error');
    } finally {
      loading = false;
    }
  }

  function animateEntrance() {
    const tl = gsap.timeline();
    tl.fromTo('.page-header', 
      { y: -20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.5, ease: 'power3.out' }
    );
    tl.fromTo('.category-card', 
      { y: 30, opacity: 0, scale: 0.95 },
      { y: 0, opacity: 1, scale: 1, duration: 0.4, stagger: 0.08, ease: 'back.out(1.5)' },
      '-=0.3'
    );
  }

  function openModal(category = null) {
    editingCategory = category;
    if (category) {
      form = { 
        name: category.name || '',
        description: category.description || ''
      };
    } else {
      form = { name: '', description: '' };
    }
    showModal = true;
    setTimeout(() => {
      gsap.fromTo('.modal-content', 
        { scale: 0.9, opacity: 0, y: 20 },
        { scale: 1, opacity: 1, y: 0, duration: 0.3, ease: 'back.out(1.5)' }
      );
    }, 0);
  }

  function closeModal() {
    gsap.to('.modal-content', {
      scale: 0.9,
      opacity: 0,
      duration: 0.2,
      onComplete: () => {
        showModal = false;
        editingCategory = null;
      }
    });
  }

  async function saveCategory() {
    try {
      if (editingCategory) {
        await api.categories.update(editingCategory.id, form);
        toastStore.show('Category updated successfully', 'success');
      } else {
        await api.categories.create(form);
        toastStore.show('Category created successfully', 'success');
      }
      await loadCategories();
      closeModal();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }

  async function deleteCategory(id) {
    if (!confirm('Are you sure you want to delete this category? Products in this category will remain, but the category association will be lost.')) return;
    
    try {
      await api.categories.delete(id);
      toastStore.show('Category deleted', 'success');
      await loadCategories();
    } catch (e) {
      toastStore.show(e.message, 'error');
    }
  }
</script>

<div class="categories-page">
  <div class="page-header">
    <div class="header-left">
      <h2 class="page-title">Category Management</h2>
      <p class="page-subtitle">Organize your products with custom categories</p>
    </div>
    <div class="header-right">
      <button class="btn btn-primary" on:click={() => openModal()}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 5v14M5 12h14"/>
        </svg>
        Add Category
      </button>
    </div>
  </div>

  <div class="categories-grid">
    {#if loading}
      <div class="loading-state">
        <div class="spinner"></div>
        <span>Loading categories...</span>
      </div>
    {:else if categories.length === 0}
      <div class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4a2 2 0 00-1 1.73v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4a2 2 0 001-1.73z"/>
        </svg>
        <span>No categories found. Start by adding one!</span>
      </div>
    {:else}
      {#each categories as cat, i (cat.id)}
        <div class="category-card" style="animation-delay: {i * 0.05}s">
          <div class="card-header">
            <div class="category-icon">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/>
              </svg>
            </div>
            <div class="card-actions">
              <button class="icon-btn" on:click={() => openModal(cat)}>
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                  <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
                </svg>
              </button>
              <button class="icon-btn danger" on:click={() => deleteCategory(cat.id)}>
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
                </svg>
              </button>
            </div>
          </div>
          <div class="card-body">
            <h3 class="category-name">{cat.name}</h3>
            <p class="category-desc">{cat.description || 'No description provided.'}</p>
          </div>
          <div class="card-footer">
            <span class="product-count">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/>
              </svg>
              Inventory Group
            </span>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

{#if showModal}
  <div class="modal-overlay" on:click={closeModal}>
    <div class="modal-content" on:click|stopPropagation>
      <div class="modal-header">
        <h2 class="modal-title">{editingCategory ? 'Edit Category' : 'Add Category'}</h2>
        <button class="modal-close" on:click={closeModal}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>

      <form on:submit|preventDefault={saveCategory}>
        <div class="form-group">
          <label class="form-label">Category Name</label>
          <input type="text" class="input-field" bind:value={form.name} required placeholder="e.g. Electronics, Furniture" />
        </div>

        <div class="form-group">
          <label class="form-label">Description</label>
          <textarea class="input-field" bind:value={form.description} rows="4" placeholder="Brief description of products in this category..."></textarea>
        </div>

        <div class="modal-actions">
          <button type="button" class="btn btn-secondary" on:click={closeModal}>Cancel</button>
          <button type="submit" class="btn btn-primary">
            {editingCategory ? 'Update Category' : 'Create Category'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .categories-page {
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .page-header {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 1.5rem;
    font-weight: 700;
    margin-bottom: 4px;
    background: linear-gradient(135deg, var(--text-primary), var(--text-muted));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .page-subtitle {
    color: var(--text-muted);
    font-size: 0.95rem;
  }

  .categories-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 24px;
  }

  .category-card {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-xl);
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    transition: all var(--transition-base);
    backdrop-filter: blur(10px);
    position: relative;
    overflow: hidden;
  }

  .category-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 4px;
    background: linear-gradient(90deg, var(--accent-primary), var(--accent-secondary));
    opacity: 0.6;
  }

  .category-card:hover {
    transform: translateY(-4px);
    border-color: var(--border-glow);
    box-shadow: var(--shadow-glow);
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .category-icon {
    width: 44px;
    height: 44px;
    background: rgba(99, 102, 241, 0.1);
    color: var(--accent-primary);
    border-radius: var(--radius-lg);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .card-actions {
    display: flex;
    gap: 8px;
  }

  .icon-btn {
    padding: 8px;
    border-radius: var(--radius-md);
    color: var(--text-muted);
    transition: all var(--transition-fast);
  }

  .icon-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .icon-btn.danger:hover {
    background: rgba(239, 68, 68, 0.1);
    color: var(--accent-danger);
  }

  .category-name {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 8px;
  }

  .category-desc {
    font-size: 0.9rem;
    color: var(--text-muted);
    line-height: 1.5;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .card-footer {
    margin-top: auto;
    padding-top: 16px;
    border-top: 1px solid var(--border-color);
  }

  .product-count {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .loading-state, .empty-state {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 80px 20px;
    color: var(--text-muted);
    background: var(--bg-card);
    border: 1px dashed var(--border-color);
    border-radius: var(--radius-xl);
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(6px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 20px;
  }

  .modal-content {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-2xl);
    padding: 32px;
    width: 100%;
    max-width: 500px;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
  }

  .modal-title {
    font-size: 1.4rem;
    font-weight: 700;
  }

  .modal-close {
    padding: 8px;
    border-radius: var(--radius-md);
    color: var(--text-muted);
  }

  .form-group {
    margin-bottom: 20px;
  }

  .form-label {
    display: block;
    margin-bottom: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .input-field {
    width: 100%;
    padding: 12px 16px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    color: var(--text-primary);
    font-size: 1rem;
    transition: all 0.2s ease;
  }

  .input-field:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 4px rgba(99, 102, 241, 0.15);
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 32px;
  }

  .btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 20px;
    border-radius: var(--radius-lg);
    font-weight: 600;
    transition: all 0.2s ease;
    cursor: pointer;
  }

  .btn-primary {
    background: var(--accent-primary);
    color: white;
  }

  .btn-primary:hover {
    background: var(--accent-secondary);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
  }

  .btn-secondary {
    background: var(--bg-tertiary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .btn-secondary:hover {
    background: var(--bg-secondary);
  }

  textarea.input-field {
    resize: vertical;
    min-height: 100px;
  }
</style>
